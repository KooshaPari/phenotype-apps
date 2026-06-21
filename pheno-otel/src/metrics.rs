//! L62 fleet-wide observability: error rate + request rate metrics via OTLP.
//!
//! # Why this module exists
//!
//! Pillar **L62 (error rate observability)** in the
//! [71-pillar framework](../findings/71-pillar-2026-06-17-schema.md) asks
//! every substrate to surface error and request rates as first-class
//! OTel metrics, not just logs. This module is the **canonical** entry
//! point for the pheno-* fleet.
//!
//! The pattern is intentionally simple: a single `OnceLock<Meter>` holds
//! the fleet's meter once `init_meter` has been called, and the
//! convenience helpers `record_error` and `record_request` grab the
//! corresponding instrument lazily. The instruments are all
//! `Option<...>`-returning so a crate that has not yet been initialized
//! (e.g. tests, or a binary that never sets up OTel) is a no-op — never
//! a panic.
//!
//! Refs: v14 cycle-3 T7, ADR-036B, ADR-042, ADR-049 (drift-detector
//! uses `record_error` to keep the error-rate counter honest).
//!
//! # Usage
//!
//! ```no_run
//! use opentelemetry::metrics::Meter;
//! use pheno_otel::metrics;
//!
//! // Called once at process start, after the OTel SDK has built a meter.
//! fn install(meter: Meter) { metrics::init_meter(meter); }
//!
//! // Anywhere in the crate, in an error path:
//! fn fallible_op() -> Result<(), std::io::Error> {
//!     // ... do work ...
//!     let err = std::io::Error::other("nope");
//!     metrics::record_error("fallible_op", "io_error");
//!     Err(err)
//! }
//!
//! // For request-shaped ops, also record latency:
//! fn handle_request() {
//!     let started = std::time::Instant::now();
//!     let status = "ok";
//!     metrics::record_request("handle_request", status, started.elapsed().as_secs_f64());
//! }
//! ```
//!
//! # When NOT to use
//!
//! - You need **traces**, not metrics → use `pheno-tracing` (ADR-036).
//! - You need **logs** → use `tracing` crate macros directly.
//! - You need the raw `opentelemetry` API surface (e.g. for
//!   `ObservableGauge`) — this module intentionally exposes only the
//!   three pre-built instruments + two convenience recorders. Pull
//!   `opentelemetry` directly when you need more.

#![deny(unsafe_code)]

use opentelemetry::metrics::{Counter, Histogram, Meter, UpDownCounter};
use opentelemetry::KeyValue;
use std::sync::OnceLock;

/// Process-wide meter handle. Set once via [`init_meter`]; subsequent
/// calls are no-ops. Reads are lock-free (it's a `OnceLock`).
static METER: OnceLock<Meter> = OnceLock::new();

/// Initialize the metrics meter. Call **once** at process start, after
/// the OTel SDK has built the global [`Meter`] you want to instrument
/// against (typically via
/// `opentelemetry::global::meter("pheno-otel")` or the SDK-specific
/// provider). Returns silently if a meter was already installed.
pub fn init_meter(meter: Meter) {
    let _ = METER.set(meter);
}

#[inline]
fn meter() -> Option<&'static Meter> {
    METER.get()
}

/// Total errors across all instrumented operations.
///
/// Labels: `op` (the operation name), `kind` (the error kind, e.g.
/// `io_error`, `validation`, `not_found`).
pub fn errors_count() -> Option<Counter<u64>> {
    meter().map(|m| m.u64_counter("errors.count").init())
}

/// Request / operation rate counter.
///
/// Labels: `op`, `status` (e.g. `ok`, `error`, `timeout`).
pub fn requests_count() -> Option<Counter<u64>> {
    meter().map(|m| m.u64_counter("requests.count").init())
}

/// Latency histogram in seconds.
///
/// Labels: `op`. Buckets are the OTel SDK default.
pub fn request_duration() -> Option<Histogram<f64>> {
    meter().map(|m| m.f64_histogram("request.duration").init())
}

/// In-flight operations gauge.
///
/// Labels: `op`. Use [`inc_inflight`] / [`dec_inflight`] around the
/// operation lifetime.
pub fn inflight() -> Option<UpDownCounter<i64>> {
    meter().map(|m| m.i64_up_down_counter("requests.inflight").init())
}

/// Convenience: record one error event with `op` + `kind` labels.
///
/// No-op when [`init_meter`] has not been called. Never panics.
pub fn record_error(op: &str, kind: &str) {
    if let Some(c) = errors_count() {
        c.add(
            1,
            &[
                KeyValue::new("op", op.to_string()),
                KeyValue::new("kind", kind.to_string()),
            ],
        );
    }
}

/// Convenience: record a request outcome (counter + latency).
///
/// No-op when [`init_meter`] has not been called. Never panics.
pub fn record_request(op: &str, status: &str, dur_secs: f64) {
    if let Some(c) = requests_count() {
        c.add(
            1,
            &[
                KeyValue::new("op", op.to_string()),
                KeyValue::new("status", status.to_string()),
            ],
        );
    }
    if let Some(h) = request_duration() {
        h.record(dur_secs, &[KeyValue::new("op", op.to_string())]);
    }
}

/// Increment the in-flight gauge for `op`. Pairs with [`dec_inflight`].
pub fn inc_inflight(op: &str) {
    if let Some(g) = inflight() {
        g.add(1, &[KeyValue::new("op", op.to_string())]);
    }
}

/// Decrement the in-flight gauge for `op`. Pairs with [`inc_inflight`].
pub fn dec_inflight(op: &str) {
    if let Some(g) = inflight() {
        g.add(-1, &[KeyValue::new("op", op.to_string())]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the helpers are no-ops before `init_meter` is called
    /// (the typical "untouched" state in unit tests).
    #[test]
    fn record_error_is_noop_when_uninitialized() {
        record_error("op_x", "io_error");
        record_request("op_x", "ok", 0.001);
        inc_inflight("op_x");
        dec_inflight("op_x");
    }
}
