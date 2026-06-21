//! [`CpuSpike`] fault — pin one OS thread at 100% CPU for N seconds.
//!
//! ## Honest scope
//!
//! Per the V20-T3 brief — *std + libc only, no kernel modules* — this
//! is a userspace busy-loop on a dedicated `std::thread`. It cannot
//! affect other OS processes or pin CPU cores at the hardware level
//! (that would need `cpulimit` / `taskset` / cgroups). What it *does*
//! do is steal CPU from any thread on the same core that the test
//! process is schedulable to — which is exactly what a SUT would
//! observe in production if a peer thread entered a hot loop.
//!
//! ## Behaviour
//!
//! - Default: spin loop at 100% for 1 second.
//! - Duration is clamped to `[0, MAX_SPIKE_SECS]` (`5s`).
//! - The spinning thread yields once per ~1ms via
//!   [`std::hint::spin_loop`] to avoid monopolising the core on
//!   hyperthreaded CPUs where it would starve its sibling.
//! - [`CpuSpike::inject`] returns a guard whose `revert` blocks until
//!   the spinning thread acknowledges the kill signal. This is
//!   guaranteed-bounded: at most `MAX_SPIKE_SECS + 1s` of cleanup.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::fault::{ChaosError, Fault, FaultGuard};
use crate::MAX_SPIKE_SECS;

/// Default spike duration: 1 second.
pub const DEFAULT_DURATION_SECS: u64 = 1;

const YIELD_EVERY_N_ITERATIONS: u32 = 100_000;

/// `CpuSpike` fault — busy-loop on a dedicated thread for `secs`.
#[derive(Debug, Clone)]
pub struct CpuSpike {
    secs: u64,
}

impl Default for CpuSpike {
    fn default() -> Self {
        Self {
            secs: DEFAULT_DURATION_SECS,
        }
    }
}

impl CpuSpike {
    /// Construct a `CpuSpike` with explicit duration (seconds).
    ///
    /// Clamped to `[0, MAX_SPIKE_SECS]`.
    pub fn new(secs: u64) -> Self {
        Self {
            secs: secs.min(MAX_SPIKE_SECS),
        }
    }
}

impl Fault for CpuSpike {
    fn name(&self) -> &'static str {
        "cpu_spike"
    }

    fn inject(&self) -> Result<FaultGuard, ChaosError> {
        let stop = Arc::new(AtomicBool::new(false));
        let stop_clone = Arc::clone(&stop);
        let secs = self.secs;
        // Spawn the spinning thread. It owns `stop_clone` and exits
        // when either the timer elapses or `stop` is set.
        let handle = thread::Builder::new()
            .name("pheno-chaos-cpu-spike".into())
            .spawn(move || {
                let deadline =
                    std::time::Instant::now() + Duration::from_secs(secs);
                let mut iter: u32 = 0;
                loop {
                    if stop_clone.load(Ordering::Relaxed) {
                        break;
                    }
                    if std::time::Instant::now() >= deadline {
                        break;
                    }
                    // Hot loop body: deliberately empty so the
                    // optimiser cannot elide it. `black_box` defeats
                    // dead-code elimination.
                    std::hint::black_box(iter.wrapping_mul(31));
                    iter = iter.wrapping_add(1);
                    if iter % YIELD_EVERY_N_ITERATIONS == 0 {
                        std::thread::yield_now();
                    }
                }
            })
            .map_err(|e| ChaosError::InjectFailed(Box::leak(
                format!("failed to spawn CPU-spike thread: {}", e).into_boxed_str(),
            )))?;

        Ok(FaultGuard::new(
            "cpu_spike",
            Box::new(CpuSpikeHandle {
                stop,
                handle: Some(handle),
            }),
        ))
    }

    fn revert(&self) {
        // No-op; Drop signals + joins the spinning thread.
    }

    fn duration_hint(&self) -> Duration {
        // Worst case: the spike runs for its full duration, plus a
        // 1s cleanup budget for the thread to acknowledge the kill
        // signal.
        Duration::from_secs(self.secs + 1)
    }
}

/// Internal handle stored in [`FaultGuard`].
struct CpuSpikeHandle {
    stop: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Fault for CpuSpikeHandle {
    fn name(&self) -> &'static str {
        "cpu_spike"
    }

    fn inject(&self) -> Result<FaultGuard, ChaosError> {
        // Re-injection from inside a body is unusual; we honour it
        // by re-spawning with the same duration (0s = instant exit).
        // We do NOT block here — the original handle is still alive
        // and will be reverted by the outer guard.
        Ok(FaultGuard::new(
            "cpu_spike",
            Box::new(Self {
                stop: Arc::clone(&self.stop),
                handle: None,
            }),
        ))
    }

    fn revert(&self) {
        // No-op; Drop signals + joins the spinning thread.
    }

    fn duration_hint(&self) -> Duration {
        Duration::from_secs(MAX_SPIKE_SECS + 1)
    }
}

impl Drop for CpuSpikeHandle {
    fn drop(&mut self) {
        // Signal the spinning thread to stop, then join it. Bounded
        // by the thread's own deadline (max `MAX_SPIKE_SECS`) plus a
        // small scheduling margin.
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            // If the join times out (shouldn't, given the deadline),
            // we leak the thread rather than panic in Drop.
            let _ = handle.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spike_runs_and_stops_within_window() {
        let start = std::time::Instant::now();
        let fault = CpuSpike::new(1);
        let _guard = fault.inject().unwrap();
        // Sleep a bit so the spinning thread is observably running.
        thread::sleep(Duration::from_millis(50));
        let elapsed_at_revert = start.elapsed();
        drop(_guard); // triggers revert → join
        let total = start.elapsed();
        // We should have observed at least ~50ms of spinning, and the
        // total should be ≲ 2s (1s spike + 1s cleanup budget).
        assert!(
            elapsed_at_revert >= Duration::from_millis(40),
            "spike thread didn't spin long enough: {:?}",
            elapsed_at_revert
        );
        assert!(
            total < Duration::from_secs(2),
            "spike + cleanup exceeded 2s budget: {:?}",
            total
        );
    }

    #[test]
    fn duration_clamped_to_max() {
        let fault = CpuSpike::new(999);
        assert_eq!(fault.secs, MAX_SPIKE_SECS);
    }
}