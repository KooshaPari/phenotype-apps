//! # Consumer-side pact contract tests for `pheno-events`
//!
//! These tests pin down the **subscriber**'s view of the event-bus
//! contract — specifically the async, pub/sub half. Here the bus is
//! the **provider** of messages and the subscriber is the **consumer**
//! of those messages, so we use the V4 Pact format with
//! `PactBuilder::message_interaction`.
//!
//! | scenario            | direction       | format      |
//! |---------------------|-----------------|-------------|
//! | `event_subscribed`  | bus → subscriber| V4 async msg|
//!
//! The HTTP request/response half (`event_published`,
//! `event_replay_from_offset`) lives in [`super::provider`].
//!
//! ## How the test is wired
//!
//! 1. A real [`EventEnvelope`] is built via the canonical
//!    `pheno_events` builder.
//! 2. `PactBuilder::new_v4(...).message_interaction(...)` declares the
//!    contract: the bus emits this JSON to the subscriber's queue.
//! 3. `pact_builder.messages()` yields the message(s) the contract says
//!    the bus will deliver. We feed each through a real
//!    `serde_json::from_slice::<EventEnvelope>` to prove the subscriber
//!    code path can deserialize what the bus emits.
//! 4. On `Drop`, the pact file lands in `./pacts/` for replay against
//!    the real bus via `pact_verifier`.
//!
//! See `tests/pact/events.proto` for the human description of the wire
//! format.

use pact_consumer::prelude::*;
use pheno_events::EventEnvelope;
use serde_json::{json, Value};

/// Subscriber endpoint we receive on in the pact scenario.
///
/// The pact file records the **consumer** (the subscriber code that
/// pulls messages off the queue) and the **provider** (the bus that
/// publishes those messages).
const CONSUMER: &str = "pheno-events-subscriber";
const PROVIDER: &str = "pheno-events-bus";

/// Scenario 3 of 3 — `event_subscribed`.
///
/// Declares the contract: the bus delivers an `EventEnvelope` to the
/// subscriber's queue whenever a relevant event is published. We pull
/// the configured message off the pact builder, deserialize it as the
/// canonical substrate type, and assert the subscriber's view of the
/// payload is correct.
///
/// In production the subscriber would be the message-handler loop in
/// `phenoEvents/src/consumer/*`; for the contract test we exercise the
/// serde path that any handler will share.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn event_subscribed() {
    // Build a real, valid envelope using the substrate builder — this is
    // the exact JSON the real bus would emit to subscribers.
    let envelope = EventEnvelope::new(
        "order.shipped",
        "fulfillment",
        2,
        json!({ "order_id": 42, "carrier": "ups" }),
    )
    .expect("envelope must satisfy substrate validation rules");

    // V4 pact format — required for async-message interactions.
    // We declare the contract: bus emits this body to the subscriber.
    // `like!` permits any value of the same shape so the real bus is
    // free to choose its own UUID/timestamp on replay.
    let mut pact_builder = PactBuilder::new_v4(CONSUMER, PROVIDER);
    pact_builder
        .message_interaction("deliver an order.shipped event to a subscriber", |mut i| {
            // Set the test name for broker visibility (no functional effect).
            i.test_name("event_subscribed");
            // Optional provider state: the bus has the order ready to ship.
            i.given("an order 42 has been shipped".to_string());
            i.json_body(json_pattern!({
                "id":              like!(envelope.id.to_string()),
                "event_type":      like!("order.shipped"),
                "source":          like!("fulfillment"),
                "timestamp":       like!("2024-01-01T00:00:00Z"),
                "causation_id":    like!(null),
                "correlation_id":  like!(null),
                "schema_version":  like!(2),
                "payload":         like!({
                    "order_id": 42,
                    "carrier":  "ups",
                }),
            }));
            i
        });

    // Yield each message the contract says the bus will deliver.
    // We accumulate them into a Vec so the assertions stay simple.
    let messages: Vec<Value> = pact_builder
        .messages()
        .map(|m| {
            let bytes = m
                .contents
                .contents
                .value()
                .expect("configured message must have a body")
                .as_bytes()
                .to_vec();
            serde_json::from_slice(&bytes).expect("configured body must be valid JSON")
        })
        .collect();

    assert_eq!(
        messages.len(),
        1,
        "the contract declares exactly one subscription message"
    );

    // Drive the real subscriber deserialization path. The handler in
    // production does exactly this; if the substrate types ever drift
    // away from the wire format, this assertion catches it.
    let delivered: EventEnvelope = serde_json::from_value(messages[0].clone())
        .expect("subscribed message must deserialize as `EventEnvelope`");

    // The subscriber doesn't pin specific UUIDs/timestamps (the bus picks
    // those at publish time) but it does care about the content fields
    // that drive downstream business logic.
    assert_eq!(delivered.event_type, "order.shipped");
    assert_eq!(delivered.source, "fulfillment");
    assert_eq!(delivered.schema_version, 2);
    assert_eq!(delivered.payload, json!({ "order_id": 42, "carrier": "ups" }));
    assert!(
        delivered.validate().is_ok(),
        "subscriber must only accept envelopes that pass `validate`"
    );
}
