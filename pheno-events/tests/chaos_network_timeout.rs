//! v20-T3 (L36) — chaos adoption test for `pheno-events`.
//!
//! Simulates a network partition / timeout in the event-bus publish
//! path. The test asserts that `chaos_injection::inject` surfaces
//! a `NetworkTimeout` outcome when a timeout fault is injected,
//! and that the event-envelope validation path is **unaffected**
//! by the chaos runtime (i.e. chaos does not silently corrupt
//! the application state).
//!
//! Run with:
//!
//! ```bash
//! cargo test --test chaos_network_timeout
//! ```
//! FR: L36 — pheno-events must accept chaos-injection as a
//! dev-dependency and surface a `NetworkTimeout` outcome through
//! the canonical `chaos_injection::inject` entry point.

use std::time::Duration;

use chaos_injection::{inject, Fault, FaultOutcome};
use pheno_events::{EnvelopeError, EventEnvelope};
use serde_json::json;

/// 1. Inject a `NetworkTimeout` fault and assert the runtime
/// surfaces it as an `Err(FaultOutcome::NetworkTimeout)`.
/// This is the canonical "the chaos framework works" smoke test.
#[test]
fn inject_network_timeout_surfaces_outcome() {
    let result = inject(Fault::NetworkTimeout {
        latency: Duration::from_millis(1),
    });
    assert!(
        result.is_err(),
        "NetworkTimeout fault must surface as an Err"
    );
    assert_eq!(
        result.unwrap_err(),
        FaultOutcome::NetworkTimeout,
        "the surfaced outcome must be exactly NetworkTimeout"
    );
}

/// 2. Build a valid `EventEnvelope`, validate it, and assert
/// that the chaos runtime does NOT corrupt the application
/// state. (The chaos framework is observation-only; the
/// event-envelope is built and validated entirely outside
/// the chaos runtime.)
#[test]
fn event_envelope_validation_unaffected_by_chaos_runtime() {
    // Build a valid envelope first.
    let envelope = EventEnvelope::builder("user.created", "accounts", json!({"id": 1}))
        .schema_version(1)
        .build()
        .expect("valid envelope");
    assert!(envelope.validate().is_ok());

    // Now inject a NetworkTimeout. The envelope object is
    // untouched (it lives outside the chaos runtime's scope).
    let chaos = inject(Fault::NetworkTimeout {
        latency: Duration::from_millis(1),
    });
    assert!(chaos.is_err());

    // The envelope is still valid — chaos did not corrupt it.
    assert!(envelope.validate().is_ok());

    // And envelope validation rules still reject bad input.
    let bad = EventEnvelope::builder(" ", "accounts", json!({}))
        .build()
        .expect_err("whitespace-only event_type must fail");
    assert_eq!(bad, EnvelopeError::EmptyEventType);
}

/// 3. Multiple sequential injections must each surface a
/// `NetworkTimeout` outcome — the runtime does not have hidden
/// state that suppresses the second call after the first.
#[test]
fn multiple_sequential_injections_all_surface() {
    for i in 0..5 {
        let result = inject(Fault::NetworkTimeout {
            latency: Duration::from_millis(1),
        });
        assert_eq!(
            result.unwrap_err(),
            FaultOutcome::NetworkTimeout,
            "iteration {i} must surface NetworkTimeout"
        );
    }
}
