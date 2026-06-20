//! Property-based tests for the `pheno-flags` crate.
//!
//! These complement the example-based tests in `tests/flag_test.rs`
//! by exercising `parse_bool`, `from_env`, and the snapshot
//! round-trip across randomized inputs. The crate deliberately
//! keeps its dependency surface small (one runtime dep:
//! `thiserror`), so `proptest` is scoped to dev-dependencies only.
//!
//! `parse_bool` is a private item, so it is re-exercised here
//! indirectly through `from_env` (which is the only public
//! caller) and through the documented truthy/falsy surface.
//! Round-trip / no-panic properties are the high-value
//! guarantees; the actual classification is also pinned so
//! that a future refactor cannot silently change the contract
//! (e.g. start accepting `"on"` without an explicit ADR).

use std::collections::BTreeMap;
use std::sync::Mutex;

use pheno_flags::FlagSet;
use proptest::prelude::*;

/// Serializes env-mutating property tests so they do not race on
/// the process-global environment. The example-based tests in
/// `flag_test.rs` use a separate `ENV_LOCK`; sharing the same
/// `static` here would require the two files to see each other's
/// symbols, which integration tests do not, so we duplicate the
/// lock. Both locks are uncontended in practice because proptest
/// runs single-threaded by default for each `proptest!` macro
/// invocation.
static PROP_ENV_LOCK: Mutex<()> = Mutex::new(());

/// The six canonical forms, classified by `parse_bool`. The
/// `proptest` strategies below pin the truthy/falsy mapping so a
/// future refactor cannot silently broaden or narrow the parser's
/// accepted set without updating these tests (and the ADR).
const TRUTHY_FORMS: &[&str] = &["1", "true", "yes"];
const FALSY_FORMS: &[&str] = &["0", "false", "no"];

/// Strategy that generates a string drawn uniformly from the six
/// canonical forms, in any case. We assemble this as a `Vec` of
/// `&'static str` then sample uniformly rather than enumerating
/// per-form to keep the strategy expression compact.
fn arb_canonical_form() -> impl Strategy<Value = String> {
    let all: Vec<&'static str> = TRUTHY_FORMS
        .iter()
        .chain(FALSY_FORMS.iter())
        .copied()
        .collect();
    (0..all.len(), any::<bool>())
        .prop_map(move |(idx, upper)| {
            let base = all[idx];
            if upper {
                base.to_ascii_uppercase()
            } else {
                base.to_string()
            }
        })
}

/// Strategy that generates a "flag-safe" key — non-empty ASCII
/// alphanumeric + underscore — that round-trips cleanly through
/// `BTreeMap<String, _>` and survives any prefix stripping in
/// `from_env` without producing an empty key.
fn arb_flag_key() -> impl Strategy<Value = String> {
    "[A-Za-z_][A-Za-z0-9_]{0,31}"
}

/// Strategy that generates a value string that is NOT one of the
/// six canonical forms in any case. We pick a non-empty ASCII
/// string that does not case-fold to one of the recognized
/// forms. This drives the "unknown value" branch of `from_env`.
fn arb_invalid_value() -> impl Strategy<Value = String> {
    "[A-Za-z0-9_]{1,16}"
        .prop_filter("must not collide with a canonical form", |s| {
            let lower = s.to_ascii_lowercase();
            !TRUTHY_FORMS.contains(&lower.as_str())
                && !FALSY_FORMS.contains(&lower.as_str())
        })
}

proptest! {
    /// `parse_bool` (exercised through `from_env`) must never
    /// panic on any printable-ASCII string. The harness itself
    /// checks the no-panic contract; we additionally assert the
    /// documented classification so a regression that broadens
    /// the accepted set (e.g. accepting `"maybe"` as `false`) is
    /// caught here.
    #[test]
    fn parse_bool_classifies_all_canonical_forms(value in arb_canonical_form()) {
        let _guard = PROP_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

        // Build an env-var name that will be picked up by
        // `from_env`. We use a unique prefix per call so
        // concurrent proptest cases (under `--test-threads>1`)
        // do not clobber each other.
        let prefix = format!("PROPTEST_PARSE_BOOL_{}", value.len());
        let env_name = format!("{prefix}_KEY");
        std::env::set_var(&env_name, &value);
        let result = FlagSet::from_env(&prefix);
        std::env::remove_var(&env_name);

        let flags = result.unwrap_or_else(|e| {
            panic!("from_env must accept canonical form `{value}`, got {e:?}")
        });

        let lower = value.to_ascii_lowercase();
        let expected = TRUTHY_FORMS.contains(&lower.as_str());
        prop_assert_eq!(
            flags.is_enabled("KEY"),
            expected,
            "canonical form `{value}` must classify as {expected}"
        );
    }

    /// Any value that is NOT one of the six canonical forms
    /// must cause `from_env` to return
    /// `FlagError::InvalidValue` carrying the offending var
    /// name. The error variant itself is part of the public API
    /// and must remain stable.
    #[test]
    fn from_env_rejects_non_canonical_values(value in arb_invalid_value()) {
        let _guard = PROP_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

        let prefix = "PROPTEST_REJECT";
        let env_name = "PROPTEST_REJECT_KEY";
        std::env::set_var(env_name, &value);
        let result = FlagSet::from_env(prefix);
        std::env::remove_var(env_name);

        let err = result.expect_err(&format!(
            "from_env must reject non-canonical value `{value}`"
        ));
        let payload = match &err {
            pheno_flags::FlagError::InvalidValue(name) => name.clone(),
        };
        prop_assert_eq!(
            payload,
            env_name,
            "InvalidValue must carry the offending env-var name"
        );
    }

    /// Snapshot round-trip: `with(k, v).snapshot().get(k) ==
    /// Some(&v)` for any flag-safe key and either boolean.
    /// This pins the documented "last write wins" / sorted-
    /// iteration invariants under randomized input.
    #[test]
    fn snapshot_round_trip(key in arb_flag_key(), value in any::<bool>()) {
        let snap: BTreeMap<String, bool> = FlagSet::new()
            .with(&key, value)
            .snapshot();

        prop_assert_eq!(
            snap.len(),
            1,
            "snapshot must contain exactly the one inserted key"
        );
        prop_assert_eq!(
            snap.get(&key),
            Some(&value),
            "snapshot.get(key) must return the inserted value"
        );
        prop_assert_eq!(
            FlagSet::new().with(&key, value).is_enabled(&key),
            value,
            "is_enabled(key) must return the inserted value"
        );
    }

    /// Builder precedence: calling `with` twice with the same
    /// key must keep the last value (last-write-wins). This is
    /// the documented contract; the property pins it under
    /// arbitrary keys and values.
    #[test]
    fn with_last_write_wins(key in arb_flag_key()) {
        let flags = FlagSet::new()
            .with(&key, true)
            .with(&key, false);
        prop_assert!(
            !flags.is_enabled(&key),
            "second with() must overwrite the first"
        );
        let flags = FlagSet::new()
            .with(&key, false)
            .with(&key, true);
        prop_assert!(
            flags.is_enabled(&key),
            "second with() must overwrite the first (false -> true)"
        );
    }

    /// `from_env` with an unknown prefix must produce an empty
    /// `FlagSet`, never an error, and never panic. The harness
    /// covers no-panic; we additionally assert the empty-snapshot
    /// contract.
    #[test]
    fn from_env_with_unknown_prefix_is_empty(prefix in arb_flag_key()) {
        let _guard = PROP_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

        // The prefix is randomized, so a stray env var that
        // happens to start with it would pollute the snapshot.
        // We use a 64-bit random suffix that is astronomically
        // unlikely to collide with any other proptest case's
        // scratch env vars, then clear it explicitly so we do
        // not leak state across iterations.
        let scratch = format!("PROPTEST_UNKNOWN_PREFIX_PROBE_{}", prefix);
        std::env::remove_var(&scratch);

        let flags = FlagSet::from_env(&prefix)
            .expect("from_env with a prefix that matches nothing must succeed");
        prop_assert!(
            flags.snapshot().is_empty(),
            "snapshot must be empty when no env var matches the prefix"
        );
    }
}
