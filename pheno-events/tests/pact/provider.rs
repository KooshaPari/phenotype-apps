//! # Provider-side pact contract tests for `pheno-events`
//!
//! These tests pin down the **event-bus**'s request/response contract
//! surface (the bus acts as the "provider" of an HTTP API; subscribers
//! are the "consumers"). The scenarios here cover the synchronous,
//! request-shaped half of the bus's contract:
//!
//! | scenario                    | request                | response          |
//! |-----------------------------|------------------------|-------------------|
//! | `event_published`           | `POST /events`         | `200 + Ack`       |
//! | `event_replay_from_offset`  | `GET /events?offset=N` | `200 + [Env,..]`  |
//!
//! The async, message-shaped half (`event_subscribed`) lives in
//! [`super::consumer`] (V4-pact format) because the bus is the *sender*
//! of those messages and the subscriber is the *receiver*.
//!
//! ## How the test is wired
//!
//! 1. A real [`EventEnvelope`] (built via the canonical `pheno_events`
//!    builder) is fed into the pact as a `like!` pattern — the mock
//!    server will accept any valid envelope shape.
//! 2. `PactBuilder::start_mock_server` spins up a mock that enforces
//!    the contract.
//! 3. We drive a tiny `reqwest::Client` against the mock and assert
//!    that the bus's response deserialises into the expected Rust type.
//! 4. On `Drop`, the pact file lands in `./pacts/` for later replay
//!    against the real bus via `pact_verifier`.
//!
//! See `tests/pact/events.proto` for the human description of the wire
//! format.

use pact_consumer::prelude::*;
use pact_consumer::mock_server::StartMockServer;
use pheno_events::{Ack, EventEnvelope};
use serde_json::{json, Value};

/// Subscriber endpoint we publish from in the pact scenario.
///
/// The pact file records the **consumer** (this test driver) and the
/// **provider** (the event-bus). When the real bus later runs
/// `pact_verifier`, it must accept publishes that match this consumer's
/// expectation.
const CONSUMER: &str = "pheno-events-subscriber";
const PROVIDER: &str = "pheno-events-bus";

/// Scenario 1 of 3 — `event_published`.
///
/// Drives the canonical "subscriber publishes an event, bus returns an
/// Ack" flow against the mock bus. Validates that:
///
/// - The serialized `EventEnvelope` matches the bus's expected request shape.
/// - The bus's response deserializes into an [`Ack`] with the expected fields.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn event_published() {
    // Build a real, valid envelope using the substrate builder — this is
    // the exact JSON the real subscriber client would emit.
    let envelope = EventEnvelope::new(
        "user.created",
        "accounts",
        1,
        json!({ "id": 1, "email": "alice@example.com" }),
    )
    .expect("envelope must satisfy substrate validation rules");

    // Declare the contract: subscriber sends this body, bus answers with this body.
    // `like!` permits any value of the same shape (string/number/bool/object),
    // which lets us drive the mock with a real UUID/email without locking the
    // test to those exact values.
    let server = PactBuilder::new(CONSUMER, PROVIDER)
        .interaction(
            "publish a user.created event to the bus",
            "core/interaction/http",
            |mut i| {
                i.request
                    .post()
                    .path("/events")
                    .content_type("application/json")
                    .json_body(json_pattern!({
                        "id":              like!(envelope.id.to_string()),
                        "event_type":      like!("user.created"),
                        "source":          like!("accounts"),
                        "timestamp":       like!("2024-01-01T00:00:00Z"),
                        "causation_id":    like!(null),
                        "correlation_id":  like!(null),
                        "schema_version":  like!(1),
                        "payload":         like!({
                            "id":    1,
                            "email": "alice@example.com",
                        }),
                    }));
                i.response
                    .status(200)
                    .content_type("application/json")
                    .json_body(json_pattern!({
                        "event_id":  like!("00000000-0000-7000-8000-000000000000"),
                        "duplicate": like!(false),
                    }));
                i.clone()
            },
        )
        .start_mock_server(None);

    // Drive a real HTTP client against the mock to prove the subscriber's
    // request shape matches the contract and the bus's response parses.
    let url = server.url();
    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/events", url))
        .json(&envelope)
        .send()
        .await
        .expect("publish request must complete");

    assert_eq!(
        response.status(),
        200,
        "bus must ack a well-formed publish with 200 OK"
    );

    let ack: Ack = response
        .json()
        .await
        .expect("bus response must deserialize as `Ack`");

    // We can't pin `ack.event_id == envelope.id` because the mock returns
    // the `like!` example value (a fixed UUID) rather than the request's
    // UUID. We do verify the structural shape of the ack.
    assert!(
        !ack.duplicate,
        "first publish of an event_id must not be flagged as duplicate"
    );
}

/// Scenario 2 of 3 — `event_replay_from_offset`.
///
/// Drives the canonical "subscriber requests historical events starting
/// at a given offset, bus returns an ordered list of envelopes" flow.
///
/// Validates that:
///
/// - The bus accepts a `GET /events?offset=N` query string.
/// - The bus returns a JSON array of `EventEnvelope` (not a single object).
/// - Each returned envelope round-trips through the substrate's serde path.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn event_replay_from_offset() {
    // Seed two historical envelopes — these are what the mock will replay.
    let historical_a = EventEnvelope::new(
        "user.created",
        "accounts",
        1,
        json!({ "id": 1 }),
    )
    .expect("valid envelope");
    let historical_b = EventEnvelope::new(
        "user.updated",
        "accounts",
        1,
        json!({ "id": 1, "fields": ["email"] }),
    )
    .expect("valid envelope");

    let server = PactBuilder::new(CONSUMER, PROVIDER)
        .interaction(
            "replay events from a given offset",
            "core/interaction/http",
            |mut i| {
                i.request
                    .get()
                    .path("/events")
                    .query("offset=0")
                    .content_type("application/json");
                // Mock returns an array of two envelopes. Each entry uses
                // `like!` so the real bus is free to choose its own UUIDs
                // and timestamps on replay.
                i.response
                    .status(200)
                    .content_type("application/json")
                    .json_body(json_pattern!({
                        "events": each_like!({
                            "id":              like!("00000000-0000-7000-8000-000000000000"),
                            "event_type":      like!("user.created"),
                            "source":          like!("accounts"),
                            "timestamp":       like!("2024-01-01T00:00:00Z"),
                            "causation_id":    like!(null),
                            "correlation_id":  like!(null),
                            "schema_version":  like!(1),
                            "payload":         like!({}),
                        }, 2),
                    }));
                i.clone()
            },
        )
        .start_mock_server(None);

    let url = server.url();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/events", url))
        .query(&[("offset", "0")])
        .send()
        .await
        .expect("replay request must complete");

    assert_eq!(
        response.status(),
        200,
        "bus must answer replay requests with 200 OK"
    );

    let body: Value = response
        .json()
        .await
        .expect("replay body must parse as JSON");

    // The contract: replay body is `{"events": [EventEnvelope, ...]}`.
    let events = body
        .get("events")
        .and_then(Value::as_array)
        .expect("replay body must contain an `events` JSON array");

    assert_eq!(
        events.len(),
        2,
        "replay from offset=0 must return exactly 2 historical events"
    );

    // Round-trip through the substrate's deserializer to confirm each
    // element conforms to the canonical `EventEnvelope` schema.
    let parsed: Vec<EventEnvelope> = serde_json::from_value(Value::Array(
        events.iter().map(|e| e.clone()).collect(),
    ))
    .expect("each replay entry must deserialize as `EventEnvelope`");

    // We don't pin specific UUIDs (the mock's `like!` examples are fixed),
    // but we do confirm the two events have the correct `event_type`s.
    assert_eq!(parsed[0].event_type, "user.created");
    assert_eq!(parsed[1].event_type, "user.updated");
    assert_eq!(parsed[0].source, historical_a.source);
    assert_eq!(parsed[1].source, historical_b.source);
}
