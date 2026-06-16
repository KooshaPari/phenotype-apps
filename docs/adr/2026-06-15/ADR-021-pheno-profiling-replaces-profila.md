# ADR-021: `pheno-profiling` replaces `Profila` — Rust-native flamegraph + tracing integration

**Status:** Accepted 2026-06-15
**Deciders:** PhenoRust 1.0 architecture circle
**Supersedes:** ad-hoc `Profila` Python profiler wrapper

## Context

`Profila` was a Python wrapper around `py-spy` + `memray` + `cProfile`,
introduced in V2 to give Python services the same profiling affordances
as the Rust services (`cargo flamegraph`, `pprof-rs`).

The V4 audit found that:

- `Profila` had no first-class Rust integration; Rust services that
  wanted a unified flamegraph had to re-implement the conversion.
- The Python wrapping added 1.5s startup time to every service
  (`import profila` pulls `py-spy`, `memray`, `cProfile`).
- The `Profila` dashboard was a custom Flask app that fell out of
  sync with the `pheno-observability` Grafana panels.

The V5 plan ("Rust-native substrate for every L3 concern") makes
`pheno-profiling` the natural replacement: a Rust crate that wraps
`pprof-rs` for Rust services, talks to `py-spy`/`memray` over a
Unix socket for Python services, and emits OTLP spans to
`pheno-tracing` (per ADR-012) so the data lands in the same
Grafana panels as the rest of the observability stack.

## Decision

1. **`pheno-profiling` is the canonical profiler** for both Rust
   and Python services. The crate ships:
   - `pprof-rs` integration for Rust (CPU + heap).
   - Unix-socket bridge to `py-spy` + `memray` for Python.
   - OTLP exporter via `pheno-tracing` (ADR-012) — same env vars,
     same dashboard.
2. **`Profila` is deprecated** — its Python wrapper and Flask
   dashboard are removed; the 6 consumers that imported it get a
   `DeprecationWarning` for 1 minor version, then the import
   becomes a hard error.
3. **`pheno-profiling` is on the V6 critical path** for the
   observability convergence work; it ships as part of the
   PRCP-coordinated rollout (ADR-018).

## Consequences

**Positive**

- 1.5s of Python service startup time is recovered.
- The Rust and Python flamegraphs land in the same Grafana
  dashboard; cross-service correlation is correct by default.
- The 5 Rust services that hand-rolled `pprof-rs` get a 1-line
  `use` change.
- The `pheno-tracing` OTLP export means no new infra (no new
  ports, no new credentials).

**Negative**

- `py-spy`/`memray` are now an external runtime dep for
  Python services that want continuous profiling (was bundled
  in `Profila`).
- The `py-spy` Unix-socket bridge is a 200-LOC Rust binary
  in `pheno-profiling` that has to be deployed alongside
  every Python service.

**Mitigation**

- `pheno-profiling` includes a `py-spy-bridge` Docker image
  that the 4 deployment templates (`pheno-ci-templates`) can
  inject automatically.
- The deprecation is 1 minor version (4 weeks) with a
  clear migration table in `L6_PHENO_REPOS_HEALTH_2026_06_15_DELTA.md`.

## Alternatives considered

- **Keep `Profila` and add a Rust bridge.** Rejected:
  `Profila` is Python; adding a Rust bridge to a Python
  wrapper inverts the layering.
- **Use only `pprof-rs` and tell Python services to switch
  to Rust.** Rejected: 6 services are Python and the
  rewrite cost is 6× of the V3 burn.
- **Adopt `Datadog Continuous Profiler` / `Pyroscope`.**
  Deferred: out of scope for V6. The `pheno-profiling` OTLP
  export is already compatible with Pyroscope's receiver, so
  a future migration is a config change, not a code change.

## References

- `Profila/setup.py` (deprecated 2026-06-15).
- `pheno-profiling/src/lib.rs` — crate root, `pprof-rs` +
  OTLP integration.
- `pheno-profiling/src/bridges/py_spy.rs` — Unix-socket
  bridge.
- `pheno-tracing/src/adapters/otlp.rs` — OTLP exporter
  (ADR-012).
- `L6_PHENO_REPOS_HEALTH_2026_06_15_DELTA.md` — consumer
  migration table.
- `V6 §Track 3` — `pheno-profiling` rollout work item.
