//! Quickstart example for pheno-otel.
//!
//! Run with stdout exporter (no OTLP collector needed):
//!   cargo run --example quickstart
//!
//! Run with the OTLP exporter (requires a collector on
//! `OTEL_EXPORTER_OTLP_ENDPOINT` or the default `http://localhost:4318`):
//!   cargo run --example quickstart --features otlp
//!
//! To also see `tracing` spans flow through the OTLP pipeline, enable
//! the `tracing` feature:
//!   cargo run --example quickstart --features tracing

#[cfg(not(feature = "otlp"))]
use pheno_otel::init_with_stdout;
#[cfg(feature = "otlp")]
use pheno_otel::init;
use pheno_otel::TelemetryGuard;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the OTLP pipeline; the guard flushes on drop.
    #[cfg(not(feature = "otlp"))]
    let _guard: TelemetryGuard = init_with_stdout("pheno-otel-quickstart")?;
    #[cfg(feature = "otlp")]
    let _guard: TelemetryGuard = init("pheno-otel-quickstart")?;

    // Emit a span (only when the `tracing` feature is on; otherwise
    // we fall back to a plain println so the default build still works).
    #[cfg(feature = "tracing")]
    {
        let span = tracing::info_span!("quickstart");
        let _enter = span.enter();
        tracing::info!("hello from pheno-otel quickstart");
    }
    #[cfg(not(feature = "tracing"))]
    {
        println!("hello from pheno-otel quickstart");
    }

    // Guard drops at end of main → flush + shutdown.
    drop(_guard);
    Ok(())
}
