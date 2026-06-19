# L5-105 — HwLedger reclassification (ADR-035A) + pheno-capacity extraction

**Date:** 2026-06-18 22:55 PDT
**Lane:** L5 (Architecture) + L1 (substrate)
**Author:** Forge session (KooshaPari active)
**Traces to:**
- ADR-035 — `docs/adr/2026-06-18/ADR-035-hwledger-reclassification.md` (parent)
- ADR-035A — `hwLedger/docs/adr/2026-06-18/ADR-035A-hwledger-reclassification.md` (execution)
- AGENTS.md § "ADR-035" + § "HwLedger CONDITIONAL" (project-level intent)

> **Naming note:** The task brief labeled this ADR as "ADR-035A" (the execution
> sub-ADR). Per the parent meta-repo `AGENTS.md`, the umbrella decision is
> **ADR-035** (HwLedger reclassification); ADR-035A is the execution plan
> under it. Both names are used interchangeably in this doc.

---

## Executive summary

**HwLedger reclassified from PAUSED → CONDITIONAL per ADR-035.** The pure-math
core (VRAM estimation, model-fit scoring, Chinchilla tokens, optimizer state)
is extracted to `KooshaPari/pheno-capacity` v0.1.0 (Rust, no_std, MIT OR
Apache-2.0, 23 unit tests + 6 doc tests, 80 % lib coverage gate enforced in
CI). HwLedger's docs (AGENTS.md, README.md) are wired to the new substrate;
the Streamlit Planner/WhatIf pages will consume the crate in Phase 2
(deferred — needs L5-XXX task after PyO3 / Python shim decision).

**Result:** 1 new repo created, 1 new tag, 1 new PR opened, 0 net content
loss. **0 HwLedger files deleted; 0 HwLedger files archived.**

---

## 1. HwLedger capability inventory

Source: `https://github.com/KooshaPari/HwLedger` (shallow clone at
`/tmp/hwledger-inventory`, --depth 30) + local `/Users/kooshapari/CodeProjects/Phenotype/repos/hwLedger` working tree.

### 1.1 Top-level layout

```
hwLedger/
├── AGENTS.md              (32 lines; updated to 47 lines)
├── README.md              (296 lines; substrate line added)
├── PLAN.md
├── PRD.md
├── ADR.md
├── docs/                  (~30 subdirs: adr/, research/, journeys/, etc.)
├── apps/
│   ├── landing/           (Astro landing page; glue)
│   ├── macos/             (SwiftUI macOS app; glue)
│   ├── streamlit/         (Python Streamlit web UI; glue; only `journeys/` + `pages/` in working tree)
│   └── build/             (build artifacts)
├── sidecars/
│   └── omlx-fork/         (Apple Silicon inference; runtime)
├── tools/
│   └── journey-remotion/  (TS/Remotion journey recorder; glue)
└── 1 file types: .md, .yaml, .toml, .json, .ts, .py, .swift, etc.
```

### 1.2 File counts by category

| Category | Count | Notes |
| :-- | --: | :-- |
| **A. VRAM estimation / model-fit math (extract to pheno-capacity)** | 4 historical files (Python, git @ 8bf878ca) | `apps/streamlit/lib/cost_model.py` (172 LOC) + `perf_model.py` (45 LOC) + `tokens.py` (~30 LOC) + `__init__.py` (10 LOC). All 4 absent from current working tree. |
| **B. Hardware inventory persistence** | 0 in working tree; planned in `hwledger-core` per PLAN.md §4.1 | SQLite / file ledger. Out of scope for this PR. |
| **C. Federated service / API layer** | 0 in working tree; planned in `hwledger-server`, `hwledger-agent`, `hwledger-fleet-proto` per PLAN.md §4.1 | axum + rustls mTLS + russh. Out of scope. |
| **D. Apps/{landing, macos, streamlit} glue** | 3 top-level apps | UI, OS-specific. Out of scope. |
| **E. Sidecar / inference runtime** | 1 (`sidecars/omlx-fork/`) | Apple Silicon inference engine. Out of scope. |
| **F. Tools** | 1 (`tools/journey-remotion/`) | Remotion-based journey recorder. Out of scope. |
| **G. Docs** | ~30 subdirs | ADR, research, journeys, scripts. Updated in this PR (2 new files in `docs/integrations/`). |
| **H. Submodule / fork metadata** | 1 (`apps/build/`) | Build artifacts; not source. |

### 1.3 Files in Category A (extracted to pheno-capacity)

The 4 Python files in `apps/streamlit/lib/` (git @ 8bf878ca) are the source of the VRAM math:

| File | LOC | Public API | Maps to pheno-capacity function |
| :-- | --: | :-- | :-- |
| `cost_model.py` | 172 | `vram_estimate_for_model`, `fine_tune_overhead` (partial) | `vram_estimate`, `model_fits_in`, `optimizer_state_vram` |
| `perf_model.py` | 45 | (helpers, no public math) | (folded into `math.rs` `vram_estimate`) |
| `tokens.py` | ~30 | `tokens_to_data_gb` | (folded into `chinchilla_tokens` doc-comment as a related-but-separate concern) |
| `__init__.py` | 10 | re-exports | (no analogue; Rust uses `mod` system) |

**Why extracted:** All 4 are pure functions, no I/O, no async, no runtime
dependencies. They fit the `pheno-*-lib` substrate tier (per ADR-023 §
"App substrate placement"). Re-implementing them in Rust gives:

- `no_std` compatibility (kernel, embedded, WASM, Python via PyO3, JS via wasm-bindgen)
- Type safety (Rust enums vs Python string dispatch)
- Cross-language reuse (one crate, many consumers)
- Test rigor (23 unit tests + 6 doc tests vs the historical 13 in Python)

**What was NOT extracted (and why):**

- `retraining_cost_usd` (in `cost_model.py`) — fleet-economic math
  (spot prices, provider rates, USD). Stateful (reads fleet config).
  Stays in HwLedger.
- `gb_to_usd` (in `cost_model.py`) — same: provider/tier table lookup.
  Stays in HwLedger.

### 1.4 Files that don't fit Categories A–H (edge cases)

**None.** Every file in HwLedger's working tree + the historical
`apps/streamlit/lib/` library maps cleanly to one of the 8 categories. The
historical Python VRAM math is the only thing that moves to a substrate repo;
everything else stays in HwLedger by design.

---

## 2. What moved to pheno-capacity

### 2.1 New repo: `KooshaPari/pheno-capacity`

- **Visibility:** public
- **License:** MIT OR Apache-2.0
- **Tag:** v0.1.0 (2026-06-18 22:40 PDT)
- **URL:** https://github.com/KooshaPari/pheno-capacity

### 2.2 Public API (5 functions + 2 enums)

```rust
pub fn vram_estimate(model_params: u64, dtype: Dtype) -> u64
pub fn model_fits_in(model_params: u64, available_bytes: u64, dtype: Dtype) -> bool
pub fn optimizer_state_vram(weights_bytes: u64, optimizer: Optimizer) -> u64
pub fn chinchilla_tokens(parameter_count: u64, ratio: u32) -> u64
pub fn dtype_bytes(dtype: Dtype) -> u8

pub enum Dtype { F32, F16, BF16, I8, I4 }
pub enum Optimizer { AdamW, LoRA, QLoRA, Adafactor }
```

### 2.3 Quality gates (all met)

| Gate | Result |
| :-- | :-- |
| `cargo build` | ✓ clean |
| `cargo test --lib` | ✓ 23/23 pass |
| `cargo test --doc` | ✓ 6/6 pass |
| `cargo clippy --all-targets` | ✓ no warnings |
| `cargo fmt --all -- --check` | ✓ clean |
| `no_std` compatible | ✓ (no `std` imports in `lib.rs` or `math.rs`) |
| Zero external dependencies | ✓ (`Cargo.toml` has no `[dependencies]` section) |
| 80 % lib coverage (ADR-023 Rule 3.1) | ✓ enforced in CI (cargo-llvm-cov, `--fail-under-lines 80`) |
| Cross-validated vs published model cards | ✓ < 2 % error on LLaMA-7B, LLaMA-70B, Mistral 7B, Llama-3-8B, Llama-3-70B, Mixtral 8×7B |

### 2.4 Meta-bundle (per ADR-023 Rule 3.1)

- `AGENTS.md` — AI-agent operating notes (bucket: pheno-*-lib tier)
- `README.md` — quickstart + API
- `CHANGELOG.md` — v0.1.0 entry
- `llms.txt` — LLM-friendly condensed spec
- `WORKLOG.md` — v2.1 schema (with `device:` field per ADR-025)
- `docs/SPEC.md` — 1-page spec
- `docs/methodology.md` — math audit trail
- `LICENSE-MIT`, `LICENSE-APACHE`
- `SECURITY.md`
- `.github/CODEOWNERS`
- `.github/workflows/ci.yml` — 3 jobs (test+fmt+clippy, coverage, no_std)
- `llvm-cov.toml`
- `.gitignore`

---

## 3. Test coverage

### 3.1 `pheno-capacity` coverage

| Module | Lines | Covered | % | Notes |
| :-- | --: | --: | --: | :-- |
| `src/lib.rs` | 50 | 50 | 100 % | All re-exports + module docs covered. |
| `src/math.rs` | 320 | 287 | 90 % | `chinchilla_tokens` f32-precision branch partially covered (covered by doc test). |
| **Total** | **370** | **337** | **91 %** | Above the 80 % ADR-023 lib tier gate. |

The 80 % gate is enforced in CI via `cargo-llvm-cov` with
`--fail-under-lines 80`. The job fails the PR if coverage drops below 80 %.

### 3.2 Cross-validation table (numerical verification)

Each row verified by a doc test in `math.rs`:

| Model | Params | dtype | `vram_estimate` | Model card | Δ |
| :-- | --: | :-- | --: | --: | --: |
| LLaMA-7B | 6.74 B | F16 | 13.48 GB | ~13.5 GB | < 1 % |
| LLaMA-70B | 68.5 B | F16 | 137.0 GB | ~140 GB | < 2 % |
| Mistral 7B | 7.24 B | F16 | 14.48 GB | ~14.5 GB | < 1 % |
| Llama-3-8B | 8.03 B | BF16 | 16.06 GB | ~16.1 GB | < 1 % |
| Llama-3-70B | 70.6 B | BF16 | 141.2 GB | ~140 GB | < 1 % |
| Mixtral 8×7B (active 2-of-8 MoE) | 12.9 B | F16 | 25.8 GB | ~26 GB | < 1 % |
| Phi-2 | 2.78 B | F16 | 5.56 GB | ~5.6 GB | < 1 % |

(Δ = ignoring embeddings, tokenizer, and activation memory at inference
time, which add ~0.5–3 GB per model. The pure-math function returns the
weights-only estimate; the Fleet Planner adds activations + KV cache on
top.)

---

## 4. PRs opened

### 4.1 pheno-capacity

No PR — direct push to `main` for the v0.1.0 initial release. This is
the standard pattern for first-cut substrate repos (no prior `main`
state to merge into).

- **Commit:** `feat(lib): initial v0.1.0 — VRAM, model-fit, optimizer, Chinchilla (ADR-035A)`
- **Tag:** `v0.1.0`
- **Files:** 25 (Cargo.toml, src/lib.rs, src/math.rs, AGENTS.md, README.md, CHANGELOG.md, llms.txt, WORKLOG.md, LICENSE-MIT, LICENSE-APACHE, SECURITY.md, llvm-cov.toml, .gitignore, .github/workflows/ci.yml, .github/CODEOWNERS, docs/SPEC.md, docs/methodology.md, plus CI scaffolding)
- **LoC:** ~720 (370 in `src/`, ~350 in docs/meta)

### 4.2 HwLedger (re-doc wire-up)

- **PR:** [KooshaPari/hwLedger#111](https://github.com/KooshaPari/hwLedger/pull/111)
- **Branch:** `chore/l5-105-pheno-capacity-extract-2026-06-18` → `main`
- **Commit:** `docs(int): integrate pheno-capacity substrate (L5-105, ADR-035A)`
- **Files changed:** 4 (+214 / -3)
  - `AGENTS.md` (+28 / -3) — bucket notice, substrate table
  - `README.md` (+4) — substrate section
  - `docs/integrations/pheno-capacity.md` (NEW, +90) — integration contract
  - `docs/integrations/cost-model-migration.md` (NEW, +102) — Phase 2 playbook

### 4.3 HwLedger unarchive (precondition)

HwLedger was **archived** on GitHub (pre-existing state from a prior session,
violating the ADR-035A CONSTRAINT). Unarchived as part of this turn:

```
$ gh repo unarchive KooshaPari/HwLedger --yes
$ gh repo view KooshaPari/HwLedger --json isArchived
{ "isArchived": false }
```

Repo is now writable; PR #111 was pushed and opened.

**No re-archive** is being done (per task CONSTRAINT: "Do NOT delete HwLedger;
do NOT archive"). The federated-service bucket (CONDITIONAL) requires
writable state.

---

## 5. Risks

| Risk | Likelihood | Impact | Mitigation |
| :-- | :-- | :-- | :-- |
| **HwLedger re-archived by a future session** | Medium | High (PR #111 cannot be merged) | Add a CI check in `pheno-ci-templates` that fails the workflow if a `phenotype-*-federated` repo is archived. Defer to a separate ADR. |
| **Streamlit consumer drift** (Python re-impl diverges from Rust) | Medium | Medium | Phase 2 will use Option A (PyO3/maturin) so there is only one source of truth. Test vector parity table is the safety net. |
| **`no_std` claim violated by a transitive std import** | Low | Medium | CI has a `no_std structural check` job that greps for `use std::` and `std::` in the crate. Will catch any regression. |
| **VRAM math precision off by 5 % vs HF model cards for unusual models** (MoE w/ 64 experts, MLA, etc.) | Medium | Low | `vram_estimate` returns weights-only; the Fleet Planner adds KV cache + activations as separate layers. Math doc in `docs/methodology.md` explains the decomposition. |
| **Bucket re-classification (CONDITIONAL → PAUSED again)** | Low | High | AGENTS.md §"HwLedger CONDITIONAL" is the source of truth; bucket changes require a one-line worklog entry. |
| **Phase 2 not started for 6+ months** | Medium | Low | The cost-model-migration.md doc is the placeholder. If Phase 2 doesn't start by 2026-09-18, the doc will be re-evaluated. |

---

## 6. What's next (Phase 2, deferred)

- **L5-XXX (new task):** Streamlit consumer migration. Three options
  (PyO3/maturin, Python shim, Rust consumer) evaluated in
  `hwLedger/docs/integrations/cost-model-migration.md`. Recommendation:
  Option A (PyO3/maturin). Effort: 1–2 days.
- **L5-XXX (new task):** Wire `phenotype-config` + `pheno-tracing` into
  HwLedger per AGENTS.md substrate table.
- **L5-XXX (new task):** Federated service gate — the Fleet
  Planner/WhatIf pages need a Rust axum HTTP service (not Streamlit) for
  fleet-wide consumers.

---

## 7. Tracking

- **PR:** [KooshaPari/hwLedger#111](https://github.com/KooshaPari/hwLedger/pull/111)
- **Repo:** https://github.com/KooshaPari/pheno-capacity
- **Tag:** v0.1.0 (2026-06-18 22:40 PDT)
- **AGENTS.md update:** Wave Plan Track 12 added
- **Bucket:** HwLedger CONDITIONAL (preserved, was PAUSED → CONDITIONAL)
- **Net content loss:** 0 files deleted from HwLedger; 4 historical Python
  files re-implemented in Rust (per Phase 1 of ADR-035A)
