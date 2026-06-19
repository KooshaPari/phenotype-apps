//! Quickstart example for pheno-port-adapter.
//!
//! Run with:
//!   cargo run --example quickstart

use pheno_port_adapter::adapters::tcp::TcpAdapter;
use pheno_port_adapter::PortAdapter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tcp = TcpAdapter::new();
    println!("TCP adapter: name={}", tcp.name());

    match tcp.health() {
        Ok(()) => println!("OK TCP adapter is healthy"),
        Err(e) => eprintln!("X  TCP adapter unhealthy: {}", e),
    }

    #[cfg(unix)]
    {
        use pheno_port_adapter::adapters::unix::UnixAdapter;
        let unix = UnixAdapter::new();
        println!("Unix adapter: name={}", unix.name());
    }

    Ok(())
}
