# ADR-042B: Substrate quality bar (formal)

**Status:** ACCEPTED
**Date:** 2026-06-18
**Author:** orchestrator (claude opus 4.7)
**L5-110.4** (v8 wave B)
**Refs:**
- ADR-023 Rule 3.1 (substrate quality bar — informal)
- ADR-024 (71-pillar L20-L27 quality pillars)
- ADR-040 (test coverage gates per tier)
- ADR-048 (substrate graduation path — uses this bar as the gate criteria)

---

## Context

ADR-023 Rule 3.1 sets the substrate quality bar in narrative form (spec, docs, tests, observability, coverage, CI, worklog). The narrative is hard to enforce consistently:

- Different teams interpret "test" differently (unit only vs unit+integ+e2e)
- "Spec" varies (1-page vs 10-page)
- "CI" varies (lint only vs lint+test+coverage+release)

A formal, named-checklist quality bar lets CI gate on a discrete signal.

## Decision

**Substrate quality is gated on 7 named checks. A repo must pass ≥ 6 of 7 to ship. Each check is owned by a specific tool + CI step + worklog entry.**

### The 7 checks

| # | Check | Tool | CI step | Worklog field |
|---|---|---|---|---|
| 1 | **Spec** (1-page max) | `pheno-spec-lint` | `lint: spec` | `spec_path` |
| 2 | **Docs** (README + 1 concept) | `pheno-docs-lint` | `lint: docs` | `docs_path` |
| 3 | **Unit tests** | `cargo test` / `pytest` / `go test` / `npm test` | `test: unit` | `test_unit_count` |
| 4 | **Integ tests** | `cargo test --test integ` / `pytest tests/integ` | `test: integ` | `test_integ_count` |
| 5 | **E2E/Perf/Chaos** (≥1 of 3) | `tests/e2e_*.rs` or `tests/perf_*.rs` or `tests/chaos_*.rs` | `test: e2e` | `test_e2e_count` |
| 6 | **Observability** (OTLP via pheno-tracing) | `pheno-otel-probe` | `lint: otel` | `otel_endpoint` |
| 7 | **Coverage gate** (per tier, ADR-040) | `cargo-llvm-cov` / `coverage.py` / `go test -cover` | `coverage: gate` | `coverage_pct` |

### Gate logic

```python
def quality_bar_passes(repo_metadata):
    checks = {
        "spec":  repo_metadata.get("spec_path") is not None,
        "docs":  repo_metadata.get("docs_path") is not None,
        "unit":  repo_metadata.get("test_unit_count", 0) > 0,
        "integ": repo_metadata.get("test_integ_count", 0) > 0,
        "e2e":   repo_metadata.get("test_e2e_count", 0) > 0,
        "otel":  repo_metadata.get("otel_endpoint") is not None,
        "cov":   repo_metadata.get("coverage_pct", 0) >= MIN_PER_TIER[repo_metadata["tier"]],
    }
    return sum(checks.values()) >= 6
```

### Per-tier coverage minima (from ADR-040)

| Tier | Coverage min |
|---|---|
| `lib` / `core` / `sdk` | 80% |
| `framework` | 70% |
| `service` | 60% |

## Consequence

- All 22 substrate repos evaluated on a uniform 7-check bar
- Repos with < 6/7 are blocked from promotion to the next substrate graduation tier (ADR-048)
- The bar is the source of truth for "is this a substrate-grade repo?"
- Coverage reports are uniform across all languages

## Cross-references

- ADR-023 Rule 3.1 (informal version of this ADR)
- ADR-040 (per-tier coverage minima referenced by check 7)
- ADR-046 (federation mTLS + OIDC — quality bar applies cross-org)
- ADR-048 (substrate graduation path — uses the bar as gate criteria)
- `findings/2026-06-18-L8-006-coverage-gates.md` (co-design with ADR-040)
