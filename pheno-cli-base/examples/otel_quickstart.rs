//! `pheno-cli-base` × `pheno-otel` quickstart.
//!
//! Demonstrates wiring the canonical CLI substrate (`Verbosity`,
//! `ConfigArg`, `setup_tracing`) with the `pheno-otel` OTLP exporter
//! (ADR-037) to ship a "CLI ready" log line to an OTLP/HTTP collector
//! (or to stderr in local dev).
//!
//! ## What this shows
//!
//! 1. Build a [`pheno_otel::StdoutExporter`] (writes OTLP/JSON to
//!    stderr; no backend required).
//! 2. Resolve a `pheno_cli_base::Verbosity` to an integer level and a
//!    `pheno_cli_base::ConfigArg::path()` lookup.
//! 3. `export()` the CLI-ready payload through the
//!    [`pheno_otel::OtlpPort`] trait — the canonical hexagonal Port
//!    (ADR-038).
//!
//! ## Run it
//!
//! ```bash
//! cargo run --example otel_quickstart
//! ```
//!
//! ## When NOT to use
//!
//! - For prod-grade OTLP shipping, install an OTel Collector and point
//!   `HttpExporter` at it. `StdoutExporter` is local-dev only.
//! - For polyglot consumers, mirror to `phenotype-go-sdk` /
//!   `phenotype-python-sdk`.

use pheno_cli_base::{ConfigArg, Verbosity};
use pheno_otel::exporters::stdout::StdoutExporter;
use pheno_otel::OtlpPort;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Build the canonical OTLP exporter (stdout for local dev).
    let exporter = StdoutExporter::new();

    // 2. Resolve CLI defaults (no clap parse; quickstart is hermetic).
    let verbosity = Verbosity::default();
    let config = ConfigArg::default();
    let level = verbosity.to_filter();
    let cfg_path = config.path().unwrap_or_else(|| "<none>".to_string());
    let body = format!("verbosity_filter={:?} config_path={}", level, cfg_path);

    // 3. Ship a one-line OTLP/JSON log payload to stderr.
    let payload = format!(
        r#"{{"resourceLogs":[{{"resource":{{}},"scopeLogs":[{{"logRecords":[{{"severityText":"INFO","body":{{"stringValue":"CLI ready: {}"}}}}]}}]}}]}}"#,
        body
    );
    exporter.export(&payload)?;
    println!("exported: {}", body);
    Ok(())
}
