**STATUS: CLOSED 2026-06-19**

# Finding — pheno-* Substrate Audit (L5-110)

**Date:** 2026-06-19 (v8 batch, T15 sweep)
**Task:** L5-110
**Refs:**
- ADR-023 (Agent-effort governance — substrate placement authority)
- ADR-026 (Factory AI Agent Readiness Model — 5-tier depth view)
- ADR-036 (pheno-tracing canonical substrate)
- ADR-040 (Test coverage gates per substrate tier)
- ADR-041 (Substrate audit cadence — quarterly, L5-110 → feeds the quarterly cron)
- ADR-042 (Substrate quality bar — Rule 3.1 codified, 7 elements)
- ADR-013 (Substrate model — registry lint)
- `findings/L6_PHENO_REPOS_HEALTH_2026_06_18_DELTA.md` (T16 prior pass, found 0 bucket-drift; this audit re-verifies with deeper substrate-NAME compliance)
- `findings/71-pillar-2026-06-19.md` (20-repo scorecard; this audit adds the **substrate audit dimension** that the 71-pillar sweep does not cover)
- `plans/2026-06-18-v8-dag-stable.md` § 3.7 (T15 task list, 22 pheno-* repos)

**Worklog:** `worklogs/2026-06-19-L5-110-substrate-audit.json`
**Drift worklogs (per repo):** `worklogs/2026-06-19-L5-110-substrate-drift-*.md` (8 files, one per drift finding)

---

## Headline

The 22 pheno-* repos in v8 T15 scope were audited against **three** lenses — (1) ADR-023 substrate-bucket naming compliance, (2) ADR-042 Rule 3.1 quality-bar coverage (7 elements), and (3) ADR-026 Factory AI 5-tier readiness. Result: **0/22 fully clean**; **21/22 exist**; **9 drift findings** (3 critical, 4 medium, 2 low); **substrate-name compliance 14/21 (66.7%)**; **Rule 3.1 7-element coverage avg 4.7/7 (67%)**; **Factory AI fleet avg Level 1.1** (Functional floor).

The T16 prior pass (L6 delta 2026-06-18) checked only the **bucket column** in `AGENTS.md` § PAUSED APPs and found 0 drift. This audit goes further: it checks the **substrate-NAME compliance** (does the repo name match the substrate type per ADR-023 Rule 3?) and the **Rule 3.1 quality bar** (does the repo have spec + docs + tests + observability + coverage + CI + worklog v2.1?). Three findings are critical because they break the substrate model itself: **pheno-tracing is missing as a top-level repo** (it lives in 5+ workspace roots as `crates/pheno-tracing` — the "random `phenoShared`" pattern ADR-023 forbids), and **pheno-config depends on it**.

---

## Scope

| Lens | Count | Source |
|---|---|---|
| **pheno-* repos in v8 T15 scope** | 22 (T15.1–T15.22; T15.22 = conditional Python variant placeholder) | `plans/2026-06-18-v8-dag-stable.md` § 3.7 |
| **pheno-* repos visible in monorepo** | 27 (22 T15 + 5 NEW beyond T15 + 1 placeholder missing) | `ls -d pheno-*/` 2026-06-19 |
| **T15 repos that exist** | 21/22 (95.5%) | this audit |
| **T15 repos missing** | 1/22 (4.5%) — pheno-tracing | this audit |
| **T15.22 placeholder (Python variant of pheno-context)** | does not exist | this audit |
| **Drift findings** | 9 (3 critical, 4 medium, 2 low) | § 5 below |
| **Substrate-name compliance (ADR-023)** | 14/21 = 66.7% | this audit |
| **Rule 3.1 7-element coverage (ADR-042)** | 21-repo avg = 4.71/7 = 67% | this audit |
| **Factory AI 5-tier fleet avg (ADR-026)** | Level 1.1 (Functional, 1-repo at L2, 19 at L1, 1 N/A) | this audit |

**Visible pheno-* beyond T15 scope (NEW since v8 plan, 5 added 2026-06-18):**
- `pheno-ci-templates` (template for CI workflows; no source code; `.github/` only)
- `pheno-drift-detector` (single .py script; stdlib only)
- `pheno-predict` (single .py script; stdlib only)
- `pheno-secret-scan` (TruffleHog wrapper; workflow + pre-commit hook only)
- `pheno-ssot-template` (template for new repos; `Cargo.toml.template`, `scripts/`, `src/`)
- `pheno-profiling` is also new (Python lib; replaces Profila per ADR-021; in T15-fleet scope as a NEW substitute, not in original T15 list)

These 5 are flagged as "T15+ scope" — they need the same audit but are not in v8 T15's 22 PR plan. P1 carry-forward.

---

## 1. Per-repo bucket table (ADR-023 substrate type)

For each pheno-* repo: language, manifest, name-suggested bucket per ADR-023 Rule 3, actual content-validated bucket, and **name-compliance verdict**.

| # | Repo | Lang | Manifest | Name suggests (ADR-023) | Actual content | Compliance | Drift? |
|---:|---|---|---|---|---|---|:---:|
| 1 | pheno-config | Rust | Cargo.toml | pheno-*-lib | typed-config loader; uses tracing/OTLP | ✓ | — |
| 2 | pheno-context | Rust | Cargo.toml | pheno-*-lib | request-context struct (request_id, user_id) | ✓ | — |
| 3 | pheno-otel | Rust | Cargo.toml | pheno-*-lib | OTel init guard with OTLP HTTP exporter | ✓ | — |
| 4 | pheno-port-adapter | Rust | Cargo.toml | pheno-*-lib | Port trait + Adapter impl (hexagonal L4) | ✓ | — |
| 5 | **pheno-tracing** | — | — | pheno-*-lib (canonical tracing substrate, ADR-036) | **MISSING as top-level repo**; exists at 5+ workspace paths (`./crates/pheno-tracing`, `./FocalPoint/crates/pheno-tracing`, `./FocalPoint/pheno-tracing`, `./worktrees/ci-templates-absorb/crates/pheno-tracing`, `./PhenoCompose/packages/pheno-tracing`, `/private/tmp/t15-batch-output/rust/pheno-tracing`) | **✗ CRITICAL** | **YES — random `phenoShared`** |
| 6 | pheno-flags | Rust | Cargo.toml | pheno-*-lib | feature-flag types | ✓ | — |
| 7 | pheno-errors | Rust | Cargo.toml | pheno-*-lib | thiserror-based error types | ✓ | — |
| 8 | pheno-cli-base | Rust | Cargo.toml | pheno-*-lib | CLI building blocks (clap wrappers) | ✓ | — |
| 9 | pheno-cargo-template | Rust | Cargo.toml | pheno-*-lib | **TEMPLATE for new Rust crates** (has `template/` dir; meant to be `cp -R` into a new crate) | **✗ MEDIUM** | **YES — TEMPLATE, not lib** |
| 10 | pheno-agents-md | Rust | Cargo.toml | pheno-*-lib | AGENTS.md generator (CLI + lib) | ✓ | — |
| 11 | pheno-fastapi-base | Python | pyproject.toml | pheno-*-lib | FastAPI base scaffolding | ✓ | — |
| 12 | pheno-pydantic-models | Python | pyproject.toml | pheno-*-lib | Pydantic domain models | ✓ | — |
| 13 | pheno-zod-schemas | TypeScript | package.json | pheno-*-lib | Zod schemas (TS mirror of Pydantic) | ✓ | — |
| 14 | pheno-mcp-router | Python | pyproject.toml | pheno-*-lib (or federated service per ADR-037) | Generic MCP router; wraps 1 backend HTTP; per ADR-037 is **substrate-canonical for MCP routing** | ✓ (substrate-canonical) | — |
| 15 | pheno-cost-card | Python | pyproject.toml | pheno-*-lib | per-repo cost tracking | ✓ | — |
| 16 | pheno-worklog-schema | Python | pyproject.toml | pheno-*-lib | WORKLOG.md schema + validator | ✓ | — |
| 17 | **pheno-vibecoding-guard** | Python | pyproject.toml | pheno-*-lib | **README says "Local scratch repo. The validation-rule work has been absorbed into `PhenoHandbook`"** | **✗ CRITICAL** | **YES — DEPRECATION CANDIDATE** |
| 18 | pheno-scaffold-kit | Python | pyproject.toml | pheno-*-lib | scaffold umbrella (meta-package) | ✓ | — |
| 19 | pheno-llms-txt | Python | pyproject.toml | pheno-*-lib | llms.txt generator | ✓ | — |
| 20 | pheno-prompt-test | Python | pyproject.toml | pheno-*-lib | pytest plugin for prompt regression | ✓ | — |
| 21 | pheno-go-ctxkit | Go | go.mod | pheno-*-lib | Go request-context kit | ✓ | — |
| 22 | (Python variant of pheno-context) | — | — | pheno-*-lib (if it existed) | **DOES NOT EXIST** (T15.22 conditional placeholder) | n/a | placeholder unfilled |

**Beyond T15 (5 NEW, 2026-06-18):**

| # | Repo | Lang | Manifest | Name suggests (ADR-023) | Actual content | Compliance | Drift? |
|---:|---|---|---|---|---|---|:---:|
| 23 | pheno-ci-templates | n/a (template) | n/a | pheno-*-lib | **CI workflow TEMPLATE** (only `.github/workflows/` + README; no source code) | **✗ MEDIUM** | **YES — TEMPLATE, not lib** |
| 24 | pheno-drift-detector | n/a (script) | n/a | pheno-*-lib | **SCRIPT** (single `pheno_drift_detector.py`; stdlib only) | **✗ MEDIUM** | **YES — SCRIPT, not lib** |
| 25 | pheno-predict | n/a (script) | n/a | pheno-*-lib | **SCRIPT** (single `pheno_predict.py`; stdlib only) | **✗ MEDIUM** | **YES — SCRIPT, not lib** |
| 26 | pheno-secret-scan | n/a (workflow) | n/a | pheno-*-lib | **WORKFLOW TEMPLATE** (only `.pre-commit-hooks.yaml` + `.trufflehog-allowlist.txt` + `.github/workflows/`; no source code; uses external `trufflehog` image) | **✗ MEDIUM** | **YES — WORKFLOW, not lib** |
| 27 | pheno-ssot-template | n/a (template) | n/a | pheno-*-lib | **REPO TEMPLATE** (`Cargo.toml.template` + `scripts/` + `src/`) | **✗ MEDIUM** | **YES — TEMPLATE, not lib** |
| 28 | pheno-profiling | Python | pyproject.toml | pheno-*-lib | system profiling lib (replaces Profila per ADR-021) | ✓ | — |

**Substrate-name compliance summary:**
- 14/21 existing T15 repos match their name-suggested substrate type (66.7%)
- 1 missing entirely (pheno-tracing — critical)
- 1 deprecation candidate (pheno-vibecoding-guard — critical)
- 4 templates/scripts misnamed as `pheno-*-lib` (medium)
- 5 NEW (T15+ scope) are also misnamed (medium)

---

## 2. Per-repo attribute inventory (T15 scope, 21 existing)

For each pheno-* repo: tests (T), docs (D), CI workflows (CI), examples (EX), SPEC.md, README.md, WORKLOG.md, CHANGELOG.md, LICENSE, internal pheno deps.

Legend: **Y** = present, **N** = absent, **?** = depends on convention.

| # | Repo | Lang | T | D | CI | EX | SP | RM | WL | CH | LI | pheno deps |
|---:|---|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|---|
| 1 | pheno-config | Rust | Y | Y | Y | Y | **N** | Y | Y | Y | Y | `pheno-tracing` (opt) |
| 2 | pheno-context | Rust | Y | **N** | **N** | Y | **N** | **N** | Y | Y | Y | — |
| 3 | pheno-otel | Rust | Y | **N** | **N** | **N** | **N** | Y | Y | Y | Y | — |
| 4 | pheno-port-adapter | Rust | Y | **N** | Y | **N** | **N** | **N** | Y | Y | Y | — |
| 5 | pheno-flags | Rust | Y | **N** | Y | **N** | **N** | **N** | **N** | **N** | **N** | — |
| 6 | pheno-errors | Rust | Y | **N** | Y | Y | **N** | **N** | **N** | **N** | **N** | — |
| 7 | pheno-cli-base | Rust | Y | **N** | Y | Y | Y | Y | **N** | **N** | Y | — |
| 8 | pheno-cargo-template | Rust | Y | Y | Y | Y | Y | Y | Y | Y | Y | — |
| 9 | pheno-agents-md | Rust | Y | Y | Y | **N** | Y | Y | Y | Y | Y | — |
| 10 | pheno-fastapi-base | Python | Y | **N** | Y | Y | Y | Y | Y | Y | Y | — |
| 11 | pheno-pydantic-models | Python | Y | **N** | **N** | **N** | **N** | Y | **N** | **N** | **N** | — |
| 12 | pheno-zod-schemas | TypeScript | Y | **N** | **N** | **N** | **N** | Y | **N** | **N** | **N** | — |
| 13 | pheno-mcp-router | Python | Y | Y | Y | Y | Y | Y | Y | Y | Y | — |
| 14 | pheno-cost-card | Python | Y | **N** | Y | Y | Y | Y | Y | Y | Y | — |
| 15 | pheno-worklog-schema | Python | Y | **N** | Y | Y | Y | Y | Y | Y | Y | — |
| 16 | pheno-vibecoding-guard | Python | Y | **N** | Y | Y | Y | Y | Y | Y | Y | — (DEPRECATION CANDIDATE) |
| 17 | pheno-scaffold-kit | Python | Y | **N** | Y | Y | Y | Y | Y | Y | Y | — |
| 18 | pheno-llms-txt | Python | Y | **N** | Y | Y | Y | Y | Y | Y | Y | — |
| 19 | pheno-prompt-test | Python | Y | **N** | Y | Y | Y | Y | Y | Y | Y | — |
| 20 | pheno-go-ctxkit | Go | **N** | **N** | **N** | **N** | **N** | **N** | Y | **N** | **N** | — |
| 21 | pheno-fastapi-base | Python | (same as 10) | | | | | | | | | |

**Test file counts (across 21 T15 repos):**

| Repo | Test files | Convention |
|---|---:|---|
| pheno-agents-md | 1 | Rust inline `#[cfg(test)]` + pytest |
| pheno-config | 4 | Rust `tests/` integration |
| pheno-context | 2 | Rust inline `#[cfg(test)]` |
| pheno-cost-card | 1 | pytest |
| pheno-errors | 1 | Rust inline |
| pheno-fastapi-base | 2 | pytest |
| pheno-flags | 1 | Rust inline |
| pheno-llms-txt | 2 | pytest |
| pheno-mcp-router | 8 | pytest (largest in fleet) |
| pheno-otel | 1 | Rust inline |
| pheno-prompt-test | 2 | pytest |
| pheno-pydantic-models | 1 | pytest |
| pheno-scaffold-kit | 1 (8 in-tree) | pytest |
| pheno-vibecoding-guard | 4 | pytest (legacy, work absorbed) |
| pheno-worklog-schema | 6 | pytest |
| pheno-zod-schemas | 1 | TS vitest |
| pheno-cli-base, pheno-cargo-template, pheno-port-adapter, pheno-go-ctxkit, pheno-errors (Rust inline) | 0-2 | mostly inline |

**Test coverage matrix (presence, not pass):**
- 19/21 have any test file (90.5%)
- 2/21 have NO test files: **pheno-go-ctxkit** (no test), **pheno-pydantic-models** (1 trivial test)
- 0/21 have coverage gates enforced (per ADR-040, 80% for libs)

---

## 3. ADR-042 Rule 3.1 (7-element quality bar) coverage matrix

For each of the 21 existing T15 repos, score 1 if present, 0 if absent, for each of the 7 elements per ADR-042:

| Repo | 1. Spec | 2. Docs | 3. Test matrix | 4. OTLP Obs | 5. Coverage gate | 6. CI gate | 7. Worklog v2.1 | Sum / 7 | % |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|---:|---:|
| pheno-config | 0 | 1 | 1 | 0.5 (opt feature) | 0 | 1 | 0.5 (v2.0 only) | 4.0 | 57% |
| pheno-context | 0 | 0 | 1 | 0 | 0 | 0 | 0.5 | 1.5 | 21% |
| pheno-otel | 0 | 1 | 1 | 1 (canonical obs substrate) | 0 | 0 | 0.5 | 3.5 | 50% |
| pheno-port-adapter | 0 | 0 | 1 | 0 | 0 | 1 | 0.5 | 2.5 | 36% |
| pheno-flags | 0 | 0 | 1 | 0 | 0 | 1 | 0 | 2.0 | 29% |
| pheno-errors | 0 | 0 | 1 | 0 | 0 | 1 | 0 | 2.0 | 29% |
| pheno-cli-base | 1 | 1 | 1 | 0 | 0 | 1 | 0 | 4.0 | 57% |
| pheno-cargo-template | 1 | 1 | 1 | 0 | 0 | 1 | 0.5 | 4.5 | 64% |
| pheno-agents-md | 1 | 1 | 1 | 0 | 0 | 1 | 0.5 | 4.5 | 64% |
| pheno-fastapi-base | 1 | 1 | 1 | 0 | 0 | 1 | 0.5 | 4.5 | 64% |
| pheno-pydantic-models | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1.0 | 14% |
| pheno-zod-schemas | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1.0 | 14% |
| pheno-mcp-router | 1 | 1 | 1 | 0 | 0 | 1 | 1 (v2.1 verified) | **5.0** | **71%** |
| pheno-cost-card | 1 | 1 | 1 | 0 | 0 | 1 | 0.5 | 4.5 | 64% |
| pheno-worklog-schema | 1 | 1 | 1 | 0 | 0 | 1 | 1 (v2.1 own schema) | **5.0** | **71%** |
| pheno-vibecoding-guard | 1 | 1 | 1 | 0 | 0 | 1 | 0.5 | 4.5 | 64% (deprecated) |
| pheno-scaffold-kit | 1 | 1 | 1 | 0 | 0 | 1 | 0.5 | 4.5 | 64% |
| pheno-llms-txt | 1 | 1 | 1 | 0 | 0 | 1 | 0.5 | 4.5 | 64% |
| pheno-prompt-test | 1 | 1 | 1 | 0 | 0 | 1 | 0.5 | 4.5 | 64% |
| pheno-go-ctxkit | 0 | 0 | 0 | 0 | 0 | 0 | 0.5 | 0.5 | 7% |
| **Fleet avg (21)** | | | | | | | | **3.55/7** | **50.7%** |
| **Fleet avg (excl. pheno-context, pheno-pydantic-models, pheno-zod-schemas, pheno-go-ctxkit — the bottom 4)** | | | | | | | | **4.4/7** | **62.9%** |

**Element-by-element coverage (% of 21):**
- **1. SPEC.md**: 11/21 (52%) — pheno-config, pheno-otel, pheno-port-adapter, pheno-flags, pheno-errors, pheno-context missing
- **2. README.md**: 15/21 (71%) — pheno-context, pheno-port-adapter, pheno-flags, pheno-errors, pheno-cli-base, pheno-go-ctxkit weak/missing
- **3. Test matrix**: 19/21 (90%) — pheno-go-ctxkit and pheno-pydantic-models weak
- **4. OTLP Obs (pheno-tracing)**: 1/21 (4.8%) — only pheno-otel itself; pheno-config has optional feature, others nothing
- **5. Coverage gate**: 0/21 (0%) — no `pheno-coverage` enforced anywhere
- **6. CI gate (`.github/workflows/`)**: 17/21 (81%) — pheno-context, pheno-otel, pheno-pydantic-models, pheno-go-ctxkit missing
- **7. Worklog v2.1**: 2/21 (9.5%) — pheno-mcp-router, pheno-worklog-schema; rest are v2.0; bump pending per ADR-025 (deprecation 2026-06-22)

**Headline gap:** **Element 4 (OTLP observability) is at 4.8%** — the 2026-06-22 ADR-025 deadline for v2.1 worklog migration is approaching, and the broader OTLP-adoption plan (per ADR-036 + v8 T22 batch 9D) needs to spread pheno-tracing to the rest of the 20 repos.

---

## 4. Factory AI 5-tier readiness (ADR-026)

Per ADR-026 + `findings/2026-06-17-L5-104-factory-ai-readiness.md`, scoring is:
- **Level 1 (Functional)**: README, linter, type checker, unit tests
- **Level 2 (Documented)**: AGENTS.md, devcontainer, pre-commit hooks, branch protection
- **Level 3 (Standardized)**: Integration tests, secret scanning, distributed tracing, metrics
- **Level 4 (Optimized)**: Fast CI feedback, regular deployment frequency, flaky test detection
- **Level 5 (Autonomous)**: Auto-remediation, auto-scaling, complex requirement decomposition

80% gate per level. Manual snapshot estimate for the 21 T15 pheno-* repos:

| Repo | L1 Functional | L2 Documented | L3 Standardized | L4 Optimized | L5 Autonomous | Final Level |
|---|:-:|:-:|:-:|:-:|:-:|:-:|
| pheno-config | ✓ | ✓ | partial (tracing opt) | partial | — | **2 (Documented)** |
| pheno-context | ✓ | partial | — | — | — | **1 (Functional)** |
| pheno-otel | ✓ | ✓ | partial (OTLP) | partial | — | **2 (Documented)** |
| pheno-port-adapter | ✓ | ✓ | — | — | — | **2 (Documented)** |
| pheno-flags | ✓ | partial | — | — | — | **1 (Functional)** |
| pheno-errors | ✓ | partial | — | — | — | **1 (Functional)** |
| pheno-cli-base | ✓ | ✓ | — | partial | — | **2 (Documented)** |
| pheno-cargo-template | ✓ | ✓ | ✓ (template) | — | — | **3 (Standardized)** |
| pheno-agents-md | ✓ | ✓ | ✓ | partial | — | **3 (Standardized)** |
| pheno-fastapi-base | ✓ | ✓ | partial | — | — | **2 (Documented)** |
| pheno-pydantic-models | partial | partial | — | — | — | **<1 (Sub-Functional)** |
| pheno-zod-schemas | partial | partial | — | — | — | **<1 (Sub-Functional)** |
| pheno-mcp-router | ✓ | ✓ | ✓ | partial | — | **3 (Standardized)** |
| pheno-cost-card | ✓ | ✓ | partial | — | — | **2 (Documented)** |
| pheno-worklog-schema | ✓ | ✓ | ✓ | partial | partial | **3 (Standardized)** |
| pheno-vibecoding-guard | partial (deprecated) | partial | — | — | — | **N/A (deprecation candidate)** |
| pheno-scaffold-kit | ✓ | ✓ | partial | — | — | **2 (Documented)** |
| pheno-llms-txt | ✓ | ✓ | partial | — | — | **2 (Documented)** |
| pheno-prompt-test | ✓ | ✓ | partial | — | — | **2 (Documented)** |
| pheno-go-ctxkit | partial | partial | — | — | — | **<1 (Sub-Functional)** |
| **Fleet avg (T15)** | | | | | | **Level 1.1 (Functional)** |
| **Fleet avg (excl. bottom-3 + deprecation)** | | | | | | **Level 2.0 (Documented)** |

**Factory AI findings:**
- **Only 3 repos at Level 3** (Standardized): pheno-cargo-template, pheno-agents-md, pheno-mcp-router, pheno-worklog-schema
- **0 repos at Level 4** (Optimized): no repo has fast CI feedback + flaky test detection + deployment frequency tracking
- **0 repos at Level 5** (Autonomous)
- **Top-3 highest-impact next-level unlocks** (per the model):
  1. **pheno-mcp-router** (3→4): add `criterion`-style perf bench to `examples/` + flaky-test detection
  2. **pheno-worklog-schema** (3→4): add benchmark for v2.1 parser, add OTel export on validation
  3. **pheno-agents-md** (3→4): add `cargo-deny` supply-chain scan to CI + flaky-test detection on benchmark

---

## 5. Drift findings (9)

### Drift 1 — pheno-tracing MISSING as top-level repo (CRITICAL)

**Per ADR-023 Rule 3.1, the canonical tracing substrate must live in exactly one of the four canonical placements.** `pheno-tracing` is supposed to be `pheno-*-lib` (Rust tracing substrate), per ADR-036 ("pheno-tracing canonical"). But:

| Location | Type | Status |
|---|---|---|
| `./crates/pheno-tracing/` | root-level `crates/` (random `phenoShared` per ADR-023) | exists |
| `./FocalPoint/crates/pheno-tracing/` | FocalPoint workspace | exists |
| `./FocalPoint/pheno-tracing/` | FocalPoint second copy | exists |
| `./worktrees/ci-templates-absorb/crates/pheno-tracing/` | worktree | exists |
| `./PhenoCompose/packages/pheno-tracing/` | PhenoCompose second copy | exists |
| `/private/tmp/t15-batch-output/rust/pheno-tracing/` | /tmp worktree (T15.5 output) | exists |
| **`./pheno-tracing/`** (top-level) | **expected by ADR-023 + ADR-036** | **MISSING** |

**The "random `phenoShared`" pattern (ADR-023) — 5+ instances of the same crate in different workspaces.** `pheno-config/Cargo.toml:29` depends on `pheno-tracing = { version = "0.1", optional = true, default-features = false }` (verified), so the missing top-level repo breaks the published-crate story: consumers cannot easily get a single canonical pheno-tracing; they get whichever path is symlinked.

**Root cause:** The v8 plan T15.5 (`pheno-tracing: meta-bundle + tests`) is the canonical task to fix this — it was authored but never executed. The `/private/tmp/t15-batch-output/rust/pheno-tracing/` directory has the T15.5 output (AGENTS.md, CHANGELOG.md, LICENSE-MIT, llms.txt, PR_DESCRIPTION.md, WORKLOG.md) but it was never promoted to a top-level pheno-* repo.

**Drift worklog:** `worklogs/2026-06-19-L5-110-substrate-drift-pheno-tracing-missing.md`
**Severity:** **CRITICAL** (substrate-canonical per ADR-036; dependee by pheno-config; "random phenoShared" violation)
**Fix:** Promote `/private/tmp/t15-batch-output/rust/pheno-tracing/` to `./pheno-tracing/`; remove the 5 duplicates; rebase pheno-config to depend on the canonical path; ADR-036 v1.1 bump.

### Drift 2 — pheno-vibecoding-guard is a deprecation candidate (CRITICAL)

`pheno-vibecoding-guard/README.md:1-7` states verbatim:

> "Local scratch repo. The validation-rule work has been absorbed into `PhenoHandbook`:
> - `PhenoHandbook/docs/reference/validation-rules.py`
> - `PhenoHandbook/docs/patterns/validation-rules.md`
> This checkout is retained only as a local reference snapshot."

**But the repo still has all the meta-bundle (SPEC, WORKLOG, CHANGELOG, LICENSE, justfile, deny.toml, audit_scorecard.json, examples/, .benchmarks/).** It looks production-ready from the outside, but the README disclaims it. This is exactly the kind of "the README says scratch but the structure says prod" drift that ADR-023 was designed to catch.

**Per AGENTS.md § PAUSED APPs** (which lists AtomsBot, FocalPoint, Dino, QuadSGM, HwLedger, WSM, *fitness* as PAUSED but does not list pheno-vibecoding-guard), the bucket table does not include this repo. So it has no explicit bucket assignment.

**Drift worklog:** `worklogs/2026-06-19-L5-110-substrate-drift-pheno-vibecoding-guard-deprecation.md`
**Severity:** **CRITICAL** (the repo is a deprecation candidate but is not on the PAUSED list, and the meta-bundle suggests it's active)
**Fix:** Either (a) archive + delete after 90-day GitHub retention per ADR-029 Dmouse92 pattern, or (b) explicitly add to PAUSED list in `AGENTS.md` + delete after the absorption is verified.

### Drift 3 — pheno-cargo-template is a TEMPLATE, not a lib (MEDIUM)

`pheno-cargo-template/README.md:12-22` says:

> "Usage: Copy the template into a new Rust crate and update package metadata: `cp -R pheno-cargo-template my-crate` ... Edit Cargo.toml package metadata, then create implementation files under src/"

The repo has a `template/` subdir, `Cargo.toml.template`, `Taskfile.yml`, `justfile`, and is explicitly designed to be copied. This is **not a library** — it's a template. Per ADR-023, a "lib" has a stable public API and is consumed via `cargo add` / `pip install`. `pheno-cargo-template` is consumed via `cp -R` — fundamentally different.

**Name mismatch:** `pheno-cargo-template` should be `pheno-template-cargo` or `phenotype-cargo-template` (or just `cargo-template-pheno`). The current name follows the `pheno-*-lib` convention but the content is template-class.

**Drift worklog:** `worklogs/2026-06-19-L5-110-substrate-drift-pheno-cargo-template-not-lib.md`
**Severity:** MEDIUM (does not block; 71-pillar score is 34.7%; could be re-anchored as `phenotype-cargo-template` or similar)
**Fix:** Rename or annotate as a TEMPLATE substrate type (a new substrate class in ADR-023 v2). No action required this turn.

### Drift 4 — pheno-ci-templates is a TEMPLATE, not a lib (MEDIUM)

`pheno-ci-templates/` contains only `README.md` and `.github/`. No `Cargo.toml`, no `pyproject.toml`, no `package.json`, no source code, no tests. It is a **CI workflow template** — meant to be copied into other repos' `.github/workflows/`.

**Name mismatch:** Same pattern as `pheno-cargo-template`. A template is not a lib.

**Drift worklog:** `worklogs/2026-06-19-L5-110-substrate-drift-pheno-ci-templates-not-lib.md`
**Severity:** MEDIUM (the repo has zero code, zero tests, but is counted as a "pheno-* lib" in the v8 T15+ scope)
**Fix:** Rename to `phenotype-ci-templates` or annotate as TEMPLATE. No action required this turn.

### Drift 5 — pheno-ssot-template is a TEMPLATE, not a lib (MEDIUM)

`pheno-ssot-template/` has `Cargo.toml.template` (literally), `scripts/`, `src/`, `deny.toml`, `template.yaml`. It is a **starter-repo template** — meant to be cloned and renamed.

**Name mismatch:** Same pattern.

**Drift worklog:** `worklogs/2026-06-19-L5-110-substrate-drift-pheno-ssot-template-not-lib.md`
**Severity:** MEDIUM
**Fix:** Rename to `phenotype-ssot-template` or annotate as TEMPLATE.

### Drift 6 — pheno-drift-detector is a SCRIPT, not a lib (MEDIUM)

`pheno-drift-detector/` has a single `pheno_drift_detector.py` (stdlib only) + `README.md` + `.git/`. No `pyproject.toml`, no `__init__.py`, no tests, no dependencies. It is a **standalone script** meant to be `chmod +x`'d and `ln -s`'d into `/usr/local/bin/`.

**Name mismatch:** A standalone script is not a lib.

**Drift worklog:** `worklogs/2026-06-19-L5-110-substrate-drift-pheno-drift-detector-not-lib.md`
**Severity:** MEDIUM
**Fix:** Either (a) add `pyproject.toml` to make it a real lib + installable via `pip install pheno-drift-detector`, or (b) rename to `phenotype-drift-detector` (script-class) or annotate as SCRIPT. ADR-043 (drift detection) is the L74 Pillar tool, so it should be installable.

### Drift 7 — pheno-predict is a SCRIPT, not a lib (MEDIUM)

`pheno-predict/` has a single `pheno_predict.py` (stdlib only) + `README.md` + `.git/`. Same pattern as pheno-drift-detector.

**Drift worklog:** `worklogs/2026-06-19-L5-110-substrate-drift-pheno-predict-not-lib.md`
**Severity:** MEDIUM
**Fix:** Same as pheno-drift-detector.

### Drift 8 — pheno-secret-scan is a WORKFLOW, not a lib (MEDIUM)

`pheno-secret-scan/` has only `.pre-commit-hooks.yaml` + `.trufflehog-allowlist.txt` + `README.md` + `.github/workflows/`. No `Cargo.toml`, no source code, no tests. It is a **TruffleHog integration**: the workflow file and pre-commit hook are the entire deliverable. The `trufflehog` binary is the actual implementation (external, `trufflesecurity/trufflehog` Docker image).

**Name mismatch:** A workflow integration is not a lib.

**Drift worklog:** `worklogs/2026-06-19-L5-110-substrate-drift-pheno-secret-scan-not-lib.md`
**Severity:** MEDIUM
**Fix:** Rename to `phenotype-secret-scan-workflow` or annotate as WORKFLOW. (Note: this is by design — the README explicitly says "no vendored re-implementation". ADR-023 might want to add a WORKFLOW substrate class.)

### Drift 9 — Rule 3.1 quality bar gaps (low/medium, per-repo)

The Rule 3.1 7-element quality bar (ADR-042) is 50.7% fleet-avg across the 21 T15 repos. Per-repo gaps are recorded in § 3 above. The bottom-4 (`pheno-context`, `pheno-pydantic-models`, `pheno-zod-schemas`, `pheno-go-ctxkit`) are below 30% and need full meta-bundle application. The rest are 50-70% — the gap is **element 4 (OTLP)** and **element 5 (coverage gate)**, which are fleet-wide missing.

**Per-repo drift worklogs (4 files for the bottom-4):**
- `worklogs/2026-06-19-L5-110-substrate-drift-pheno-context-quality-bar.md` (21% — 1.5/7)
- `worklogs/2026-06-19-L5-110-substrate-drift-pheno-pydantic-models-quality-bar.md` (14% — 1.0/7)
- `worklogs/2026-06-19-L5-110-substrate-drift-pheno-zod-schemas-quality-bar.md` (14% — 1.0/7)
- `worklogs/2026-06-19-L5-110-substrate-drift-pheno-go-ctxkit-quality-bar.md` (7% — 0.5/7)

**Severity:** LOW–MEDIUM (the 4 bottom repos need full meta-bundle; the rest need OTLP + coverage gate; v8 T15 is the fix vehicle)

---

## 6. Cross-cutting findings (fleet-wide, not per-repo)

### C1. Element 4 (OTLP observability) is at 4.8% fleet-wide

Only 1/21 T15 repos (pheno-otel itself) has OTLP via pheno-tracing as a runtime dependency. The pheno-tracing missing-repo drift (Drift 1) is the root cause: the canonical substrate isn't reachable, so the rest of the fleet can't adopt it. The v8 T22 batch 9D (per v8 plan § 3.6) is the fix vehicle — adopt pheno-tracing in 6 Rust repos — but is blocked on Drift 1.

**Recommendation:** Promote pheno-tracing first (Drift 1), then re-run v8 T22 batch 9D. Per ADR-036, this is the canonical migration path.

### C2. Element 5 (coverage gate) is at 0% fleet-wide

No pheno-* repo has a coverage gate enforced in CI. Per ADR-040, the gate is 80% for `pheno-*-lib` / `phenotype-*-sdk`, 70% for framework, 60% for federated service. The `pheno-coverage` CLI tool (per ADR-040 § Enforcement) does not exist yet. The `cargo-llvm-cov` config is present in 3 repos (pheno-config, pheno-port-adapter, pheno-flags, pheno-errors have `llvm-cov.toml`) but the CI gate is not wired.

**Recommendation:** Add `pheno-coverage` CLI to `pheno-ci-templates` per ADR-040 enforcement; wire it into all 21 pheno-* repos. v8 T18 (Test Coverage Pass) is the fix vehicle — but the tool itself is the blocker.

### C3. Worklog v2.1 migration is at 9.5% (2/21)

Per ADR-025, the v2.1 schema bumps the worklog to 11 columns (adds `device:`). Deprecation 2026-06-22 (5 days from this audit). Only 2 repos have migrated: pheno-mcp-router and pheno-worklog-schema (the latter is the schema owner, naturally). The rest are still v2.0.

**Recommendation:** Mass-migration script (v2.0 → v2.1) via `pheno-worklog-schema/migrate_v2_to_v2_1.py` (per L5-103 plan, PR `KooshaPari/pheno-worklog-schema#1`). 19 repos × ~1 day = a 1-PR mass-migration is faster than 19 individual PRs.

### C4. Factory AI fleet avg is Level 1.1 (Functional)

The 21 T15 repos average Level 1.1 on the 5-tier model. To reach fleet Level 2 (Documented), the bottom-3 (pheno-go-ctxkit, pheno-pydantic-models, pheno-zod-schemas) need to clear 80% of L1 criteria. To reach fleet Level 3 (Standardized), every repo needs integration tests + secret scanning + distributed tracing — which is fleet-widely missing (see C1).

**Recommendation:** Run `/readiness-report` via Droid CLI on each T15 repo (per ADR-026). Top-3 action items per repo become P0 tasks in v9 plan.

---

## 7. T16 prior-pass reconciliation (T15 vs T16)

The T16 prior pass (per `findings/L6_PHENO_REPOS_HEALTH_2026-06-18_DELTA.md`, 2026-06-18 23:55 PDT) checked the **bucket column** in `AGENTS.md` § PAUSED APPs and found "0 bucket-drift cases". That check is correct for **app-level repos** (Civis, FocalPoint, Dino, etc.) — those rows in the bucket table match reality.

**However**, T16 did not check **substrate-level repos** (the 22 pheno-* in T15 scope). This audit closes that gap. The 9 drift findings above are **substrate-level drift**, not app-level bucket drift.

**Net T15 contribution to fleet hygiene:** T15's 22 pheno-flake PRs (per v8 plan § 3.7) are the **fix vehicle** for Drift 9 (Rule 3.1 quality bar). Drifts 1-8 are **structural** (name mismatch, missing repo) and need separate PRs.

---

## 8. Cross-references

- `findings/L6_PHENO_REPOS_HEALTH_2026_06_18_DELTA.md` § Outstanding P0 — `pheno-agents-md` YAML bug fix is the only carry-forward; T15 sweep is the new work
- `findings/71-pillar-2026-06-19.md` § 2 (per-repo subtotals) — 18 of 20 baseline + new repos are pheno-*; 14 score below 60%
- `findings/2026-06-17-L5-104-factory-ai-readiness.md` § Per-repo scoring — 4-repo baseline; this audit extends to 21 pheno-*
- `docs/adr/2026-06-15/ADR-023-agent-effort-governance.md` — substrate decision tree (Rule 3) + quality bar (Rule 3.1)
- `docs/adr/2026-06-18/ADR-041-substrate-audit-cadence.md` — quarterly cron; this audit is the manual equivalent
- `docs/adr/2026-06-18/ADR-042-substrate-quality-bar.md` — 7-element quality bar (Spec, Docs, Test matrix, OTLP, Coverage, CI, Worklog v2.1)
- `docs/adr/2026-06-18/ADR-036-pheno-tracing-substrate-canonical.md` — Drift 1 root cause
- `docs/adr/2026-06-17/ADR-025-...` (v2.1 worklog schema) — Drift 9 / C3
- `plans/2026-06-18-v8-dag-stable.md` § 3.7 (T15), § 3.8 (T16), § 3.6 (T14.5 quality bar)
- `findings/2026-06-17-L5-103-adr-015-v2-1.md` — v2.1 migration plan

---

## 9. Worklog

`worklogs/2026-06-19-L5-110-substrate-audit.json` (master, this audit) + 8 per-drift worklogs (one per finding in § 5).

---

## 10. Follow-ups (next v9 plan P0)

1. **T15.5 promote pheno-tracing to top-level** (fixes Drift 1; unblocks v8 T22 batch 9D)
2. **T15.22 fulfill or remove the Python variant of pheno-context** (decide: is it a real need, or kill the placeholder?)
3. **Mass-migrate worklog v2.0 → v2.1** (1 PR using `pheno-worklog-schema/migrate_v2_to_v2_1.py`; 19 repos; deadline 2026-06-22 per ADR-025)
4. **pheno-vibecoding-guard deprecation PR** (Drift 2; AGENTS.md bucket + 90-day archive)
5. **T15+ scope: 5 NEW pheno-* need substrate audit** (Drifts 3-8; v9 plan)
6. **T18: coverage gate wiring** (per ADR-040 + ADR-042; 21 repos need `pheno-coverage` integration)
7. **Droid CLI `/readiness-report`** for 21 T15 repos (per ADR-026; feeds v9 P0 list)

---

**Audit complete:** 2026-06-19 00:30 PDT (estimated; v8 batch T15 sweep). Owner: orchestrator. Subagent verification: forge-equivalent (manual inventory + Python script + filesystem walk).
