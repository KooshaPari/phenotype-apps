//! `pheno-context` × `pheno-otel` quickstart.
//!
//! Demonstrates wiring the canonical request-context substrate (per-
//! request identifiers + metadata bag) with the `pheno-otel` OTLP
//! exporter (ADR-037) to ship a "Context created" log line to an
//! OTLP/HTTP collector (or to stderr in local dev).
//!
//! ## What this shows
//!
//! 1. Build a [`pheno_otel::StdoutExporter`] (writes OTLP/JSON to
//!    stderr; no backend required).
//! 2. Build a `pheno_context::Context` via the builder.
//! 3. `export()` the context snapshot through the
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

use pheno_context::Context;
use pheno_otel::exporters::stdout::StdoutExporter;
use pheno_otel::OtlpPort;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Build the canonical OTLP exporter (stdout for local dev).
    let exporter = StdoutExporter::new();

    // 2. Build a Context (request_id / span_id / trace_id required).
    let ctx = Context::new()
        .with_request_id("req-quickstart")
        .with_span_id("span-quickstart")
        .with_trace_id("trace-quickstart")
        .with_user_id("user-otel-demo")
        .with_org_id("org-phenotype")
        .build()?;

    // 3. Ship a one-line OTLP/JSON log payload to stderr.
    let body = ctx.to_string();
    let payload = format!(
        r#"{{"resourceLogs":[{{"resource":{{}},"scopeLogs":[{{"logRecords":[{{"severityText":"INFO","body":{{"stringValue":"Context created: {}"}}}}]}}]}}]}}"#,
        body
    );
    exporter.export(&payload)?;
    println!("exported context: {}", body);
    Ok(())
}
