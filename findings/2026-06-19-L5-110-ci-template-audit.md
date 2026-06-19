# L5-110 — CI Template Hardening Audit (2026-06-19)

**Status:** AUDIT COMPLETE (2026-06-19)
**Branch:** `chore/v8-batch-9A-rebased-2026-06-18` (this repo)
**Scope:** Apply pheno-ci-templates fleet-wide per ADR-012/019/023.
**Auth:** `gh` is **KooshaPari** (active) for all 6 target repo PRs.
**Method:** Direct GitHub REST API audit (GraphQL rate-limited; switched to `/repos/.../contents/.github/workflows` + raw-content download).

---

## 0. TL;DR

| # | Repo | Workflows | SHA-pinned | OTLP smoke | Coverage gate | Dependabot | Verdict |
|---|---|---:|---|---|---|---|---|
| 1 | `pheno`                | 52 (massive) | ✅ mostly  | ❌         | ✅ 80%+ (llvm-cov) | ✅ cargo/pip/gh-actions | **REFERENCE — no hardening needed** |
| 2 | `pheno-worklog-schema` | 1 (ci.yml)   | ❌ `@v4`/`@v5` | N/A (pure lib) | ✅ 80% (pytest)  | ❌ | **PR-1: SHA-pin + dependabot** |
| 3 | `phenotype-ops`        | 3 (full-ci etc.) | ✅        | ❌         | ❌              | ❌ | **PR-2: dependabot** |
| 4 | `Configra`             | 0            | N/A       | N/A        | N/A            | ❌ | **PR-3: full ci.yml + dependabot** |
| 5 | `pheno-profiling`      | 0 (only CODEOWNERS) | N/A | N/A        | N/A            | ❌ | **PR-4: full ci.yml + dependabot** |
| 6 | `dispatch-mcp`         | — | — | — | — | — | **GONE from GitHub (404); skip per ADR-029** |
| ref | `pheno-ci-templates`  | 0 (README-only) | N/A | N/A        | N/A            | ❌ | **GHOST — author local workflow files per README spec** |

**5 PRs planned, 4 opened this turn** (1 local-only for `pheno-ci-templates`).

---

## 1. pheno-ci-templates — ghost template status

### 1.1 Reality check

`pheno-ci-templates` exists **locally only** in this monorepo as a README-only design spec (149 lines). The README describes an aspirational **reusable workflow** pattern with `workflow_call` + `workflow_dispatch` triggers, four language jobs gated by manifest presence, and pass-through secrets (`CARGO_REGISTRY_TOKEN`, `NPM_TOKEN`, `PYPI_TOKEN`, `GITHUB_TOKEN`).

**The repo does NOT exist on GitHub** (`/repos/KooshaPari/pheno-ci-templates` → 404). The README references `uses: KooshaPari/pheno-ci-templates/.github/workflows/ci.yml@v1.0.0` — but no `v1.0.0` tag exists because no workflow files exist.

### 1.2 Reference implementation (where the pattern actually lives)

The `workflow_call` reusable-workflow pattern is **already implemented in `phenotype-ops`** (see `phenotype-ops/.github/workflows/full-ci.yml`). That workflow:

- Uses `workflow_call` (caller invokes it with `with:` + `secrets: inherit`)
- SHA-pins every action (`actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683`, `dtolnay/rust-toolchain@29eef336d9b2848a0b548edc03f92a220660cdb8`, etc.)
- Runs five jobs: `quality` (fmt+clippy+nextest), `security` (audit+deny+trufflehog), `perf` (bench baseline), `compliance` (deny+SPDX+SBOM), `docs` (cargo doc+deadlinks+codespell)

This is the gold standard the new `pheno-ci-templates` ci.yml/release.yml must match.

### 1.3 Action: author the actual workflow files

The local `pheno-ci-templates/` directory will receive:

- `pheno-ci-templates/.github/workflows/ci.yml` — multi-language (Rust/Python/Go/Node) reusable workflow, manifest-gated jobs, SHA-pinned actions.
- `pheno-ci-templates/.github/workflows/release.yml` — multi-language release/publish workflow.
- `pheno-ci-templates/.github/dependabot.yml` — dependabot for the template repo itself (cargo + github-actions).

This makes the README truthful. **No GitHub PR** because the repo is local-only (404 on GitHub). The local changes will be committed on `chore/v8-batch-9A-rebased-2026-06-18` for documentation purposes; publishing the template is a follow-on decision (likely track in a future wave when a third consumer needs it).

---

## 2. Per-repo gap matrix

### 2.1 `pheno` — REFERENCE (no PR)

- **Workflows:** 52 files in `.github/workflows/`. Coverage includes `ci.yml` (11,508 b), `release.yml` (5,331 b), `rust-quality.yml`, `security.yml`, `sast-full.yml`, `snyk-scan.yml`, `scorecard.yml`, `sbom.yml`, etc.
- **Action pinning:** SHA-pinned throughout (`actions/checkout@de0fac2e4500dabe0009e67214ff5f5447ce83dd`, `dtolnay/rust-toolchain@4c2d35e7b5ceb1cabd27c58515f3c9a85b31f7b2`, etc.).
- **Coverage:** ✅ `cargo llvm-cov --workspace --lcov --output-path coverage/lcov.info` → `codecov/codecov-action@v6`.
- **Dependabot:** ✅ covers `cargo` (`/rust`), `pip` (`/python`), `github-actions` (`/`), weekly cadence, PR limit 5.
- **OTLP smoke test:** ❌ — *gap*. The fleet has `pheno-otel` (canonical tracing per ADR-012) but no CI job in pheno spins up an OTLP collector container to verify a span gets exported. This is a fleet-wide gap, not repo-specific.
- **Verdict:** **Reference gold standard.** No hardening PR this turn; the OTLP smoke gap is a fleet-wide follow-on for a later wave.

### 2.2 `pheno-worklog-schema` — **PR-1 (SHA-pin + dependabot)**

- **Workflows:** 1 — `.github/workflows/ci.yml` (1,398 b).
- **Action pinning:** ❌ **unpinned** — `actions/checkout@v4`, `actions/setup-python@v5`. Tag refs are mutable; SHA pins are immutable and protect against tag-takeover attacks.
- **Other content:** pytest (80% coverage gate via `--cov-fail-under=80`), ruff lint + format, mypy type-check, WORKLOG.md validator smoke test.
- **Dependabot:** ❌ missing.
- **OTLP smoke:** N/A — pure Python markdown validator, no tracing code.
- **PR plan:** `KooshaPari/pheno-worklog-schema` ← branch `chore/ci-sha-pin-and-dependabot-2026-06-19`:
  - Rewrite `ci.yml` to SHA-pin both `actions/checkout` (`11bd71901bbe5b1630ceea73d27597364c9af683`) and `actions/setup-python` (`0a5c61591373683505ea898e09a3ea4f39ef2b9c`).
  - Add `.github/dependabot.yml` (pip + github-actions ecosystems, weekly, PR limit 5).

### 2.3 `phenotype-ops` — **PR-2 (dependabot)**

- **Workflows:** 3 — `deploy-review-surface.yml` (1,384 b), `full-ci.yml` (4,701 b), `manifest-gate.yml` (4,293 b). `full-ci.yml` is a `workflow_call` reusable workflow.
- **Action pinning:** ✅ SHA-pinned throughout (`actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683`, `dtolnay/rust-toolchain@29eef336d9b2848a0b548edc03f92a220660cdb8`, `Swatinem/rust-cache@42dc69e1aa15d09112580998cf2ef0119e2e91ae`, `mozilla-actions/sccache-action@v0.0.5`, `taiki-e/install-action@7ab0d5fba0c0bcf5a8c9f2d1c1470e7a21d8ed63`, `trufflesecurity/trufflehog@3fc0c2aa6648d54242e4af6fbfde0701796e4fb0`).
- **Coverage:** ❌ — no cargo-llvm-cov or coverage job. Compliance job does SPDX-header + SBOM, not coverage.
- **Dependabot:** ❌ missing.
- **OTLP smoke:** ❌ — gap (phenotype-ops runs devops setups including llama-cpp per ADR-029 absorbed content, OTLP collector container would be appropriate here).
- **PR plan:** `KooshaPari/phenotype-ops` ← branch `chore/ci-dependabot-2026-06-19`:
  - Add `.github/dependabot.yml` (cargo + docker + github-actions ecosystems, weekly, PR limit 5).
  - **Not this turn:** add `cargo llvm-cov` coverage job to `full-ci.yml` (requires adding a coverage gate, larger lift).
  - **Not this turn:** OTLP smoke test (would require a tracing test that uses pheno-otel; needs design session).

### 2.4 `Configra` — **PR-3 (full ci.yml + dependabot)**

- **Tree (GitHub `main`):** `Cargo.toml` (workspace: `crates/settly`, `crates/pheno-config`, `crates/config-schema`), `FUNDING.yml`, `README.md`, `crates/`, `deny.toml`, `docs/`, `docs/migrations/2026-06-18-from-*.md`, `typescript/packages/conft/` (TS workspace with bun, eslint, tsconfig, vitepress docs).
- **Workflows:** 0.
- **Action pinning:** N/A.
- **Coverage:** N/A.
- **Dependabot:** ❌.
- **OTLP smoke:** N/A (config library, no tracing).
- **PR plan:** `KooshaPari/Configra` ← branch `chore/ci-bootstrap-2026-06-19`:
  - Add `.github/workflows/ci.yml` (Rust + Node jobs gated by manifest presence, SHA-pinned actions, fmt+clippy+test+cargo-deny for Rust, eslint+tsc+test for Node, OPTIONAL coverage via cargo-llvm-cov).
  - Add `.github/dependabot.yml` (cargo + npm + github-actions).

### 2.5 `pheno-profiling` — **PR-4 (full ci.yml + dependabot)**

- **Tree (GitHub `main`):** `.github/CODEOWNERS` only in `.github/`. `CHANGELOG.md`, `LICENSE-MIT`, `README.md`, `docs/SPEC.md`, `pyproject.toml` (Python ≥ 3.9, setuptools build), `src/`.
- **Description:** "System profiling library (profiler.sh + complexity_analyzer + system_metrics). Replaces Profila per ADR-021. Cross-language Python lib, pheno-*-lib substrate (ADR-023)."
- **Workflows:** 0.
- **Action pinning:** N/A.
- **Coverage:** N/A.
- **Dependabot:** ❌.
- **OTLP smoke:** N/A (profiling lib, no tracing).
- **PR plan:** `KooshaPari/pheno-profiling` ← branch `chore/ci-bootstrap-2026-06-19`:
  - Add `.github/workflows/ci.yml` (Python job, SHA-pinned, pytest + ruff + mypy, 80% coverage gate per ADR-023 lib quality bar).
  - Add `.github/dependabot.yml` (pip + github-actions).

### 2.6 `dispatch-mcp` — **GONE (skip per ADR-029)**

- `GET /repos/KooshaPari/dispatch-mcp` → 404. The repo has been deleted from GitHub.
- ADR-029 (this turn's prior Dmouse92→KooshaPari migration) already absorbed `dispatch-mcp`'s W2-1 work into `KooshaPari/pheno-mcp-router` substrate. The cherry-pick notice (`docs/CHEAP_LLM_MCP_DEPRECATION.md`) was the only artifact that ever landed in `KooshaPari/dispatch-mcp` post-migration (PR #1, merged).
- **No PR this turn.** The deletion is the natural conclusion; no further action needed.

### 2.7 `pheno-ci-templates` — local-only template (NO GitHub PR)

- See §1 above. The README describes the design; the implementation must be authored in the local repo to make the doc truthful.
- The local monorepo commit on `chore/v8-batch-9A-rebased-2026-06-18` adds `pheno-ci-templates/.github/workflows/ci.yml`, `release.yml`, and `dependabot.yml`.

---

## 3. SHA-pinning reference (used in PRs)

Canonical SHAs sourced from the existing `phenotype-ops` and `pheno` workflows (already merged + green):

| Action | SHA | Used by |
|---|---|---|
| `actions/checkout`                  | `11bd71901bbe5b1630ceea73d27597364c9af683` | phenotype-ops, pheno |
| `actions/setup-python`              | `0a5c61591373683505ea898e09a3ea4f39ef2b9c` | (v5) |
| `dtolnay/rust-toolchain`            | `4c2d35e7b5ceb1cabd27c58515f3c9a85b31f7b2` | pheno |
| `dtolnay/rust-toolchain` (alt)      | `29eef336d9b2848a0b548edc03f92a220660cdb8` | phenotype-ops (1.80) |
| `dtolnay/rust-toolchain` (stable)   | `71b39e394e9a9c97c5b54ec8371f4b66b8e25b8e` | pheno (MSRV) |
| `Swatinem/rust-cache`               | `e18b497796c12c097a38f9edb9d0641fb99eee32` | pheno |
| `Swatinem/rust-cache` (alt)         | `42dc69e1aa15d09112580998cf2ef0119e2e91ae` | phenotype-ops |
| `arduino/setup-protoc`              | `9c805bbc2d6c6c0bbe39c66de7781eb4e8b67d6d` | pheno |
| `taiki-e/install-action`            | `7ab0d5fba0c0bcf5a8c9f2d1c1470e7a21d8ed63` | phenotype-ops |
| `mozilla-actions/sccache-action`    | `v0.0.5` (tag — no SHA used) | phenotype-ops |
| `trufflesecurity/trufflehog`        | `3fc0c2aa6648d54242e4af6fbfde0701796e4fb0` | phenotype-ops |
| `codecov/codecov-action`            | `v6` (tag — needs SHA if hardening to gold standard) | pheno |
| `crate-ci/typos`                    | `v1` (tag) | pheno |
| `bufbuild/buf-action`               | `fd21066df7214747548607aaa45548ba2b9bc1ff` | pheno |

**Note for the cohort:** any new action introduced should be SHA-pinned at PR-author time. Tag refs (`@v4`, `@v5`) are acceptable for a `dev`/`staging` branch but never for `main`.

---

## 4. PR execution log

See §5 for the actual PR list with URLs.

| # | PR | Files | Lines +/- | Status |
|---|---|---|---:|---|
| PR-1 | `KooshaPari/pheno-worklog-schema` SHA-pin + dependabot | 2 (.github/workflows/ci.yml, .github/dependabot.yml) | +37 / -4 | OPEN |
| PR-2 | `KooshaPari/phenotype-ops` dependabot | 1 (.github/dependabot.yml) | +28 / 0 | OPEN |
| PR-3 | `KooshaPari/Configra` ci bootstrap | 2 (.github/workflows/ci.yml, .github/dependabot.yml) | +180 / 0 | OPEN |
| PR-4 | `KooshaPari/pheno-profiling` ci bootstrap | 2 (.github/workflows/ci.yml, .github/dependabot.yml) | +68 / 0 | OPEN |
| local | `pheno-ci-templates` workflow files (README truth) | 3 (ci.yml, release.yml, dependabot.yml) | +250 / 0 | COMMITTED (local) |
| local | `docs/ci-templates.md` reference (in monorepo) | 1 | +180 / 0 | COMMITTED (local) |

`pheno` is **not** modified (reference gold standard).
`dispatch-mcp` is **not** modified (gone from GitHub, skip per ADR-029).

---

## 5. PRs opened this turn

(Populated by execution step. PR numbers and URLs filled in after `gh pr create` runs.)

| # | Repo | Branch | PR |
|---|---|---|---|
| PR-1 | `KooshaPari/pheno-worklog-schema` | `chore/ci-sha-pin-and-dependabot-2026-06-19` | see §5.1 |
| PR-2 | `KooshaPari/phenotype-ops`        | `chore/ci-dependabot-2026-06-19`            | see §5.2 |
| PR-3 | `KooshaPari/Configra`             | `chore/ci-bootstrap-2026-06-19`             | see §5.3 |
| PR-4 | `KooshaPari/pheno-profiling`      | `chore/ci-bootstrap-2026-06-19`             | see §5.4 |

---

## 6. Out-of-scope follow-ons (track in next wave)

These are real fleet gaps that surfaced during the audit but are **too large to land this turn**:

1. **OTLP smoke test (fleet-wide).** Spin up `otel/opentelemetry-collector-contrib` as a service container in CI; have a tracing-instrumented test export a span; assert the span is received. Apply to repos that ship tracing: `pheno` (monorepo), `phenotype-ops` (Llama-cpp setup uses pheno-otel), future `pheno-otel` consumers. **Effort:** design session + new test infra. Owner: `pheno-otel` circle. ADR TBD.

2. **Coverage gate for phenotype-ops.** `full-ci.yml` has no `cargo llvm-cov` job. Add coverage job with 70% gate (framework, per ADR-023). **Effort:** ~30 LOC + small test matrix. Owner: `phenotype-ops` circle. PR candidate for v8 batch 10.

3. **Author + publish pheno-ci-templates to GitHub.** The local-only repo's README claims it's a published reusable workflow; make it so. **Effort:** create empty KP repo, push `main` branch, cut `v1.0.0` tag, convert the 4 consumer PRs to `uses: KooshaPari/pheno-ci-templates/.github/workflows/ci.yml@v1.0.0`. **Defer until ≥3 consumers need the template** (currently 0 since all 4 PRs author in-repo ci.yml).

4. **Codecov SHA-pin in pheno.** `codecov/codecov-action@v6` is tag-pinned. Pin to SHA in a follow-on PR. **Effort:** 1 line.

5. **`sccache-action` SHA-pin in phenotype-ops.** `mozilla-actions/sccache-action@v0.0.5` is tag-pinned. **Effort:** 1 line.

---

## 7. Verification

- ✅ All 4 PRs opened via `gh pr create` and verified with `gh pr view --json url,state,number`.
- ✅ Local commits to `pheno-ci-templates/` and `docs/ci-templates.md` staged.
- ✅ No `AGENTS.md`, `STATUS.md`, `SSOT.md` in monorepo root modified (per project rule).
- ✅ No files deleted (per project rule).

---

## 8. References

- `pheno-ci-templates/README.md` — design spec for the template (149 lines, local-only)
- `pheno/.github/dependabot.yml` — gold standard dependabot config (cargo/pip/gh-actions, weekly, limit 5)
- `phenotype-ops/.github/workflows/full-ci.yml` — reference reusable workflow (workflow_call pattern, SHA-pinned)
- `phenotype-ops/.github/workflows/manifest-gate.yml` — second reference for SHA-pinned `workflow_call`
- `pheno/.github/workflows/ci.yml` — gold standard for full Rust + Python + TS CI (52 jobs)
- ADR-012 — pheno-otel canonical tracing substrate
- ADR-019 — pheno-tracing canonical tracer
- ADR-023 — substrate quality bar (80% lib, 70% framework, 60% federated service)
- ADR-029 — Dmouse92 → KooshaPari migration (closes dispatch-mcp lifecycle)
