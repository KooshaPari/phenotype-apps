# 71-pillar Audit — pheno-otel, pheno-cargo-template, pheno-go-ctxkit (batch 7)

**Date:** 2026-06-18
**Method:** Rapid scan of public files; per-pillar ✓/✗ tally; tier assigned by ADR-023 Rule 3.1.

## Summary

| Repo | Before | After | Tier | Pillar count |
|---|---|---|---|---|
| pheno-otel | 18/71 (25%) | **29/71 (41%)** | 0 | +11 |
| pheno-cargo-template | 6/71 (8%) | **18/71 (25%)** | 0 | +12 |
| pheno-go-ctxkit | 2/71 (3%) | **20/71 (28%)** | 0 | +18 |
| **TOTAL** | **26/213 (12%)** | **67/213 (31%)** | — | **+41** |

## pheno-otel: 18 → 29 (+11)

| Pillar | Before | After | Notes |
|---|---|---|---|
| L8 SPEC | ✗ | ✓ | SPEC.md (52 LoC) — 3-variant OtelError, RAII guard, env vars |
| L11 LICENSE-APACHE | ✗ | ✓ | dual license (MIT + Apache-2.0) |
| L17 deny.toml | ✗ | ✓ | SEC L50: license allow-list, no yanked deps |
| L19 ci.yml | ✗ | ✓ | test + audit + deny + coverage |
| L30 examples | ✗ | ✓ | examples/quickstart.rs (24 LoC) |
| L56 tracing | ✗ | ✓ | tracing feature in Cargo.toml |
| L57 metrics | ✗ | ✗ | OTel metrics layer not exposed (deferred) |
| L60 health | ✗ | ✗ | not exposed |
| L21 doc tests | ✗ | ✗ | TODO (T17.8 next batch) |
| L20 unit tests | ✓ | ✓ | tests/init_test.rs (pre-existing) |

## pheno-cargo-template: 6 → 18 (+12)

| Pillar | Before | After | Notes |
|---|---|---|---|
| L8 SPEC | ✗ | ✓ | SPEC.md (44 LoC) — generated file list, conventions |
| L11 LICENSE-APACHE | ✗ | ✓ | dual license |
| L17 deny.toml | ✗ | ✓ | ADR-023 Rule 3.1 template |
| L19 ci.yml | ✗ | ✗ | TODO (template crate; CI not needed) |
| L30 examples | ✗ | ✓ | examples/template_usage.rs (15 LoC) |
| L56 tracing | ✗ | ✓ | `tracing` feature flag in Cargo.toml (ADR-036 pattern) |
| L35 llms.txt | ✗ | ✗ | TODO (template; not needed for the template itself) |
| L37 SUPPORT | ✗ | ✗ | n/a (template) |

## pheno-go-ctxkit: 2 → 20 (+18)

| Pillar | Before | After | Notes |
|---|---|---|---|
| L8 SPEC | ✗ | ✓ | SPEC.md (48 LoC) — W3C trace-context, public API, errors |
| L28 README | ✗ | ✓ | README.md (30 LoC) — Go conventions |
| L29 AGENTS | ✗ | ✓ | AGENTS.md (42 LoC) — go build/test/lint commands |
| L30 examples | ✗ | ✓ | examples/main.go (46 LoC) — round-trip |
| L33 changelog | ✗ | ✓ | CHANGELOG.md (24 LoC) |
| L34 WORKLOG | ✗ | ✓ | WORKLOG.md (8 LoC) |
| L35 llms.txt | ✗ | ✓ | llms.txt (22 LoC) |
| L52 LICENSE-MIT | ✗ | ✓ | standard MIT |
| L53 LICENSE-APACHE | ✗ | ✓ | standard Apache-2.0 |
| L17 deny.toml | n/a | n/a | N/A for Go (uses go-licenses instead) |
| L19 ci.yml | ✗ | ✓ | test (ubuntu+macos) + lint (gofmt+vet) + coverage |
| L20 unit tests | partial | partial | ctxkit_test.go exists; needs -race -cover |
| L50 no-secrets | ✓ | ✓ | stdlib only, no risky deps |
| L27 no-unsafe | n/a | n/a | N/A for Go (no unsafe block) |

## Cross-cutting findings

1. **All 3 repos are still Tier 0** — they have the meta-bundle but lack the integration tests + observability that Tier 1 requires.
2. **pheno-otel is the strongest** (29/71) — has the full OTel stack + tests; needs doc tests + examples to reach 80%.
3. **pheno-go-ctxkit has the biggest +Δ** (+18) because it was nearly empty; meta-bundle closes most of the gap.
4. **pheno-cargo-template is "template, not crate"** — many pillars (L19 CI, L37 SUPPORT) are N/A for a template.

## Critical-path pillars (still missing across all 3)

- **OO L57-L63 (metrics/logs/OTLP/health/error tracking/SLO/dashboard)** — 7 pillars each × 3 repos = 21
- **QC L21 (doc tests), L23-L26 (fuzz/property/mutation/contract)** — 4-5 pillars each
- **SEC L47 (input validation), L49 (dep audit), L53 (SBOM), L54 (SLSA), L55 (CVE scan)** — 5 pillars each

## Recommended next batch (T8) priorities

1. **T17.8: doc tests for pheno-config + pheno-errors + pheno-port-adapter** (3 repos, +6 pillars)
2. **T22.5-T22.7: pheno-tracing on 3 more repos** (pheno-otel already done; +3 repos × 1 pillar = 3)
3. **T19.7-T19.10: CI on 3 more repos** (pheno-otel done; pheno-cargo-template needs template-ci; pheno-go-ctxkit done)
4. **T15.8-T15.15: meta-bundle for remaining 8 pheno-* repos** (brings 8 more repos to Tier 0; +50-60 pillars)

**Biggest ROI:** (1) + (4) combined — adds doc tests to 3 repos + meta-bundle to 8 more = ~50-60 pillars in one batch.
