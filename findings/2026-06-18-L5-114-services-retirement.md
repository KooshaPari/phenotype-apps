# L5-114 — Services retirement (CycloneDX SBOM registry)

**Status:** COMPLETE (ADR-040 Steps 1-4 done; Step 5 manual UI delete pending)
**Date:** 2026-06-18 → 2026-06-19
**Decision owner:** `KooshaPari/phenotype-apps` monorepo
**Template:** ADR-037 (4-repo retirement), ADR-040 (5-step deletion recipe)

---

## Source

`KooshaPari/services` — CycloneDX SBOM registry, 15 files, grade D (30-pillar score 46).

## Final state (2026-06-19)

| ADR-040 step | Action | Status | Evidence |
|---|---|---|---|
| 1 | Audit artifact (this file) | DONE 2026-06-18 | (this file, restored 2026-06-19) |
| 2 | 90-day retention window | DONE 2026-06-18 | (no urgent delete) |
| 3 | Migration content to permanent homes | DONE 2026-06-19 | [phenotype-org-audits#44](https://github.com/KooshaPari/phenotype-org-audits/pull/44) (6 files), [phenodocs#189](https://github.com/KooshaPari/phenodocs/pull/189) (1 file) |
| 3.5 | Pre-deletion inventory + governance docs | DONE 2026-06-19 | [phenotype-org-audits#44](https://github.com/KooshaPari/phenotype-org-audits/pull/44) PR body |
| 4 | Archive source repo | DONE 2026-06-19 | `gh api -X PATCH repos/KooshaPari/services -f archived=true` → `archived: true` |
| 5 | Manual UI delete (90-day grace) | **PENDING** (operator action) | <https://github.com/KooshaPari/services/settings#dangerZone> |

## Migration matrix (14 of 15 files accounted for)

### To `KooshaPari/phenotype-org-audits/audits/services/` (PR #44)

| Source | Destination | Purpose |
|---|---|---|
| `docs/audit/BLOCK-C-AUDIT.md` | `BLOCK-C-AUDIT.md` | Canonical fleet-history audit, 2026-06-15 |
| `docs/audit/BLOCK-C-CONSOLIDATION-PLAN.md` | `BLOCK-C-CONSOLIDATION-PLAN.md` | Consolidation plan, 2026-06-16 |
| `audit_scorecard.json` | `audit_scorecard-2026-06-18.json` | 30-pillar scorecard (grade D, score 46) |
| `.github/workflows/ci.yml` | `ci.yml` | TruffleHog + JSON validation (archived for history) |
| `graphql-gateway/focus-graphql-gateway.cdx.json` | `sbom-graphql-gateway-2026-04-26.json` | CycloneDX 1.3 SBOM (stale snapshot, fleet-history) |
| `templates-registry/templates-registry.cdx.json` | `sbom-templates-registry-2026-04-26.json` | CycloneDX 1.3 SBOM (stale snapshot, fleet-history) |

### To `KooshaPari/phenodocs/docs/security/` (PR #189)

| Source | Destination | Purpose |
|---|---|---|
| `docs/security/THREAT_MODEL.md` | `sbom-registry-threat-model.md` | SBOM-registry-specific STRIDE threat model |

### Discarded (1 file)

- PR #1 (dependabot actions/checkout 4→6 bump) — closed as no longer relevant

## PRs

- [KooshaPari/phenotype-org-audits#44](https://github.com/KooshaPari/phenotype-org-audits/pull/44) — 6 files, 2 SBOMs, 1 scorecard, 2 audit docs, 1 CI workflow
- [KooshaPari/phenodocs#189](https://github.com/KooshaPari/phenodocs/pull/189) — 1 file, SBOM-registry threat model

## Source repo state (verified 2026-06-19)

```
archived:        true
pushed_at:       2026-06-18T10:11:42Z
open_issues:     0
```

The repo is read-only; pushes will fail with "This repository was archived so it is read-only."

## Next step (operator action, 90-day window)

Manual UI delete via GitHub Settings → Danger Zone:

- <https://github.com/KooshaPari/services/settings#dangerZone>

Per ADR-040, this step is **operator-only** (the `gh` CLI's `delete_repo` scope is intentionally not used as a safety property; UI action creates a deliberate friction gate).

## Worklog

- `worklogs/L5-114-services-migration-2026-06-19.json` (committed 2026-06-19, `3491fbe3cf`)

## Refs

- ADR-040 (5-step deletion recipe)
- ADR-037 (4-repo retirement template)
- v8 plan § 3.6 Track T14 (L8-020)
- `findings/2026-06-18-v9-closeout-postmortem.md`
