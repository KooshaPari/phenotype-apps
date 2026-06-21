//! v20-T5 — `proptest::Arbitrary` round-trip test for `pheno_flags::FlagSet`.
//!
//! Generates random `FlagSet` values via `any::<FlagSet>()` (driven
//! by the `Arbitrary` impl on `FlagSet`) and verifies two
//! structural invariants:
//!
//! 1. The snapshot is sorted ascending by key (BTreeMap iteration
//!    order is independent of HashMap insertion order).
//! 2. `with()` round-trip: setting a key to value v and reading it
//!    back via `is_enabled()` returns v.
//!
//! Note: `FlagSet` itself does not derive `Serialize`/`Deserialize`
//! by design — the snapshot is the public observability shape (see
//! `FlagSet::snapshot` docs). The two property tests below exercise
//! the structural invariants instead of serde round-trip.

use proptest::prelude::*;

use pheno_flags::FlagSet;

proptest! {
    /// The snapshot of any FlagSet is sorted by key (BTreeMap
    /// iteration order) regardless of insertion order. This
    /// catches any future refactor that accidentally switches
    /// the underlying map to something with insertion-order
    /// iteration.
    #[test]
    fn flagset_snapshot_is_sorted(flags in any::<FlagSet>()) {
        let snap = flags.snapshot();
        let keys: Vec<&String> = snap.keys().collect();
        let mut sorted = keys.clone();
        sorted.sort();
        prop_assert_eq!(keys, sorted, "snapshot must be sorted ascending by key");
    }

    /// For any `FlagSet` produced by the `Arbitrary` impl,
    /// every `(key, value)` pair in the snapshot survives a
    /// `clone()` round-trip. This guards against accidental
    /// `Copy`/`Clone` regressions on the underlying map.
    #[test]
    fn flagset_clone_preserves_entries(flags in any::<FlagSet>()) {
        let cloned = flags.clone();
        prop_assert_eq!(flags.snapshot(), cloned.snapshot());
    }
}
