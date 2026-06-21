//! # pheno-events
//!
//! Canonical event-envelope substrate for the `pheno-*` fleet.
//!
//! This crate defines the [`EventEnvelope`] type — the wire-level
//! carrier for every cross-service event in the fleet — plus
//! acknowledgement ([`Ack`]) and validation error ([`EnvelopeError`])
//! types. The full event-bus implementation (in-memory + SQLite-backed
//! `Bus` adapters, schema registry, projection helpers) lives in the
//! `phenoEvents/` directory in this repo and will eventually be folded
//! in here once the SqliteBus implementation is stabilised (v21+).
//!
//! ## v20-T5: `proptest::Arbitrary` impls
//!
//! Every public type in this crate derives [`proptest_derive::Arbitrary`]
//! so property tests can fuzz random valid events and verify the
//! serde JSON round-trip property. The `#[proptest(map = ...)]`
//! attribute is **not** used here — the validation rules
//! (`event_type` non-empty, `source` non-empty, `schema_version >= 1`)
//! are enforced at the *builder* level (callable from arbitrary
//! generation) rather than at type construction, so the `Arbitrary`
//! impl is free to produce any string/u32 it likes. A second property
//! test (`*_validation_rejects_empty_strings`) explicitly exercises
//! the validation rules against clearly-invalid inputs.

#![warn(missing_docs)]

pub mod core;

pub use core::{EnvelopeError, EventEnvelope};

use proptest_derive::Arbitrary;
use uuid::Uuid;

/// Acknowledgement of a published event.
///
/// Returned from `Bus::publish` to confirm the event has been
/// accepted (and whether it was a duplicate).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Arbitrary)]
pub struct Ack {
    /// The event id that was acknowledged.
    pub event_id: Uuid,
    /// `true` if this id had already been published previously
    /// (the bus chose to ignore the duplicate rather than re-queue it).
    pub duplicate: bool,
}