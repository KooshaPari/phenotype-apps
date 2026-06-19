use thiserror::Error;

/// Error type for port adapter operations.
#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("connect failed: {0}")]
    ConnectFailed(String),
    #[error("disconnect failed: {0}")]
    DisconnectFailed(String),
    #[error("health check failed: {0}")]
    HealthCheckFailed(String),
    #[error("timeout")]
    Timeout,
}

/// Opaque handle representing an active connection.
///
/// Returned from [`PortAdapter::connect`] to acknowledge a successful
/// connection; the value is currently a thin wrapper around the endpoint
/// string used to open it.
#[derive(Debug)]
#[allow(dead_code)]
#[must_use = "Connection acknowledges a successful connect; an unused value is almost always a logic bug"]
pub struct Connection {
    /// Endpoint string this connection was opened against. Useful for
    /// log correlation; the concrete stream is held inside the adapter
    /// and is not exposed to callers.
    pub id: String,
}

/// Trait for port adapters.
///
/// Implementors must be safe to share across threads (`Send + Sync`)
/// because the fleet pattern is one adapter per service, accessed
/// concurrently by every request handler.
///
/// # Example
///
/// ```no_run
/// use pheno_port_adapter::{AdapterError, Connection, PortAdapter};
///
/// struct MockAdapter;
///
/// impl PortAdapter for MockAdapter {
///     fn name(&self) -> &str { "mock" }
///     fn health(&self) -> Result<(), AdapterError> { Ok(()) }
///     fn connect(&self, endpoint: &str) -> Result<Connection, AdapterError> {
///         Ok(Connection { id: endpoint.to_string() })
///     }
///     fn disconnect(&self) -> Result<(), AdapterError> { Ok(()) }
/// }
///
/// let adapter = MockAdapter;
/// assert_eq!(adapter.name(), "mock");
/// assert!(adapter.health().is_ok());
/// ```
pub trait PortAdapter: Send + Sync {
    /// Return a stable, lowercase identifier for this adapter
    /// (e.g. `"tcp"`, `"unix"`). Used in logs and metrics labels.
    fn name(&self) -> &str;

    /// Probe the underlying connection for liveness. Implementations
    /// should be cheap (a single syscall is ideal).
    #[must_use = "Result-returning; ignoring the Err arm silently masks an unhealthy adapter"]
    fn health(&self) -> Result<(), AdapterError>;

    /// Open a connection to `endpoint`. The format of `endpoint` is
    /// adapter-specific (TCP: `host:port`, Unix: absolute path).
    #[must_use = "Result-returning; ignoring the Err arm silently masks a failed connect"]
    fn connect(&self, endpoint: &str) -> Result<Connection, AdapterError>;

    /// Close the underlying connection. Idempotent — calling on an
    /// already-disconnected adapter returns `Ok(())`.
    #[must_use = "Result-returning; ignoring the Err arm silently masks a failed disconnect"]
    fn disconnect(&self) -> Result<(), AdapterError>;
}

/// Concrete transport adapters (TCP, Unix-domain socket).
pub mod adapters;

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
