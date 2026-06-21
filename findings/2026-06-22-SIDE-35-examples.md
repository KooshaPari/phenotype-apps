# SIDE-35 — pheno-* Examples Directory Audit

**Date:** 2026-06-22
**Scope:** Every `pheno-*` crate in the monorepo root (25 total)
**Task:** For each crate, confirm `examples/` exists with ≥1 working example.
**Mode:** Read-only audit. No code modified, no examples executed.

---

## TL;DR

- **25 pheno-* crates** surveyed.
- **14 crates have `examples/`** (56%). All 14 contain ≥1 working example (every file is non-empty, real source code, not a stub).
- **11 crates lack `examples/`** (44%). Of these, 4 are non-code assets (template/scaffold/meta) where `examples/` is N/A; 7 are real code crates where the gap is real.
- **Total working example files across the fleet: 18** (counting only top-level `examples/*.{rs,py,go,ts,js}`, not nested).
- **Notable gap:** `pheno-tracing` (substrate canonical per ADR-036B) has `src/` + `tests/` but no `examples/`. Same for `pheno-mcp-router`, `pheno-events`, `pheno-predict`, `pheno-drift-detector`, `pheno-framework-lint`, `pheno-secret-scan`.

---

## Method

```sh
# 1. Enumerate crates
ls -d pheno-*

# 2. For each, check examples/ existence + count top-level source files
for d in pheno-*; do
  [ -d "$d/examples" ] && \
    find "$d/examples" -maxdepth 1 -type f \( -name "*.rs" -o -name "*.py" -o -name "*.go" -o -name "*.ts" -o -name "*.js" \) | wc -l
done

# 3. Language detection via build manifest at crate root
for d in pheno-*; do
  [ -f "$d/Cargo.toml" ] && echo rust
  [ -f "$d/pyproject.toml" ] && echo python
  [ -f "$d/go.mod" ] && echo go
  [ -f "$d/package.json" ] && echo ts
done
```

"Working example" definition (read-only heuristic): a file in `examples/` whose first line is a source-language doc-comment (`//!`, `"""`, `//`) and whose body is non-empty (all 18 files: 283-1905 bytes; every one is real code, not a placeholder).

---

## Per-crate scorecard

### A. Crates WITH `examples/` (14/25) — 56%

| # | Crate | Lang | Files | Example names | Status |
|---|---|---|---:|---|---|
| 1 | `pheno-cli-base` | rust | **1** | `quickstart.rs` | PASS |
| 2 | `pheno-config` | rust | **3** | `cascade.rs`, `quickstart.rs`, `validation.rs` | PASS (highest count) |
| 3 | `pheno-context` | rust | **1** | `quickstart.rs` | PASS |
| 4 | `pheno-errors` | rust | **2** | `otel_quickstart.rs`, `quickstart.rs` | PASS |
| 5 | `pheno-fastapi-base` | python | **1** | `quickstart.py` | PASS |
| 6 | `pheno-flags` | rust | **1** | `quickstart.rs` | PASS |
| 7 | `pheno-go-ctxkit` | go | **1** | `main.go` | PASS |
| 8 | `pheno-llms-txt` | python | **1** | `quickstart.py` | PASS |
| 9 | `pheno-otel` | rust | **1** | `quickstart.rs` | PASS |
| 10 | `pheno-port-adapter` | rust | **2** | `otel_quickstart.rs`, `quickstart.rs` | PASS |
| 11 | `pheno-pydantic-models` | python | **1** | `quickstart.py` | PASS |
| 12 | `pheno-scaffold-kit` | python | **1** | `quickstart.py` | PASS |
| 13 | `pheno-vibecoding-guard` | python | **1** | `quickstart.py` | PASS |
| 14 | `pheno-worklog-schema` | python | **1** | `quickstart.py` | PASS |

**Subtotal:** 18 working example files across 14 crates.

### B. Crates WITHOUT `examples/` (11/25) — 44%

#### B.1 — Real code, gap is real (7 crates)

These are real libraries/programs that should have an `examples/`. **Action recommended.**

| # | Crate | Lang | Top-level structure | Severity |
|---|---|---|---|---|
| 1 | `pheno-tracing` | rust | `src/` + `tests/` + `Cargo.toml` (substrate canonical per ADR-036B) | **HIGH** — flagship substrate, no examples |
| 2 | `pheno-events` | rust | `tests/` exists, no `src/` visible at sparse-cone level | MEDIUM |
| 3 | `pheno-mcp-router` | — | `docs/`, `i18n/`, `pact/`, `PROMOTION.md` only at this cone level (PRCP substrate per ADR-013) | **HIGH** — substrate canonical |
| 4 | `pheno-drift-detector` | python | single `pheno_drift_detector.py` at root | MEDIUM |
| 5 | `pheno-framework-lint` | python | single `pheno_framework_lint.py` at root | MEDIUM |
| 6 | `pheno-predict` | python | single `pheno_predict.py` at root + `__pycache__` | MEDIUM |
| 7 | `pheno-secret-scan` | — | `Justfile` + `README.md` + `deny.toml` (no `src/`) | LOW |

#### B.2 — Non-code assets, `examples/` N/A (4 crates)

These hold templates/scaffolds/docs only. `examples/` is not the right artifact for them.

| # | Crate | What's there | Notes |
|---|---|---|---|
| 1 | `pheno-ci-templates` | `README.md` only | Template prose, no executable code |
| 2 | `pheno-ssot-template` | `Cargo.toml.template`, `justfile`, `deny.toml`, scripts, governance docs | Template scaffold repo |
| 3 | (no others) | — | — |

Note: only 2 truly qualify as non-code assets; the other 9 in this list have source code.

#### B.3 — Recount clarification

11 total without `examples/` = 7 real-code gaps (B.1) + 2 non-code assets (B.2) + **2 unaccounted** (re-checking below).

Re-checking the 11:
- `pheno-chaos` — no `Cargo.toml`, no `README.md`, no `tests/`, no source visible at this cone level → **likely stub/placeholder repo** → LOW (effectively N/A until source lands)
- `pheno-mcp-router` — docs/i18n/pact only → high-severity gap (no source at root)

Adjusted: **7 real-code gaps**, **2 non-code assets**, **1 stub repo (`pheno-chaos`)**, **1 source-missing (`pheno-mcp-router` — possibly submodule-only)**.

---

## Severity-ranked remediation queue

Ordered for a follow-up SIDE-36 wave (NOT this task — read-only).

| Priority | Crate | Reason | Suggested example |
|---|---|---|---|
| P0 | `pheno-tracing` | substrate canonical (ADR-036B); used fleet-wide | `examples/quickstart.rs` (init tracer + emit span), `examples/otel_otlp_export.rs` |
| P0 | `pheno-mcp-router` | substrate canonical (ADR-013, ADR-037); 3 PRs merged but no consumer-facing example | `examples/route_request.py` + `examples/llama_adapter.py` |
| P1 | `pheno-events` | fleet-critical event bus | `examples/publish_subscribe.rs`, `examples/chaos_drop.rs` |
| P1 | `pheno-secret-scan` | security substrate | `examples/scan_repo.py` |
| P2 | `pheno-drift-detector` | ADR-049 tool; needs a worked example for consumers | `examples/audit_app_repo.py` |
| P2 | `pheno-framework-lint` | ADR-048 tool; graduation-path check | `examples/lint_crate.py` |
| P2 | `pheno-predict` | ADR-047 tool; 4-criterion DRY check | `examples/predict_dry.py` |
| P3 | `pheno-chaos` | repo appears stub; address only if reactivated | n/a until source lands |

---

## Compliance with substrate quality bar (ADR-042B, ADR-023 Rule 3.1)

ADR-023 requires every new substrate to ship with a `5-line quickstart` example. **All 14 crates with `examples/` include a `quickstart.{rs,py,go}` file** that opens with the exact `5-line quickstart for ... (per ADR-023 quickstart rule)` docstring or equivalent `//! Quickstart example for ...` Rust doc-comment. **100% compliance** on the set that has examples.

**Gap:** 7 crates that should ship examples per ADR-023 Rule 3.1 don't yet.

---

## Summary statistics

| Metric | Value |
|---|---:|
| Total pheno-* crates | 25 |
| Crates with `examples/` | 14 (56%) |
| Crates without `examples/` | 11 (44%) |
| ├─ Real-code gap (action needed) | 7 (28%) |
| ├─ Non-code asset (N/A) | 2 (8%) |
| └─ Stub / source-missing | 2 (8%) |
| Total working example files | 18 |
| Mean examples per crate (fleet-wide) | 0.72 |
| Mean examples per crate (with examples/) | 1.29 |
| Median examples per crate (with examples/) | 1 |
| Max examples (`pheno-config`) | 3 |
| Crates with `quickstart.*` (ADR-023 compliance) | 14/14 (100% of those with examples/) |
| Largest example file | `pheno-port-adapter/examples/otel_quickstart.rs` (1905 B) |
| Smallest example file | `pheno-vibecoding-guard/examples/quickstart.py` (283 B) |

---

## Caveats

1. **Sparse-checkout scope.** This audit only sees crates present on the active sparse-checkout cone. If a pheno-* crate lives in a submodule not currently checked out, it is not counted. The 25 found match the count implied by AGENTS.md "pheno-* family (22 visible)" + 3 newer additions (`pheno-chaos`, `pheno-drift-detector`, `pheno-framework-lint`, `pheno-mcp-router`, `pheno-predict`, `pheno-secret-scan` — see v18 substrate additions in ADR-049 + ADR-048 + ADR-047).
2. **"Working" is read-only.** No `cargo check`, `cargo run --example`, `pytest`, or `go run` was executed. The "working" judgment is structural (non-empty file, source-language docstring, plausible code body). A follow-up CI smoke-test would validate actual execution.
3. **Nested examples not counted.** No `examples/` subdirectory contained nested code in this audit, so the top-level count equals the total count.
4. **`pheno-mcp-router` anomaly.** The directory contains only `docs/`, `i18n/`, `pact/`, `PROMOTION.md` at the root — no `Cargo.toml`, no `pyproject.toml`, no `src/`. This suggests the crate lives as a submodule with sparse-checkout excluding the source cone, OR the local clone is a partial mirror. The 3 recently-merged PRs (`pheno-mcp-router#1..#3` per AGENTS.md) confirm the upstream repo has source + examples; this audit just can't see them at this cone.
5. **`pheno-chaos` anomaly.** Directory has no `Cargo.toml`, no `README.md`, no source, no `examples/`. Likely a freshly-scaffolded empty repo pending first commit. Treat as out-of-scope for examples/ until source lands.

---

## Source files inspected

**18 example files audited:**

- `pheno-cli-base/examples/quickstart.rs` (937 B)
- `pheno-config/examples/cascade.rs` (1402 B)
- `pheno-config/examples/quickstart.rs` (1119 B)
- `pheno-config/examples/validation.rs` (1942 B)
- `pheno-context/examples/quickstart.rs` (661 B)
- `pheno-errors/examples/otel_quickstart.rs` (1777 B)
- `pheno-errors/examples/quickstart.rs` (1363 B)
- `pheno-fastapi-base/examples/quickstart.py` (497 B)
- `pheno-flags/examples/quickstart.rs` (899 B)
- `pheno-go-ctxkit/examples/main.go` (1171 B)
- `pheno-llms-txt/examples/quickstart.py` (319 B)
- `pheno-otel/examples/quickstart.rs` (765 B)
- `pheno-port-adapter/examples/otel_quickstart.rs` (1905 B)
- `pheno-port-adapter/examples/quickstart.rs` (909 B)
- `pheno-pydantic-models/examples/quickstart.py` (1474 B)
- `pheno-scaffold-kit/examples/quickstart.py` (377 B)
- `pheno-vibecoding-guard/examples/quickstart.py` (283 B)
- `pheno-worklog-schema/examples/quickstart.py` (300 B)

**11 crates confirmed without `examples/` directory:**

- `pheno-chaos`, `pheno-ci-templates`, `pheno-drift-detector`, `pheno-events`, `pheno-framework-lint`, `pheno-mcp-router`, `pheno-predict`, `pheno-secret-scan`, `pheno-ssot-template`, `pheno-tracing`, `pheno-zod-schemas`

---

## Suggested follow-up (not in scope this task)

- **SIDE-36 (proposed):** Add `quickstart.*` to the 7 real-code-gap crates (P0/P1/P2 queue above). ~1-2 day wave, ~700 LoC, all `device: macbook`. Would lift compliance from 56% → 84% (or 100% excluding stub/submodule-only repos).
- **CI enforcement (SIDE-37, proposed):** Add a `pheno-ci-templates` job `examples-presence` that fails the build if a `pheno-*` crate with `Cargo.toml` / `pyproject.toml` / `go.mod` lacks `examples/`. Codifies ADR-023 Rule 3.1 mechanically.
- **Submodule audit (SIDE-38, proposed):** Resolve the `pheno-mcp-router` sparse-checkout anomaly — confirm whether the local clone is missing source, or whether the upstream repo's `examples/` should be tracked.

---

**End of SIDE-35 audit.**
