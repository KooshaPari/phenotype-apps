//! Pact contract tests for `pheno-events`.
//!
//! This integration-test binary aggregates the pact scenarios that lock
//! down the wire contract between the event-bus (provider) and the
//! subscribers (consumers). Each submodule owns one direction:
//!
//! - [`provider`] — the bus's HTTP request/response contract surface
//!   (`event_published`, `event_replay_from_offset`).
//! - [`consumer`] — the bus's pub/sub message contract surface
//!   (`event_subscribed`).
//!
//! All scenarios use the [`pact_consumer`](pact_consumer) crate to
//! (a) declare the expected interaction, (b) spin up a mock server that
//! enforces it, and (c) drive real subscriber code against the mock.
//! Generated `.pact` files land in `./pacts/` (relative to where the test
//! runs) and can be replayed against the real bus via `pact_verifier`.
//!
//! See `tests/pact/events.proto` for the human-readable wire contract
//! and `docs/adr/2026-06-17/ADR-084` for the originating ADR.

mod pact {
    pub mod provider;
    pub mod consumer;
}
