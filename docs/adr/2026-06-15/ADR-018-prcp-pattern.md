# ADR-018: PRCP — Polyglot Repository Coordination Pattern

**Status:** Accepted 2026-06-15
**Deciders:** PhenoRust 1.0 architecture circle
**Supersedes:** ad-hoc `monorepo` + `git subtree` + `cargo workspace` mix

## Context

The 182 repos on the phenotype-org are managed as 100+ small git
repos + 4 polyglot monorepos (`pheno`, `helioscope`, `phenoAI`,
`AtomsEcosystem/AtomsBot`) + ~30 archived/dispatched shims. The
relationships between them are documented in 4 places (STATUS,
SSOT, ARCHITECTURE, DAG) and are not always in sync.

The recurring pain: a single feature (e.g. "trace context across
MCP servers") requires coordinated changes in:

- 1× Rust substrate (`pheno-tracing`)
- 3× Rust consumers (the pheno-mcp-* servers)
- 1× TypeScript/JS consumer (`helioscope`)
- 1× Python consumer (`dispatch-mcp`)
- 1× docs ADR (this one)
- 1× CI matrix (the `ci-shared` action)

That's 8 PRs across 8 repos, with a critical-path DAG that no
single PR-creation tool handles well.

## Decision

**PRCP** (Polyglot Repository Coordination Pattern) is the
canonical way to ship a cross-repo change. The rules:

1. **One "coordinator" PR per change** — a single PR in the
   `phenotype-org/repos` meta-repo's `findings/` directory that
   links the N child PRs, lists the dependency order, and exposes
   the merge gate.
2. **N child PRs in N repos** — each is independently reviewable
   and mergeable; the child PRs are the units of work.
3. **ADR is the lockstep artifact** — if a child PR needs an
   architectural decision, the ADR is written first, accepted, and
   then each child PR cites it (e.g. "Implements ADR-018 §3.2").
4. **Worklog is the rollback artifact** — the worklog JSON (per
   ADR-015 v2 schema) records which child PRs went in, the
   coordinator PR merges the worklog update last (after all
   children are merged and tagged).
5. **`ci-shared` is the gate** — the `pheno-ci-templates` action
   (`pheno-ci-templates` repo, ADR-009) enforces that no child
   PR is merged without its worklog entry, ADR citation, and
   cross-repo status check.

## Consequences

**Positive**

- One reviewer can coordinate N repos without writing N RFCs.
- The DAG-of-DAGs (`plans/2026-06-15-DAG-v6-100TASK-v1.md`) is the
  natural input to PRCP — each leaf task is a child PR, each
  cluster is a coordinator PR.
- The `findings/` directory becomes the source of truth for
  cross-repo decisions; the `STATUS.md`/`SSOT.md` lag is closed.

**Negative**

- Coordinator PRs are review-bottlenecks by design.
- `ci-shared` becomes a critical-path dependency (already true
  for the SOTA rollout).
- The 4 existing "single PR for everything" repos (FocalPoint,
  KodeVibe, helioscope, phenoAI) need a one-time cleanup to split
  the next cross-repo change into PRCP.

**Mitigation**

- The V6 plan (Track 3) ships a `prcp` Rust binary in
  `helios-router` that creates coordinator + child PRs from a
  single TOML manifest (`prcp.toml`); this is the same role
  `pheno-ci-templates` plays for CI.
- The 4 monoliths are tracked in the migration table.

## Alternatives considered

- **One giant monorepo.** Rejected: 182 repos + 4 polyglot
  monorepos + the language diversity (Rust, TypeScript, Go,
  Python, Swift, Kotlin) make a single VCS impractical.
- **Per-repo ADRs only.** Rejected: this is the current state; it
  produces the lag that PRCP fixes.
- **`git subtree` to make every repo a sub-tree of a
  meta-monorepo.** Rejected: `git subtree` history is the same
  problem in a different shape, and the tooling for splitting
  back out is worse than `gh pr create`.

## References

- `plans/2026-06-15-DAG-v6-100TASK-v1.md` §Track 3 — PRCP
  manifest format.
- `helios-router/src/bin/prcp.rs` — coordinator PR generator.
- `docs/adr/2026-06-15/ADR-009-pheno-ci-templates.md` — `ci-shared`
  action (the gate).
- `docs/adr/2026-06-15/ADR-015-v2-worklog-schema.md` — worklog
  schema (the rollback artifact).
- `docs/adr/2026-06-15/ADR-016-fork-only-not-rewrite.md` — fork
  policy (PRCP applies it at the PR level).
