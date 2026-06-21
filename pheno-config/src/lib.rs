//! `pheno-config` — Layered configuration cascade for the pheno-* fleet.
//!
//! Per ADR-023 Rule 3.1 every substrate ships observability. The
//! public entry points ([`build_cascade`], [`build_cascade_from_str`],
//! [`service_name`]) are instrumented with `#[tracing::instrument]` and
//! wired up via [`pheno_tracing`] (the canonical `TracePort` substrate,
//! per ADR-012/036B). Callers should invoke [`init_telemetry`] once at
//! process start before the first config load; the returned
//! [`pheno_tracing::adapters::InMemoryAdapter`] handle can be plugged
//! into an OTLP exporter per the `pheno-otel` substrate.
//!
//! ## When to use
//!
//! - Building a new service in the fleet that needs typed config.
//! - Need a single, fleet-wide priority order for dev / CI / runtime
//!   overrides.
//! - Want to read both `.idea/runConfigurations/*.xml` and `PHENO_*`
//!   env vars.
//! - Cascading several config sources into one [`figment::Figment`].
//!
//! ## When NOT to use
//!
//! - Pure secret storage (use a vault like HashiCorp Vault or
//!   `phenotype-vault`).
//! - Hot-reloading runtime config (use a config server like
//!   `phenotype-configd`).
//! - Non-Phenotype projects — the priority order is opinionated.
//!
//! ## Quickstart
//!
//! ```
//! use pheno_config::build_cascade;
//! let cfg = build_cascade();
//! let port: u16 = cfg.find_value("server.port").expect("default port");
//! ```

#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]

use std::sync::OnceLock;

use pheno_tracing::adapters::InMemoryAdapter;
use pheno_tracing::port::TracePort;

pub mod cascade;

pub use cascade::{build_cascade, build_cascade_from_str, DEFAULT_TOML};

// =============================================================================
// Observability bootstrap (v17-T5 / L8)
//
// [`init_telemetry`] installs a process-wide [`InMemoryAdapter`] (the
// canonical `TracePort` impl from `pheno-tracing`, ADR-036B) and stores
// it in a `OnceLock` so the rest of the crate can pull it via
// [`telemetry_port`]. The OTLP wire-format export (per ADR-037) is the
// caller's job — set the standard `OTEL_EXPORTER_OTLP_ENDPOINT` env var
// (default `http://localhost:4317`) and plug the adapter into whatever
// collector SDK is in use; the substrate does not bundle one to keep
// the dependency footprint small.
//
// Failure to initialize the telemetry port is non-fatal: callers may
// opt out by not invoking `init_telemetry`, in which case
// `telemetry_port()` returns `None` and the `#[tracing::instrument]`
// attributes short-circuit. We deliberately do not return an error from
// `init_telemetry` because the substrate adapter constructors are
// infallible today; the `Result` return is preserved as a forward-compat
// seam for when a real OTLP exporter is wired in (L8 v18+).
// =============================================================================

/// Process-wide telemetry port for `pheno-config`.
///
/// Populated by [`init_telemetry`]; `None` until then.
static TELEMETRY: OnceLock<InMemoryAdapter> = OnceLock::new();

/// Initialize the process-wide telemetry port for `pheno-config`.
///
/// Installs an [`InMemoryAdapter`] (the canonical `TracePort` impl from
/// the `pheno-tracing` substrate) and exposes it via [`telemetry_port`].
/// Idempotent: subsequent calls return `Ok(())` without re-initializing.
///
/// The OTLP wire-format export lives in the `pheno-otel` substrate
/// (ADR-037); this function only wires up the producer side. To export
/// spans, set `OTEL_EXPORTER_OTLP_ENDPOINT` (defaults to
/// `http://localhost:4317`) and plug the adapter into a collector SDK.
///
/// Errors are surfaced as [`TelemetryError`]. The current
/// `InMemoryAdapter` constructor is infallible, so this function returns
/// `Err` only when the substrate reports an unexpected failure (which
/// is not expected to happen in practice).
pub fn init_telemetry() -> Result<(), TelemetryError> {
    let adapter = InMemoryAdapter::new();
    let _ = TELEMETRY.set(adapter);
    Ok(())
}

/// Returns the process-wide telemetry port installed by
/// [`init_telemetry`], or `None` if telemetry has not been initialized.
pub fn telemetry_port() -> Option<&'static InMemoryAdapter> {
    TELEMETRY.get()
}

/// Error type for telemetry initialization failures.
///
/// Kept as a local enum (not re-exported from `pheno-tracing`) because
/// the substrate does not yet expose its own error type — `pheno-otel`
/// owns the OTLP wire errors. This type is a forward-compat seam:
/// once `pheno-tracing` ships a typed error, this enum can be replaced
/// with `pheno_tracing::Error` in a single PR.
#[derive(Debug, thiserror::Error)]
pub enum TelemetryError {
    /// The `pheno-tracing` substrate reported a failure during adapter
    /// construction. Not expected in practice (the in-memory adapter is
    /// infallible today) but reserved for forward-compat with future
    /// adapters that might fail (e.g. gRPC, OTLP/HTTP with TLS).
    #[error("pheno-tracing substrate reported: {0}")]
    Substrate(String),
}

/// Stable service name used in OTLP `service.name` resource attributes
/// and in any `tracing` span metadata emitted by the crate.
///
/// Surfaced as a `pub fn` (rather than a `pub const`) so the value can
/// be overridden by callers that want to label their service differently
/// (e.g. `service_name()` from a wrapper crate). The default is
/// `"pheno-config"`; downstream consumers should use the wrapper's
/// value when one is available.
#[tracing::instrument(level = "info")]
pub fn service_name() -> &'static str {
    "pheno-config"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_name_is_pheno_config() {
        assert_eq!(service_name(), "pheno-config");
    }

    #[test]
    fn init_telemetry_is_idempotent_and_succeeds() {
        // init_telemetry must succeed both for first call and any
        // subsequent call (the OnceLock absorbs the second set attempt).
        assert!(init_telemetry().is_ok());
        assert!(init_telemetry().is_ok());
        // After init, telemetry_port returns Some.
        assert!(telemetry_port().is_some());
    }
}
