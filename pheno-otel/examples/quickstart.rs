//! Quickstart example for pheno-otel.
//!
//! Run with:
//!   OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318 cargo run --example quickstart
//!
//! Or with stdout exporter (no OTLP collector needed):
//!   cargo run --example stdout_demo

use pheno_otel::{init_with_stdout, TelemetryGuard};
use tracing::info;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize OTLP pipeline; guard flushes on drop
    let _guard: TelemetryGuard = init_with_stdout("pheno-otel-quickstart")?;

    // Emit a span (requires `tracing` feature)
    let span = tracing::info_span!("quickstart");
    let _enter = span.enter();
    info!("hello from pheno-otel quickstart");

    // Guard drops at end of main → flush + shutdown
    drop(_guard);
    Ok(())
}
