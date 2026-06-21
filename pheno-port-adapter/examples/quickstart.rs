//! Quickstart example for pheno-port-adapter.
//!
//! Demonstrates the canonical `PortAdapter` trait with the in-process
//! `MockAdapter` (for tests/dev) and the production `TcpAdapter`.
//!
//! Run with:
//!   cargo run --example quickstart

use pheno_port_adapter::adapters::{MockAdapter, TcpAdapter};
use pheno_port_adapter::PortAdapter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. In-process mock (great for tests)
    let mock = MockAdapter::new("mock-1").with_healthy(true);
    println!("Mock adapter: name={}, healthy={:?}", mock.name(), mock.health().is_ok());

    // 2. Production TCP adapter
    let tcp = TcpAdapter::new("tcp-prod");
    println!("TCP adapter: name={}", tcp.name());

    // Health check
    match tcp.health() {
        Ok(()) => println!("✓ TCP adapter is healthy"),
        Err(e) => eprintln!("✗ TCP adapter unhealthy: {}", e),
    }

    Ok(())
}
