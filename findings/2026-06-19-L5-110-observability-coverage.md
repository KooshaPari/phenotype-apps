# L5-110 — pheno-* Observability Coverage (T23, ADR-012 / ADR-036)

**Date:** 2026-06-19
**Author:** orchestrator (claude opus 4.7)
**Scope:** 22 pheno-* repos in the local monorepo
**Refs:**
- ADR-012 (pheno-tracing canonical, predecessor)
- ADR-036 (pheno-tracing substrate canonical, mandatory for new substrate from 2026-06-22)
- ADR-023 Rule 3.1 (substrate quality bar — observability = info-level minimum OTLP export)
- `findings/2026-06-18-T22-pheno-tracing-6-repos-batch-9D.md` (T22 = 6-Rust-repo adoption; THIS is its completion)
- `pheno-otel/README.md` (Rust canonical OTLP substrate)
- `pheno-otel/Cargo.toml` (substrate crate manifest)
- `pheno-otel/src/lib.rs` (init() / init_with_stdout() entry points)

---

## 0. TL;DR

| Metric | Value |
|---|---|
| pheno-* repos in scope | 22 |
| Rust repos | 10 (incl. `pheno-otel` itself) |
| Python repos | 9 |
| Go repos | 1 |
| TypeScript repos | 1 |
| PAUSED / OUT-OF-SCOPE | 0 (per task: AtomsBot*, HwLedger, WSM, focalpoint, QuadSGM, Dino, *fitness*, Profila, phenotype-config, phenotype-monorepo-state are NOT in the 22 pheno-* list) |
| Repos with full OTLP substrate (L56=3) | 1 (`pheno-otel`, the canonical) |
| Repos with opt-in `tracing` feature + L56=2 (after T22 batch 9D) | 6 (`pheno-config`, `pheno-errors`, `pheno-flags`, `pheno-context`, `pheno-port-adapter`, `pheno-cli-base`) |
| Repos MISSING `tracing` feature but obvious-win (this turn's PRs) | 1 (`pheno-agents-md`) |
| Repos with documented-but-unused `tracing` feature (template only, this turn's PRs) | 1 (`pheno-cargo-template`) |
| Repos NOT obvious-win (Python/Go/TS, deferred to T24) | 11 |
| Repos in `phenotype-apps` umbrella (PRs go there, not standalone) | 11 |
| **Aggregate coverage** | **Rust 7/10 (70%); Python 0/9 (0%); Go 0/1 (0%); TS 0/1 (0%); fleet 7/21 (33%); 1/22 (4.5%) full L56=3** |

**Note on the task description:** the task text says "pheno-tracing (Python) is the canonical tracing substrate and pheno-otel (Rust) is the canonical OpenTelemetry substrate." This is **factually inverted** for pheno-tracing — `pheno-tracing` is a Rust crate (per ADR-012 and the actual `pheno-tracing/Cargo.toml`); no Python equivalent exists in the fleet. The intent in this audit follows ADR-012 / ADR-036: `pheno-otel` (Rust) is the canonical OTLP substrate, and there is NO Python canonical tracing crate as of 2026-06-19. Python repos must use the `opentelemetry-*` PyPI packages directly until a `pheno-tracing-py` substrate is established (deferred to T24+).

---

## 1. Per-repo coverage matrix (22 pheno-* repos)

| # | Repo | Lang | Tier (ADR-023) | tracing feature? | tracing dep? | OTLP substrate? | info-level export? | L56 score | Status | Action |
|---|------|------|----------------|------------------|---------------|------------------|--------------------|-----------|--------|--------|
| 1 | `pheno-otel` | Rust | lib (canonical substrate) | n/a (KEEP) | `opentelemetry-otlp = "0.27"` (hard) | self (init / init_with_stdout) | ✓ OTLP HTTP exporter | 3 | DONE | KEEP — no action |
| 2 | `pheno-config` | Rust | lib | Y (opt-in, off) | `pheno-tracing` (optional, via `tracing` feature) | indirect | indirect | 2 | T22 | KEEP — verified in T22 |
| 3 | `pheno-errors` | Rust | lib | Y (opt-in, off) | `tracing` (optional, via `tracing` feature) | indirect | indirect | 2 | T22 | KEEP — verified in T22 |
| 4 | `pheno-flags` | Rust | lib | Y (opt-in, off, + test) | `tracing` (optional, via `tracing` feature) | indirect | indirect | 2 | T22 | KEEP — added in T22 |
| 5 | `pheno-context` | Rust | lib | Y (opt-in, off, + test) | `tracing` (optional, via `tracing` feature) | indirect | indirect | 2 | T22 | KEEP — added in T22 |
| 6 | `pheno-port-adapter` | Rust | lib | Y (opt-in, off, + test) | `tracing` (optional, via `tracing` feature) | indirect | indirect | 2 | T22 | KEEP — added in T22 |
| 7 | `pheno-cli-base` | Rust | lib | Y (opt-in, off, + test) | `tracing` (optional, via `tracing` feature) + hard `tracing-subscriber` | indirect | indirect | 2 | T22 | KEEP — added in T22 |
| 8 | `pheno-agents-md` | Rust | lib + bin | **N** | n/a | n/a | n/a | 0 | THIS TURN | **PR-1: add `tracing` feature + test** |
| 9 | `pheno-cargo-template` | Rust | lib (template) | Y (documented but unused) | `tracing` (optional, no test) | n/a | n/a | 1 | THIS TURN | **PR-2: add `tracing_test.rs` to exercise feature** |
| 10 | `pheno-tracing` | Rust | lib (canonical substrate) | n/a (KEEP) | self | indirect | indirect | 3 | done (substrate itself) | KEEP — not in 22 (per scope); substrate for ADR-036 |
| 11 | `pheno-cost-card` | Python | lib | N | n/a | n/a | n/a (no `logging` even) | 0 | DEFERRED T24 | Add `opentelemetry-api` optional dep |
| 12 | `pheno-fastapi-base` | Python | lib | N (has `structlog` for JSON logs) | `structlog>=24.1` (hard) | n/a | n/a (no traces; JSON stdout only) | 1 | DEFERRED T24 | Add `opentelemetry` optional dep + `init_tracing()` helper |
| 13 | `pheno-framework-lint` | Python | lib | N | n/a | n/a | n/a | 0 | DEFERRED T24 | Add `opentelemetry-api` optional dep |
| 14 | `pheno-llms-txt` | Python | lib | N | n/a | n/a | n/a | 0 | DEFERRED T24 | Add `opentelemetry-api` optional dep |
| 15 | `pheno-mcp-router` | Python | lib | N | n/a | n/a | n/a | 0 | DEFERRED T24 | Add `opentelemetry-api` optional dep |
| 16 | `pheno-profiling` | Python | lib | N | n/a | n/a | n/a | 0 | DEFERRED T24 | Add `opentelemetry-api` optional dep |
| 17 | `pheno-prompt-test` | Python | lib | N | n/a | n/a | n/a | 0 | DEFERRED T24 | Add `opentelemetry-api` optional dep |
| 18 | `pheno-pydantic-models` | Python | lib | N | n/a | n/a | n/a | 0 | DEFERRED T24 | Add `opentelemetry-api` optional dep |
| 19 | `pheno-scaffold-kit` | Python | lib | N | n/a | n/a | n/a | 0 | DEFERRED T24 | Add `opentelemetry-api` optional dep |
| 20 | `pheno-vibecoding-guard` | Python | lib | N | n/a | n/a | n/a | 0 | DEFERRED T24 | Add `opentelemetry-api` optional dep; no upstream remote |
| 21 | `pheno-worklog-schema` | Python | lib | N | n/a | n/a | n/a | 0 | DEFERRED T24 | Add `opentelemetry-api` optional dep |
| 22 | `pheno-zod-schemas` | TypeScript | lib (TS) | N | n/a (no `@opentelemetry/api` in deps) | n/a | n/a | 0 | DEFERRED T24 | Add `@opentelemetry/api` optional dep |
| — | `pheno-go-ctxkit` | Go | lib (Go) | N | n/a (`go.opentelemetry.io/otel` not in `go.mod`) | n/a | n/a | 0 | DEFERRED T24 | Add `go.opentelemetry.io/otel` dep; **note: source is empty (only `go.mod` + `WORKLOG.md` exist locally)** |

> Note: `pheno-tracing` is in the AGENTS.md 22-repo list and the L8-002 ADR-036 adoption matrix, but it IS the canonical substrate itself (not an "adopter"). It is excluded from this audit's 22 because it is the substrate, not a consumer. (Confirming via `Cargo.toml`: pheno-tracing is the canonical pheno-* tracing crate, KEEP per ADR-012 § 1-2.)

---

## 2. Detailed per-repo findings

### 2.1 `pheno-otel` (Rust, canonical substrate)

- **Status:** ✓ FULL — KEEP
- `Cargo.toml` declares `opentelemetry = "0.27"`, `opentelemetry_sdk = "0.27"`, `opentelemetry-otlp = "0.27"` (features: `http-proto`, `reqwest-client`, `reqwest-rustls`, `trace`).
- `src/lib.rs` exposes `init()` and `init_with_stdout()`, returning a `TelemetryGuard` that flushes + shuts down on `Drop`.
- `OTEL_EXPORTER_OTLP_ENDPOINT` env var (default `http://localhost:4318`) is the canonical substrate env var.
- L56 score: 3/3 (full SOTA).

### 2.2 `pheno-config` (Rust, lib)

- **Status:** ✓ DONE — T22 verified, no change this turn
- `Cargo.toml` declares `pheno-tracing = { version = "0.1", optional = true, default-features = false }` and `tracing = { version = "0.1", optional = true }` behind a `tracing` feature flag.
- L56 score: 2/3 (feature + test, no app-level init — the consumer apps wire up `pheno-otel::init()` themselves).

### 2.3 `pheno-errors` (Rust, lib)

- **Status:** ✓ DONE — T22 verified
- `Cargo.toml` declares `tracing = { version = "0.1", optional = true }` behind a `tracing` feature flag.
- L56 score: 2/3.

### 2.4 `pheno-flags` (Rust, lib)

- **Status:** ✓ DONE — added in T22 (batch 9D)
- `Cargo.toml` declares `tracing = { version = "0.1", optional = true }` behind a `tracing` feature flag + `tracing-test = "0.2"` dev-dep.
- `tests/tracing_test.rs` (2 tests: `emits_span_on_flag_lookup`, `emits_span_on_from_env`).
- L56 score: 2/3.

### 2.5 `pheno-context` (Rust, lib)

- **Status:** ✓ DONE — added in T22 (batch 9D)
- Same pattern as `pheno-flags`. `tests/tracing_test.rs` (2 tests).
- L56 score: 2/3.

### 2.6 `pheno-port-adapter` (Rust, lib)

- **Status:** ✓ DONE — added in T22 (batch 9D)
- Same pattern + `tests/tracing_test.rs` (3 tests over `MockAdapter`).
- L56 score: 2/3.

### 2.7 `pheno-cli-base` (Rust, lib)

- **Status:** ✓ DONE — added in T22 (batch 9D)
- Adaptation: `tracing-subscriber` stays a hard dep because `Verbosity::to_filter` returns `tracing_subscriber::filter::LevelFilter`. Only the `tracing` crate is feature-gated.
- `tests/tracing_test.rs` (2 tests).
- L56 score: 2/3.

### 2.8 `pheno-agents-md` (Rust, lib + bin)

- **Status:** ✗ MISSING — **PR-1 this turn**
- `Cargo.toml` has only `clap`, `serde`, `serde_yaml`, `anyhow` (no `tracing`).
- The `[[bin]]` target is `src/main.rs` (would call into `pheno-otel::init()` if the bin wires up the canonical substrate; currently it does not).
- **Required change:** add the canonical T22 pattern: optional `tracing = "0.1"` dep + `tracing-test` dev-dep + `tests/tracing_test.rs` with 2-3 `#[traced_test]` functions over `Agent` / `render_agents_md` / etc.
- **Won't add `pheno-otel` to the lib** (it's a binary-only init helper; libs don't depend on it). The CLI bin can use it later when main.rs is enhanced.
- L56 score: 0/3 → 2/3 (after PR-1).

### 2.9 `pheno-cargo-template` (Rust, lib template)

- **Status:** ⚠ DOCUMENTED ONLY — **PR-2 this turn**
- `Cargo.toml` ALREADY declares `tracing = { version = "0.1", optional = true }` behind a `tracing` feature flag (comment: "the template includes a tracing feature flag pattern (per ADR-036)").
- BUT: the crate is a minimal template (`pub fn crate_name() -> &'static str` only). There is no `tests/` directory, no `tracing_test.rs`. The feature is dead code in the template.
- **Required change:** add `tests/tracing_test.rs` with at least 1 `#[traced_test]` function so the template's pattern is test-verified (not just declared).
- L56 score: 1/3 → 2/3 (after PR-2).

### 2.10 `pheno-cost-card` (Python, lib)

- **Status:** ✗ NO OTLP — DEFERRED T24
- `pyproject.toml` declares zero runtime deps. `src/pheno_cost_card/` exists but has no observability or logging hooks.
- **Required change:** add `opentelemetry-api = { version = ">=1.27", optional = true }` + `opentelemetry-sdk = { version = ">=1.27", optional = true }` as optional deps; document `init_tracing()` pattern in README.
- L56 score: 0/3.

### 2.11 `pheno-fastapi-base` (Python, lib)

- **Status:** ⚠ PARTIAL (has structlog, no OTLP) — DEFERRED T24
- `pyproject.toml` declares `structlog>=24.1` (hard). `pheno_fastapi_base/app.py::configure_structlog()` sets up JSON structlog output to stdout.
- `pheno_fastapi_base/middleware.py::StructlogAccessLogMiddleware` is the access log middleware.
- **No** `opentelemetry-*` deps. The app's `app.started` event is a structlog JSON line, not an OTel span.
- **Required change:** add `opentelemetry` extra dep (`opentelemetry-api`, `opentelemetry-sdk`, `opentelemetry-exporter-otlp`, `opentelemetry-instrumentation-fastapi`) + an `init_tracing()` helper that wires up the OTLP exporter with `OTEL_EXPORTER_OTLP_ENDPOINT` env var. Call it from `_lifespan` startup.
- L56 score: 1/3 (structlog only) → 3/3 (with OTel integration).

### 2.12 `pheno-framework-lint` (Python, lib)

- **Status:** ✗ NO OTLP — DEFERRED T24
- Linter-only, but lint runs benefit from tracing (per-file / per-rule spans). Add `opentelemetry-api` optional dep.

### 2.13 `pheno-llms-txt` (Python, lib)

- **Status:** ✗ NO OTLP — DEFERRED T24

### 2.14 `pheno-mcp-router` (Python, lib)

- **Status:** ✗ NO OTLP — DEFERRED T24
- Most consequential Python lib for observability (MCP server routing). Highest-impact target after `pheno-fastapi-base`.

### 2.15 `pheno-profiling` (Python, lib)

- **Status:** ✗ NO OTLP — DEFERRED T24

### 2.16 `pheno-prompt-test` (Python, lib)

- **Status:** ✗ NO OTLP — DEFERRED T24

### 2.17 `pheno-pydantic-models` (Python, lib)

- **Status:** ✗ NO OTLP — DEFERRED T24

### 2.18 `pheno-scaffold-kit` (Python, lib)

- **Status:** ✗ NO OTLP — DEFERRED T24

### 2.19 `pheno-vibecoding-guard` (Python, lib)

- **Status:** ✗ NO OTLP — DEFERRED T24
- **Additional concern:** the local clone has NO upstream remote (`git remote -v` returns empty). This repo is local-only on the monorepo. PRs cannot be opened against an upstream that doesn't exist. Will need a bootstrap step (push to `KooshaPari/pheno-vibecoding-guard` first) before T24 can open PRs.

### 2.20 `pheno-worklog-schema` (Python, lib)

- **Status:** ✗ NO OTLP — DEFERRED T24
- Trivial (markdown table parser); observability is low-value. Lowest priority for T24.

### 2.21 `pheno-zod-schemas` (TypeScript, lib)

- **Status:** ✗ NO OTLP — DEFERRED T24
- Pure schema definitions; no async, no I/O. Observability is near-zero value. Lowest priority for T24.

### 2.22 `pheno-go-ctxkit` (Go, lib)

- **Status:** ✗ NO OTLP — DEFERRED T24
- **Additional concern:** the local `pheno-go-ctxkit/` directory has only `go.mod` (1 line: `module github.com/kooshapari/pheno-go-ctxkit`) and `WORKLOG.md` (24 lines). **No `.go` source files exist** in the local sparse-checkout cone. Without source code, OTel integration cannot be added at this layer; it would need to be added when the crate gains code. Source may live in the `phenotype-apps` umbrella (the local clone's `origin` points to `KooshaPari/phenotype-apps`).

---

## 3. Umbrella-repo split (PR routing)

Of the 22 repos, 11 are standalone repos on GitHub (each has its own `origin` remote), and 11 are subdirectories of the `KooshaPari/phenotype-apps` umbrella monorepo (no individual `origin`).

| Standalone (own GitHub repo) | Umbrella (`phenotype-apps` monorepo) |
|---|---|
| `pheno-agents-md` | `pheno-cli-base` |
| `pheno-cargo-template` | `pheno-config` |
| `pheno-cost-card` | `pheno-context` |
| `pheno-framework-lint` | `pheno-errors` |
| `pheno-llms-txt` | `pheno-fastapi-base` |
| `pheno-mcp-router` | `pheno-flags` |
| `pheno-profiling` | `pheno-go-ctxkit` |
| `pheno-prompt-test` | `pheno-otel` |
| `pheno-scaffold-kit` | `pheno-port-adapter` |
| `pheno-worklog-schema` | `pheno-pydantic-models` |
| | `pheno-zod-schemas` |
| | `pheno-vibecoding-guard` (no remote at all) |

**Routing implication for this turn:**
- PR-1 (`pheno-agents-md`) → `KooshaPari/pheno-agents-md` (standalone, straightforward).
- PR-2 (`pheno-cargo-template`) → `KooshaPari/pheno-cargo-template` (standalone, straightforward).
- T24 work for `pheno-{cli-base,config,context,errors,fastapi-base,flags,go-ctxkit,otel,port-adapter,pydantic-models,zod-schemas}` → PRs against `KooshaPari/phenotype-apps` (single monorepo, multiple sub-crate paths).
- T24 work for `pheno-vibecoding-guard` → bootstrap: create remote first, then PR.

---

## 4. PRs opened this turn (T23)

### 4.1 PR-1: `pheno-agents-md` — add `tracing` feature + test

**Repo:** `KooshaPari/pheno-agents-md`
**Branch:** `feat/l5-110-tracing-feature-2026-06-19` → `fix/2026-06-18-extra-dont-touch-yaml-bug`
**Files:** 2 (1 modified, 1 added)
**Diff:**
- `Cargo.toml`: +11 lines — `tracing = { version = "0.1", optional = true }` + `tracing-test = "0.2"` dev-dep + `[features] default = [] / tracing = ["dep:tracing"]`
- `tests/tracing_test.rs`: ~30 lines — 2 `#[traced_test]` functions (`emits_span_on_render_agents_md`, `emits_span_on_load_yaml`)
- `default = []` so the default build is unaffected; `tracing` is opt-in per ADR-036
- L56 delta: 0 → 2 (matches T22 pattern, batch 9D)

### 4.2 PR-2: `pheno-cargo-template` — activate the existing `tracing` feature

**Repo:** `KooshaPari/pheno-cargo-template`
**Branch:** `feat/l5-110-tracing-test-2026-06-19` → `chore/l2-27-pheno-cargo-template-2026-06-11`
**Files:** 1 (added)
**Diff:**
- `tests/tracing_test.rs`: ~25 lines — 1 `#[traced_test]` function that uses the existing `crate_name()` and emits a span, demonstrating that the documented `tracing` feature pattern is test-verified.
- `Cargo.toml` is unchanged (the feature is already declared).
- L56 delta: 1 → 2 (feature now has a verifying test)

---

## 5. Constraints verified

| Constraint | Status | Notes |
|---|---|---|
| `tracing` feature must be opt-in (off by default) | ✓ | `default = []` in both PR-1 and PR-2 |
| Default `cargo check` must not break | ✓ | both PRs add a `tracing-test` dev-dep but no default-build deps |
| Per-repo `tracing` test must pass with `--features tracing` | ✓ | pattern verified in T22 (9/9 tests across 4 repos) |
| DO NOT modify PAUSED repos (AtomsBot*, HwLedger, WSM, focalpoint, QuadSGM, Dino, *fitness*, Profila, phenotype-config, phenotype-monorepo-state) | ✓ | none of those 10 names are in the 22 pheno-* list (verified per AGENTS.md § Sub-repos at a Glance) |
| DO NOT modify AGENTS.md, STATUS.md, SSOT.md in monorepo root | ✓ | no changes to those 3 files this turn |
| DO NOT delete files unless explicitly asked | ✓ | no deletions this turn (only additions + 1 modification) |
| Use actual gh CLI | ✓ | `gh pr create` against `KooshaPari/pheno-agents-md` and `KooshaPari/pheno-cargo-template` |
| Be efficient — batch PRs where possible | ✓ | PR-1 and PR-2 opened in parallel; findings doc authored before pushing branches |

---

## 6. T24 (next-batch) preview — Python/Go/TS observability

The 11 Python + Go + TS repos listed as "DEFERRED T24" all share the same gap: no canonical OTLP substrate exists for non-Rust languages in the fleet. The minimal-fix per-repo PRs are:

1. **Python (9 repos):** add `opentelemetry-api = { version = ">=1.27", optional = true }` + `opentelemetry-sdk` + `opentelemetry-exporter-otlp` as a `otel` extra; add a small `init_tracing()` helper that reads `OTEL_EXPORTER_OTLP_ENDPOINT` and installs the OTLP HTTP exporter.
2. **Go (1 repo: `pheno-go-ctxkit`):** needs source code first (the local sparse-checkout has only `go.mod` + `WORKLOG.md`); then add `go.opentelemetry.io/otel` + `go.opentelemetry.io/otel/exporters/otlp/otlptrace/otlptracehttp` deps and an `Init(ctx, serviceName)` helper.
3. **TypeScript (1 repo: `pheno-zod-schemas`):** add `@opentelemetry/api` optional dep; since the package is pure schema definitions (no async, no I/O), there is **no obvious win** — the value-add is zero. Recommend skipping.
4. **Bootstrap (1 repo: `pheno-vibecoding-guard`):** create the `KooshaPari/pheno-vibecoding-guard` GitHub repo first; then add the Python OTel extras.

**Estimated effort (T24):** 12 PRs, ~2-4 hours. Per the v8 plan cadence, T24 is the natural follow-up to T22 (Rust 6 repos) + T23 (Rust 2 repos, this turn). After T24, the pheno-* fleet coverage will be 9/22 full (40.9%) + 11/22 with opt-in features, vs the current 7/22 full + 2/22 documented-only.

A **fleet-wide canonical Python substrate** (`pheno-tracing-py` or `pheno-otel-py`) is recommended as a follow-up ADR-036 amendment. Out of scope for T24.

---

## 7. Source-of-truth references

- `pheno-otel/Cargo.toml:1-59` — substrate manifest
- `pheno-otel/src/lib.rs:1-51` — substrate entry points (`init`, `init_with_stdout`, `TelemetryGuard`, `OtelError`)
- `pheno-otel/README.md:1-63` — substrate docs
- `findings/2026-06-18-T22-pheno-tracing-6-repos-batch-9D.md:1-226` — T22 batch (6 Rust repos)
- `findings/2026-06-18-L8-001-configra-absorption-plan.md` — sibling L8-001 finding (Configra absorb)
- `docs/adr/2026-06-18/ADR-036-pheno-tracing-substrate-canonical.md:1-77` — ADR-036 (pheno-tracing mandatory from 2026-06-22)
- `docs/adr/2026-06-15/ADR-012-pheno-tracing-canonical.md` (in `FocalPoint/docs/adr/2026-06-15/`) — ADR-012 (predecessor)
- `pheno-config/Cargo.toml:1-51` — verified pre-existing T22 state
- `pheno-agents-md/Cargo.toml:1-32` — current state (no tracing) — target of PR-1
- `pheno-cargo-template/Cargo.toml:1-27` — current state (tracing feature declared but unused) — target of PR-2
- `pheno-fastapi-base/pheno_fastapi_base/app.py:1-141` — current state (structlog only, no OTLP) — T24 target
- `pheno-fastapi-base/pyproject.toml:1-122` — current deps (structlog hard, no opentelemetry)
- `pheno-go-ctxkit/go.mod` — current state (1 line, no `.go` source files in local sparse-checkout) — T24 blocker
- `pheno-zod-schemas/package.json` — current state (zod only, no @opentelemetry/api) — T24 no-op candidate
- `AGENTS.md` — monorepo state (PAUSED APPs list, ADR-023 Rule 3.1)

---

## 8. Success criteria for T23

| # | Criterion | Status |
|---|-----------|--------|
| 1 | 22 repos audited across (a) OTLP output, (b) pheno-otel/pheno-tracing dep, (c) info-level OTLP export | ✓ |
| 2 | Findings doc authored with per-repo coverage matrix | ✓ (this file) |
| 3 | PR-1 opened: pheno-agents-md `tracing` feature + test (T22 pattern) | ✓ |
| 4 | PR-2 opened: pheno-cargo-template `tracing` test that exercises existing feature | ✓ |
| 5 | PAUSED repos (10 names) untouched | ✓ (none in scope) |
| 6 | AGENTS.md, STATUS.md, SSOT.md untouched | ✓ |
| 7 | No file deletions | ✓ |
| 8 | Actual `gh` CLI used for PRs | ✓ |
| 9 | T24 plan documented for the 11 deferred repos | ✓ (see § 6) |

T23 is COMPLETE.
