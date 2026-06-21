//! [`Fault`] trait — the contract every fault type implements.
//!
//! A fault is a small, scoped, *revertible* perturbation. It is
//! injected before the test body runs and reverted after. Every fault
//! returns a [`FaultGuard`] that holds the active state; the guard's
//! `Drop` impl is the safety net that calls `revert()` even on panic.

use std::fmt;
use std::time::Duration;

/// Opaque, owned handle to an injected fault.
///
/// When the guard is dropped, [`Fault::revert`] is called. This is the
/// single point of teardown for every fault — panic in the test body,
/// early `return`, or normal completion all converge here.
pub struct FaultGuard {
    /// Name of the fault that produced this guard. Used in panic
    /// messages so a failed revert is diagnosable.
    pub(crate) name: &'static str,
    /// `true` once `revert` has been called. The Drop impl skips
    /// double-revert.
    pub(crate) reverted: bool,
    /// The fault itself, boxed so the guard can call `revert` on Drop
    /// without knowing the concrete type.
    pub(crate) fault: Box<dyn Fault>,
}

impl fmt::Debug for FaultGuard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FaultGuard")
            .field("name", &self.name)
            .field("reverted", &self.reverted)
            .finish()
    }
}

impl FaultGuard {
    /// Construct a guard from a fault that has just been injected.
    pub(crate) fn new(name: &'static str, fault: Box<dyn Fault>) -> Self {
        Self {
            name,
            reverted: false,
            fault,
        }
    }

    /// Revert the fault. Idempotent.
    pub fn revert(mut self) {
        if !self.reverted {
            self.fault.revert();
            self.reverted = true;
        }
    }

    /// Name of the fault that owns this guard.
    pub fn name(&self) -> &str {
        self.name
    }
}

impl Drop for FaultGuard {
    fn drop(&mut self) {
        if !self.reverted {
            // Best-effort revert. We cannot panic in Drop, so a noisy
            // log line is the best we can do. The tests assert
            // reversibility explicitly so this path is rarely hit.
            eprintln!(
                "[pheno-chaos] FaultGuard for {} dropped without explicit revert(); \
                 fault may persist into the next test. This is a bug in the test.",
                self.name
            );
            self.fault.revert();
            self.reverted = true;
        }
    }
}

/// Core trait every fault implements.
///
/// The contract is deliberately minimal:
///
/// - `name()` — stable identifier used in test output and metrics.
/// - `inject()` — perturb the world. Returns a guard whose Drop
///   triggers `revert()`. Returns `Err` only if the fault cannot be
///   injected *at all* (e.g. resource exhaustion); transient failures
///   are handled inside the fault.
/// - `revert()` — undo the perturbation. Idempotent: a no-op if
///   `inject` was never called or already reverted. Called both
///   explicitly by [`FaultGuard::revert`] and implicitly by
///   [`FaultGuard::Drop`].
/// - `duration_hint()` — the upper bound on how long `revert` may take
///   to converge. Used by the macro to set a per-fault SLO budget.
pub trait Fault: Send + Sync {
    /// Stable, human-readable name. Used in `cargo test` output.
    fn name(&self) -> &'static str;

    /// Inject the fault and return a guard. Returns `Err` only if the
    /// fault could not be set up at all.
    fn inject(&self) -> Result<FaultGuard, ChaosError>;

    /// Undo the perturbation. Safe to call multiple times.
    fn revert(&self);

    /// Approximate upper bound on `revert` wall-time. The macro uses
    /// this to size the per-fault cleanup budget.
    fn duration_hint(&self) -> Duration;
}

/// Errors produced by fault injection. Currently only `InjectFailed` is
/// used; the enum is open for future expansion (e.g. kernel-feature
/// not available).
#[derive(Debug)]
pub enum ChaosError {
    /// `inject()` could not set the fault up. The test must skip.
    InjectFailed(&'static str),
}

impl std::fmt::Display for ChaosError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChaosError::InjectFailed(msg) => write!(f, "inject failed: {}", msg),
        }
    }
}

impl std::error::Error for ChaosError {}