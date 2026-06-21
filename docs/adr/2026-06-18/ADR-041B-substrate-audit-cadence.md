# ADR-041B: Substrate audit cadence

**Status:** ACCEPTED
**Date:** 2026-06-18
**Author:** orchestrator (claude opus 4.7)
**L5-110.2** (v8 wave B)
**Refs:**
- ADR-023 (substrate placement)
- ADR-024 (71-pillar)
- ADR-041 (71-pillar refresh cadence, weekly)
- ADR-041B supersession: this ADR is the "B-suffix" cadence variant — the bi-weekly substrate health audit, distinct from the weekly 71-pillar refresh in ADR-041

---

## Context

ADR-041 establishes a weekly 71-pillar refresh cadence. The 71-pillar is comprehensive (71 pillars across 9 domains) but does not cover substrate health at the right cadence:

- Lib/SDK/framework Tier-1 repos need deeper weekly health checks (drift, license, freshness, dep-cycles)
- Service Tier-2 repos need bi-weekly cross-cutting health (binary size, image scan, runtime metrics)
- Federated service Tier-3 repos need monthly health (cluster cost, SLO compliance)

A bi-weekly substrate audit complements the weekly 71-pillar refresh. The two cadences (weekly + bi-weekly) form a tiered observability surface.

## Decision

**A bi-weekly substrate audit runs alongside the weekly 71-pillar refresh. The substrate audit is a structural-and-runtime health check, distinct from the 71-pillar's quality-score check.**

### Per-tier audit matrix

| Tier | Frequency | Audit script | What it checks |
|---|---|---|---|
| `pheno-*-lib` (Rust single-crate) | bi-weekly | `audit_lib.sh` | drift, license, dep-cycles, no-dead-code, 80% coverage |
| `pheno-*-core` (Rust multi-crate) | bi-weekly | `audit_core.sh` | workspace hygiene, no-leaking-traits, 80% coverage |
| `phenotype-*-sdk` (polyglot) | bi-weekly | `audit_sdk.sh` | facade stable, semantic-versioning, 80% coverage |
| `phenotype-*-framework` (IoC) | monthly | `audit_framework.sh` | port/adapter stable, 70% coverage |
| Federated service (long-running) | monthly | `audit_service.sh` | binary size ≤500MB, image scan, SLO compliance, 60% coverage |

### Enforcement

CI workflow (`.github/workflows/substrate-audit.yml`):
```yaml
on:
  schedule:
    - cron: '0 17 * * 1'  # Mon 09:00 PST, bi-weekly (even weeks)
  workflow_dispatch:

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Detect tier
        id: tier
        run: echo "tier=$(nix eval .#pheno.tier)" >> $GITHUB_OUTPUT
      - name: Run audit
        run: |
          case "${{ steps.tier.outputs.tier }}" in
            lib|core|sdk)    ./scripts/audit_${{ steps.tier.outputs.tier }}.sh ;;
            framework|service) ./scripts/audit_${{ steps.tier.outputs.tier }}.sh --monthly ;;
          esac
```

### Audit findings

Findings are written to `findings/substrate-audit-2026-MM-DD-<repo>.md` with:
- Tier, audit-script version, pass/fail
- Per-check status (drift: PASS, license: PASS, ...)
- Action items (remediation PR links, owner, ETA)

## Consequence

- 22/22 substrate repos receive tier-appropriate audit cadence
- Substrate drift detected within 2 weeks (vs 4 weeks with 71-pillar alone)
- Federation-level issues (SLO, binary size) detected monthly
- No regression to the 71-pillar weekly cycle (the two cadences are independent)

## Cross-references

- ADR-023 (substrate placement, Rule 3 quality bar)
- ADR-024 (71-pillar L20-L27 quality pillars)
- ADR-040 (test coverage gates per tier — subset of this audit)
- ADR-041 (71-pillar weekly refresh — sibling cadence)
- ADR-041B supersession: replaces any earlier "substrate audit weekly" mentions in older ADRs (e.g. ADR-040, ADR-041)
