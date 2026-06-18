# OKR & KPI Alignment

**Vision**: Phenotype-org dependency management (Cargo workspace) - single source of truth for cross-crate versions, MSRV, and shared error types.
**Last reviewed**: 2026-06-16
**Review cadence**: Quarterly

## Quarterly Objectives (current cycle)
  - Reduce cross-crate version skew to <= 0.05 stddev by Q3
  - Achieve 100% of public crates exposing the unified error type by Q4
  - Cut CI build time by 30% via shared target-dir cache

## Outcome KPIs
  - version_skew_stddev (target: <= 0.05)
  - unified_error_coverage (target: 100%)
  - ci_build_time_p95 (target: -30%)

## Process Notes
- Reviewed at start of each sprint; updated at end of each quarter.
- Targets are measurable and time-bound.
- Owner: TBD (assign via CODEOWNERS).

> Per 30-pillar framework L05 (OKR/KPI Alignment). Created by the org-wide 30-pillar audit on 2026-06-16.
