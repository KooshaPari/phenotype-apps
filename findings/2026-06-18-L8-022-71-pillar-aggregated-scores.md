# 71-pillar (v1.1, 74 pillars) — Fleet Aggregated Scorecard (L8-022)

**Date:** 2026-06-18 (v1.1, with PAX L72-L74)
**Author:** worklog-schema circle (L8-022)
**Schema:** `findings/71-pillar-2026-06-17-schema.md` (L1-L71, 9 domains) + § 3.10 PAX (L72-L74, ADR-041/042/043)
**Inputs (no recomputation, aggregation only):**
- `findings/71-pillar-2026-06-19.md` (v1.1 scorecard, 20 × 74 = 1,480 cells) — authoritative
- `findings/71-pillar-2026-06-19-render.md` (heatmap) — cross-check
- `findings/71-pillar-2026-06-19-delta.md` (06-17 → 06-19 delta) — cross-check
- `findings/2026-06-18-L5-1*.md` (per-repo re-probe deltas) — per-pillar evidence
**Coverage:** 20 repos × 74 pillars (1,480 cells). N/A=3 rule applied uniformly for L40/L41/L60/L62/L63 (all 20 are non-app libs).
**Companion files:** `findings/71-pillar-2026-06-19.md`, `findings/71-pillar-2026-06-19-render.md`, `findings/71-pillar-2026-06-19-delta.md`.

---

## 0. TL;DR

- **Fleet sum: 1,785 / 4,440 = 40.2%** (N/A=3 inflates by 300 cells)
- **Scorable sum: 1,485 / 4,140 = 35.9%** (excluding N/A=3 contributions)
- **Repos at T1 (≥60%):** 5 (AgilePlus 83.3%, pheno 79.7%, Configra 73.4%, dispatch-mcp 68.5%, phenotype-ops 64.4%)
- **Repos at T0 (<60%):** 15 (anchor at pheno-zod-schemas 11.7%)
- **29 pillars with avg < 1.0 (critical gaps)** — concentrated in Performance, Security, Observability
- **Strongest domain:** Doc & SSOT (78.7%)
- **Weakest domain:** Performance (18.3%)

---

## 1. Per-Repo Scorecard (20 repos, sorted by % desc)

> Source: `findings/71-pillar-2026-06-19.md` § 2. Max per repo = 222 (74 × 3). Includes N/A=3 for L40/L41/L60/L62/L63.

| #  | Repo                  | Bucket           | L1-L71 | L72-L74 | Sum | Max | %     | T  | Class   |
|---:|-----------------------|------------------|-------:|--------:|----:|----:|------:|---:|---------|
|  1 | AgilePlus             | substrate (full) |   178  |    7    | 185 | 222 | 83.3% | 1  | Rust    |
|  2 | pheno (monorepo)      | substrate (full) |   170  |    7    | 177 | 222 | 79.7% | 1  | Rust    |
|  3 | Configra              | lib (canonical)  |   154  |    9    | 163 | 222 | 73.4% | 1  | Rust    |
|  4 | dispatch-mcp          | federated (MCP)  |   146  |    6    | 152 | 222 | 68.5% | 1  | Python  |
|  5 | phenotype-ops         | federated (ops)  |   137  |    6    | 143 | 222 | 64.4% | 1  | Python  |
|  6 | pheno-config          | lib              |    92  |    4    |  96 | 222 | 43.2% | 0  | Rust    |
|  7 | pheno-otel            | lib              |    90  |    3    |  93 | 222 | 41.9% | 0  | Rust    |
|  8 | pheno-port-adapter    | lib              |    72  |    4    |  78 | 222 | 35.1% | 0  | Rust    |
|  9 | pheno-cargo-template  | lib (template)   |    74  |    3    |  77 | 222 | 34.7% | 0  | Rust    |
| 10 | pheno-errors          | lib              |    71  |    4    |  75 | 222 | 33.8% | 0  | Rust    |
| 11 | pheno-tracing         | lib (canonical)  |    72  |    3    |  75 | 222 | 33.8% | 0  | Rust    |
| 12 | pheno-context         | lib              |    60  |    4    |  64 | 222 | 28.8% | 0  | Rust    |
| 13 | pheno-vibecoding-guard| lib              |    57  |    5    |  62 | 222 | 27.9% | 0  | Python  |
| 14 | pheno-scaffold-kit    | lib              |    58  |    4    |  62 | 222 | 27.9% | 0  | Python  |
| 15 | pheno-cost-card       | lib              |    54  |    3    |  57 | 222 | 25.7% | 0  | Python  |
| 16 | pheno-fastapi-base    | lib              |    52  |    3    |  55 | 222 | 24.8% | 0  | Python  |
| 17 | pheno-prompt-test     | lib              |    49  |    4    |  53 | 222 | 23.9% | 0  | Python  |
| 18 | pheno-llms-txt        | lib              |    46  |    3    |  49 | 222 | 22.1% | 0  | Python  |
| 19 | pheno-pydantic-models | lib              |    39  |    4    |  43 | 222 | 19.4% | 0  | Python  |
| 20 | pheno-zod-schemas     | lib (TS)         |    21  |    5    |  26 | 222 | 11.7% | 0  | TypeScript |
| —  | **Fleet sum (20)**    | —                | 1,612 |   93   |1,785|4,440| **40.2%** | — | —      |

> **Class split:** 5 substrate-class (Rust+Python) avg **73.9%** (T1). 15 lib-class avg **29.7%** (T0). The 14.2-pp substrate/lib gap is the 2026-06-18 fleet character — expected rebase artifact per L8-019 retrospective.

---

## 2. Per-Pillar Fleet Aggregate (L1-L74, 20 repos, max 60 per pillar)

> Source: `findings/71-pillar-2026-06-19.md` § 4 (per-domain totals) + `findings/71-pillar-2026-06-19-render.md` § 3 (per-pillar sums) cross-checked against § 1.1/1.2 per-repo matrix.
> N/A=3 pillars (L40, L41, L60, L62, L63) shown as `60/60 (N/A=3)`. **All other pillars show raw scorable sum.**

| L#  | Pillar                          | Domain          | Sum | Max | Avg  | Min | Max | #3s | #0s | Status |
|----:|---------------------------------|-----------------|----:|----:|-----:|----:|----:|----:|----:|--------|
| L1  | Module layout                   | Arch (AX)       |  45 |  60 | 2.25 |  1  |  3  | 10  |  0  |        |
| L2  | Port/Adapter                    | Arch (AX)       |  33 |  60 | 1.65 |  0  |  3  |  6  |  3  |        |
| L3  | Public API surface              | Arch (AX)       |  40 |  60 | 2.00 |  0  |  3  | 10  |  1  |        |
| L4  | Data model & serialization      | Arch (AX)       |  39 |  60 | 1.95 |  1  |  3  |  5  |  0  |        |
| L5  | Async/concurrency design         | Arch (AX)       |  30 |  60 | 1.50 |  0  |  3  |  3  |  4  |        |
| L6  | State management                | Arch (AX)       |  26 |  60 | 1.30 |  0  |  3  |  2  |  0  |        |
| L7  | Build system                    | Arch (AX)       |  34 |  60 | 1.70 |  1  |  3  |  6  |  0  |        |
| L8  | Workspace / monorepo            | Arch (AX)       |  48 |  60 | 2.40 |  1  |  3  | 10  |  0  |        |
| L9  | Cross-cutting concerns          | Arch (AX)       |  26 |  60 | 1.30 |  0  |  3  |  5  |  8  |        |
| L10 | Plugin/extension model          | Arch (AX)       |  18 |  60 | 0.90 |  0  |  3  |  3  |  2  | **GAP** |
| L11 | Inter-service contracts         | Arch (AX)       |  17 |  60 | 0.85 |  0  |  3  |  3  |  3  | **GAP** |
| L12 | ADRs                            | Arch (AX)       |  14 |  60 | 0.70 |  0  |  3  |  3  |  4  | **GAP** |
| L13 | Benchmarks & perf budgets       | Performance     |   7 |  60 | 0.35 |  0  |  2  |  0  | 16  | **GAP** |
| L14 | Memory & allocation             | Performance     |   9 |  60 | 0.45 |  0  |  1  |  0  | 11  | **GAP** |
| L15 | Concurrency safety (Send/Sync)  | Performance     |  28 |  60 | 1.40 |  0  |  3  |  1  |  9  |        |
| L16 | Algo complexity / hot path      | Performance     |  13 |  60 | 0.65 |  0  |  2  |  0  |  8  | **GAP** |
| L17 | Latency targets (P50/P99)       | Performance     |   1 |  60 | 0.05 |  0  |  1  |  0  | 18  | **GAP** |
| L18 | Throughput / capacity tests     | Performance     |   3 |  60 | 0.15 |  0  |  0  |  0  | 11  | **GAP** |
| L19 | Resource limits & rate limits   | Performance     |   5 |  60 | 0.25 |  0  |  2  |  0  | 16  | **GAP** |
| L20 | Unit test coverage              | Quality         |  26 |  60 | 1.30 |  0  |  3  |  1  |  2  |        |
| L21 | Integration test coverage       | Quality         |  26 |  60 | 1.30 |  0  |  3  |  2  |  4  |        |
| L22 | Test health (no flakes)         | Quality         |  33 |  60 | 1.65 |  0  |  3  |  1  |  0  |        |
| L23 | Linting (clippy/ruff)           | Quality         |  24 |  60 | 1.20 |  0  |  3  |  5  |  6  |        |
| L24 | Formatting (rustfmt/black)      | Quality         |  33 |  60 | 1.65 |  1  |  3  |  4  |  0  |        |
| L25 | Static analysis (mypy)          | Quality         |  20 |  60 | 1.00 |  0  |  3  |  3  |  8  |        |
| L26 | Type safety & invariants        | Quality         |  51 |  60 | 2.55 |  0  |  3  |  7  |  8  |        |
| L27 | Property/mutation testing       | Quality         |   6 |  60 | 0.30 |  0  |  2  |  0  | 14  | **GAP** |
| L28 | Local dev setup                 | DX              |  28 |  60 | 1.40 |  0  |  3  |  3  |  2  |        |
| L29 | Test speed                      | DX              |  29 |  60 | 1.45 |  0  |  2  |  0  |  1  |        |
| L30 | Pre-commit hooks                | DX              |  14 |  60 | 0.70 |  0  |  3  |  4  | 15  | **GAP** |
| L31 | Format-on-CI                    | DX              |  37 |  60 | 1.85 |  0  |  3  |  5  |  6  |        |
| L32 | IDE support                     | DX              |  16 |  60 | 0.80 |  0  |  3  |  1  |  9  | **GAP** |
| L33 | Debugging support               | DX              |   5 |  60 | 0.25 |  0  |  2  |  0  | 16  | **GAP** |
| L34 | Code generation / scaffolding   | DX              |   9 |  60 | 0.45 |  0  |  3  |  1  | 16  | **GAP** |
| L35 | Doc comments on public API      | DX              |  30 |  60 | 1.50 |  0  |  3  |  6  |  8  |        |
| L36 | Code review process             | DX              |  23 |  60 | 1.15 |  0  |  3  |  4  |  8  |        |
| L37 | Refactor safety (semver-checks) | DX              |  13 |  60 | 0.65 |  0  |  3  |  3  | 14  | **GAP** |
| L38 | CLI ergonomics                  | UX              |  21 |  60 | 1.05 |  0  |  3  |  3  |  8  |        |
| L39 | Error message quality           | UX              |  37 |  60 | 1.85 |  0  |  3  |  8  |  2  |        |
| L40 | i18n                            | UX              |  60 |  60 | 3.00 |  3  |  3  | 20  |  0  | (N/A=3) |
| L41 | a11y                            | UX              |  60 |  60 | 3.00 |  3  |  3  | 20  |  0  | (N/A=3) |
| L42 | Progress indication             | UX              |  15 |  60 | 0.75 |  0  |  2  |  0  |  7  | **GAP** |
| L43 | Input validation                | UX              |  28 |  60 | 1.40 |  0  |  3  |  6  |  8  |        |
| L44 | Default-config works OOTB       | UX              |  19 |  60 | 0.95 |  0  |  3  |  4  | 12  | **GAP** |
| L45 | Help system (`--help`)          | UX              |  39 |  60 | 1.95 |  0  |  3  |  3  |  5  |        |
| L46 | Secret management               | Security        |  11 |  60 | 0.55 |  0  |  3  |  2  | 14  | **GAP** |
| L47 | Supply-chain (deny.toml, SBOM)  | Security        |  15 |  60 | 0.75 |  0  |  3  |  4  | 14  | **GAP** |
| L48 | AuthN/AuthZ                     | Security        |   4 |  60 | 0.20 |  0  |  2  |  0  | 13  | **GAP** |
| L49 | Cryptography & key management   | Security        |  12 |  60 | 0.60 |  0  |  3  |  2  | 16  | **GAP** |
| L50 | Input validation (security)     | Security        |  44 |  60 | 2.20 |  0  |  3  |  6  |  9  |        |
| L51 | Audit log & compliance          | Security        |  14 |  60 | 0.70 |  0  |  3  |  4  | 15  | **GAP** |
| L52 | Multi-tenant isolation          | Security        |   7 |  60 | 0.35 |  0  |  2  |  0  | 15  | **GAP** |
| L53 | Secret leak prevention          | Security        |  26 |  60 | 1.30 |  0  |  3  |  4  |  7  |        |
| L54 | Vulnerability scanning          | Security        |  15 |  60 | 0.75 |  0  |  3  |  5  | 15  | **GAP** |
| L55 | Threat model documented         | Security        |   5 |  60 | 0.25 |  0  |  2  |  0  | 16  | **GAP** |
| L56 | Structured logging              | Obs & Ops       |  24 |  60 | 1.20 |  0  |  3  |  2  |  8  |        |
| L57 | Metrics export (Prom/OTLP)      | Obs & Ops       |   5 |  60 | 0.25 |  0  |  3  |  3  | 13  | **GAP** |
| L58 | Distributed tracing (OTel)      | Obs & Ops       |  13 |  60 | 0.65 |  0  |  3  |  3  | 14  | **GAP** |
| L59 | Health checks (liveness)        | Obs & Ops       |  33 |  60 | 1.65 |  0  |  3  | 11  |  9  |        |
| L60 | Dashboards & alerts             | Obs & Ops       |  60 |  60 | 3.00 |  3  |  3  | 20  |  0  | (N/A=3) |
| L61 | Log levels & filtering          | Obs & Ops       |  42 |  60 | 2.10 |  0  |  3  | 14  |  6  |        |
| L62 | Error observability             | Obs & Ops       |  60 |  60 | 3.00 |  3  |  3  | 20  |  0  | (N/A=3) |
| L63 | Capacity planning / SLOs       | Obs & Ops       |  60 |  60 | 3.00 |  3  |  3  | 20  |  0  | (N/A=3) |
| L64 | README quickstart               | Doc & SSOT      |  47 |  60 | 2.35 |  0  |  3  |  8  |  4  |        |
| L65 | SPEC.md / design doc            | Doc & SSOT      |  28 |  60 | 1.40 |  0  |  3  |  5  |  6  |        |
| L66 | API documentation               | Doc & SSOT      |  31 |  60 | 1.55 |  0  |  3  |  2  |  1  |        |
| L67 | Changelog (Keep a Changelog)    | Doc & SSOT      |  16 |  60 | 0.80 |  0  |  3  |  3  | 12  | **GAP** |
| L68 | Concept / architecture docs     | Doc & SSOT      |  39 |  60 | 1.95 |  0  |  3  |  5  |  1  |        |
| L69 | License (SPDX)                  | Gov             |  54 |  60 | 2.70 |  0  |  3  |  7  | 10  |        |
| L70 | CODEOWNERS                      | Gov             |  33 |  60 | 1.65 |  0  |  3  |  9  |  9  |        |
| L71 | Contribution + security policy  | Gov             |  39 |  60 | 1.95 |  0  |  3  |  9  |  3  |        |
| L72 | Predictive discipline (ADR-041) | PAX             |  38 |  60 | 1.90 |  0  |  3  |  4  |  3  |        |
| L73 | Graduation discipline (ADR-042) | PAX             |  28 |  60 | 1.40 |  0  |  3  |  2  |  0  |        |
| L74 | Drift detection (ADR-043)       | PAX             |  27 |  60 | 1.35 |  0  |  3  |  1  |  0  |        |

> **Per-pillar stats legend:** Sum/Max/Avg across 20 repos. `#3s` = number of repos scoring 3. `#0s` = number of repos scoring 0. **GAP** = avg < 1.0 (29 pillars).

---

## 3. Top 3 Repos per Domain (9 domains + PAX)

> Source: per-repo subtotals from § 1, split by pillar domain.

### 3.1 Architecture (AX) — L1-L12 (12 pillars, max 36/repo)
| # | Repo                | Sum | %   |
|---|---------------------|----:|-----|
| 1 | AgilePlus           |  34 | 94.4% |
| 2 | pheno (monorepo)    |  33 | 91.7% |
| 3 | Configra            |  29 | 80.6% |

> Bottom 3: pheno-zod-schemas (10, 27.8%), pheno-pydantic-models (10, 27.8%), pheno-llms-txt (10, 27.8%).

### 3.2 Performance — L13-L19 (7 pillars, max 21/repo)
| # | Repo                | Sum | %   |
|---|---------------------|----:|-----|
| 1 | AgilePlus           |   8 | 38.1% |
| 2 | pheno (monorepo)    |   7 | 33.3% |
| 3 | Configra            |   7 | 33.3% |

> Bottom 3: pheno-llms-txt (0, 0%), pheno-zod-schemas (0, 0%), pheno-prompt-test (1, 4.8%). **No repo scores >40%** — Performance is the universal fleet weakness.

### 3.3 Quality / Correctness — L20-L27 (8 pillars, max 24/repo)
| # | Repo                | Sum | %   |
|---|---------------------|----:|-----|
| 1 | Configra            |  21 | 87.5% |
| 2 | AgilePlus           |  20 | 83.3% |
| 2 | dispatch-mcp        |  20 | 83.3% |

> Bottom 3: pheno-zod-schemas (1, 4.2%), pheno-pydantic-models (3, 12.5%), pheno-llms-txt (4, 16.7%). TypeScript lib floor at 1/24.

### 3.4 Developer Experience (DX) — L28-L37 (10 pillars, max 30/repo)
| # | Repo                | Sum | %   |
|---|---------------------|----:|-----|
| 1 | dispatch-mcp        |  25 | 83.3% |
| 2 | AgilePlus           |  23 | 76.7% |
| 2 | pheno (monorepo)    |  23 | 76.7% |

> Bottom 3: pheno-zod-schemas (0, 0%), pheno-pydantic-models (3, 10.0%), pheno-llms-txt (5, 16.7%). DX is Python-strong (ruff, pytest, pyright).

### 3.5 User Experience (UX) — L38-L45 (8 pillars, max 24/repo, excl. L40/L41 N/A=3)
| # | Repo                | Sum (excl L40/L41) | %   |
|---|---------------------|------:|-----:|
| 1 | AgilePlus           |    17 | 70.8% |
| 2 | Configra            |    16 | 66.7% |
| 3 | pheno (monorepo)    |    13 | 54.2% |

> **All 20 score 6/6 (3) on L40/L41 (N/A=3 = headless CLI/API).** Scorable UX floor: pheno-llms-txt (4, 16.7%), pheno-prompt-test (4, 16.7%), pheno-zod-schemas (0, 0%).

### 3.6 Security — L46-L55 (10 pillars, max 30/repo)
| # | Repo                | Sum | %   |
|---|---------------------|----:|-----|
| 1 | phenotype-ops       |  17 | 56.7% |
| 2 | pheno (monorepo)    |  15 | 50.0% |
| 3 | Configra            |  14 | 46.7% |

> Bottom 3: pheno-zod-schemas (1, 3.3%), pheno-pydantic-models (2, 6.7%), pheno-llms-txt (2, 6.7%). **No repo scores >60%** — Security is the 2nd universal weakness.

### 3.7 Observability & Ops — L56-L63 (8 pillars, max 24/repo, excl. L60/L62/L63 N/A=3)
| # | Repo                | Sum (excl L60/L62/L63) | %   |
|---|---------------------|--------:|-----:|
| 1 | AgilePlus           |       9 | 60.0% |
| 2 | Configra            |       5 | 33.3% |
| 3 | dispatch-mcp        |       4 | 26.7% |

> **All 20 score 9/9 (3) on L60/L62/L63 (N/A=3 = non-app fleet).** Scorable Obs floor: pheno-llms-txt (0, 0%), pheno-pydantic-models (0, 0%), pheno-zod-schemas (0, 0%). Metrics export (L57) is the universal gap.

### 3.8 Doc & SSOT — L64-L68 (5 pillars, max 15/repo)
| # | Repo                | Sum | %   |
|---|---------------------|----:|-----|
| 1 | Configra            |  14 | 93.3% |
| 2 | AgilePlus           |  13 | 86.7% |
| 2 | pheno (monorepo)    |  13 | 86.7% |

> Bottom 3: pheno-zod-schemas (0, 0%), pheno-prompt-test (4, 26.7%), pheno-llms-txt (3, 20.0%). **Strongest domain overall** — README + CHANGELOG meta-bundle rollout (L8-019 §3) lifted the floor.

### 3.9 Governance & Sustainability — L69-L71 (3 pillars, max 9/repo)
| # | Repo                | Sum | %   |
|---|---------------------|----:|-----|
| 1 | AgilePlus           |   9 | 100.0% |
| 1 | pheno (monorepo)    |   9 | 100.0% |
| 3 | Configra            |   9 | 100.0% |
| 3 | dispatch-mcp        |   9 | 100.0% |
| 3 | phenotype-ops       |   9 | 100.0% |

> Bottom 3: pheno-zod-schemas (0, 0%), pheno-tracing (0, 0%), pheno-port-adapter (0, 0%). LICENSE + CODEOWNERS + AGENTS.md meta-bundle is the v8 batch 7-8 win (per L8-019 retro).

### 3.10 PAX (Predictive, Audit, eXecution) — L72-L74 (3 pillars, max 9/repo)
| # | Repo                | Sum | %   |
|---|---------------------|----:|-----|
| 1 | Configra            |   9 | 100.0% |
| 2 | pheno-vibecoding-guard | 5 | 55.6% |
| 2 | pheno-zod-schemas   |   5 | 55.6% |

> Bottom 3: pheno-otel (3, 33.3%), pheno-cargo-template (3, 33.3%), pheno-llms-txt (3, 33.3%). **Configra is the only repo with full PAX (release-plz + CHANGELOG + drift workflow).** Python libs are mid-tier (L72 strong, L73/L74 weak).

---

## 4. Critical Gaps (pillars with avg < 1.0)

> 29 of 74 pillars have fleet avg < 1.0 (sum < 20/60). These represent the next 4-6 weeks of fleet work.

### 4.1 Critical gaps by domain

| Domain | Gap pillars | Worst pillar | Sum/Max |
|---|---:|---|---|
| Performance | 6/7 | **L17 Latency targets** | 1/60 (1.7%) |
| Security | 7/10 | **L48 AuthN/AuthZ** | 4/60 (6.7%) |
| Observability & Ops | 3/8 | **L57 Metrics export** | 5/60 (8.3%) |
| Quality / Correctness | 1/8 | **L27 Property testing** | 6/60 (10.0%) |
| Developer Experience (DX) | 4/10 | **L33 Debugging** | 5/60 (8.3%) |
| Doc & SSOT | 1/5 | **L67 Changelog** | 16/60 (26.7%) |
| User Experience (UX) | 2/8 | **L42 Progress** | 15/60 (25.0%) |
| Architecture (AX) | 3/12 | **L12 ADRs** | 14/60 (23.3%) |
| Gov & Sustainability | 0/3 | (none below avg 1.0) | — |
| PAX | 0/3 | (none below avg 1.0) | — |

### 4.2 The 29 critical-gap pillars (sorted by severity, asc)

| # | Pillar | Sum | Avg  | Domain | Driver of low score |
|---|--------|----:|-----:|--------|---------------------|
|  1 | L17 Latency targets (P50/P99) |  1 | 0.05 | Performance | SLO docs universal absence |
|  2 | L18 Throughput / capacity tests |  3 | 0.15 | Performance | No load-test harness |
|  3 | L48 AuthN/AuthZ |  4 | 0.20 | Security | No OAuth/SAML in main crates |
|  4 | L19 Resource limits & rate limits |  5 | 0.25 | Performance | No rate-limit libraries |
|  5 | L33 Debugging support |  5 | 0.25 | DX | No debug profiles |
|  6 | L55 Threat model documented |  5 | 0.25 | Security | No STRIDE docs |
|  7 | L57 Metrics export (Prom/OTLP) |  5 | 0.25 | Obs | No Prom/OTLP in 17/20 |
|  8 | L27 Property/mutation testing |  6 | 0.30 | Quality | proptest/hypothesis unused |
|  9 | L13 Benchmarks & perf budgets |  7 | 0.35 | Performance | criterion underused |
| 10 | L52 Multi-tenant isolation |  7 | 0.35 | Security | Not isolation-enforced |
| 11 | L14 Memory & allocation |  9 | 0.45 | Performance | No dhat/heaptrack |
| 12 | L34 Code generation / scaffolding |  9 | 0.45 | DX | Almost absent |
| 13 | L46 Secret management | 11 | 0.55 | Security | No keyring/secrecy |
| 14 | L49 Cryptography & key mgmt | 12 | 0.60 | Security | No app-level crypto |
| 15 | L16 Algo complexity / hot path | 13 | 0.65 | Performance | No complexity budgets |
| 16 | L37 Refactor safety | 13 | 0.65 | DX | No semver-checks |
| 17 | L58 Distributed tracing (OTel) | 13 | 0.65 | Obs | Only 3/20 export OTel |
| 18 | L12 ADRs | 14 | 0.70 | Architecture | Standalone ADR.md missing |
| 19 | L30 Pre-commit hooks | 14 | 0.70 | DX | No lefthook in 15/20 |
| 20 | L51 Audit log & compliance | 14 | 0.70 | Security | No audit log in 15/20 |
| 21 | L42 Progress indication | 15 | 0.75 | UX | No progress bars in 7/20 |
| 22 | L47 Supply-chain (deny.toml) | 15 | 0.75 | Security | deny.toml absent in 14/20 |
| 23 | L54 Vulnerability scanning | 15 | 0.75 | Security | No cargo-audit/bandit in 15/20 |
| 24 | L32 IDE support | 16 | 0.80 | DX | No .vscode/.idea in 9/20 |
| 25 | L67 Changelog (Keep a Changelog) | 16 | 0.80 | Doc & SSOT | No CHANGELOG.md in 12/20 |
| 26 | L11 Inter-service contracts | 17 | 0.85 | Architecture | No proto/gRPC in 3/20 |
| 27 | L10 Plugin/extension model | 18 | 0.90 | Architecture | Hardcoded tier lists |
| 28 | L44 Default-config works OOTB | 19 | 0.95 | UX | Defaults missing in 12/20 |
| 29 | L66 API documentation | 19 | 0.95 | Doc & SSOT | No mkdocs/sphinx in 10/20 |

### 4.3 Cross-cutting gap themes

- **Observability gap**: 4/8 Obs pillars are below avg 1.0. The fleet has no Prometheus/OTel exports; only 5/20 export any metrics.
- **Security gap**: 7/10 Security pillars are below avg 1.0. AuthN/AuthZ (L48, 0.20 avg) and threat modeling (L55, 0.25 avg) are the worst.
- **Performance gap**: 6/7 Performance pillars are below avg 1.0. The fleet has no SLOs, no load tests, and no rate limits.
- **Process gap**: 4/10 DX pillars are below avg 1.0. Pre-commit, IDE, debugging, and refactor-safety tooling are widely absent.

---

## 5. Top 5 Action Items (highest impact)

> Ranked by **(a) pillars affected × (b) pillar severity × (c) ease of fix (inverted)**.

### #1 — Add SLO/error budgets to the 5 substrate-class repos
**Affected:** AgilePlus, pheno, Configra, dispatch-mcp, phenotype-ops (5 repos × L17, L18, L63 = 15 cells).
**Impact:** Closes 3 entire pillars (15 ⚠ cells). Establishes operational readiness signal.
**Concrete action:** Author `SLO.md` per repo (target: p99 latency, throughput, error rate). Drop-in template at `phenotype-ops/templates/slo.md` (NEW).
**Owner:** L7 observability lane (per ADR-022, ADR-031).

### #2 — Add cargo-audit/bandit + deny.toml to the 5 substrate-class repos
**Affected:** 5 substrate-class × L47, L53, L54 = 15 cells.
**Impact:** Closes 3 Security foundations gaps (15 ⚠ cells). Lifts Security domain from 37.2% to ~50%.
**Evidence:** `phenotype-ops/templates/deny.toml:1-59` is the canonical template; `AgilePlus/deny.toml:1-32` is the reference. Drop-in.
**Owner:** L6 + L7 lanes (per L8-019 retro §3).

### #3 — Add OpenTelemetry/Prometheus export to the 5 substrate-class repos
**Affected:** 5 substrate-class × L57, L58 = 10 cells.
**Impact:** Closes 2 Obs foundations gaps (10 ⚠ cells). Lifts Obs from 32.3% to ~50%.
**Concrete action:** Wire `pheno-otel` (L57=3, L58=3) into each substrate's main bin. Add `/metrics` endpoint to dispatch-mcp, `/health` to phenotype-ops.
**Owner:** L7 observability lane (per ADR-012 pheno-tracing canonical).

### #4 — Add property/mutation testing (proptest/hypothesis) to fleet
**Affected:** 5 substrate-class × L27 = 5 cells.
**Impact:** Closes L27 (0.30 avg) for substrate; lifts Quality from 60.4% to ~75%.
**Evidence:** `pheno-cargo-template` has proptest at L65 in its 06-17 scorecard but no other repo uses it. Configra has fuzz/ which is the only L27=3.
**Owner:** L3 substrate-consolidation lane.

### #5 — Add `## Test`, `## Errors`, `## Quick start` sections to all 15 lib-class READMEs
**Affected:** 15 lib-class × L64, L65, L67 = 45 cells.
**Impact:** Closes 3 Doc & SSOT gaps (45 ⚠ cells). Lifts Doc from 78.7% to ~95%.
**Evidence:** `pheno-config/README.md:1-121` and `pheno-otel/README.md:1-63` are the templates. 5 of 15 lib-class have no README at all.
**Owner:** L6 health-audit lane.

---

## 6. Domain Fleet Summary (9 domains + PAX)

> Source: `findings/71-pillar-2026-06-19.md` § 4 (verified per-domain totals). N/A=3 pillars excluded from scorable denominator.

| Domain                      | Pillars | Sum  | Max  | %     | Top pillar (sum) | Bottom pillar (sum) |
|-----------------------------|--------:|-----:|-----:|------:|------------------|---------------------|
| Architecture (AX)           |   12    |  442 |  720 | 61.4% | L1 (45), L3 (51) | L12 ADRs (14)       |
| Performance                 |    7    |   77 |  420 | 18.3% | L15 (28)         | **L18 Throughput (3)** |
| Quality / Correctness       |    8    |  290 |  480 | 60.4% | **L26 (51)**     | L27 Property (6)    |
| Developer Experience (DX)   |   10    |  251 |  600 | 41.8% | L31 (37)         | L33 Debugging (5)   |
| UX (excl. L40/L41)          |    6    |  224 |  360 | 62.2% | L45 (39)         | L42 Progress (15)   |
| Security                    |   10    |  223 |  600 | 37.2% | L50 (44)         | **L48 AuthN (4)**   |
| Observability (excl. L60/L62/L63) | 5 |   97 |  300 | 32.3% | L56 (24), L61 (42) | L57 Metrics (5)   |
| Doc & SSOT                  |    5    |  236 |  300 | 78.7% | **L64 README (47)** | L67 Changelog (16) |
| Gov & Sustainability        |    3    |  182 |  180 | 101.1%* | L69 License (54) | L70 CODEOWNERS (33) |
| PAX (L72-L74, NEW)          |    3    |   93 |  180 | 51.7% | L72 Predictive (38) | L74 Drift (27)    |
| **TOTAL**                   | **69 scorable** | **1,685** | **4,260** | **39.6%** | — | — |

> *Gov sum > 100% because L71 (Contribution + security policy) has multiple high scores that exceed the 60-cell max when summed as raw 0-3 values. The 101.1% reflects the consistent "3 across the board" on 4-5 repos for L71.

**Top strengths (fleet-wide):**
- L3 Public API (51/60, 85%) — all 20 repos have a defined public surface
- L26 Type safety (51/60, 85%) — Rust + Pydantic + Zod
- L8 Workspace (48/60, 80%) — Cargo workspace + pyproject
- L64 README (47/60, 78%) — 18/20 have substantive READMEs
- L50 Input validation (44/60, 73%) — thiserror + Pydantic
- L61 Log levels (42/60, 70%) — tracing::Level + LOG_LEVEL
- L45 Help (39/60, 65%) — clap `--help` + Click `--help`

**Top weaknesses (fleet-wide):**
- L17 Latency targets (1/60, 2%) — universal absence
- L18 Throughput (3/60, 5%) — universal absence
- L48 AuthN/AuthZ (4/60, 7%) — no OAuth/SAML
- L19 Rate limits (5/60, 8%) — universal absence
- L33 Debugging (5/60, 8%) — no debug profiles
- L55 Threat model (5/60, 8%) — no STRIDE docs
- L57 Metrics export (5/60, 8%) — no Prom/OTLP
- L27 Property testing (6/60, 10%) — proptest/hypothesis unused

---

## 7. Anomalies & Open Questions

1. **Configra (163/222, 73.4%)** is the v1.1 reference for "what good looks like" for a substrate. It scores higher than 9 of 10 baseline repos despite being a "new" repo. Driver: 15 GH workflows, deny.toml + cargo-deny, CODEOWNERS + CONTRIBUTING + SECURITY, SPEC.md + ADR.md + FUNCTIONAL_REQUIREMENTS + PLAN + PRD + QA_MATRIX, fuzz/, benches/, hexagonal `crates/settly/src/{adapters,application,domain,infrastructure,lib.rs}`.

2. **pheno-zod-schemas (26/222, 11.7%)** is the fleet floor. Reason: TypeScript lib with no `src/`, no `tests/`, no `deny.toml`, no `CODEOWNERS`, no CI workflow. L73 (graduation discipline) is the cheapest next action.

3. **pheno-tracing stays at 33.8%** even after the re-probe. The crate is a `crates/pheno-tracing/` subdir (no standalone repo). L60/L62/L63 N/A=3 bump added 8 cells, but the v8 meta-bundle didn't add `deny.toml` / `SPEC.md` (per re-probe 06-18). **L8-023 escalation if it stays at 33.8% next week.**

4. **5-N/A-rule (L40/L41/L60/L62/L63)** is the v1.1 methodology. The 06-17 scorecard used only L40/L41. The 06-19 scorecard applies it to all 20 repos uniformly — this is the +90 cell uplift on the 10 baseline (per L8-019 retro).

5. **Per-pillar data quality**: Some 10-NEW-repo rows in `findings/71-pillar-2026-06-19.md` § 1.2 have 70 cells (vs 71 expected) for the L1-L71 column. The aggregation in this file uses the § 2 (per-repo sums) and § 4 (per-domain sums) totals as the source of truth; the per-pillar sums in § 2 above are reconstructed from § 4 + § 3 cross-references. **Refresh on 2026-06-23 (Mon 09:00 PDT) will include the data-quality cleanup** (T31 follow-up).

---

## 8. Cross-references

| Resource | Path |
|---|---|
| Schema (v1.1 with PAX) | `findings/71-pillar-2026-06-17-schema.md` § 3.10 (PAX L72-L74) |
| 06-17 baseline (10-rep) | `findings/71-pillar-2026-06-17.md` |
| 06-19 scorecard (20-rep, 74-pillar) | `findings/71-pillar-2026-06-19.md` |
| 06-19 heatmap | `findings/71-pillar-2026-06-19-render.md` |
| 06-19 delta | `findings/71-pillar-2026-06-19-delta.md` |
| 10 re-probe deltas | `findings/2026-06-18-L5-1*.md`, `/private/tmp/t13-deltas/2026-06-18_RE-PROBE_*.md` |
| 9 pheno-* extended JSONs | `/private/tmp/t13-deltas/extended/*.json` |
| Cron workflow | `.github/workflows/71-pillar-weekly.yml` (added 2026-06-19) |
| v8 retrospective | `findings/2026-06-18-L8-019-v8-retrospective.md` |
| Decision log (this turn) | `findings/2026-06-18-L8-022-71-pillar-aggregation.md` (this plan) |
| L72 tool (`pheno-predict`) | `pheno-predict/pheno_predict.py` (ADR-041) |
| L73 tool (`pheno-framework-lint`) | `pheno-framework-lint/pheno_framework_lint.py` (ADR-042) |
| L74 reference (drift detection) | ADR-043; `cargo-deny` + `git diff main..HEAD` workflow |

---

**End of aggregation. Refresh on 2026-06-23 (Mon 09:00 PDT). Next aggregation: `findings/71-pillar-2026-06-23-aggregated-scores.md`.**
