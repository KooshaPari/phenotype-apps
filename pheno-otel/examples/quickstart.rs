//! Quickstart example for pheno-otel.
//!
//! Run with:
//!   cargo run --example quickstart

use pheno_otel::init_with_stdout;
use pheno_otel::TelemetryGuard;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard: TelemetryGuard = init_with_stdout("pheno-otel-quickstart")?;
    println!("hello from pheno-otel quickstart");
    drop(_guard);
    Ok(())
}
