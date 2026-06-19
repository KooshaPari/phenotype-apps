//! Tracing-feature smoke test (ADR-036).
//!
//! Verifies that the `tracing` opt-in feature compiles + links
//! correctly and that `FlagSet` operations can be captured in a
//! `tracing` span. The crate itself does not emit spans in the
//! default code path; this test exercises the dependency
//! graph end-to-end with `tracing-test::traced_test`.

#![cfg(feature = "tracing")]

use pheno_flags::FlagSet;

#[tracing_test::traced_test]
#[test]
fn emits_span_on_flag_lookup() {
    let flags = FlagSet::new()
        .with("dark_mode", true)
        .with("beta_export", false);

    let span = tracing::info_span!("flag_lookup", key = "dark_mode");
    let _enter = span.enter();
    tracing::info!(value = flags.is_enabled("dark_mode"), "lookup");

    assert!(flags.is_enabled("dark_mode"));
    assert!(!flags.is_enabled("beta_export"));
    assert!(!flags.is_enabled("unknown_key"));
}

#[tracing_test::traced_test]
#[test]
fn emits_span_on_from_env() {
    let span = tracing::info_span!("from_env", prefix = "PHENO_FLAGS_TEST_NEVER_SET_xyz");
    let _enter = span.enter();
    let flags = FlagSet::from_env("PHENO_FLAGS_TEST_NEVER_SET_xyz").unwrap();
    tracing::info!(count = flags.snapshot().len(), "loaded");
    assert!(flags.snapshot().is_empty());
}
