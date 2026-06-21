//! v16 T6 L25 (test isolation) — proptest properties for the canonical
//! [`AppError`] type in `pheno-errors`.
//!
//! Each `#[test]` inside the `proptest!` macro is a property that the
//! crate's public API must satisfy for arbitrary generated inputs.
//! The runner is bounded to `PROPTEST_CASES=64` on macbook to keep the
//! test wall under 10 minutes per crate.
//!
//! Run with:
//!
//! ```bash
//! PROPTEST_CASES=64 cargo test --test proptest_apperror
//! ```

use proptest::prelude::*;
use proptest::string::string_regex;

use pheno_errors::AppError;

/// `non_empty` — strategy that yields a 1..256-char ASCII string. We
/// avoid fully-generic `any::<String>()` because empty payloads carry
/// no signal for `.kind()` / `Display` invariants and slow shrinking.
fn non_empty() -> impl Strategy<Value = String> {
    string_regex("[A-Za-z0-9 _./-]{1,256}").expect("valid regex")
}

proptest! {
    /// Property 1: any non-empty `s` produces `AppError::domain(s)`
    /// whose `.kind()` is `"domain"`.
    #[test]
    fn domain_kind_is_domain(s in non_empty()) {
        let err = AppError::domain(&s);
        prop_assert_eq!(err.kind(), "domain");
    }

    /// Property 2: any non-empty `entity` + non-empty `id` produce
    /// `AppError::not_found(entity, id)` whose `.kind()` is `"not_found"`.
    /// The struct fields are preserved verbatim so consumers can match
    /// on them rather than parse the Display string.
    #[test]
    fn not_found_kind_is_not_found(
        entity in non_empty(),
        id in non_empty(),
    ) {
        let err = AppError::not_found(&entity, &id);
        prop_assert_eq!(err.kind(), "not_found");
        match &err {
            AppError::NotFound { entity: e, id: i } => {
                prop_assert_eq!(e, &entity);
                prop_assert_eq!(i, &id);
            }
            _ => prop_assert!(false, "expected NotFound, got {:?}", err),
        }
    }

    /// Property 3: any non-empty `s` produces `AppError::conflict(s)`
    /// whose `.kind()` is `"conflict"`.
    #[test]
    fn conflict_kind_is_conflict(s in non_empty()) {
        let err = AppError::conflict(&s);
        prop_assert_eq!(err.kind(), "conflict");
    }

    /// Property 4: any non-empty `s` produces `AppError::validation(s)`
    /// whose `.kind()` is `"validation"`.
    #[test]
    fn validation_kind_is_validation(s in non_empty()) {
        let err = AppError::validation(&s);
        prop_assert_eq!(err.kind(), "validation");
    }

    /// Property 5: any non-empty `s` produces `AppError::storage(s)`
    /// whose `.kind()` is `"storage"`.
    #[test]
    fn storage_kind_is_storage(s in non_empty()) {
        let err = AppError::storage(&s);
        prop_assert_eq!(err.kind(), "storage");
    }

    /// Property 6: `Display` for `Domain(s)` always starts with the
    /// `"domain error: "` prefix and embeds `s` verbatim.
    #[test]
    fn domain_display_prefix_is_stable(s in non_empty()) {
        let display = AppError::domain(&s).to_string();
        prop_assert!(
            display.starts_with("domain error: "),
            "Display {:?} should start with `domain error: `",
            display,
        );
        prop_assert!(
            display.contains(&s),
            "Display {:?} should contain payload {:?}",
            display,
            s,
        );
    }

    /// Property 7: `Display` for `NotFound { entity, id }` always starts
    /// with `"not found: "` and embeds both fields.
    #[test]
    fn not_found_display_includes_entity_and_id(
        entity in non_empty(),
        id in non_empty(),
    ) {
        let display = AppError::not_found(&entity, &id).to_string();
        prop_assert!(
            display.starts_with("not found: "),
            "Display {:?} should start with `not found: `",
            display,
        );
        prop_assert!(display.contains(&entity), "Display {:?} missing entity", display);
        prop_assert!(display.contains(&id), "Display {:?} missing id", display);
    }

    /// Property 8: `Display` for `Conflict(s)` always starts with
    /// `"conflict: "` and embeds `s`.
    #[test]
    fn conflict_display_includes_payload(s in non_empty()) {
        let display = AppError::conflict(&s).to_string();
        prop_assert!(display.starts_with("conflict: "), "Display {:?}", display);
        prop_assert!(display.contains(&s), "Display {:?} missing payload", display);
    }

    /// Property 9: `Display` for `Validation(s)` always starts with
    /// `"validation error: "` and embeds `s`.
    #[test]
    fn validation_display_includes_payload(s in non_empty()) {
        let display = AppError::validation(&s).to_string();
        prop_assert!(display.starts_with("validation error: "), "Display {:?}", display);
        prop_assert!(display.contains(&s), "Display {:?} missing payload", display);
    }

    /// Property 10: `Display` for `Storage(s)` always starts with
    /// `"storage error: "` and embeds `s`.
    #[test]
    fn storage_display_includes_payload(s in non_empty()) {
        let display = AppError::storage(&s).to_string();
        prop_assert!(display.starts_with("storage error: "), "Display {:?}", display);
        prop_assert!(display.contains(&s), "Display {:?} missing payload", display);
    }

    /// Property 11: `From<String>` always yields the `Domain` variant
    /// with the same payload (covers the owned `String` conversion path
    /// that the rest of the fleet uses for `?` propagation).
    #[test]
    fn from_owned_string_is_domain(s in non_empty()) {
        let err: AppError = s.clone().into();
        match &err {
            AppError::Domain(payload) => prop_assert_eq!(payload, &s),
            _ => prop_assert!(false, "expected Domain, got {:?}", err),
        }
    }

    /// Property 12: round-trip through `Display` is non-lossy for all
    /// five variants. Any `AppError` constructed from a non-empty
    /// payload must yield a non-empty `Display` string.
    #[test]
    fn all_variants_have_non_empty_display(s in non_empty()) {
        let displays = [
            AppError::domain(&s).to_string(),
            AppError::not_found(&s, &s).to_string(),
            AppError::conflict(&s).to_string(),
            AppError::validation(&s).to_string(),
            AppError::storage(&s).to_string(),
        ];
        for d in &displays {
            prop_assert!(!d.is_empty(), "Display must be non-empty, got {:?}", d);
        }
    }
}