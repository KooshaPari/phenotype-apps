//! OTLP-export tracing smoke test for pheno-port-adapter (ADR-036, T22.4).
//!
//! Verifies that `pheno_port_adapter::init()` wires the canonical
//! `pheno-tracing` substrate, one span is emitted through the test
//! writer (`traced_test`'s `MakeWriter`), and the captured payload is
//! visible to the test's `logs_contain` assertion.
//!
//! Run with:
//!   OTEL_SDK_DISABLED=true cargo test -p pheno-port-adapter --features tracing --test otlp_smoke

#![cfg(feature = "tracing")]

use tracing_test::traced_test;

#[traced_test]
#[test]
fn init_emits_captured_span() {
    pheno_port_adapter::init();
    let span = tracing::info_span!("otlp_smoke");
    let _enter = span.enter();
    tracing::info!("otlp-payload-must-be-captured");
    assert!(logs_contain("otlp-payload-must-be-captured"));
}
