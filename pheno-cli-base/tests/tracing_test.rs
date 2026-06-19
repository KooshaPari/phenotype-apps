//! Tracing-feature smoke test (ADR-036).
//!
//! Verifies that the `tracing` opt-in feature compiles + links
//! correctly and that `setup_tracing` integrates with the
//! `tracing` macros. The crate's `tracing-subscriber` plumbing is
//! always available (it's a hard dep — see ADR-036 adapted note
//! in Cargo.toml); the `tracing` crate itself is gated on the
//! `tracing` feature. This test exercises both together.

#![cfg(feature = "tracing")]

use pheno_cli_base::{setup_tracing, Verbosity};

#[tracing_test::traced_test]
#[test]
fn emits_span_after_setup_tracing() {
    setup_tracing(Verbosity::default().to_filter());
    let span = tracing::info_span!("setup_test");
    let _enter = span.enter();
    tracing::info!("emitted from inside span");
}

#[tracing_test::traced_test]
#[test]
fn verbosity_default_yields_info_filter() {
    use tracing_subscriber::filter::LevelFilter;
    let v = Verbosity::default();
    assert_eq!(LevelFilter::INFO, v.to_filter());
}
