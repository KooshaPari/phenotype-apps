# ADR-103: Encode Latency Budgets into CI

**Status:** DRAFT
**Date:** 2026-06-25
**Owner:** fleet-perf WG
**Supersedes:** none
**Refs:** v27 T2 (71-pillar L19), ADR-095 (pheno-context canonical)

## Context

Service-level objectives require measurable latency guarantees at the per-crate and fleet level. Without budget enforcement, latency regressions are discovered reactively — during incident response rather than during CI review.

Key drivers:

- **Per-crate SLOs**: Each pheno-* crate has distinct latency characteristics (error-path in `pheno-errors`, telemetry serialization in `pheno-otel`, adapter dispatch in `pheno-port-adapter`, flag evaluation in `pheno-flags`). A single fleet-wide budget hides crate-level degradation.
- **Fleet-wide SLOs**: Aggregate p50 / p90 / p99 across all crates must remain within bounds for the 71-pillar audit (L19).
- **Shift-left**: Detecting a budget breach during CI (before merge) is cheaper than detecting it in production. The goal is to catch regressions within 3 consecutive runs before blocking.
- **Adoption path**: Start with a structural-compliance gate (this ADR) and later wire real measurement (benchmark compare, OpenTelemetry exemplars from T5).

### Current state

No latency budget tooling exists in this repo. Latency is measured ad-hoc via manual benchmark runs. CI has no awareness of budget boundaries.

## Decision

Adopt a **2-level latency budget** encoded as a JSON catalog and enforced in CI:

### Level 1: Per-crate budget

Each crate declares its own latency budget in `latency-budgets/catalog.json`:

| Field     | Type    | Description                                      |
|-----------|---------|--------------------------------------------------|
| `crate`   | string  | Crate name (e.g. `pheno-errors`)                 |
| `p50`     | ms      | Median latency target                             |
| `p90`     | ms      | 90th percentile latency target                    |
| `p99`     | ms      | 99th percentile latency target                    |
| `enabled` | boolean | `true` to enforce; `false` to exempt              |

### Level 2: Fleet budget

A synthetic `_fleet` entry aggregates all enabled crates. The fleet budget is the **max** of individual crate budgets at each percentile — any crate exceeding its budget flags the fleet.

### CI enforcement (drift-gate)

A `latency-budget` workflow runs on every PR:

1. **Phase 1 (this ADR):** Validate JSON structure — ensure `catalog.json` is parseable, all required fields exist, types are correct. No actual measurement runs yet.
2. **Phase 2 (v28+):** Wire benchmark-comparison data into the budget check. Compare `p50` / `p90` / `p99` from `cargo bench` against catalog values.
3. **Phase 3 (v29+):** Enable auto-block — if a crate exceeds its budget on **3 consecutive CI runs** (not just 1, to avoid flaky failures), the drift-gate fails the PR.

### Budget catalog location

`latency-budgets/catalog.json` at repo root. This file is the single source of truth for latency targets. Adding or modifying a crate entry requires a PR review.

## Consequences

### Positive

- **Shift-left latency enforcement**: budgets are checked before code reaches main.
- **Gradual adoption**: Phase 1 (structural) + Phase 2 (measurement) + Phase 3 (auto-block) avoids a big-bang rollout.
- **3-run consecutive window**: prevents flaky CI failures from single-run noise.
- **Per-crate visibility**: a crate that stays green while the fleet goes red immediately identifies the offending crate.

### Negative

- **Catalog drift**: if the catalog is not updated when a crate's true latency changes (e.g. after an optimisation), the gate becomes a nuisance. Mitigation: quarterly budget review as part of the 71-pillar cycle.
- **Fleet budget is a simple max**: this is conservative but may report false positives if one crate has a brief spike while others are well under budget. Mitigation: the 3-consecutive-run window absorbs transient spikes.
- **No measurement yet**: Phase 1 is purely structural; it documents intent without enforcement. Teams may ignore it until Phase 2 lands.

### Drift-gate details

```
Trigger: PR opened or synchronized
Action: validate catalog.json schema
Block condition: 3 consecutive CI runs where measured latency > budget
                   (Phase 3 — for now, block only on parse failure)
Escalation: workflow fails with `drift-gate: BUDGET_EXCEEDED` message
             listing which crate(s) violated which percentile
```

## Alternatives considered

- **TOML per-crate files (`latency.toml`)** — rejected in favour of a single JSON catalog for simpler CI parsing and a single source of truth.
- **YAML instead of JSON** — rejected: JSON is natively parseable in GitHub Actions (`fromJSON`), TypeScript, and Rust without extra dependencies.
- **Single fleet-wide budget only** — rejected: hides per-crate degradation; violates the 71-pillar L19 requirement for per-crate observability.
- **Fail on first breach (no 3-run window)** — rejected: single-run noise from CI infrastructure (cold cache, noisy neighbour) would cause spurious failures.

## Implementation steps

1. ✅ Create `latency-budgets/catalog.json` with 4 core crate entries (this ADR).
2. ✅ Create `.github/workflows/latency-budget.yml` with structural validation (this ADR).
3. ☐ Phase 2: Wire `cargo bench` output into budget check (v28).
4. ☐ Phase 3: Enable 3-run consecutive breach detection and auto-block (v29).
5. ☐ Quarterly: Review and update budgets as part of 71-pillar cycle.

## Refs

- ADR-095 (pheno-context canonical) — context propagation dependency for measurement.
- v27 T2 — 71-pillar cycle 17 P0 reduction track.
- 71-pillar L19 — latency budget requirement.
