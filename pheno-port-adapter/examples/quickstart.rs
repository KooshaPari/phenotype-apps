//! Quickstart example for pheno-port-adapter.
//!
//! Demonstrates the canonical `PortAdapter` trait with the in-tree
//! `TcpAdapter` and the Unix-only `UnixAdapter`.
//!
//! Run with:
//!   cargo run --example quickstart
//!
//! (The `UnixAdapter` example is gated on Unix; the file still compiles
//! on Windows because the `cfg(unix)`-gated items simply become absent.)

use pheno_port_adapter::adapters::tcp::TcpAdapter;
use pheno_port_adapter::PortAdapter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Production TCP adapter
    let tcp = TcpAdapter::new();
    println!("TCP adapter: name={}", tcp.name());

    // Health check (not connected, so we expect an error here)
    match tcp.health() {
        Ok(()) => println!("OK TCP adapter is healthy"),
        Err(e) => eprintln!("X  TCP adapter unhealthy: {}", e),
    }

    // 2. (Unix only) Unix-domain socket adapter
    #[cfg(unix)]
    {
        use pheno_port_adapter::adapters::unix::UnixAdapter;
        let unix = UnixAdapter::new();
        println!("Unix adapter: name={}", unix.name());
    }

    Ok(())
}
