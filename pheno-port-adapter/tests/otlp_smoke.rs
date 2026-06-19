//! OTLP smoke test for pheno-port-adapter (T22.4, ADR-036).
//!
//! Verifies the pheno-tracing substrate is adopted: `init()` is
//! callable, idempotent (multiple calls do not panic), and a
//! follow-up `tracing` emit flows through the registered
//! subscriber without panicking.
//!
//! Run with:
//!   cargo test --test otlp_smoke
//!
//! This test does not bind a network OTLP exporter; it only
//! validates the in-process substrate wiring.

use pheno_port_adapter::init;

#[test]
fn init_initializes_tracing_substrate() {
    // init() must not panic. The first call sets the global
    // tracing subscriber via pheno_tracing::init() (try_init).
    init();
}

#[test]
fn init_is_idempotent() {
    // pheno_tracing::init() uses try_init(), so the second call
    // is a silent no-op. The substrate must remain usable after
    // repeated init() calls.
    init();
    init();
}

#[test]
fn init_then_emit_event_does_not_panic() {
    // After init(), emitting a tracing event must flow through
    // the registered subscriber without panicking.
    init();
    tracing::info_span!("otlp_smoke", test = "emit").in_scope(|| {
        tracing::info!("otlp_smoke_event_emitted");
    });
}
