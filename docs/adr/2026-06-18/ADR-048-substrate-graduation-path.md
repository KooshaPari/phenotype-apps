# ADR-048: Substrate Graduation Path (4-tier gate table)

**Date:** 2026-06-19 (declared 2026-06-18; ratified 2026-06-19; first persisted 2026-06-22)
**Status:** ACCEPTED
**Pillar:** L25 (Predictive DRY) + L23 (Test data factories) + L42 (Substrate quality)
**Deciders:** phenotype-architecture circle + substrate maintainers
**Supersedes:** none (replaces the implicit "raise bar ad-hoc" pattern)
**Superseded by:** none
**Backlinks:** ADR-051 (Bifrost-as-library), ADR-052 (Plugin SDK spec)

## Context

The fleet has 5 fleet-critical substrates (`pheno-config`, `pheno-tracing`, `pheno-mcp-router`, `pheno-errors`, `pheno-port-adapter`) and 6 supporting substrates (`pheno-context`, `pheno-flags`, `pheno-events`, `pheno-otel`, `pheno-predict`, `pheno-drift-detector`). As new use cases emerge (v19 T1-T5 added `pheno-mtls`, `pheno-oidc`, `pheno-secret-scan`; v20 will add `chaos-injection`, `pact-consumer`), there is no consistent rubric for:

1. **When a new module graduates to substrate** (when does "code in `crates/`" become a standalone crate?)
2. **What quality bar must it meet at each tier** (alpha → beta → stable → fleet-critical)
3. **Who has authority to bump tiers** (architect circle vs substrate maintainer vs fleet-owners)

Without this rubric, the fleet is at risk of:
- **Re-implementation** (multiple repos solve the same problem; see ADR-047 Predictive DRY)
- **Premature stabilization** (an alpha crate gets a v1.0.0 with insufficient tests)
- **Premature deprecation** (a beta crate is killed because no one knew it was beta)

## Decision

We adopt a **4-tier graduation path** with a **gate table per tier**:

| Tier | Name | Definition | Required gates | Authority to promote |
|------|------|------------|----------------|----------------------|
| **T0** | **alpha** | Module-internal, single-crate, no public API commitment | • Compiles<br>• Has at least 1 happy-path test<br>• Lives in `crates/` or a single substrate crate | Substrate maintainer |
| **T1** | **beta** | Standalone crate, published to internal registry, semver pre-1.0 | • T0 +<br>• ≥70% test coverage<br>• CI on every PR<br>• CHANGELOG.md exists<br>• Has ≥1 consumer | Substrate maintainer + 1 architect review |
| **T2** | **stable** | Semver 1.0+, public API, opt-in by default | • T1 +<br>• ≥80% coverage<br>• 3+ consumers across fleet<br>• ≥90d in T1 with no P0 bugs<br>• Public API doc (cargo doc, no warnings)<br>• Per ADR-042B quality bar (≥6 of 7 elements) | Architect circle (3 of 5) |
| **T3** | **fleet-critical** | Adopted by all fleet members; breakage blocks releases | • T2 +<br>• ≥85% coverage<br>• All fleet members use it (no opt-out)<br>• Chaos tests pass<br>• Pen-test on file<br>• Documented failover | Fleet-owners unanimous |

### Current tier assignments (2026-06-22)

| Substrate | Tier | Promoted | Notes |
|-----------|------|----------|-------|
| `pheno-config` | **T3** | 2026-06-15 (ADR-022) | Per ADR-031 Configra absorb |
| `pheno-tracing` | **T3** | 2026-06-18 (ADR-036) | OTLP-bridged to OTel |
| `pheno-mcp-router` | **T2** | 2026-06-21 (v19 closure) | 3 consumers: phenoMCP, phenoEvents, dispatch-mcp |
| `pheno-errors` | **T2** | 2026-06-21 (v19 closure) | 5 consumers |
| `pheno-port-adapter` | **T2** | 2026-06-20 (v18 closure) | 4 consumers |
| `pheno-context` | **T1** | 2026-06-21 (v19 closure) | 1 consumer (phenodag) |
| `pheno-flags` | **T1** | 2026-06-21 (v19 closure) | 1 consumer (Cli-base) |
| `pheno-events` | **T1** | 2026-06-21 (v19 closure) | 2 consumers |
| `pheno-otel` | **T1** | 2026-06-21 (v19 closure) | 1 consumer (pheno-tracing bridge) |
| `pheno-predict` | **T0** | 2026-06-18 (L5-114) | 8 detection patterns (predictive DRY) |
| `pheno-drift-detector` | **T0** | 2026-06-18 (L5-114) | 3-pass algorithm |
| `pheno-mtls` | **T0** | v20 P1 (planned) | Per ADR-046 |
| `pheno-oidc` | **T0** | v20 P1 (planned) | Per ADR-046, ADR-079 |
| `chaos-injection` | **T0** | v20 P1 (T3, planned) | Per v20 P1 plan |
| `pact-consumer` | **T0** | v20 P1 (T4, planned) | Per v20 P1 plan |

## Consequences

### Positive

- **Predictability:** anyone can read a substrate's tier and know what to expect
- **Quality bar progression:** a crate must EARN each tier, not assume it
- **No false-stability:** a T0 crate marked v0.0.1 makes no API commitment
- **Authority distributed:** T0/T1 by maintainer (low ceremony), T2/T3 by circle (high ceremony)
- **ADR-047 (Predictive DRY)** is now implementable: check tier overlap before extracting a new substrate

### Negative

- **Promotion overhead:** T2/T3 require 3-of-5 architect vote (process cost)
- **Inertia:** a T1 crate stuck at T1 for 90+ days may be over-deferred
- **Audit cost:** quarterly tier audit needed (new finding cadence: `tier-audit-YYYY-Qx`)

### Neutral

- The 4-tier model aligns with semver (0.x.x = T0/T1, 1.x.x = T2, post-1.0 with chaos = T3)
- The `pheno-framework-lint` tool (L73) will enforce the gate table at PR time

## Alternatives considered

### A. 2-tier (alpha/beta) with semver only

Use semver major version as the only signal: 0.x.x = alpha, 1.x.x = stable.

- **Pro:** Zero process; just look at the Cargo.toml
- **Con:** "Stable at 1.0" doesn't tell you if it's a fleet-critical substrate; conflates "API stable" with "adopted"
- **Verdict:** REJECTED — semver is necessary but not sufficient

### B. 6-tier (chaos-tier, pre-alpha, alpha, beta, stable, gold)

More granular tiers with chaos-tier and gold as extremes.

- **Pro:** More expressive
- **Con:** 6 tiers × 11 substrates = 66 cells to track; most crates will live in 2-3 of them
- **Verdict:** REJECTED — over-engineered; 4 tiers captures the meaningful distinctions

### C. No tiers (ad-hoc)

Keep the current ad-hoc approach where each substrate maintainer decides.

- **Pro:** Zero process
- **Con:** Inconsistent; the exact problem this ADR is solving
- **Verdict:** REJECTED — status quo

## Implementation plan

| Wave | Action | Status |
|------|--------|--------|
| v18 cycle 8 | Adopt 4-tier model in this ADR | ✅ Done |
| v19 cycle 9 | T0 → T1 promotions for 4 supporting substrates | ✅ Done |
| v20 cycle 10 | Build `pheno-framework-lint` (L73) to enforce gates at PR time | ⏳ Planned |
| v20 cycle 10 | Quarterly tier audit: `findings/2026-06-22-tier-audit-Q2-2026.md` | ⏳ This turn |
| v21+ | T1 → T2 promotion for 4 supporting substrates (if consumers grow) | ⏳ Pending |

## Tier audit template

```markdown
# Tier audit YYYY-Qx

| Substrate | Current tier | Gates passing | Gate gap | Recommended action |
|-----------|-------------:|---------------|----------|-------------------|
| pheno-config | T3 | 5/5 | none | maintain |
| pheno-tracing | T3 | 5/5 | none | maintain |
| pheno-mcp-router | T2 | 5/6 | pen-test pending | schedule pen-test Q3 |
...
```

## References

- **ADR-051** — Bifrost-as-library (T2 substrate; v11 router rebuild)
- **ADR-052** — Plugin SDK spec (T2 substrate; v11 router rebuild)
- **ADR-031** — Configra absorb (T2 → T3 transition for pheno-config)
- **ADR-036** — pheno-tracing substrate canonical (T2 → T3 transition)
- **ADR-042B** — Substrate quality bar (T2 gate table)
- **ADR-047** — Predictive DRY (uses this tier table to detect extraction candidates)
- **ADR-049** — App-substrate drift detector (3-pass algorithm for T3 detection)
- **L73 pheno-framework-lint** — gate-table enforcement at PR time
- **L72 pheno-predict** — 8 detection patterns (predictive DRY tool)
- **L74 pheno-drift-detector** — app-substrate drift detection
- **Semver 2.0** — <https://semver.org/spec/v2.0.0.html>
- **Project context:** `worklogs/2026-06-21-L5-151-v19-state-realignment.json` (L73 entry)
