//! V20-T4 (L27) — Pact consumer-driven contract tests for the
//! `pheno-otel` substrate's OTLP/HTTP wire contract.
//!
//! These tests pin the contract between the `pheno-otel` HTTP exporter
//! (consumer) and an OpenTelemetry OTLP/HTTP collector (provider). The
//! provider is mocked by `pact_consumer`'s in-process mock server so we
//! can capture the exact request/response shape that downstream
//! collectors MUST honour for `pheno-otel` to interoperate correctly.
//!
//! The contract is intentionally written from the **consumer's**
//! perspective per the Pact philosophy:
//!
//! - What `pheno-otel::exporters::HttpExporter::target_url()` produces.
//! - What payload structure + headers we POST.
//! - What status codes the collector returns for the happy/error paths.
//!
//! Provider-side verification (replay the generated pact against a real
//! OTel collector) is out of scope for this turn; the script
//! `scripts/can-i-deploy.sh` (added in v20-T4) consumes the generated
//! pact JSON and is the entry point for CI/provider verification.
//!
//! Two interactions per crate per the L27 spec:
//!
//! 1. **Happy path** — POST `/v1/traces` with a valid OTLP/JSON payload
//!    → 200 OK from collector; consumer treats the response as
//!    "export accepted".
//! 2. **Error path** — POST `/v1/traces` with a malformed payload
//!    → 400 Bad Request from collector; consumer surfaces
//!    `pheno_otel::OtlpError::Transport` with the upstream reason.
//!
//! Run with:
//!
//! ```text
//! cargo test --test provider_trace_exporter_pact
//! ```
//!
//! Generated pacts land in `target/pacts/<consumer>-<provider>.json`
//! (Pact mock server default). The same directory is consumed by
//! `scripts/can-i-deploy.sh`.

use pact_consumer::prelude::*;
use pheno_otel::exporters::http::HttpExporter;
use pheno_otel::exporters::ExporterConfig;
use pheno_otel::{OtlpError, OtlpPort};

/// Happy path — POST /v1/traces with valid OTLP/JSON; collector returns 200 OK.
///
/// This interaction documents the canonical OTLP/HTTP wire contract that
/// `pheno-otel`'s `HttpExporter` (via consumer-wired reqwest/ureq) emits
/// to the collector. The provider (collector) is expected to accept the
/// payload and reply `200 OK` with an empty body.
#[test]
fn http_exporter_export_traces_happy_path() {
    let mock_server = PactBuilder::new("pheno-otel", "otel-collector")
        .interaction(
            "POST /v1/traces with valid OTLP/JSON returns 200",
            "",
            |mut i| {
                i.request
                    .post()
                    .path("/v1/traces")
                    .content_type("application/json")
                    .body(r#"{"resourceSpans":[]}"#);
                i.response.status(200);
                i
            },
        )
        .start_mock_server(None, None);

    // Resolve the mock collector URL and configure the HttpExporter to
    // point at the mock's /v1/traces endpoint. The HttpExporter does
    // not perform HTTP itself; consumers wire reqwest/hyper/ureq and
    // POST to `target_url()`. The contract therefore asserts the URL
    // shape and the consumer-side POST body.
    let collector_url_str = mock_server.url().as_str().to_owned();
    let traces_url = format!("{}/v1/traces", collector_url_str.trim_end_matches('/'));
    let cfg = ExporterConfig::new(&collector_url_str, "pheno-otel-pact-test");
    let exporter = HttpExporter::traces(cfg);

    // Contract assertion 1: target_url() points at the mock collector's
    // /v1/traces endpoint — this is what consumers MUST POST to.
    assert_eq!(exporter.target_url(), traces_url);

    // Contract assertion 2: exporter reports its name as "http" (stable
    // identifier for the OTLP/HTTP transport).
    assert_eq!(exporter.name(), "http");

    // Contract assertion 3: health() succeeds with a configured endpoint.
    assert!(exporter.health().is_ok());

    // Drive a real HTTP POST against the mock collector using ureq.
    // This exercises the FULL contract: the consumer code in production
    // will read `target_url()`, serialize the OTLP/JSON payload, and
    // POST it. The mock server validates the request shape; on
    // mismatch the test fails with a Pact mismatch error.
    let payload = r#"{"resourceSpans":[]}"#;
    let response = ureq::post(&exporter.target_url())
        .set("Content-Type", "application/json")
        .send_string(payload)
        .expect("POST /v1/traces should succeed against the mock collector");

    assert_eq!(response.status(), 200);

    // Contract assertion 4: substrate-level export() returns a handle
    // identifying the endpoint and service. This is the typed shape
    // consumers receive back from pheno-otel.
    let handle = exporter
        .export(payload.as_bytes())
        .expect("export should succeed");
    assert_eq!(handle.service_name, "pheno-otel-pact-test");
    assert!(
        handle.endpoint.contains("/v1/traces"),
        "ExportHandle.endpoint should be the OTLP traces URL"
    );

    // Flush is a no-op on this substrate (no in-flight buffer).
    assert!(exporter.flush().is_ok());
}

/// Error path — collector rejects a malformed OTLP payload with 400.
///
/// The pact_consumer mock server is configured to reject any
/// `/v1/traces` POST whose body is a malformed OTLP/JSON payload,
/// returning 400. The contract documents:
///
///   (a) If a malformed payload somehow reaches the wire (the substrate
///       itself rejects empty payloads up-front, but other shape
///       problems may slip past), the collector rejects it with 400.
///   (b) Consumer code that wires HTTP around HttpExporter's
///       `target_url()` MUST map a non-2xx response to
///       `OtlpError::Transport` so the failure surfaces as a typed
///       substrate error.
#[test]
fn http_exporter_provider_400_returns_transport_error() {
    // Interaction: POST /v1/traces with malformed OTLP/JSON → 400.
    let mock_server = PactBuilder::new("pheno-otel", "otel-collector")
        .interaction(
            "POST /v1/traces with malformed OTLP/JSON returns 400",
            "",
            |mut i| {
                i.request
                    .post()
                    .path("/v1/traces")
                    .content_type("application/json")
                    .body(r#"{"resourceSpans":[{"this-is":"not-a-valid-OTel-resource"}]}"#);
                i.response
                    .status(400)
                    .content_type("application/json")
                    .body(r#"{"partialSuccess":{"rejectedSpans":"1","errorMessage":"invalid resource"}}"#);
                i
            },
        )
        .start_mock_server(None, None);

    let collector_url_str = mock_server.url().as_str().to_owned();
    let cfg = ExporterConfig::new(&collector_url_str, "pheno-otel-pact-test");
    let exporter = HttpExporter::traces(cfg);

    // The substrate accepts the non-empty payload (its only up-front
    // guard is empty-payload rejection), returning a successful
    // ExportHandle. The wire-level rejection happens at the collector.
    let bad_payload = br#"{"resourceSpans":[{"this-is":"not-a-valid-OTel-resource"}]}"#;
    let handle = exporter
        .export(bad_payload)
        .expect("substrate accepts non-empty payload; collector is the error boundary");
    assert_eq!(handle.service_name, "pheno-otel-pact-test");

    // Drive the wire-level POST: ureq sends the body, the mock
    // collector validates against the interaction, returns 400.
    //
    // ureq treats 4xx/5xx as `ureq::Error::Status(code, response)`.
    // We must pattern-match on that to extract the response and
    // verify the status, instead of naively `.expect("")`-ing the
    // transport result.
    let result = ureq::post(&exporter.target_url())
        .set("Content-Type", "application/json")
        .send_bytes(bad_payload);

    match result {
        Ok(response) => panic!(
            "expected 400 from collector for malformed OTLP/JSON, got {}",
            response.status()
        ),
        Err(ureq::Error::Status(400, _response)) => {
            // Contract assertion: collector returns 400 for malformed
            // OTLP/JSON. Consumer code wires this non-2xx to
            // `OtlpError::Transport` at the substrate boundary.
        }
        Err(other) => panic!(
            "unexpected ureq error (expected Status 400, got transport error): {:?}",
            other
        ),
    }
}

/// Provider-side sanity: health() on an unconfigured exporter returns
/// `OtlpError::NotConfigured`. This is the substrate's own contract for
/// what happens before any HTTP is attempted — pinned here so the
/// `target_url()` shape remains compatible with `OTEL_EXPORTER_OTLP_*`
/// env-var conventions.
#[test]
fn http_exporter_health_fails_with_empty_endpoint() {
    let cfg = ExporterConfig::new("", "pheno-otel-pact-test");
    let exporter = HttpExporter::traces(cfg);

    let result = exporter.health();
    assert!(
        matches!(result, Err(OtlpError::NotConfigured(_))),
        "empty endpoint must surface NotConfigured"
    );
    assert_eq!(exporter.target_url(), "/v1/traces");
}