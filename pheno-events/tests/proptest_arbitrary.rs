//! v20-T5 — `proptest::Arbitrary` round-trip test for `pheno_events::EventEnvelope`.
//!
//! Generates random `EventEnvelope` values via `any::<EventEnvelope>()`
//! (driven by the `Arbitrary` derive) and verifies the serde JSON
//! round-trip property: serialize → deserialize → equal. This catches
//! any future refactor that breaks `Serialize`/`Deserialize` symmetry
//! (e.g. adding a field to `EventEnvelope` but forgetting the
//! `Serialize` derive, or accidentally introducing non-JSON-safe types).

use proptest::prelude::*;

use pheno_events::{Ack, EventEnvelope};

proptest! {
    /// For any `EventEnvelope` produced by the `Arbitrary` impl,
    /// `serde_json::to_string` followed by `serde_json::from_str`
    /// must yield the original value. Any panic / data loss / wrong
    /// round-trip here is a regression on the `Serialize` +
    /// `Deserialize` symmetry of the type.
    #[test]
    fn event_envelope_serde_roundtrip(envelope in any::<EventEnvelope>()) {
        let json = serde_json::to_string(&envelope).expect("serialize EventEnvelope");
        let back: EventEnvelope = serde_json::from_str(&json).expect("deserialize EventEnvelope");
        prop_assert_eq!(envelope, back);
    }

    /// Same property for the `Ack` type.
    #[test]
    fn ack_serde_roundtrip(ack in any::<Ack>()) {
        let json = serde_json::to_string(&ack).expect("serialize Ack");
        let back: Ack = serde_json::from_str(&json).expect("deserialize Ack");
        prop_assert_eq!(ack, back);
    }

    /// The validation rules reject:
    /// - empty / whitespace-only `event_type`
    /// - empty / whitespace-only `source`
    /// - `schema_version == 0`
    /// but only AFTER construction. The `Arbitrary` impl can
    /// produce any string / u32, including invalid ones; this test
    /// proves `validate()` correctly rejects each invariant
    /// violation in turn.
    #[test]
    fn empty_event_type_fails_validation(envelope in any::<EventEnvelope>()) {
        let mut bad = envelope;
        bad.event_type = " ".to_string();
        prop_assert_eq!(
            bad.validate(),
            Err(pheno_events::EnvelopeError::EmptyEventType),
            "whitespace-only event_type must fail validation"
        );
    }

    #[test]
    fn empty_source_fails_validation(envelope in any::<EventEnvelope>()) {
        let mut bad = envelope;
        bad.source = "\t".to_string();
        prop_assert_eq!(
            bad.validate(),
            Err(pheno_events::EnvelopeError::EmptySource),
            "whitespace-only source must fail validation"
        );
    }

    #[test]
    fn zero_schema_version_fails_validation(envelope in any::<EventEnvelope>()) {
        let mut bad = envelope;
        bad.schema_version = 0;
        prop_assert_eq!(
            bad.validate(),
            Err(pheno_events::EnvelopeError::InvalidSchemaVersion),
            "schema_version 0 must fail validation"
        );
    }
}