//! v20-T5 — proptest smoke test for `pheno-port-adapter`.
//!
//! Generates random `AdapterError` and `Connection` values via the
//! hand-rolled `proptest::Arbitrary` impls (in `src/lib.rs`) and
//! verifies the round-trip / Display invariants:
//!
//! 1. `AdapterError` — every variant produces a non-empty `Display`
//!    string (no accidentally-blank diagnostic output for any
//!    variant). This is a regression guard against a future
//!    `thiserror::Error` change that drops a `#[error("...")]`
//!    attribute and silently swallows the error message.
//! 2. `Connection` — every `Connection` produces a non-empty
//!    `Debug` string. The `id` field is `pub(crate)` (intentionally
//!    hidden from external callers so the only construction path is
//!    via `PortAdapter::connect`), so the smoke test verifies the
//!    opaque handle round-trips through `Debug` without panicking
//!    or producing blank output.
//!
//! Property tests are scoped to 100 cases by default; proptest
//! tunes this in CI via the `PROPTEST_CASES` env var.

use proptest::prelude::*;

use pheno_port_adapter::{AdapterError, Connection};

proptest! {
    /// For any `AdapterError` produced by the `Arbitrary` impl,
    /// the `Display` output is non-empty. Catches the
    /// `#[error("...")]`-attribute-dropped regression class.
    #[test]
    fn adapter_error_display_is_nonempty(err in any::<AdapterError>()) {
        let s = format!("{}", err);
        prop_assert!(!s.is_empty(), "Display output for {:?} was empty", err);
    }

    /// For any `Connection` produced by the `Arbitrary` impl, the
    /// `Debug` output is non-empty. Catches regressions where the
    /// `Debug` impl is accidentally short-circuited (e.g. via
    /// `#[derive(Debug)]` being removed).
    #[test]
    fn connection_debug_is_nonempty(conn in any::<Connection>()) {
        let dbg = format!("{:?}", conn);
        prop_assert!(!dbg.is_empty(), "Debug output for Connection was empty");
    }
}
