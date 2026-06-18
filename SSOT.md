# SSOT — Single Source of Truth (focalpoint)

This document records the canonical authority for cross-cutting facts in the
focalpoint repository. When a fact conflicts across docs, the source listed
here wins.

## Scope

| Domain | Authoritative source |
| --- | --- |
| Build & test commands | `justfile` (root) and per-crate `Cargo.toml` |
| Release & versioning | `cliff.toml` + `CHANGELOG.md` (git-cliff generated) |
| Security disclosure process | `SECURITY.md` |
| Dependency updates | `.github/dependabot.yml` |
| Branch & commit policy | `.github/workflows/governance.yml` |
| Repository health score | `.github/workflows/scorecard.yml` (OpenSSF) |
| Editor / formatting baseline | `.editorconfig` |
| Workspace architecture | `ARCHITECTURE.md` (per-crate breakdown) |
| Agent operating model | `AGENTS.md` |
| Canonical config (Rust) | `phenoShared/phenotype-config-core/src/lib.rs` (ADR-012) |
| Canonical config (Python) | `pheno-config/` (ADR-012 PR-6/7) |
| Config consolidation plan | `findings/ADR-012_CONFIG_CONSOLIDATION_PR1-3_DONE-2026_06_15.md` (PR-1/2/3 done; 4-11 planned) |
| **Agent-effort governance** | `docs/adr/2026-06-15/ADR-023-agent-effort-governance.md` (device-fit gate, app-level repo triage, app substrate placement) |
| **App-level repo triage table** | `AGENTS.md` § "App-level repo triage & app substrate placement (ADR-023)" + mirrored in `STATUS.md` § "App-level repo triage" |
| **Worklog schema** | `docs/adr/2026-06-15/ADR-015-v2-worklog-schema.md` (v2.1 bump pending — 11th column `device:`, enum `macbook \| heavy-runner`) |

## Precedence order

1. Executable config (workflows, `justfile`, `Cargo.toml`, `deny.toml`) — observed behavior.
2. `*.md` governance files in this SSOT table.
3. `ARCHITECTURE.md` crate-level contracts.
4. The L5 governance ADRs (ADR-023 and successors) override any L3/L4 substrate decision where the conflict is "should the agent be working on this at all?" — substrate decisions (which crate to use) are L3/L4; effort-decision (whether to work on it, on which device) is L5.
5. Anything else.

## Updating this file

- Keep the table narrow and unambiguous.
- Cite the canonical file by path; do not duplicate content.
- Update via a `chore(governance):` commit referencing the change.
