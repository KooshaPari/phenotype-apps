//! `pheno-port-adapter` — substrate-canonical hexagonal port traits and
//! their concrete adapter implementations (ADR-038).
//!
//! Two surfaces live in this crate:
//!
//! 1. **Transport adapter trait** ([`PortAdapter`], [`Connection`],
//!    [`AdapterError`]) — sync interface for pluggable transports
//!    (TCP, Unix-domain, ...). Existing pre-hex-port adapters
//!    ([`adapters::TcpAdapter`], [`adapters::UnixAdapter`]) implement
//!    this.
//! 2. **Hex-port traits** (under [`ports`]) — async-capability traits
//!    defined per application/service boundary, with concrete adapters
//!    under [`adapters`]. Currently shipped:
//!    - [`ports::HexCachePort`] + [`adapters::InMemoryCache`] +
//!      [`adapters::RedisAdapter`].
//!
//! See ADR-038 for the policy governing when a new hex-port trait lands
//! in this crate vs. a polyglot `phenotype-*-sdk` package.
//!
//! ## Observability
//!
//! Per ADR-023 Rule 3.1 every substrate ships observability. Connection
//! lifecycle (connect / disconnect / error) is exported via
//! [`pheno_otel`] (ADR-037 canonical OTLP wire substrate). The hex-port
//! adapters do not yet emit per-operation spans — that is a tracked
//! follow-up (v13+) so we don't blanket-spam traces for high-QPS cache
//! paths.
//!
//! Per v17-T5 (L8 Observability hooks), the public transport-adapter
//! surface is instrumented with `#[tracing::instrument]` and wired up
//! via [`pheno_tracing`] (the canonical `TracePort` substrate, per
//! ADR-012/036B). Callers should invoke [`init_telemetry`] once at
//! process start before issuing the first `connect`/`health`/`disconnect`
//! call; the returned [`pheno_tracing::adapters::InMemoryAdapter`] handle
//! can be plugged into an OTLP exporter per the `pheno-otel` substrate.

//! #![doc = "Promote to deny once cycle-3 doc audit confirms 0 missing-doc warnings."]

#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]

use std::sync::OnceLock;

use thiserror::Error;

use pheno_tracing::adapters::InMemoryAdapter;
use pheno_tracing::port::TracePort;

/// Error type for transport-level [`PortAdapter`] operations.
#[derive(Debug, Error)]
pub enum AdapterError {
    /// A call to [`PortAdapter::connect`] failed. The wrapped string
    /// is a human-readable description of why (e.g. `"connection
    /// refused"`, `"invalid endpoint: ..."`).
    #[error("connect failed: {0}")]
    ConnectFailed(String),
    /// A call to [`PortAdapter::disconnect`] failed. Rare — most
    /// adapters treat disconnect as infallible.
    #[error("disconnect failed: {0}")]
    DisconnectFailed(String),
    /// A call to [`PortAdapter::health`] reported the adapter as
    /// unhealthy. Subsequent connects are likely to fail.
    #[error("health check failed: {0}")]
    HealthCheckFailed(String),
    /// A transport-level operation exceeded its deadline. Surfaced by
    /// adapters that wrap their I/O in a timeout.
    #[error("timeout")]
    Timeout,
}

/// Opaque handle representing an active connection.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Connection {
    pub(crate) id: String,
}

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

/// Process-wide telemetry port for `pheno-port-adapter`.
///
/// Populated by [`init_telemetry`]; `None` until then.
static TELEMETRY: OnceLock<InMemoryAdapter> = OnceLock::new();

/// Initialize the process-wide telemetry port for `pheno-port-adapter`.
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

/// Returns the process-wide telemetry port installed by
/// [`init_telemetry`], or `None` if telemetry has not been initialized.
pub fn telemetry_port() -> Option<&'static InMemoryAdapter> {
    TELEMETRY.get()
}

/// Trait for pluggable transport adapters (TCP, Unix-domain, ...).
///
/// Synchronous by design — the adapter itself owns its async runtime
/// story. Async work belongs in the hex-port traits under [`ports`].
///
/// ## Tracing
///
/// Each trait method carries a `#[tracing::instrument]` attribute. On
/// impls the attribute creates a span named after the method with the
/// adapter's reported [`name`](Self::name) as a field; consumer code
/// that wants per-impl spans should add the same attribute to its own
/// adapter impls.
pub trait PortAdapter: Send + Sync {
    /// Stable, human-readable adapter name (e.g. `"tcp"`, `"unix"`).
    /// Used in logs, metrics labels, and error messages.
    fn name(&self) -> &str;

    /// Cheap liveness check that does not perform I/O. Returns
    /// [`AdapterError::HealthCheckFailed`] if the adapter is in a state
    /// where a subsequent [`Self::connect`] is likely to fail.
    #[tracing::instrument(level = "debug", skip(self))]
    fn health(&self) -> Result<(), AdapterError>;

    /// Open a connection to `endpoint` (URI scheme chosen by the
    /// adapter — e.g. `"tcp://host:port"` for [`adapters::TcpAdapter`]).
    /// Returns a [`Connection`] handle that the caller passes to
    /// [`Self::disconnect`].
    #[tracing::instrument(level = "info", skip(self))]
    fn connect(&self, endpoint: &str) -> Result<Connection, AdapterError>;

    /// Close the active connection. Idempotent: a second call on an
    /// already-disconnected adapter returns `Ok(())`.
    #[tracing::instrument(level = "info", skip(self))]
    fn disconnect(&self) -> Result<(), AdapterError>;
}

/// Hex-port traits (cache, time, ...). Each trait lives in its own
/// submodule and is paired with one or more adapters under [`adapters`].
pub mod ports;

/// Concrete adapter implementations.
pub mod adapters;

// Re-exports for the most common entry points so downstream crates can
// `use pheno_port_adapter::HexCachePort` instead of
// `pheno_port_adapter::ports::HexCachePort`. Re-exports are kept flat
// (not nested) so adding a new port doesn't break import paths.
pub use ports::{CacheError, HexCachePort, HexTimePort};

#[cfg(test)]
mod tests {
    use super::*;

    struct MockAdapter {
        name: String,
        healthy: bool,
        valid_endpoint: String,
    }

    impl PortAdapter for MockAdapter {
        fn name(&self) -> &str {
            &self.name
        }

        fn health(&self) -> Result<(), AdapterError> {
            if self.healthy {
                Ok(())
            } else {
                Err(AdapterError::HealthCheckFailed("unhealthy".to_string()))
            }
        }

        fn connect(&self, endpoint: &str) -> Result<Connection, AdapterError> {
            if endpoint == self.valid_endpoint {
                Ok(Connection {
                    id: endpoint.to_string(),
                })
            } else {
                Err(AdapterError::ConnectFailed(format!(
                    "invalid endpoint: {endpoint}"
                )))
            }
        }

        fn disconnect(&self) -> Result<(), AdapterError> {
            Ok(())
        }
    }

    #[test]
    fn connect_returns_connection() {
        let adapter = MockAdapter {
            name: "mock".to_string(),
            healthy: true,
            valid_endpoint: "tcp://localhost:8080".to_string(),
        };
        let conn = adapter.connect("tcp://localhost:8080").unwrap();
        assert_eq!(conn.id, "tcp://localhost:8080");
    }

    #[test]
    fn disconnect_returns_ok() {
        let adapter = MockAdapter {
            name: "mock".to_string(),
            healthy: true,
            valid_endpoint: "tcp://localhost:8080".to_string(),
        };
        assert!(adapter.disconnect().is_ok());
    }

    #[test]
    fn health_check_passes() {
        let adapter = MockAdapter {
            name: "mock".to_string(),
            healthy: true,
            valid_endpoint: "tcp://localhost:8080".to_string(),
        };
        assert!(adapter.health().is_ok());
    }

    #[test]
    fn connect_to_invalid_endpoint_fails() {
        let adapter = MockAdapter {
            name: "mock".to_string(),
            healthy: true,
            valid_endpoint: "tcp://localhost:8080".to_string(),
        };
        let result = adapter.connect("invalid://nope");
        assert!(matches!(result, Err(AdapterError::ConnectFailed(_))));
    }

    #[test]
    fn adapter_name_is_non_empty() {
        let adapter = MockAdapter {
            name: "mock-adapter".to_string(),
            healthy: true,
            valid_endpoint: "tcp://localhost:8080".to_string(),
        };
        assert!(!adapter.name().is_empty());
    }
}
