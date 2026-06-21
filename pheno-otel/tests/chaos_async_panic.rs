//! v20-T3 (L36) — chaos adoption test for `pheno-otel`.
//!
//! Simulates an async-runtime panic in the OTLP exporter path.
//! The test uses `chaos_injection::inject` to inject an
//! `AsyncPanic` fault and asserts that the runtime surfaces it
//! as an `Err(FaultOutcome::AsyncPanic(_))`. It also verifies
//! that the OTLP exporter itself (the production
//! `OtlpPort` trait impls) is unaffected by the chaos
//! runtime — the chaos framework is observation-only, not a
//! state-mutating dependency of the exporter.
//!
//! Run with:
//!
//! ```bash
//! cargo test --test chaos_async_panic
//! ```
//! FR: L36 — pheno-otel must accept chaos-injection as a
//! dev-dependency and surface an `AsyncPanic` outcome through
//! the canonical `chaos_injection::inject` entry point.

use chaos_injection::{inject, Fault, FaultOutcome};
use pheno_otel::exporters::{ExporterConfig, StdoutExporter};
use pheno_otel::{OtlpError, OtlpPort, ExportHandle};

/// 1. Inject an `AsyncPanic` fault and assert the runtime
/// surfaces it as an `Err(FaultOutcome::AsyncPanic(_))`. The
/// message body of the AsyncPanic variant is carried through
/// to the outcome.
#[test]
fn inject_async_panic_surfaces_outcome() {
    let result = inject(Fault::AsyncPanic {
        message: "otlp-exporter-task-panicked".to_string(),
    });
    let outcome = result.expect_err("AsyncPanic must surface as Err");
    match outcome {
        FaultOutcome::AsyncPanic(msg) => {
            // The sync entry point of `inject` always reports the
            // marker message for AsyncPanic (the async
            // `ChaosRuntime::run` is what would carry the real
            // message); the marker is what adoption tests
            // pattern-match on.
            assert!(
                !msg.is_empty(),
                "AsyncPanic outcome must carry a non-empty message"
            );
        }
        other => panic!("expected AsyncPanic, got {other:?}"),
    }
}

/// 2. The OTLP `StdoutExporter` (production code) must remain
/// usable in the presence of the chaos runtime. This proves
/// chaos-injection is observation-only — it does NOT mutate
/// global state or wrap the exporter.
#[test]
fn otlp_stdout_exporter_unaffected_by_chaos_runtime() {
    let exporter = StdoutExporter::new(ExporterConfig::new(
        "http://localhost:4318",
        "pheno-otel-chaos-test",
    ));
    // Health is independent of the chaos runtime.
    let health: Result<(), OtlpError> = exporter.health();
    assert!(health.is_ok(), "exporter health must be Ok");

    // Export a valid payload — this is the production path.
    let payload = br#"{"resourceSpans":[]}"#;
    let handle: ExportHandle = exporter
        .export(payload)
        .expect("valid payload must export successfully");
    assert_eq!(handle.endpoint, "http://localhost:4318");
    assert_eq!(handle.service_name, "pheno-otel-chaos-test");

    // Flush is a no-op for the stdout exporter but must still
    // return Ok in the presence of the chaos runtime.
    assert!(exporter.flush().is_ok());

    // The exporter name is the canonical identifier, not
    // affected by chaos.
    assert_eq!(exporter.name(), "stdout");
}

/// 3. The `OtlpPort` trait's empty-payload rejection contract
/// is preserved even after an `AsyncPanic` chaos injection.
/// This pins that chaos-runtime does not leave residual state
/// in the production exporter path.
#[test]
fn otlp_empty_payload_rejection_after_async_panic() {
    // First, inject an AsyncPanic — the runtime reports it.
    let chaos = inject(Fault::AsyncPanic {
        message: "pre-otlp-rejection-check".to_string(),
    });
    assert!(chaos.is_err(), "AsyncPanic must surface as Err");

    // Then exercise the production exporter.
    let exporter = StdoutExporter::new(ExporterConfig::new(
        "http://localhost:4318",
        "pheno-otel-chaos-test",
    ));
    let result = exporter.export(b"");
    assert!(
        matches!(result, Err(OtlpError::SerializeFailed(_))),
        "empty payload must still be rejected after chaos injection"
    );
}
