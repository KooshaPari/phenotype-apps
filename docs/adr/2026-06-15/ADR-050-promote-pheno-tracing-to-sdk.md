# ADR-050 — Promote `pheno-tracing` from lib tier to SDK tier

**Status:** PROPOSED (W6-2)
**Date:** 2026-06-19
**Owner:** worklog-schema circle
**Tier:** lib → **SDK**

## Decision

Promote `pheno-tracing` from a Rust workspace `lib` (V4 §6 SOTA crate) to an SDK-grade substrate that any polyglot consumer (Rust, Python, Go, TypeScript) can depend on without rebuilding the tracing stack.

## Background

`pheno-tracing` currently lives at `pheno-tracing/src/lib.rs` in the monorepo and exports a thin wrapper around `tracing`, `tracing-subscriber`, and `tracing-appender`. It is currently consumed only by `pheno-mcp-router` and a handful of `pheno-*` lib crates.

The SDK promotion requires:

1. **Polyglot consumers** — at least 2 distinct language bindings (Python via `pyo3`/`maturin`, Go via `cgo`, TypeScript via `napi-rs`)
2. **Stability commitment** — semver, MSRV, no breaking changes in minor
3. **Public API documentation** — `docs.rs` page + `llms.txt`
4. **Adoption** — at least 2 in-production users (not just the workspace)

## Promotion criteria checklist

- [ ] `pheno-tracing` currently has 1 consumer (`pheno-mcp-router`); needs 1 more
- [ ] No Python binding exists; needs `pyo3` wrapper around `init_json()` and `init_with_file()`
- [ ] No Go binding exists; needs `cgo` shim or separate `pheno-tracing-go` crate
- [ ] `llms.txt` not generated (only `AGENTS.md` exists)
- [ ] MSRV not pinned (`rust-version = "1.75"` is workspace-wide; should be per-crate)

## Action items

1. **Add `pheno-tracing-py` Python crate** (pyo3 + maturin) — exposes `init_json()`, `init_with_file()`, `EnvFilter::builder()`
2. **Add `pheno-tracing-go` Go package** (cgo shim) — exposes `Init()`, `WithJson()`, `WithFile()`
3. **Pin MSRV to 1.75** in `pheno-tracing/Cargo.toml`
4. **Generate `llms.txt`** via `pheno-llms-txt` (the canonical generator)
5. **Adopt in 1 more focus repo** — `dispatch-mcp` (currently uses `tracing` directly; could swap)
6. **Cut v0.2.0 release** with the SDK contract

## Acceptance criteria

- [ ] `pheno-tracing-py` published to PyPI
- [ ] `pheno-tracing-go` tagged in monorepo
- [ ] `docs.rs/pheno-tracing` builds clean
- [ ] `llms.txt` present and ≤ 200 lines
- [ ] 2+ consumers in production (e.g. `pheno-mcp-router` + `pheno-cost-card` + `dispatch-mcp`)

## References

- V4 §6 SOTA (tracing ecosystem replacement)
- ADR-042 (substrate quality bar)
- ADR-048 (substrate graduation path, 4-tier gate)
- `pheno-tracing/Cargo.toml`, `pheno-tracing/src/lib.rs`
- `pheno-tracing/AGENTS.md`, `pheno-tracing/llms.txt`
