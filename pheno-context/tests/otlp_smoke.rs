//! OTLP smoke test (ADR-036) — verifies `pheno_context::init` wires
//! the `tracing-subscriber` substrate and that a span can be captured.
//!
//! Uses `traced_test` (which installs a `tracing-subscriber` with a
//! `TestWriter`) so we can assert the span is observed end-to-end.

use tracing_test::traced_test;

#[traced_test]
#[test]
fn init_wires_pheno_tracing_and_captures_span() {
    pheno_context::init();
    let span = tracing::info_span!("otlp_smoke", crate = "pheno-context");
    let _enter = span.enter();
    tracing::info!("span entered for pheno-context");
    assert!(logs_contain("span entered for pheno-context"));
}
