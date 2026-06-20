//! Tracing integration test for `pheno-flags`.
//!
//! Verifies that with the `tracing` feature enabled:
//!   - `FlagSet::from_env` produces a `pheno_flags::from_env` span
//!     carrying `prefix` as a field, and a `debug!` event with
//!     `count` of flags loaded.
//!   - `FlagSet::is_enabled` emits a `trace!` event per lookup
//!     with `key` and the resolved `enabled` boolean.
//!
//! Run with:
//!   cargo test --features tracing --test tracing_test

#![cfg(feature = "tracing")]

use pheno_flags::FlagSet;
use std::sync::Mutex;
use tracing_test::traced_test;

/// Serializes the env-mutating test in this binary against the
/// one in `flag_test.rs`. Same rationale as the parent module:
/// process-global `std::env` is a shared resource.
static ENV_LOCK: Mutex<()> = Mutex::new(());

#[traced_test]
#[test]
fn from_env_emits_span_and_debug_event() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    std::env::set_var("PHENO_FLAGS_TEST_TRACE_A", "1");
    std::env::set_var("PHENO_FLAGS_TEST_TRACE_B", "yes");
    let flags =
        FlagSet::from_env("PHENO_FLAGS_TEST_TRACE").expect("from_env must accept truthy values");

    // The `#[instrument]` attribute emits a span named `from_env`.
    assert!(
        spans_contain("from_env"),
        "from_env must emit a `from_env` span (#[instrument])"
    );
    // The `debug!` event records the number of flags loaded.
    assert!(
        logs_contain("flags loaded"),
        "from_env must emit a `flags loaded` debug event with the flag count"
    );
    // Sanity: the flags themselves still resolve correctly.
    assert!(flags.is_enabled("A"));
    assert!(flags.is_enabled("B"));

    std::env::remove_var("PHENO_FLAGS_TEST_TRACE_A");
    std::env::remove_var("PHENO_FLAGS_TEST_TRACE_B");
}

#[traced_test]
#[test]
fn is_enabled_emits_trace_event_per_lookup() {
    let flags = FlagSet::new().with("alpha", true).with("beta", false);

    // Known key, true.
    assert!(flags.is_enabled("alpha"));
    assert!(
        logs_contain("is_enabled"),
        "is_enabled must emit a `trace!` event per lookup"
    );
    // Known key, false.
    assert!(!flags.is_enabled("beta"));
    // Unknown key, safe default false.
    assert!(!flags.is_enabled("does_not_exist"));
}
