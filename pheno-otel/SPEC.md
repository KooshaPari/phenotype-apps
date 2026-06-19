# SPEC — pheno-otel

## Public API

### `init`
```rust
pub fn init(service_name: &str) -> Result<TelemetryGuard, OtelError>
```
Initializes the global tracer provider with OTLP HTTP/proto exporter. Reads `OTEL_EXPORTER_OTLP_ENDPOINT` (defaults to `http://localhost:4318`).

### `init_with_stdout`
```rust
pub fn init_with_stdout(service_name: &str) -> Result<TelemetryGuard, OtelError>
```
Local-dev variant: uses a stdout exporter (no network). Useful for `cargo test` and demos.

### `TelemetryGuard`
```rust
pub struct TelemetryGuard { /* private */ }

impl Drop for TelemetryGuard {
    fn drop(&mut self) { /* flush + shutdown */ }
}
```
RAII guard: dropping it flushes the global tracer provider and shuts it down. Bound the telemetry pipeline's lifetime to a scope's lifetime.

### `OtelError`
```rust
pub enum OtelError {
    EndpointParse(String),
    ExporterInit(String),
    Shutdown(String),
}
```
3-variant error type, intentionally narrower than `pheno_errors::AppError`.

## Environment
| Var | Default | Purpose |
|---|---|---|
| `OTEL_EXPORTER_OTLP_ENDPOINT` | `http://localhost:4318` | OTLP HTTP/proto endpoint |
| `OTEL_SDK_DISABLED` | unset | When `true`, all spans are no-ops (use in CI per ADR-036) |

## Design decisions
- **3-variant error** — narrow by design; consumers map to `pheno_errors::AppError` at boundaries
- **`#![deny(missing_docs)]`** — public API must be documented
- **`#![deny(rust_2018_idioms)]`** — no legacy `extern crate`
- **RAII guard** — Drop-based shutdown eliminates "forgot to flush" bugs
- **Tier 0 substrate** — meta-bundle required (per ADR-023)

## Versioning
- `0.1.0` — initial release (2026-06-18)
- Backward-compatible within `0.x` per Cargo semver
