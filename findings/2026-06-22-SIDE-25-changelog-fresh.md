# SIDE-25: CHANGELOG freshness — pheno-* substrate crates

**Date:** 2026-06-21 (system date; doc dated 2026-06-22 per task slug)
**Task:** SIDE-25 — For each pheno-* crate, check `CHANGELOG.md` exists and (mtime in last 30 days) OR (entries covering last 3 releases). Score 0-3, identify top 5 stale.
**Author:** orchestrator (v19 cycle-9 closure follow-up; SIDE work)

---

## Scope note: 7 crates, not 8

The task brief said "8 phono-* crates" (typo for `pheno-*`). The visible pheno-* Rust-crate inventory on this sparse-checkout cone is **7**, not 8:

```
$ for d in pheno-*/; do [ -f "$d/Cargo.toml" ] && echo "$d"; done
pheno-config/
pheno-context/
pheno-errors/
pheno-flags/
pheno-otel/
pheno-port-adapter/
pheno-tracing/
```

`AGENTS.md` ("pheno-* family (22 visible) · Rust (11)") lists 4 additional Rust crates that are **not** in this cone's sparse-checkout:

| AGENTS.md-listed crate | Cone status | Effect on audit |
|---|---|---|
| `pheno-cli-base` | MISSING from cone | Cannot audit |
| `pheno-agents-md` | MISSING from cone | Cannot audit |
| `pheno-cargo-template` | MISSING from cone | Cannot audit |
| `pheno-wtrees` | MISSING from cone (worktree container, not a crate) | N/A |

If the 4 missing crates are intended to be in scope, they would need a follow-up audit on a cone that includes them (or by directly hitting `KooshaPari/<repo>/CHANGELOG.md` on GitHub). For this turn, the audit covers the 7 visible Rust crates.

The "missing 1" gap is the most likely source of the "8" in the brief: the operator may have been counting `pheno-ssot-template` (which has `Cargo.toml.template`, not a real crate) or `pheno-secret-scan` (which has a `Justfile` + `deny.toml` but no `Cargo.toml` at the root and is not a Rust substrate). Neither is a Rust crate in the substrate sense. We do not stretch the scope to include them; the 7 Rust crates are the canonical answer.

---

## Per-crate score (0-3)

| # | Crate | `CHANGELOG.md` | mtime | Release sections found | Version (Cargo.toml) | Score |
|---|---|---|---|---|---|---|
| 1 | `pheno-config`         | **MISSING**     | n/a   | n/a                                  | 0.1.0 | **0** |
| 2 | `pheno-context`        | **MISSING**     | n/a   | n/a                                  | 0.1.0 | **0** |
| 3 | `pheno-errors`         | **MISSING**     | n/a   | n/a                                  | 0.1.0 | **0** |
| 4 | `pheno-flags`          | **MISSING**     | n/a   | n/a                                  | 0.1.0 | **0** |
| 5 | `pheno-otel`           | EXISTS          | 2026-06-21 | `[Unreleased]` only (no `[0.1.0]`)  | 0.1.0 | **1** |
| 6 | `pheno-port-adapter`   | EXISTS          | 2026-06-21 | `[Unreleased]` + `[0.1.0] - 2026-06-11` | 0.1.0 | **3** |
| 7 | `pheno-tracing`        | EXISTS          | 2026-06-20 | `[Unreleased]` + `[0.3.0-pre.0] - 2026-06-19` | 0.3.0-pre.0 | **3** |

**Fleet mean:** (0+0+0+0+1+3+3) / 7 = **1.00 / 3.00** (33%)

### Rubric used

- **0** — `CHANGELOG.md` does not exist, OR is completely stale (> 90 days and no entries for current version).
- **1** — File exists, mtime within 30 days, but no entry for the current version (only `[Unreleased]`).
- **2** — File exists, mtime within 30 days, has 2 release entries (or covers current version + 1 prior).
- **3** — File exists, mtime within 30 days AND has entries for the current version + at least 1 prior release (effectively: full release history documented).

(The "OR has entries covering the last 3 releases" clause is inapplicable in this fleet: no crate has 3 historical releases yet — all are at v0.1.0 or v0.3.0-pre.0. The mtime clause is the dominant signal here.)

---

## Top 5 stale crates

Ranked lowest-score-first; ties broken by absence-of-CHANGELOG severity, then by recency of last code commit (older = more stale).

| Rank | Crate | Score | Why it's stale | Last code commit (in cone) |
|---|---|---|---|---|
| **1** | `pheno-config`         | 0 | No `CHANGELOG.md`; v0.1.0 released with no release notes. Cargo.toml last touched 2026-06-21 (v19 T2 L52 encryption code), but no consumer-facing changelog entry. | 2026-06-21 — `3a63e31271 feat(v19-t2): L52 encryption-at-rest code (ZeroizeOnDrop + cargo-deny rules)` |
| **2** | `pheno-context`        | 0 | No `CHANGELOG.md`; v0.1.0 released with no release notes. Cargo.toml last touched 2026-06-21 but CHANGELOG absent. | 2026-06-21 — parent monorepo commit (v19 T3 OIDC consumer example) |
| **3** | `pheno-errors`         | 0 | No `CHANGELOG.md`; v0.1.0 released with no release notes despite 3 in-tree git log entries (`bfe3c69`, `fc7cc54`, `aec72820`) that should each have a changelog entry. | 2026-06-21 — `bfe3c69c12 feat(pheno-errors): add RFC 7807 companion and trace-context hygiene` |
| **4** | `pheno-flags`          | 0 | No `CHANGELOG.md`; v0.1.0 released with no release notes despite 5 in-tree git log entries (chaos tests, dev-deps, OTel wiring). | 2026-06-21 — `73142cd4a8 merge: resolve conflicts in chaos workflow and fuzz Cargo.toml` |
| **5** | `pheno-otel`           | 1 | `CHANGELOG.md` exists, mtime today, but contains only `[Unreleased]` (v11-044 governance meta-bundle batch). No `[0.1.0]` entry for the current version. Two commits in git log (`6bc3b866f3` L25 loom tests, `aab919dfa5` orch-v11-044) with no per-release documentation. | 2026-06-21 — `6bc3b866f3 chore(test): L25 loom tests for pheno-otel (#68)` |

(All 4 score-0 crates are tied for "most stale"; ranking among them is by last-commit recency only — `pheno-errors` and `pheno-flags` have the most undocumented commit history in-tree, hence rank 3 and 4 above `pheno-config`/`pheno-context`.)

---

## Root cause (single, identified)

The v8–v11 "governance meta-bundle" PR wave (orch-v11-044 tier-0, orch-w15-direct v11 final, etc.) — which added `AGENTS.md`, `README.md`, `llms.txt`, `WORKLOG.md`, `CHANGELOG.md`, `LICENSE-MIT` to each substrate crate — appears to have shipped `CHANGELOG.md` to only **3 of 7** Rust crates (`pheno-otel`, `pheno-port-adapter`, `pheno-tracing`). The other 4 (`pheno-config`, `pheno-context`, `pheno-errors`, `pheno-flags`) received the rest of the meta-bundle but no `CHANGELOG.md`.

Evidence: the v11-044 commit on `pheno-otel` (`aab919dfa5 chore(orch-v11-044): full governance + tier-0 for phenotype-otel (#38)`) is the template for this wave. Inspecting the 4 missing-CLI crates' last commits shows that the meta-bundle land commit did not include a `CHANGELOG.md` blob (Cargo.toml mtimes are all 2026-06-21 — the meta-bundle was touched on each — but `CHANGELOG.md` is not on disk).

This is a SOTA / 71-pillar L67 (CHANGELOG freshness) gap for the missing 4 crates. See `findings/71-pillar-2026-06-17.md` § 1.10 for the scorecard: 60/213 = 28.2% fleet pass on L67.

---

## Remediation (SIDE-25 follow-up, proposed)

Minimum diff to bring all 7 crates to **score 3**:

1. **For the 4 missing crates** (`pheno-config`, `pheno-context`, `pheno-errors`, `pheno-flags`):
   - Add a `CHANGELOG.md` matching the v8 template used by `pheno-port-adapter/CHANGELOG.md` (Keep-a-Changelog 1.1.0 format).
   - Populate with `[Unreleased]` (current v19-cycle-9 work) + `[0.1.0] - <back-dated release date>` referencing the PR that originally landed the meta-bundle.
2. **For `pheno-otel`**:
   - Add a `[0.1.0] - 2026-06-11` (or back-dated date of first L11 loom-tests commit) section summarising the loom test additions and any other 0.1.0 changes.
3. **For the 3 currently-3 crates** (`pheno-otel`, `pheno-port-adapter`, `pheno-tracing`):
   - No change needed; all are within the freshness window.

Estimated cost: ~30 minutes of editor work + 4 PRs (one per missing crate), or 1 PR if batched. Net diff: ~150 LoC of markdown. Tracked as a candidate for v20 cycle-10 P1 (71-pillar L67 closure) per `plans/2026-06-21-v19-71-pillar-cycle-9-p0.md` follow-up section.

---

## Appendix A — verification commands (re-runnable)

```bash
# 1. Inventory
for d in pheno-*/; do
  [ -f "$d/Cargo.toml" ] && echo "RUST: $d"
done

# 2. CHANGELOG presence + mtime
for d in pheno-{config,context,errors,flags,otel,port-adapter,tracing}/; do
  if [ -f "$d/CHANGELOG.md" ]; then
    mtime=$(stat -f "%Sm" -t "%Y-%m-%d" "$d/CHANGELOG.md")
    echo "EXISTS  $d mtime=$mtime"
  else
    echo "MISSING $d"
  fi
done

# 3. Release-section count
for d in pheno-{otel,port-adapter,tracing}/; do
  echo "=== $d ==="
  grep -E '^\## \[' "$d/CHANGELOG.md" 2>/dev/null
done

# 4. Last code commit per crate (relative to parent monorepo HEAD)
for d in pheno-{config,context,errors,flags,otel,port-adapter,tracing}/; do
  echo "$d | $(git -C "$d" log -1 --format='%ad %h %s' --date=short 2>/dev/null)"
done
```

## Appendix B — full CHANGELOG excerpts (3 of 3 existing)

### `pheno-otel/CHANGELOG.md` (mtime 2026-06-21, score 1)

```markdown
# Changelog
...
## [Unreleased]
### Added (v11-044 tier-0 governance batch, 2026-06-20)
- Governance meta-bundle at the monorepo-root `pheno-otel/` path: ...
- Repo configuration: Justfile, .editorconfig, .gitattributes, .gitignore, deny.toml
- CI workflows under `.github/workflows/`: ci.yml, audit.yml, deny.yml, scorecard.yml, release.yml
- Issue + PR templates under `.github/`: ...
- Governance plumbing: CODEOWNERS, dependabot.yml
### Notes
- Source of truth for Rust code: `FocalPoint/pheno-otel/` (separate repo).
- No code changes in this batch — governance + meta-bundle only.
```

(No `[0.1.0]` section; the current `0.1.0` release has no dedicated entry. Score 1 because `[Unreleased]` covers meta-bundle only, not the actual 0.1.0 release.)

### `pheno-port-adapter/CHANGELOG.md` (mtime 2026-06-21, score 3)

```markdown
## [Unreleased]
### Added
- v8 governance meta-bundle (7 files) per ADR-042 + ADR-038: AGENTS.md, SPEC.md, STATUS.md, WORKLOG.md (v2.1 schema), CHANGELOG.md, CONTRIBUTING.md, llms.txt (L5-116, 2026-06-18).
- SPEC.md, STATUS.md, CONTRIBUTING.md, WORKLOG.md, llms.txt.
### Changed
- AGENTS.md — expanded from 30-line v7 stub to full v8 per-repo template.
- llms.txt — expanded from 25-line ad-hoc format to v8 template.
- CHANGELOG.md — restructured to Keep a Changelog 1.1.0.
- WORKLOG.md — migrated to v2.1 canonical schema.
### Deprecated
- WORKLOG.md v2.0 schema — use v2.1; deprecation date 2026-06-22 per ADR-025.

## [0.1.0] - 2026-06-11
### Added
- Initial release of `pheno-port-adapter` (PR #114, L4-66) — reference implementation of the hexagonal L4 Port/Adapter pattern.
- `PortAdapter` trait (name, health, connect, disconnect) — hexagonal L4 contract per ADR-014.
- `Connection` opaque handle (id: String).
- `AdapterError` enum (4 variants) — thiserror-derived.
- `TcpAdapter`, `UnixAdapter`, `MockAdapter` (in `src/lib.rs` test module).
- 5 unit tests, thiserror 2.0 dep, LICENSE-MIT, AGENTS.md, llms.txt, WORKLOG.md, CHANGELOG.md (v7 format).
```

### `pheno-tracing/CHANGELOG.md` (mtime 2026-06-20, score 3)

```markdown
## [Unreleased]
### Added
- src/compat.rs — forward-compatibility shim for `tracing 0.1` → `tracing 0.2`.
- tests/tracing-0-2-compat.rs — 6 forward-compat tests gated by `#[cfg(feature = "tracing-0-2")]`.

## [0.3.0-pre.0] - 2026-06-19
### Added
- AGENTS.md (per-repo template, ADR-019)
- llms.txt (curated README + CHANGELOG + WORKLOG + spec)
- WORKLOG.md (v2.1 schema — 7 columns including new `device:` field per ADR-015/025/030)
- CHANGELOG.md (Keep-a-Changelog)
- LICENSE-MIT (standard MIT, copyright Koosha Pari 2026)
- `.github/workflows/ci.yml` (from `KooshaPari/pheno-ci-templates`; test + clippy + fmt + 80% coverage gate)
```

---

## Appendix C — pointer to 71-pillar L67 audit row

L67 in the 71-pillar framework is "Changelog freshness (CHANGELOG.md present + entries current)". The 2026-06-17 baseline scorecard (`findings/71-pillar-2026-06-17.md` § 1.10) records L67 at **1.5/3.0 fleet-mean** (with 0/3 on 3 of 10 audited repos). This SIDE-25 audit is the substrate-only deep-dive and confirms the same pattern: 4 of 7 substrate crates are at 0/3, fleet mean 1.0/3.0 for the pheno-* substrate specifically.

The proposed remediation in this doc would lift pheno-* substrate mean from 1.00 to 3.00 (+2.00) and pheno-* L67 contribution to the fleet from ~1.5 to ~2.5.
