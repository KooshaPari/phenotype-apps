//! # `pheno-chaos` — Chaos engineering substrate for Rust services
//!
//! Application-layer fault injection (network latency, connection drop,
//! CPU spike) plus a `#[chaos_test]` proc-macro that wraps a test body,
//! randomly injects a configured fault per run, and asserts the system
//! recovers within a per-fault SLO.
//!
//! ## Scope (V20-T3 brief)
//!
//! - **std + libc only** for fault injection. No kernel modules, no
//!   `iptables` / `tc` / `dummynet`, no `LD_PRELOAD`, no eBPF.
//! - **Runtime-agnostic.** The `#[chaos_test]` macro drives a synchronous
//!   executor on `std::thread` so the substrate works in `no_std +
//!   alloc` tests and inside any async runtime (the fault fires before
//!   the body runs; the body itself is opaque).
//!
//! ## Public surface
//!
//! - [`Fault`] trait — the contract every fault type implements.
//! - [`NetworkLatency`], [`ConnectionDrop`], [`CpuSpike`] — the three
//!   fault types called out in the V20-T3 brief.
//! - [`chaos_test`] — re-exported proc-macro attribute that wraps a
//!   test function.
//! - [`runtime`] — internal handle used by the macro to install + run
//!   a fault schedule.
//!
//! ## Safety guards
//!
//! Every fault type caps the damage it can do:
//!
//! - `NetworkLatency` clamps delay to `[0, MAX_LATENCY_MS]` (5s).
//! - `ConnectionDrop` is **opt-in** per call (`drop_now()`); it never
//!   fires spontaneously.
//! - `CpuSpike` clamps duration to `[0, MAX_SPIKE_SECS]` (5s) and pins
//!   the spiking thread to a single OS thread so it cannot starve
//!   siblings.
//! - All faults are RAII — a panic in the test body still triggers
//!   `revert()` via the [`FaultGuard`] Drop impl.
//!
//! See `findings/2026-06-22-V20-T3-chaos-framework.md` for the design
//! rationale and test output.
//!
//! ## Example
//!
//! ```rust,no_run
//! use pheno_chaos::chaos_test;
//! use std::time::Duration;
//!
//! #[chaos_test(faults = "latency,drop,cpu", slo_ms = 500, runs = 3)]
//! fn my_resilient_endpoint() {
//!     // body must tolerate any of the three faults and complete <500ms
//!     let _ = do_request_with_retry();
//! }
//! # fn do_request_with_retry() -> Result<(), ()> { Ok(()) }
//! ```

// `unsafe` is required for libc::setsockopt / libc::close in the
// ConnectionDrop fault (see crates/pheno-chaos/src/connection.rs).
// We use it deliberately and audibly — there are no other `unsafe`
// sites in the crate.
#![warn(unsafe_code)]
#![deny(rust_2018_idioms)]
#![warn(missing_docs)]

use std::time::Duration;

pub mod connection;
pub mod cpu;
pub mod fault;
pub mod network;
pub mod runtime;

// Re-export the proc-macro under the canonical `chaos_test` name so
// downstream crates only need `use pheno_chaos::chaos_test;`. The
// Cargo dep name is `pheno-chaos-macros`; the Rust identifier is
// `pheno_chaos_macros`.
pub use pheno_chaos_macros::chaos_test;

pub use connection::{simulate_drop, simulate_rst, ConnectionDrop, RstGuard};
pub use cpu::CpuSpike;
pub use fault::{Fault, FaultGuard};
pub use network::{chaos_call, NetworkLatency};

/// Default upper bound on any injected network latency, in milliseconds.
///
/// 5s is chosen because it exceeds any reasonable request SLO; longer
/// values indicate a misconfiguration, not a chaos test.
pub const MAX_LATENCY_MS: u64 = 5_000;

/// Default upper bound on any injected CPU spike, in seconds.
pub const MAX_SPIKE_SECS: u64 = 5;

/// Default jitter envelope (as a fraction of base delay) for
/// [`NetworkLatency`]. `0.5` means delay ∈ `[0.5x, 1.5x]`.
pub const DEFAULT_JITTER_FRACTION: f64 = 0.5;

/// Helper: convert milliseconds to [`Duration`].
#[inline]
pub fn ms(millis: u64) -> Duration {
    Duration::from_millis(millis)
}

/// Helper: convert seconds to [`Duration`].
#[inline]
pub fn secs(secs: u64) -> Duration {
    Duration::from_secs(secs)
}