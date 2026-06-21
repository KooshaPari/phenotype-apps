//! # chaos-injection
//!
//! L36 — fault-injection framework for the `pheno-*` fleet.
//!
//! This crate is the canonical substrate for **chaos testing** across
//! the fleet. It exposes:
//!
//! - [`Fault`] — the enum of injectable fault kinds
//!   (`NetworkTimeout`, `DiskFull`, `AsyncPanic`, `RateLimit`).
//! - [`inject`] — the public entry point that runs a fault through
//!   the runtime and returns either an `Ok(())` (fault absorbed by the
//!   system under test) or a [`FaultOutcome`] (fault surfaced).
//! - [`ChaosRuntime`] — a configurable runtime that schedules a
//!   sequence of faults with stochastic or deterministic triggers.
//!
//! ## When to use
//!
//! - You are testing a fleet component's behaviour under partial
//!   failure (network partition, disk full, async runtime panic,
//!   rate-limit rejection).
//! - You want a single API that surfaces a `Result<(), FaultOutcome>`
//!   so adoption tests can `assert!(inject(Fault::X).is_ok())`.
//! - You want proptest-driven stochastic fault schedules.
//!
//! ## When NOT to use
//!
//! - You need a property-based fuzzer of protocol bytes — use
//!   `proptest` directly.
//! - You need full distributed-systems chaos (process kill, network
//!   blackhole at the OS level) — that lives in `pheno-chaos/`
//!   (out of scope for this in-process substrate).
//!
//! See `findings/2026-06-22-v20-T3-L36-fault-injection-framework.md`
//! for the design rationale and adoption test plan.
#![warn(missing_docs)]

pub mod faults;
pub mod inject;

pub use faults::{Fault, FaultOutcome};
pub use inject::{ChaosRuntime, InjectError, Schedule};

use std::time::Duration;

/// Run a single fault through the in-process runtime and observe the
/// outcome.
///
/// This is the **synchronous** entry point most adoption tests use.
/// It is allocation-free (no tokio runtime is started) and returns
/// `Result<(), FaultOutcome>`:
///   - `Ok(())` — the fault was either absorbed (e.g. a
///     `RateLimit` was back-pressured successfully) or did not fire
///     (the runtime's trigger probability excluded it).
///   - `Err(FaultOutcome::Fired(_))` — the fault fired and produced
///     an observable effect (timeout, full-disk error, panic,
///     rate-limit rejection).
///
/// Most adoption tests assert `is_err()` (the fault must surface)
/// or `is_ok()` (the system absorbed it gracefully) depending on the
/// scenario.
pub fn inject(fault: Fault) -> Result<(), FaultOutcome> {
    inject::run_once(fault, Duration::from_millis(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn network_timeout_inject_surfaces_outcome() {
        // `inject` runs once; a NetworkTimeout should always fire
        // (deterministic fault, not stochastic).
        let result = inject(Fault::NetworkTimeout {
            latency: Duration::from_millis(1),
        });
        assert!(
            result.is_err(),
            "NetworkTimeout must surface as a fired fault"
        );
    }

    #[test]
    fn rate_limit_inject_is_absorbed_by_default() {
        // RateLimit with max=0 → immediately rejected → Err.
        let result = inject(Fault::RateLimit { max_per_sec: 0 });
        assert!(result.is_err(), "max=0 must immediately reject");
    }
}
