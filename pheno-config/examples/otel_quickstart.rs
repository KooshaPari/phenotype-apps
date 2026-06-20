//! `pheno-config` × `pheno-otel` quickstart.
//!
//! Demonstrates wiring the canonical typed-config loader (env + JSON
//! file cascade) with the `pheno-otel` OTLP exporter (ADR-037) to
//! ship a "Config resolved" log line to an OTLP/HTTP collector (or
//! to stderr in local dev).
//!
//! ## What this shows
//!
//! 1. Build a [`pheno_otel::StdoutExporter`] (writes OTLP/JSON to
//!    stderr; no backend required).
//! 2. Build a `pheno_config::Config` with default values and serialize
//!    it to JSON.
//! 3. `export()` the resolved-config payload through the
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

use pheno_config::Config;
use pheno_otel::exporters::stdout::StdoutExporter;
use pheno_otel::OtlpPort;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Build the canonical OTLP exporter (stdout for local dev).
    let exporter = StdoutExporter::new();

    // 2. Resolve a Config (default values; no env / file cascade).
    let cfg = Config::default();
    let body = serde_json::to_string(&cfg)?;

    // 3. Ship a one-line OTLP/JSON log payload to stderr.
    let payload = format!(
        r#"{{"resourceLogs":[{{"resource":{{}},"scopeLogs":[{{"logRecords":[{{"severityText":"INFO","body":{{"stringValue":"Config resolved: {}"}}}}]}}]}}]}}"#,
        body
    );
    exporter.export(&payload)?;
    println!("exported config: {}", body);
    Ok(())
}
