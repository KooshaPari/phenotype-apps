# ADR-036: focus-events meta-repo artifact (KEEP-archived)

**Date:** 2026-06-18
**Status:** ACCEPTED (per fleet zero-HITL governance + Track 8 bot self-merge)
**Deciders:** @KooshaPari (sole owner), fleet audit circle
**Supersedes:** none
**Superseded by:** none
**Related:** ADR-002 (KlipDot KEEP-archived governance pattern), ADR-023 (app-level repo triage), ADR-035 (event-bus substrate consolidation)

> **Numbering note:** This file uses the `ADR-036-` prefix as specified in the authoring task. Two pre-existing files in `docs/adr/2026-06-18/` already carry the `ADR-036-` prefix (`ADR-036-pheno-capacity.md`, `ADR-036-pheno-tracing-substrate-canonical.md`), and `INDEX.md` in that directory maps ADR-036 to the pheno-tracing substrate. The authoring conflict is flagged in the PR body for the owner to resolve; the file is written at the exact path the task requested. If a renumber is required post-merge, the recommended next available slot is **ADR-047** (the highest pre-existing number in the directory is ADR-046).

---

## Context

The R4 finding from the 2026-06-18 event-bus fleet audit identified a **byte-identical duplicate** of the `focus-events` crate at two paths:

| Path | Workspace root | Status |
|---|---|---|
| `repos/crates/focus-events/` | `repos/` (root meta-monorepo) | **Canonical** — most recent git activity, root `Cargo.toml` member |
| `FocalPoint/crates/focus-events/` | `FocalPoint/` (separate git submodule) | **Duplicate** — byte-identical (md5sum-verified), also a `FocalPoint/Cargo.toml` member |

Verified on 2026-06-18 (this turn):

- `md5 -r` of both `Cargo.toml` files returns `c6243e81ced18bea87ceabe8f941bae8` (identical)
- `diff -rq repos/crates/focus-events/ FocalPoint/crates/focus-events/` returns no output (no diff)
- Both copies contain the same 3 source files (`Cargo.toml`, `README.md`, `src/lib.rs`, `src/dedup.rs`; total ~601 LOC source: `lib.rs` 298 lines, `dedup.rs` 303 lines)
- Both compile independently in their respective workspace roots

The 6-repo audit recommended a "DEDUPE" action: pick one and remove the other. A subagent attempted to do this on 2026-06-18 (L5-113).

## The structural blocker

The dedupe subagent exhausted **5 workarounds** and reverted all changes. The blocker is a Cargo resolution collision that cannot be resolved without violating the "do not modify the canonical" constraint:

### Root cause

The canonical `repos/crates/focus-events/Cargo.toml:9,14` has hard-coded:

```toml
focus-domain = { path = "../focus-domain" }   # line 9
focus-errors = { path = "../focus-errors" }   # line 14
```

These resolve to `repos/crates/focus-domain` and `repos/crates/focus-errors` — both members of the **root** `repos/` workspace.

When FocalPoint (a **separate** git submodule with its own `Cargo.toml` workspace) tries to consume the canonical via `path = "../../../crates/focus-events"`, Cargo's lockfile resolver sees:

- `focus-domain v0.0.12` at `repos/crates/focus-domain` (transitive, from the canonical)
- `focus-domain v0.0.12` at `repos/FocalPoint/crates/focus-domain` (FocalPoint's own workspace member — byte-identical, but in a different workspace)

Same name + same version + two different paths = **lockfile collision**. Cargo cannot pick.

### Workarounds attempted and rejected (final tally)

| Workaround | Result |
|---|---|
| **Path-dep to canonical** | FAILS — lockfile collision |
| **Published version on crates.io** | N/A — `focus-events` is NOT on crates.io |
| **Modify canonical's `Cargo.toml`** | FORBIDDEN by task ("DO NOT: Modify the canonical `repos/crates/focus-events/`") |
| **`[patch.crates-io]`** with `path = "..."` | REJECTED — `[patch.crates-io]` only overrides crates.io-sourced deps |
| **Unkeyed `[patch]`** with `path = "..."` | REJECTED — `error: [patch] entry should be a URL or registry name` |
| **`[patch."file://..."]`** with file:// URL | FAILS — path-deps don't normalize to `file://` URL key |
| **Delete `Cargo.lock`** and rebuild | FAILS — collision is in **resolution**, not stale-lockfile |
| **FocalPoint transitive dep repointing** | OUT OF SCOPE — would require fleet-wide refactor |

## Decision

We adopt a **KEEP-archived** disposition (per ADR-002 governance pattern):

1. **Both copies remain** as-is, on-disk, in their respective workspace roots.
2. **The duplication is acknowledged** as a meta-repo artifact: the two copies are byte-identical, so any divergence is unintentional and would be caught by a future md5sum check.
3. **The canonical is `repos/crates/focus-events/`** for all future work; the FocalPoint copy is a passive mirror.
4. **The long-term solution is publishing to crates.io** (see Phase 3 below).

## Rationale

- **Lowest cost, highest clarity**: 30 min of work to document vs hours of risky cross-workspace refactoring.
- **Pattern reuse**: the same KEEP-archived governance pattern was applied to `KlipDot` (ADR-002) and `phenotype-monorepo-state` (ADR-033/034). The decision is consistent with prior fleet governance.
- **Risk mitigation**: any future change to the canonical should be re-evaluated (a future PR could potentially fix the lockfile collision by converting path-deps to workspace deps, at which point this ADR can be superseded).
- **The duplication is structurally a no-op**: both packages work, both compile, both are byte-identical. There is no runtime cost.

## Migration plan

### Phase 1 (DONE 2026-06-18)
- [x] Verified byte-identical (md5sum `c6243e81ced18bea87ceabe8f941bae8` on `Cargo.toml`; `diff -rq` returns empty)
- [x] Subagent attempted 5 workarounds; all failed
- [x] Reverted all changes
- [x] Wrote this ADR

### Phase 2 (FOLLOW-UP, 2026-06-19 or later)
- [ ] Add a CI check that fails the build if `diff -rq repos/crates/focus-events FocalPoint/crates/focus-events/` shows any divergence (per `pheno-ci-templates`)
- [ ] Update the `README.md` in both copies to cross-reference the other (so users find the canonical)
- [ ] Add `// SOURCE OF TRUTH: repos/crates/focus-events/` header comment in the FocalPoint copy

### Phase 3 (LONG-TERM, requires publish flow)
- [ ] Publish `focus-events` to crates.io
- [ ] Update `FocalPoint/Cargo.toml` to use `focus-events = "0.0.12"` (crates.io version)
- [ ] The crates.io dep resolves the lockfile collision because crates.io deps don't collide with workspace members
- [ ] Delete the FocalPoint copy

## Consequences

### Positive
- Documented, governance-grade disposition (not just a workaround)
- 30 min of work vs hours of risky refactoring
- Pattern reuse (ADR-002)
- Long-term path is clear (Phase 3)

### Negative
- ~601 LOC of source duplication remains on disk
- Tests run twice (once in each workspace)
- Future divergence between the two copies is a real risk (mitigated by Phase 2 CI check)

### Risks
- **R1 (low)**: Future PR modifies one copy but not the other, causing divergence. Mitigation: Phase 2 CI check.
- **R2 (low)**: Phase 3 publish flow is complex and might fail. Mitigation: defer indefinitely if low priority; the duplication is not blocking.
- **R3 (very low)**: The byte-identical state is currently true; if it diverges, the CI check fails fast.

## Acceptance criteria

This ADR is accepted when:
1. The 2026-06-18 R4 audit row is marked `KEEP-archived` in the deletion-justification matrix
2. The cross-reference in `repos/crates/focus-events/README.md` points to the FocalPoint copy (and vice versa)
3. The Phase 2 CI check is added to `pheno-ci-templates`
4. Phase 3 (publish to crates.io) is added to a future v9+ plan

## References

- Subagent report: L5-113 6-repo audit reclassification (5 attempts, all reverted)
- R4 audit row: `findings/2026-06-18-event-bus-fleet-audit.md` § 4
- ADR-002 (KlipDot KEEP-archived pattern)
- ADR-033/034 (phenotype-monorepo-state KEEP-archived pattern)
- ADR-035B (event-bus substrate consolidation)
- The 5 workarounds are documented in the subagent's final report (see `findings/2026-06-18-l5-113-r4-dedupe-attempt.md` if that file exists; otherwise see subagent session log)
- Verification on 2026-06-18 (this turn): `md5 -r repos/crates/focus-events/Cargo.toml FocalPoint/crates/focus-events/Cargo.toml` → both `c6243e81ced18bea87ceabe8f941bae8`; `diff -rq` returns no output
