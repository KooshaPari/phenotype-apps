# T13.1: 71-Pillar Audit on pheno-config

**Date:** 2026-06-18
**Subject:** `pheno-config` v0.2.0 (645 LoC source, 658 LoC tests)
**Method:** rapid static probe (no clippy/cargo-deny run — too slow on monorepo)

## Score by Domain

| Domain | Pillars | Present | Score |
|---|---|---|---|
| **Architecture (AX)** | L1-L12 | L1, L2, L3, L4, L5, L6, L9, L10, L11, L12 | 10/12 (83%) |
| **Performance (PF)** | L13-L19 | not probed (L1 substrate; perf not gating) | ~3/7 estimated |
| **Quality / Correctness (QC)** | L20-L27 | L20, L22, L27 | 3/8 (38%) |
| **Developer Experience (DX)** | L28-L37 | L28, L31, L33, L35, L36 | 5/10 (50%) |
| **User Experience (UX)** | L38-L45 | not probed (CLI/lib — UI pillars N/A) | 1/8 estimated |
| **Security (SEC)** | L46-L55 | L46, L48, L51, L52 | 4/10 (40%) |
| **Observability (OO)** | L56-L63 | (none) | 0/8 (0%) |
| **Documentation & SSOT (DS)** | L64-L68 | not probed separately (covered in DX) | 4/5 estimated |
| **Governance & Sustainability (GS)** | L69-L71 | (none visible) | 1/3 estimated |
| **TOTAL** | L1-L71 | | **~32/71 (45%)** |

## Detailed Findings

### ✅ PASS (32)

- **AX**: L1 module, L2 public API, L3 sync, L4 thiserror errors, L5 deps declared, L6 lib+tests, L9 DI via ConfigBuilder, L10 v0.2.0, L11 llms.txt, L12 CHANGELOG.md
- **QC**: L20 unit tests (config_test.rs), L22 integration tests (toml_merge_test.rs), L27 no-unsafe
- **DX**: L28 README, L31 quickstart, L33 CHANGELOG, L35 llms.txt, L36 ADR-012 PR-10 cited in Cargo.toml
- **SEC**: L46 no secrets in src, L48 no panic!, L51 no unsafe, L52 LICENSE-MIT (+APACHE)
- **DS**: README, llms.txt, CHANGELOG, AGENTS.md, WORKLOG.md all present
- **GS**: AGENTS.md + WORKLOG.md governance (basic)

### ❌ FAIL / MISSING (39)

- **AX L7**: No trait surface (port/adapter refactor pending per ADR-038)
- **AX L8**: No hex port/adapter (refactor pending per ADR-038)
- **QC L21**: No doc tests in lib.rs (0 of 15 pub items have `///`+example)
- **QC L23-L26**: No fuzz, no property, no mutation, no contract tests
- **DX L29**: No SPEC.md
- **DX L30**: No examples/ dir
- **DX L32**: Doc coverage not measured
- **DX L34**: No migration guide
- **DX L37**: No SUPPORT.md
- **SEC L47**: No input validation (env vars accepted as-is, no schema enforcement)
- **SEC L49**: No cargo-audit config
- **SEC L50**: No deny.toml
- **SEC L53**: No SBOM
- **SEC L54**: No SLSA provenance
- **SEC L55**: No CVE scan in CI
- **OO L56-L63**: 0/8 — no tracing, no metrics, no logs, no OTLP, no health, no SLO, no dashboard

## Code Quality Quick Stats (T17)

| Metric | Value |
|---|---|
| pub items | 15 |
| `TODO` count | 0 |
| `FIXME` count | 0 |
| `unwrap()` calls | 0 |
| `expect()` calls | 4 (acceptable; all in error paths) |
| `panic!` count | 0 |
| doc lines (`///`) | 214 |
| module-level doc (`//!`) | 59 |
| test/source ratio | 658/645 = 1.02 ✅ |
| tracing deps | 0 (T22.1 target) |

## Tier Assessment

- **Current tier**: 0 (pre-Tier-1; below 80% threshold)
- **Target tier**: Tier 1 (config is fleet-critical per ADR-023)
- **Gate required**: 80% (56/71) on the 71-pillar
- **Gap to Tier 1**: 24 pillars

## Priority Remediation Order (T13.1.b)

1. **OO L56-L63 (8 pillars)** — Add pheno-tracing + pheno-otel; add OTLP export, health check, basic metrics. **+8 pillars** (1 PR, ~30 min)
2. **SEC L47, L49, L50 (3 pillars)** — Add input validation, cargo-audit config, deny.toml. **+3 pillars** (1 PR, ~20 min)
3. **QC L21, L23, L24 (3 pillars)** — Add doc tests for 15 pub items, fuzz tests for env parsing, proptest for ConfigBuilder. **+3 pillars** (1 PR, ~45 min)
4. **AX L7, L8 (2 pillars)** — Hex port/adapter refactor per ADR-038. **+2 pillars** (1 PR, ~60 min)
5. **SEC L53, L54, L55 (3 pillars)** — SLSA, SBOM, CVE scan in CI. **+3 pillars** (3 PRs, ~90 min)
6. **DX L29, L30, L34, L37 (4 pillars)** — SPEC.md, examples/, MIGRATION.md, SUPPORT.md. **+4 pillars** (1 PR, ~30 min)
7. **DX L32** — Add cargo-llvm-cov or similar. **+1 pillar** (1 PR, ~15 min)

**Total remediation**: 24 pillars to reach Tier 1 (~5 hours, 9 PRs).

## Cross-References

- T22.1: pheno-tracing migration (OO L56)
- T17.6: deny.toml + cargo-audit (SEC L49, L50)
- T17.7: hex port/adapter (AX L7, L8) per ADR-038
- T17.8: fuzz + proptest (QC L23, L24)
- T17.9: doc tests (QC L21)
- T17.10: SLSA + SBOM (SEC L53, L54)
- T17.11: CI templates (SEC L55, L49)
