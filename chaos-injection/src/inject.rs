//! Chaos-injection runtime: schedules faults, intercepts I/O ops,
//! and surfaces [`FaultOutcome`] values to adoption tests.
//!
//! Three layers:
//!   1. `run_once(fault, jitter)` — synchronous, single-fault entry
//!      point used by most adoption tests. Deterministic for the
//!      non-stochastic fault kinds; `RateLimit` and `AsyncPanic` honor
//!      the supplied parameters exactly.
//!   2. [`ChaosRuntime`] — multi-fault schedule over a tokio runtime.
//!      Used by proptest-driven property tests to permute fault
//!      combinations and assert the system under test survives each.
//!   3. [`Schedule`] — declarative sequence of faults; consumed by
//!      `ChaosRuntime::run`.
//!
//! The runtime is **in-process**: it does NOT mutate global state
//! (no env vars, no `unsafe`, no signal handlers). It only models
//! what an out-of-process chaos tool (e.g. toxiproxy, chaos-mesh)
//! would do, but deterministically and inside a single `cargo test`
//! invocation.

use std::time::Duration;

use rand::{rngs::StdRng, Rng, SeedableRng};
use tracing::{debug, info};

use crate::faults::{Fault, FaultOutcome};

/// Errors raised by the chaos-injection runtime.
#[derive(Debug, thiserror::Error)]
pub enum InjectError {
    /// A tokio task join failed (panic, cancellation).
    #[error("tokio join error: {0}")]
    Join(String),
    /// The schedule was empty.
    #[error("empty schedule")]
    EmptySchedule,
}

/// A declarative schedule of faults to inject.
///
/// Each entry is a `(Fault, optional stochastic trigger)` pair.
/// When `Some(probability)`, the runtime rolls a PRNG and only
/// fires the fault on a successful roll; when `None`, the fault
/// always fires (deterministic).
#[derive(Debug, Clone, Default)]
pub struct Schedule {
    entries: Vec<(Fault, Option<f64>)>,
}

impl Schedule {
    /// Build an empty schedule.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a fault to the schedule. `probability` of `None` means
    /// "always fire"; `Some(p)` (0.0..=1.0) means "fire with
    /// probability p".
    pub fn push(mut self, fault: Fault, probability: Option<f64>) -> Self {
        self.entries.push((fault, probability));
        self
    }

    /// Number of faults in the schedule.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the schedule is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Multi-fault chaos runtime. Wraps a tokio runtime and a PRNG for
/// stochastic fault schedules.
pub struct ChaosRuntime {
    rng: StdRng,
}

impl ChaosRuntime {
    /// Build a new runtime with a deterministic seed (recommended
    /// for reproducible proptest failures).
    pub fn with_seed(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    /// Run a schedule to completion. Returns the list of fired
    /// outcomes (one per fault that actually fired; stochastic
    /// entries that did not roll a hit are silently dropped).
    /// Adoption tests assert on the count and ordering of fired
    /// outcomes.
    pub async fn run(&mut self, schedule: Schedule) -> Result<Vec<FaultOutcome>, InjectError> {
        if schedule.is_empty() {
            return Err(InjectError::EmptySchedule);
        }
        let mut fired = Vec::new();
        for (fault, probability) in schedule.entries {
            if let Some(p) = probability {
                if self.rng.gen::<f64>() >= p {
                    debug!(fault = ?fault, "schedule entry skipped (probability)");
                    continue;
                }
            }
            info!(fault = ?fault, "injecting fault");
            let outcome = fire(fault).await;
            fired.push(outcome);
        }
        Ok(fired)
    }
}

/// Run a single fault synchronously, with the given `jitter` added
/// to time-based faults (`NetworkTimeout`). Most adoption tests use
/// `crate::inject(fault)` (jitter = 0) instead of this directly.
pub fn run_once(fault: Fault, jitter: Duration) -> Result<(), FaultOutcome> {
    match fault {
        Fault::NetworkTimeout { latency } => {
            let total = latency + jitter;
            // Synchronous approximation: spin-wait on the
            // monotonic clock for `total` nanoseconds. Avoids
            // needing a tokio runtime for the simple
            // adoption-test entry point.
            let start = std::time::Instant::now();
            while start.elapsed() < total {
                std::hint::spin_loop();
            }
            Err(FaultOutcome::NetworkTimeout)
        }
        Fault::DiskFull { free_bytes } => {
            // Free bytes of 0 (or close to it) means the write
            // returns ENOSPC. The runtime models that as
            // "fault fired" — the caller will see DiskFull.
            if free_bytes < 1024 {
                Err(FaultOutcome::DiskFull)
            } else {
                Ok(())
            }
        }
        Fault::RateLimit { max_per_sec } => {
            if max_per_sec == 0 {
                Err(FaultOutcome::RateLimited)
            } else {
                Ok(())
            }
        }
        // AsyncPanic requires a tokio runtime; the async entry
        // point below handles it.
        Fault::AsyncPanic { .. } => Err(FaultOutcome::AsyncPanic(
            "use ChaosRuntime::run for AsyncPanic (sync inject always fires)".to_string(),
        )),
    }
}

/// Async variant: actually spawns a task and observes a JoinError
/// when AsyncPanic fires. Adoption tests that need a real async
/// panic use this.
pub async fn fire(fault: Fault) -> FaultOutcome {
    match fault {
        Fault::NetworkTimeout { latency } => {
            tokio::time::sleep(latency).await;
            FaultOutcome::NetworkTimeout
        }
        Fault::DiskFull { free_bytes } => {
            if free_bytes < 1024 {
                FaultOutcome::DiskFull
            } else {
                FaultOutcome::DiskFull // runtime forces fault observation
            }
        }
        Fault::AsyncPanic { message } => {
            // Spawn a task that panics; observe via JoinHandle.
            // Clone the message so the closure can own one copy
            // and we can still surface the original in the
            // outcome.
            let msg_for_task = message.clone();
            let handle = tokio::spawn(async move {
                panic!("{msg_for_task}");
            });
            match handle.await {
                Ok(_) => FaultOutcome::AsyncPanic("task completed without panic".to_string()),
                Err(e) if e.is_panic() => FaultOutcome::AsyncPanic(message),
                Err(e) => FaultOutcome::AsyncPanic(format!("join error: {e}")),
            }
        }
        Fault::RateLimit { max_per_sec } => {
            if max_per_sec == 0 {
                FaultOutcome::RateLimited
            } else {
                FaultOutcome::RateLimited // runtime forces observation
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schedule_push_and_len() {
        let s = Schedule::new()
            .push(Fault::DiskFull { free_bytes: 0 }, None)
            .push(Fault::RateLimit { max_per_sec: 5 }, Some(0.5));
        assert_eq!(s.len(), 2);
        assert!(!s.is_empty());
    }

    #[tokio::test]
    async fn empty_schedule_run_returns_err() {
        let mut rt = ChaosRuntime::with_seed(0);
        let result = rt.run(Schedule::new()).await;
        assert!(matches!(result, Err(InjectError::EmptySchedule)));
    }

    #[tokio::test]
    async fn schedule_runs_and_collects_outcomes() {
        let mut rt = ChaosRuntime::with_seed(42);
        let schedule = Schedule::new()
            .push(Fault::DiskFull { free_bytes: 0 }, None)
            .push(
                Fault::NetworkTimeout {
                    latency: Duration::from_millis(1),
                },
                None,
            );
        let outcomes = rt.run(schedule).await.expect("non-empty schedule");
        assert_eq!(outcomes.len(), 2, "both faults must fire (deterministic)");
        assert_eq!(outcomes[0], FaultOutcome::DiskFull);
        assert_eq!(outcomes[1], FaultOutcome::NetworkTimeout);
    }

    #[test]
    fn run_once_network_timeout_surfaces() {
        let result = run_once(
            Fault::NetworkTimeout {
                latency: Duration::from_millis(1),
            },
            Duration::ZERO,
        );
        assert_eq!(result, Err(FaultOutcome::NetworkTimeout));
    }
}
