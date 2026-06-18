# ADR Index — 2026-06-17

This index lists the ADRs accepted on 2026-06-17 for the v7 governance sweep
(`plans/2026-06-17-v7-dag-stable.md`). All files live in this directory.

| #     | Title                                                                                | Status   | File                                                  |
| :---- | :----------------------------------------------------------------------------------- | :------- | :---------------------------------------------------- |
| 024   | 71-pillar industry-standard audit framework (L1–L71, 9 domains)                     | Accepted | `ADR-024-71-pillar-audit-framework.md`                |
| 025   | Worklog schema v2.1 (11th column `device:`) — supersedes v2.0 from ADR-015           | Accepted | `ADR-025-worklog-v2-1-device-column.md`               |
| 026   | Factory AI Agent Readiness Model as cross-cutting external standard                 | Accepted | `ADR-026-factory-ai-agent-readiness.md`               |
| 027   | Git LFS 3-tier policy (always-track / on-demand / never-track)                       | Accepted | `ADR-027-git-lfs-strategy.md`                         |
| 028   | Monorepo architecture eval: hybrid-with-staging-repo                                | Accepted | `ADR-028-monorepo-architecture-eval.md`               |

## Scope

- **ADR-024** formalizes the 71-pillar audit framework (superset of 30-pillar + UX/AX/DX layers), replacing the prior 30-pillar as the *internal* scoring model.
- **ADR-025** adds the 11th `device:` column to the worklog schema (mandatory for new worklogs starting 2026-06-17; v2.0 deprecates 2026-06-22). Required by ADR-023 Rule 1 (device-fit gate).
- **ADR-026** adopts the Factory AI Agent Readiness Model as the cross-cutting external standard, pairing with the 71-pillar to give a *depth* view alongside the 71-pillar *breadth* view.
- **ADR-027** closes L66 (DX: git LFS guidance) with a 3-tier policy: always-track (iOS/Unity/Unreal/disk images/native libs), on-demand (release binaries/wheels/fixtures > 5 MB), never-track (source/configs/lockfiles/docs).
- **ADR-028** closes L25 (Monorepo Polyrepo Trade-off) with a hybrid model: keep the local monorepo for DX, add `KooshaPari/phenotype-org-audits` as the staging repo for governance artifacts.

## Decision-log files

Each ADR has a corresponding decision-log finding at `findings/2026-06-17-L5-10X-*.md`:

- ADR-024 → `findings/2026-06-17-L5-102-71-pillar-audit.md`
- ADR-025 → `findings/2026-06-17-L5-103-worklog-v2-1.md`
- ADR-026 → `findings/2026-06-17-L5-104-factory-ai-readiness.md`
- ADR-027 → `findings/2026-06-17-L5-105-git-lfs-3-tier.md`
- ADR-028 → `findings/2026-06-17-L5-106-monorepo-hybrid-staging.md`

> **Note on L5-104 ID collision:** the `findings/2026-06-17-L5-104-*.md` prefix is also used by the 2026-06-17 Dmouse92→KooshaPari migration track (5 files, separate concern: `dmouse92-to-kooshapari.md`, `forgecode-migration.md`, `dispatch-mcp-migration-plan.md`, `bulk-rust-ts-migration.md`, `pheno-adr012-migration-plan.md`). The Factory AI finding is intentionally co-numbered L5-104 (per the user-specified L5-ID allocation in the v7 plan); the two are unrelated and the ID collision is preserved for traceability.

## Supersedes

- **ADR-024** supersedes the 30-pillar audit as the *internal* scoring model; the 30-pillar files (`audit-30-pillar-L0..L29.md`) remain valid for tech-only deep dives.
- **ADR-025** supersedes ADR-015 v2.0 10-column schema for *new* worklogs only; v2.0 remains valid for historical entries until 2026-06-22.

## Cross-references

- `AGENTS.md` "2026-06-17 wave" table (rows ADR-024..ADR-028)
- `audit-71-pillar-2026-06-17-wrapup.md` § 6 (strands) + § 8.2 (pillar status) + § 10 (Factory AI crosswalk)
- `findings/71-PILLAR-AUDIT-FRAMEWORK-2026-06-17.md` (71-pillar schema)
- `.gitattributes.example` (Tier 1/2/3 policy template, ADR-027)
- `AGENTS.md` § "App-level repo triage" (ADR-023 substrate placement referenced by ADR-028)
