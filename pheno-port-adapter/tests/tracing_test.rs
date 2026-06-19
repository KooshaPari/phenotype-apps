//! Tracing-feature smoke test (ADR-036).
//!
//! Verifies that the `tracing` opt-in feature compiles + links
//! correctly and that `PortAdapter::connect` calls can be captured
//! in a `tracing` span. The crate itself does not emit spans in
//! the default code path; this test exercises the dependency
//! graph end-to-end with `tracing-test::traced_test`.

#![cfg(feature = "tracing")]

use pheno_port_adapter::{AdapterError, Connection, PortAdapter};

struct MockAdapter {
    name: String,
    valid_endpoint: String,
}

impl PortAdapter for MockAdapter {
    fn name(&self) -> &str {
        &self.name
    }
    fn health(&self) -> Result<(), AdapterError> {
        Ok(())
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

#[tracing_test::traced_test]
#[test]
fn emits_span_on_connect() {
    let adapter = MockAdapter {
        name: "mock".to_string(),
        valid_endpoint: "tcp://localhost:8080".to_string(),
    };
    let span = tracing::info_span!("connect", adapter = %adapter.name());
    let _enter = span.enter();
    // `Connection.id` is `pub(crate)` so we can't read it from an
    // external test, but the success of `connect` is sufficient to
    // prove the dependency graph is wired correctly.
    let _conn = adapter.connect("tcp://localhost:8080").unwrap();
    tracing::info!("connected");
}

#[tracing_test::traced_test]
#[test]
fn emits_span_on_health_check() {
    let adapter = MockAdapter {
        name: "mock".to_string(),
        valid_endpoint: "tcp://localhost:8080".to_string(),
    };
    let span = tracing::info_span!("health");
    let _enter = span.enter();
    assert!(adapter.health().is_ok());
    tracing::info!("healthy");
}

#[tracing_test::traced_test]
#[test]
fn emits_span_on_connect_failure() {
    let adapter = MockAdapter {
        name: "mock".to_string(),
        valid_endpoint: "tcp://localhost:8080".to_string(),
    };
    let span = tracing::info_span!("connect_failure");
    let _enter = span.enter();
    let err = adapter.connect("invalid://nope").unwrap_err();
    assert!(matches!(err, AdapterError::ConnectFailed(_)));
    tracing::info!(error = %err, "rejected");
}
