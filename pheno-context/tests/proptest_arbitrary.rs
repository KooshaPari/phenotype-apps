//! v20-T5 — `proptest::Arbitrary` round-trip test for `pheno-context::Context`.
//!
//! Generates random `Context` values via `any::<Context>()` (driven by the
//! `Arbitrary` derive on `Context`) and verifies the serde JSON
//! round-trip property: serialize → deserialize → equal. This catches
//! any future refactor that breaks `Serialize`/`Deserialize` symmetry
//! (e.g. adding a field to `Context` but forgetting the `Serialize`
//! derive, or accidentally introducing non-JSON-safe types).

use proptest::prelude::*;

use pheno_context::Context;

proptest! {
    /// For any `Context` produced by the `Arbitrary` impl,
    /// `serde_json::to_string` followed by `serde_json::from_str`
    /// must yield the original value. Any panic / data loss / wrong
    /// round-trip here is a regression on the `Serialize` + `Deserialize`
    /// symmetry of the type.
    #[test]
    fn context_serde_roundtrip(ctx in any::<Context>()) {
        let json = serde_json::to_string(&ctx).expect("serialize Context");
        let back: Context = serde_json::from_str(&json).expect("deserialize Context");
        prop_assert_eq!(ctx, back);
    }
}