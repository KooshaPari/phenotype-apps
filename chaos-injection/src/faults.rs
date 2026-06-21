//! Concrete fault kinds + their observable outcomes.
//!
//! Each `Fault` variant maps to a single in-process failure mode. The
//! variant's payload is the **trigger** (latency, capacity, message);
//! the corresponding [`FaultOutcome`] variant is the **observation**
//! (what the system under test saw).
//!
//! Mapping:
//! - `Fault::NetworkTimeout` → `FaultOutcome::NetworkTimeout` (after
//!   the configured latency, the I/O op times out).
//! - `Fault::DiskFull` → `FaultOutcome::DiskFull` (the write returns
//!   an ENOSPC-style error).
//! - `Fault::AsyncPanic` → `FaultOutcome::AsyncPanic` (the spawned
//!   task panics with the given message; caller observes via
//!   `JoinError`).
//! - `Fault::RateLimit` → `FaultOutcome::RateLimited` (the request is
//!   rejected with the configured `max_per_sec`).

use std::time::Duration;

/// Fault kinds injectable into the `pheno-*` fleet for chaos testing.
///
/// Each variant carries the **trigger parameters**; the
/// [`crate::inject`] runtime applies them and produces the
/// corresponding [`FaultOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fault {
    /// Simulate a network timeout: the I/O op blocks for `latency`
    /// then returns a timeout error. Use to test partial-failure
    /// handling in event bus, OTLP exporter, and config loader paths.
    NetworkTimeout {
        /// How long the simulated I/O op should block before timing out.
        latency: Duration,
    },
    /// Simulate a full disk: the write returns an ENOSPC-style
    /// error. Use to test persistence layer degradation.
    DiskFull {
        /// Free bytes the simulated filesystem reports (typically 0
        /// or a small value to trigger a write-failure path).
        free_bytes: u64,
    },
    /// Simulate an async-runtime panic: the spawned task panics
    /// with `message`. Use to test supervisor / restart / circuit
    /// breaker logic in async services.
    AsyncPanic {
        /// Panic message attached to the simulated task panic.
        message: String,
    },
    /// Simulate a rate-limit rejection: the (max_per_sec)th request
    /// inside a 1s window is rejected. Use to test back-pressure and
    /// token-bucket adapters.
    RateLimit {
        /// Maximum requests per second before rejection.
        max_per_sec: u32,
    },
}

/// The observable outcome of an injected fault.
///
/// Adoption tests assert on this enum's variant to verify the
/// system under test surfaced the fault correctly (vs. silently
/// swallowing it).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FaultOutcome {
    /// A `NetworkTimeout` fault fired; the I/O op timed out.
    NetworkTimeout,
    /// A `DiskFull` fault fired; the write returned an ENOSPC error.
    DiskFull,
    /// An `AsyncPanic` fault fired; the spawned task panicked.
    AsyncPanic(String),
    /// A `RateLimit` fault fired; the request was rejected.
    RateLimited,
}

impl FaultOutcome {
    /// Short, human-readable name (matches the `Fault` variant tag).
    pub fn name(&self) -> &'static str {
        match self {
            FaultOutcome::NetworkTimeout => "NetworkTimeout",
            FaultOutcome::DiskFull => "DiskFull",
            FaultOutcome::AsyncPanic(_) => "AsyncPanic",
            FaultOutcome::RateLimited => "RateLimited",
        }
    }
}
