# ADR-042: pheno-tracing Migration Order Across pheno-* Fleet

**Date:** 2026-06-18
**Status:** ACCEPTED
**Deciders:** orchestrator (T22 design), fleet substrate circle
**Related:** ADR-012 (superseded by ADR-036), ADR-036, ADR-037

## Context

ADR-036 mandates `pheno-tracing` as the canonical OTLP substrate for all pheno-* Rust crates
and the 4 federated services. ADR-015 v2.1 requires `device:` field in worklogs (deprecation 2026-06-22).
T13.1 found `pheno-config` has 0 tracing deps (OO L56 fail).

The fleet has 22 pheno-* Rust crates and 4 federated services = 26 total adoption sites.
Sequential migration would take ~13 hours (30 min per site).

## Decision

**Migrate in 4 waves** by Tier (per ADR-023), parallelizable across sites within each wave.

## Wave Plan

### Wave 1: Fleet-critical (Tier 1) — 4 sites, ~2 h

| # | Site | Tier | Effort | Notes |
|---|---|---|---|---|
| 1 | pheno-config | 1 (config) | 30 min | Highest priority; blocks 8 other sites |
| 2 | pheno-tracing | 1 (tracing) | skip | Self — already has tracing |
| 3 | pheno-mcp-router | 1 (mcp) | 45 min | Critical path for Dmouse92 migration |
| 4 | phenotype-registry | 1 (registry) | 30 min | High-traffic service |

### Wave 2: Fleet-critical (Tier 1) — 6 sites, ~3 h

| # | Site | Tier | Effort | Notes |
|---|---|---|---|---|
| 5 | pheno-context | 1 | 30 min | Fleet substrate |
| 6 | pheno-port-adapter | 1 | 30 min | L4 ports |
| 7 | pheno-otel | 1 | skip | Self — already has tracing |
| 8 | pheno-flags | 1 | 30 min | Feature flags |
| 9 | pheno-errors | 1 | 30 min | Error types |
| 10 | pheno-cli-base | 1 | 45 min | CLI infrastructure |

### Wave 3: Tier 2 — 8 sites, ~4 h

| # | Site | Tier | Effort | Notes |
|---|---|---|---|---|
| 11-18 | pheno-{agents-md,cargo-template,fastapi-base,llms-txt,prompt-test,pydantic-models,scaffold-kit,vibecoding-guard,worklog-schema} | 2 | 30 min ea | Mostly Python; tracing via OpenTelemetry Python SDK + OTLP |

### Wave 4: Tier 3 — 8 sites, ~4 h

| # | Site | Tier | Effort | Notes |
|---|---|---|---|---|
| 19-26 | remaining pheno-* + federated services (4) | 3 | 30 min ea | Including 4 federated services (phenotype-hub, phenotype-bus, phenoMCP, phenoObservability) |

## Adoption Mechanics

Each migration PR adds:

1. **Cargo.toml / pyproject.toml**: add `pheno-tracing = "0.1"` (Rust) or `pheno-tracing-py = "0.1"` (Python) dep
2. **lib.rs / __init__.py**: call `pheno_tracing::init()` in startup
3. **OTLP export**: enable via env var `OTEL_EXPORTER_OTLP_ENDPOINT`
4. **Spans**: add `#[tracing::instrument]` to all public async fn
5. **Tests**: add `tracing-test` crate dependency; test that spans emit correctly
6. **CI**: add `OTEL_SDK_DISABLED=true` to test env (don't actually export in tests)

## Acceptance Criteria (per site)

- ✅ `pheno-tracing` dep in manifest
- ✅ `pheno_tracing::init()` called at startup
- ✅ All public functions have `#[tracing::instrument]` (or `#[tracing::instrument]`-equivalent)
- ✅ `OTEL_EXPORTER_OTLP_ENDPOINT` env var respected
- ✅ Test suite passes with `OTEL_SDK_DISABLED=true`
- ✅ WORKLOG.md updated with migration entry + `device:` field (ADR-015 v2.1)
- ✅ Closes one OO pillar in the 71-pillar audit (OO L56-L63)

## Enforcement

- CI: `pheno-tracing` dep check (must appear in any pheno-* Rust crate PR)
- 71-pillar: re-audit post-migration; target: OO L56-L63 8/8 ✅
- Weekly: T13 71-pillar refresh detects regressions

## Total Fleet Impact

- 26 sites, ~13 h engineering, 26 PRs
- Fleet-wide OO score: 0/8 → 8/8 across all sites
- ADR-036 compliance: 100%
- ADR-015 v2.1 (device: field) ready by 2026-06-22
