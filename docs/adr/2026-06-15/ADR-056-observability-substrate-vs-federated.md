# ADR-056 — Observability consolidation: substrate vs federated service (W7-4)

**Status:** PROPOSED (W7-4)
**Date:** 2026-06-19
**Owner:** substrate-audit circle
**Category:** Strategic consolidation (Phase 8 governance)

## Context

There are two observability-related repos in the fleet that have overlapping scope:

| Repo | Path | Role | Status |
|------|------|------|--------|
| `pheno-observability` | `pheno-observability/` (substrate) | Observability primitives (a `tracing` + `metrics` + `otlp` wrapper for embedding in libs) | L1 lib (early stage) |
| `PhenoObservability` | `/Users/kooshapari/CodeProjects/Phenotype/PhenoObservability/` (federated service) | The `phenotype-observably-macros` proc-macro crate + 14 connector crates' `#[async_instrumented]` consumers | L2 SOTA v0.2.0 (verified 8/8 cargo test) |

The naming is confusing because they live in different directories with different cases (`pheno-observability` lowercase vs `PhenoObservability` PascalCase).

The V3 execution log Phase 8 listed this as an open question: "pheno-observability vs PhenoObservability — one is the substrate, one is the federated service."

## The 3 options

### Option A: pheno-observability is canonical, PhenoObservability archived
**Pro:** Lowercase `pheno-*` matches the rest of the substrate family
**Con:** PhenoObservability has the *real* code (v0.2.0 cargo test 8/8 pass)
**Con:** 14 downstream crates already depend on the `phenotype-observably-macros` path

### Option B: PhenoObservability is canonical, pheno-observability merged in
**Pro:** PhenoObservability is the federated service (stateful, mTLS, OTLP)
**Con:** Substrates should be `pheno-*` per ADR-042

### Option C: Document the boundary (both stay, distinct roles)
- **`pheno-observability`** = the *substrate lib* (Rust lib, low-level primitives; what consumers `use`)
- **`PhenoObservability`** = the *federated service* (binary that OTLP-exports spans/metrics/logs; what operators run)
- The name difference is intentional: `pheno-*` (substrate) vs `Phenotype*` (federated service)
**Pro:** Both are real, distinct, and have non-overlapping scope
**Pro:** Name difference is a feature, not a bug — it matches ADR-023 §3
**Con:** Slightly confusing for newcomers

## Recommendation: **Option C (document the boundary)**

The two have distinct roles:
- **Substrate** (`pheno-observability`): a Rust lib that other crates depend on. Provides `tracing_subscriber::EnvFilter` helpers, `tracing_appender` wrappers, OTLP pipeline setup as a `fn init() -> Result<...>`.
- **Federated service** (`PhenoObservability`): a binary that aggregates spans from many sources and ships them to the central OTLP collector. Stateful, mTLS-secured, independently scalable.

The 14 connector crates depend on the **substrate's `phenotype-observably-macros` proc-macro** (which is the *attribute* they slap on async fns). That's the substrate's job, not the federated service's.

## Boundary

| Aspect | `pheno-observability` (substrate) | `PhenoObservability` (federated) |
|--------|-----------------------------------|----------------------------------|
| Crate type | Library | Binary (stateful) |
| Role | Embed in your service | Run on its own infra |
| Dependencies | `tracing`, `tracing-subscriber`, `tracing-appender` | `tracing`, `tracing-opentelemetry`, `opentelemetry-otlp`, `axum` |
| Manifest | `pheno-observability/Cargo.toml` | `PhenoObservability/Cargo.toml` |
| Deploy | `cargo add pheno-observability` | `cargo install pheno-observability` + run |
| Tests | Unit + integration | E2E (pushes to central OTLP) |
| Owner | substrate team | ops team |

## Action items

1. **Write ADR-056** (this file)
2. **Verify the name distinction is enforced** in:
   - `pheno-observability/` exists (substrate lib)
   - `PhenoObservability/` exists (federated service)
   - `Cargo.toml` workspace members lists `pheno-observability` (but NOT `PhenoObservability`, since that's a sibling repo)
3. **Update ADR-042** to mention both explicitly
4. **Add a `Cargo.toml` to `pheno-observability/`** if missing (it's listed in the workspace members but has no source code yet — it's a placeholder for the substrate)

## Acceptance criteria

- [ ] ADR-056 documents the boundary
- [ ] The substrate role is clear: lib for embedding
- [ ] The federated role is clear: binary for ops
- [ ] No naming confusion in cross-references
- [ ] At least 1 downstream crate uses the substrate's macros

## References

- V3 execution log Phase 8 (open question)
- ADR-023 §3 (substrate vs federated service)
- ADR-042 (substrate promotion criteria)
- ADR-036 (pheno-tracing substrate; the substrate lineage)
- `phenotype-observably-macros` v0.2.0 (verified in W6-1)
- 14 connector crates that depend on the substrate (focus-always-on, connector-linear, etc.)
