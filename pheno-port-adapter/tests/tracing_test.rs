//! Tracing-feature smoke test (ADR-036).
//!
//! Verifies that the `tracing` opt-in feature compiles + links
//! correctly and that `PortAdapter::connect` calls can be captured
//! in a `tracing` span. The crate itself does not emit spans in
//! the default code path; this test exercises the dependency
//! graph end-to-end with `tracing-test::traced_test`.

#![cfg(feature = "tracing")]

use pheno_port_adapter::PortAdapter;

struct MockAdapter {
    name: String,
    valid_endpoint: String,
}

impl PortAdapter for MockAdapter {
    fn name(&self) -> &str {
        &self.name
    }
    fn health(&self) -> Result<(), pheno_port_adapter::AdapterError> {
        Ok(())
    }
    fn connect(
        &self,
        endpoint: &str,
    ) -> Result<pheno_port_adapter::Connection, pheno_port_adapter::AdapterError> {
        if endpoint == self.valid_endpoint {
            Ok(pheno_port_adapter::Connection {
                id: endpoint.to_string(),
            })
        } else {
            Err(pheno_port_adapter::AdapterError::ConnectFailed(
                format!("invalid endpoint: {endpoint}"),
            ))
        }
    }
    fn disconnect(&self) -> Result<(), pheno_port_adapter::AdapterError> {
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
    let conn = adapter.connect("tcp://localhost:8080").unwrap();
    tracing::info!(endpoint = %conn.id, "connected");
    assert_eq!(conn.id, "tcp://localhost:8080");
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
}
