//! [`NetworkLatency`] fault — adds artificial latency to a virtual
//! "network round-trip" call.
//!
//! ## Honest scope
//!
//! True kernel-level latency injection (Linux `tc qdisc`, macOS
//! `dummynet`, `iptables`) requires root and a kernel module. Per the
//! V20-T3 brief — *std + libc only, no kernel modules* — this fault
//! operates at the **application layer**: code under test must opt in
//! by wrapping I/O in [`chaos_call`].
//!
//! This is the same model used by `tokio::time::pause` / `tokio-test`
//! and by Java's `Delay` fault in chaos-monkey libraries. There is no
//! way to transparently shim the kernel layer without root, so we
//! pick the next-most-honest design.
//!
//! ## Behaviour
//!
//! - Default delay: `50ms ± 25ms` (i.e. uniform in `[25ms, 75ms]`).
//! - Probability per `chaos_call`: `0.1` (10%).
//! - Delay is clamped to `[0, MAX_LATENCY_MS]` (`5000ms`) per the
//!   substrate safety guard.

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::fault::{ChaosError, Fault, FaultGuard};
use crate::MAX_LATENCY_MS;

/// Default base delay for [`NetworkLatency`]: 50ms.
pub const DEFAULT_BASE_DELAY_MS: u64 = 50;
/// Default jitter envelope: 25ms (50% of base).
pub const DEFAULT_JITTER_MS: u64 = 25;
/// Default probability that a given `chaos_call` is delayed: 0.1 (10%).
pub const DEFAULT_PROBABILITY: f64 = 0.1;

/// Inner state of an armed `NetworkLatency` fault.
///
/// We share via `Arc` so the `FaultGuard` (which lives on the test
/// thread's stack) and the [`chaos_call`] helper can read the same
/// parameters.
#[derive(Debug, Clone)]
pub(crate) struct LatencyConfig {
    pub(crate) base_ms: u64,
    pub(crate) jitter_ms: u64,
    pub(crate) probability: f64,
}

impl LatencyConfig {
    /// Compute the actual delay for a single `chaos_call`. Returns
    /// `Some(d)` if the fault fires (per probability), `None`
    /// otherwise.
    ///
    /// Uses a simple LCG-like mix of two `Instant` reads so we don't
    /// pull in a `rand` dependency (the V20-T3 brief: std + libc
    /// only).
    pub(crate) fn sample_delay(&self) -> Option<Duration> {
        let nanos1 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos() as u64)
            .unwrap_or(0);
        let nanos2 = std::time::Instant::now()
            .elapsed()
            .subsec_nanos() as u64;
        let mix = nanos1 ^ nanos2.rotate_left(17);

        // Probability gate: top 16 bits as a fraction in [0, 1).
        let roll = (mix & 0xFFFF) as f64 / 65535.0;
        if roll >= self.probability {
            return None;
        }

        // Jitter: uniform in [-jitter_ms, +jitter_ms].
        let span = 2 * self.jitter_ms + 1;
        let jitter_signed = ((mix >> 16) % span) as i64 - self.jitter_ms as i64;
        let delay_ms = (self.base_ms as i64 + jitter_signed).max(0) as u64;
        let delay_ms = delay_ms.min(MAX_LATENCY_MS);

        Some(Duration::from_millis(delay_ms))
    }
}

/// `NetworkLatency` fault — adds artificial delay to a wrapped call.
///
/// `base_ms` = base delay, `jitter_ms` = uniform jitter envelope
/// (delay ∈ `[base-jitter, base+jitter]`), `probability` = chance a
/// given `chaos_call` is delayed.
#[derive(Debug, Clone)]
pub struct NetworkLatency {
    base_ms: u64,
    jitter_ms: u64,
    probability: f64,
}

impl Default for NetworkLatency {
    fn default() -> Self {
        Self {
            base_ms: DEFAULT_BASE_DELAY_MS,
            jitter_ms: DEFAULT_JITTER_MS,
            probability: DEFAULT_PROBABILITY,
        }
    }
}

impl NetworkLatency {
    /// Construct a `NetworkLatency` with explicit parameters.
    ///
    /// `probability` is clamped to `[0.0, 1.0]`. `base_ms` and
    /// `jitter_ms` are clamped so the worst-case delay never exceeds
    /// [`MAX_LATENCY_MS`].
    pub fn new(base_ms: u64, jitter_ms: u64, probability: f64) -> Self {
        let probability = probability.clamp(0.0, 1.0);
        let base_ms = base_ms.min(MAX_LATENCY_MS);
        let jitter_ms = jitter_ms.min(base_ms); // jitter cannot exceed base
        Self {
            base_ms,
            jitter_ms,
            probability,
        }
    }
}

impl Fault for NetworkLatency {
    fn name(&self) -> &'static str {
        "network_latency"
    }

    fn inject(&self) -> Result<FaultGuard, ChaosError> {
        let cfg = Arc::new(LatencyConfig {
            base_ms: self.base_ms,
            jitter_ms: self.jitter_ms,
            probability: self.probability,
        });
        ARMED.with(|cell| cell.set(Some(cfg)));
        Ok(FaultGuard::new(
            "network_latency",
            Box::new(NetworkLatencyHandle {
                base_ms: self.base_ms,
                jitter_ms: self.jitter_ms,
                probability: self.probability,
            }),
        ))
    }

    fn revert(&self) {
        // No-op on the outer struct; the handle's Drop does the work.
    }

    fn duration_hint(&self) -> Duration {
        Duration::from_millis(self.base_ms + self.jitter_ms + 50)
    }
}

/// Internal handle stored in [`FaultGuard`] so `revert` can clear the
/// thread-local. Implements `Fault` (so it can live behind a
/// `Box<dyn Fault>`) and `Drop` (so `FaultGuard::Drop` flows through
/// the clear).
struct NetworkLatencyHandle {
    base_ms: u64,
    jitter_ms: u64,
    probability: f64,
}

impl Fault for NetworkLatencyHandle {
    fn name(&self) -> &'static str {
        "network_latency"
    }

    fn inject(&self) -> Result<FaultGuard, ChaosError> {
        // The outer `NetworkLatency::inject` already armed the
        // thread-local. This path only fires if a test re-injects
        // from inside the body; we honour it but emit a debug note
        // because re-injection is unusual.
        let cfg = Arc::new(LatencyConfig {
            base_ms: self.base_ms,
            jitter_ms: self.jitter_ms,
            probability: self.probability,
        });
        ARMED.with(|cell| cell.set(Some(cfg)));
        Ok(FaultGuard::new(
            "network_latency",
            Box::new(Self {
                base_ms: self.base_ms,
                jitter_ms: self.jitter_ms,
                probability: self.probability,
            }),
        ))
    }

    fn revert(&self) {
        // No-op: Drop clears the thread-local.
    }

    fn duration_hint(&self) -> Duration {
        Duration::from_millis(self.base_ms + self.jitter_ms + 50)
    }
}

impl Drop for NetworkLatencyHandle {
    fn drop(&mut self) {
        ARMED.with(|cell| cell.set(None));
    }
}

thread_local! {
    /// Thread-local handle to the currently-armed `NetworkLatency`
    /// fault, if any. The macro arms this on the test thread before
    /// running the body, and clears it via the `Drop` impl on
    /// `NetworkLatencyHandle`.
    static ARMED: std::cell::Cell<Option<Arc<LatencyConfig>>> =
        const { std::cell::Cell::new(None) };
}

/// Wrap a closure so that, while a [`NetworkLatency`] fault is armed
/// on the current thread, each invocation sleeps for the sampled
/// delay with the configured probability.
///
/// This is the *opt-in* entry point. Code under test must call this
/// for the fault to have any effect.
pub fn chaos_call<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    // `take()` swaps the value out, then we put it back. This works
    // because `Cell::get` requires `Copy`, which `Option<Arc<T>>` is
    // not. `take` is the correct primitive here.
    let cfg = ARMED.with(|cell| cell.take());
    let delay = cfg.as_ref().and_then(|c| c.sample_delay());
    ARMED.with(|cell| cell.set(cfg));
    if let Some(d) = delay {
        thread::sleep(d);
    }
    f()
}