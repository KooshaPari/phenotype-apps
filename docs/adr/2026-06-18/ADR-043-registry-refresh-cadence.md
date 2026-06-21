# ADR-043: Registry refresh cadence

**Status:** ACCEPTED
**Date:** 2026-06-18
**Author:** orchestrator (claude opus 4.7)
**L5-110.5** (v8 wave B)
**Refs:**
- ADR-023 (substrate placement)
- ADR-024 (71-pillar)
- ADR-040 (test coverage gates per tier)
- ADR-048 (substrate graduation path — registry is the source of truth for tier transitions)

---

## Context

The `phenotype-registry` repo holds the canonical catalog of all 22 substrate repos + the 5 federated services. The catalog is currently updated ad-hoc when a new repo is added or graduated.

Risks of ad-hoc updates:
- Tier transitions (lib → framework → service) drift from actual repo state
- New repos can be added without quality bar enforcement (ADR-042B)
- Archive/deletion status can be stale

A bi-weekly refresh cadence keeps the registry in sync with actual repo state.

## Decision

**A bi-weekly registry refresh runs on the same day as the substrate audit (ADR-041B). The refresh is a structural scan of all 22 substrate repos + 5 federated services + 9 app-level repos.**

### Refresh operations

| Operation | What it does | Failure handling |
|---|---|---|
| Tier transition detection | Compare repo's `flake.nix` tier declaration vs registry row | Update registry row + open PR |
| New repo detection | Compare registry vs org repo list | Add row with `status: pending-review` |
| Archive detection | Compare `gh repo view --json archived` vs registry | Update row `status: archived` + remove from active tier |
| Quality bar check | Apply ADR-042B 7-check bar to each repo | Update row `quality_bar_passes: true/false` |
| Substrate audit cross-ref | Apply ADR-041B substrate audit | Embed audit-report link in registry row |

### Enforcement

CI workflow (`.github/workflows/registry-refresh.yml`):
```yaml
on:
  schedule:
    - cron: '0 17 * * 1'  # Mon 09:00 PST, bi-weekly (odd weeks)
  workflow_dispatch:

jobs:
  refresh:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with: { repository: 'KooshaPari/phenotype-registry' }
      - name: Run refresh
        run: |
          python3 scripts/registry_refresh.py \
            --audit-link "findings/substrate-audit-2026-MM-DD.md" \
            --out registry/registry/disposition-index.json
      - name: Open PR
        uses: peter-evans/create-pull-request@v6
        with:
          title: "chore(registry): bi-weekly refresh (2026-MM-DD)"
          commit-message: "chore(registry): bi-weekly refresh"
          branch: "chore/registry-refresh-2026-MM-DD"
```

### Registry output

`registry/registry/disposition-index.json`:
```json
{
  "repos": [
    {
      "name": "pheno-config",
      "tier": "lib",
      "status": "active",
      "quality_bar_passes": true,
      "last_audit": "2026-06-15",
      "last_audit_pass_rate": 1.0
    }
  ],
  "last_refresh": "2026-06-15T17:00:00-07:00",
  "next_refresh": "2026-06-29T17:00:00-07:00"
}
```

## Consequence

- Registry always reflects actual repo state within 2 weeks
- New repos are added with `status: pending-review` (must pass ADR-042B before promotion)
- Archived repos are auto-removed from active tiers
- Quality bar is uniformly applied (ADR-042B)
- The registry is the source of truth for fleet state (replaces the deleted `phenotype-monorepo-state` per ADR-033)

## Cross-references

- ADR-023 (substrate placement)
- ADR-040 (test coverage gates per tier)
- ADR-041B (substrate audit — sibling cadence, same day)
- ADR-042B (quality bar — applied by this refresh)
- ADR-048 (substrate graduation path — registry is the gate authority)
- ADR-033 (monorepo-state deletion — registry replaces it)
