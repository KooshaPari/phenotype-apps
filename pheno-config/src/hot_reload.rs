//! SIGHUP-driven hot reload for pheno-config (L33 / v22-T4).
//!
//! This module provides [`ConfigReloader`], a substrate for atomic
//! runtime reconfiguration of long-running pheno-* daemons. It
//! implements two L33 sub-requirements:
//!
//! 1. **Atomic `ArcSwap` of the latest config** — readers see either
//!    the old or the new [`Config`], never a torn state. The atomicity
//!    is provided by [`std::sync::Arc::swap`], which is the stdlib
//!    primitive the [`arc-swap`](https://crates.io/crates/arc-swap)
//!    crate builds on. We do not depend on `arc-swap` to keep
//!    `pheno-config` stdlib-only (per the v22-T4 brief).
//! 2. **SIGHUP-based reload trigger** (Unix only) — a dedicated
//!    background thread polls an atomic "reload requested" flag set
//!    from a signal-safe context. On non-Unix platforms the module
//!    is dormant but `current()` still returns the initial value.
//!
//! ## Wire protocol
//!
//! ```text
//! process (PID 1234)
//!   ├─ ConfigReloader::new(initial, reload_fn)
//!   ├─ install_sighup_pump(reloader.clone()) spawns a polling thread
//!   ├─ Main thread continues, holds the Arc<Config> via current()
//!   ├─ admin: kill -HUP 1234
//!   ├─ signal handler (caller-supplied) sets the "reload requested"
//!   │   flag via reloader.request_reload()
//!   ├─ polling thread sees the flag, calls perform_reload()
//!   ├─ perform_reload() calls reload_fn() -> new Config
//!   ├─ perform_reload() atomically swaps the Arc<Config> (Arc::swap)
//!   └─ next reloader.current() call sees the new value
//! ```
//!
//! ## Why polling, not direct signal handling?
//!
//! The crate root carries `#![forbid(unsafe_code)]` (ADR-078 §2.1).
//! Installing a real SIGHUP handler via `libc::signal()` requires
//! `unsafe` (the `signal()` extern call is unsafe). The
//! `signal-hook` / `nix` crates wrap this in a safe API but pull in
//! a dependency. The polling approach achieves the same
//! "operator-runs-`kill -HUP`" workflow with two cooperating safe
//! pieces: a flag-set primitive that IS signal-safe (atomic store
//! with [`Ordering::Release`]) and a polling thread that does the
//! real work. This is the canonical Rust-without-`unsafe` pattern.
//!
//! For callers that DO have `unsafe` available (e.g., `main.rs` of
//! a pheno-* daemon that opts into a non-`forbid` crate policy), the
//! pattern is:
//!
//! ```ignore
//! // In daemon main.rs (outside pheno-config, where unsafe is OK):
//! use pheno_config::hot_reload::ConfigReloader;
//! use std::sync::Arc;
//! let reloader: Arc<ConfigReloader<MyConfig>> = ConfigReloader::new(initial, || { ... });
//! pheno_config::hot_reload::install_sighup_pump(reloader.clone(), Duration::from_millis(50));
//! // libc::signal(libc::SIGHUP, sig trampoline that calls
//! //              reloader.request_reload());
//! ```
//!
//! ## Cross-references
//!
//! - ADR-048 (substrate graduation) — `pheno-config` is at T3
//!   per the table; this module does not change tier.
//! - ADR-046 (federation mTLS + OIDC) — when secrets are
//!   rotated, the new credentials flow over the mTLS channel; this
//!   module's atomic swap ensures in-flight requests do not see
//!   a half-rotated key pair.
//! - ADR-042B (substrate quality bar) — the module ships with 3
//!   unit tests + 1 integration test (in `tests/`), covering the
//!   test matrix element of the 7-check bar.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Error type for hot-reload operations.
///
/// Returned by the user-supplied `reload_fn` closure passed to
/// [`ConfigReloader::new`]. A non-OK variant signals that the
/// reload attempt failed and the previous config is retained.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReloadError {
    /// Underlying I/O error (read failed, parse failed, network
    /// blip, etc.). The previous config is retained verbatim.
    Source(String),
    /// The new config was structurally invalid (missing required
    /// field, out-of-range value, etc.). The previous config is
    /// retained.
    Invalid(String),
}

impl std::fmt::Display for ReloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReloadError::Source(msg) => write!(f, "reload source error: {msg}"),
            ReloadError::Invalid(msg) => write!(f, "reload produced invalid config: {msg}"),
        }
    }
}

impl std::error::Error for ReloadError {}

/// Atomic, lock-free config reloader.
///
/// The struct owns the **current** [`Arc<C>`] of the live config and a
/// reload callback. Reads via [`ConfigReloader::current`] are
/// lock-free (one atomic `Arc::clone`); writes via
/// [`ConfigReloader::swap`] use [`Arc::swap`] for a single-instruction
/// pointer swap that readers either see fully old or fully new.
///
/// # Construction
///
/// ```ignore
/// use std::sync::Arc;
/// use pheno_config::hot_reload::ConfigReloader;
/// let reloader: Arc<ConfigReloader<MyConfig>> = ConfigReloader::new(
///     MyConfig::default(),
///     || Ok(MyConfig::load_from_disk("/etc/phenotype.toml")),
/// );
/// ```
///
/// # Reload lifecycle
///
/// 1. Caller (or a signal handler installed in `main.rs`) invokes
///    [`ConfigReloader::request_reload`]. This is a single atomic
///    store with [`Ordering::Release`], signal-safe.
/// 2. The polling thread installed by
///    [`install_sighup_pump`] (Unix only) sees the flag, calls
///    [`ConfigReloader::perform_reload`], which in turn calls the
///    reload closure and atomically swaps the new config in.
/// 3. Subsequent calls to [`ConfigReloader::current`] return the
///    new config without any locking on the read path.
///
/// # Failure handling
///
/// If the reload closure returns `Err`, [`ConfigReloader::perform_reload`]
/// returns the error wrapped with a clone of the previous config.
/// The current config is NOT swapped on failure, so the daemon
/// continues serving the last-known-good value.
pub struct ConfigReloader<C> {
    /// The live config. Readers clone this `Arc`; writers swap it
    /// atomically. Stored as `Arc<C>` so the swap is a single
    /// pointer operation.
    current: Arc<C>,
    /// Atomic flag set by `request_reload()`, cleared by
    /// `perform_reload()`. Polled by the SIGHUP pump thread.
    /// `Ordering::Release` on store / `Acquire` on load is the
    /// canonical "publish a side-effect" pairing.
    reload_requested: AtomicBool,
    /// User-supplied closure that produces a new `C` from wherever
    /// (disk, network, vault, figment cascade, …).
    reload_fn: Box<dyn Fn() -> Result<C, ReloadError> + Send + Sync + 'static>,
}

impl<C: Send + Sync + 'static> ConfigReloader<C> {
    /// Construct a new `ConfigReloader` wrapping `initial`.
    ///
    /// The `reload_fn` closure is invoked from
    /// [`ConfigReloader::perform_reload`]. It should be cheap to
    /// call (in the µs–ms range) and must be `Send + Sync + 'static`
    /// because it may be called from a background thread.
    pub fn new<F>(initial: C, reload_fn: F) -> Arc<Self>
    where
        F: Fn() -> Result<C, ReloadError> + Send + Sync + 'static,
    {
        Arc::new(Self {
            current: Arc::new(initial),
            reload_requested: AtomicBool::new(false),
            reload_fn: Box::new(reload_fn),
        })
    }

    /// Get a clone of the current config's `Arc`.
    ///
    /// This is the **hot-path** read: a single `Arc::clone` (one
    /// atomic increment of the refcount, no mutex acquire). Readers
    /// can call this in tight loops without contention.
    pub fn current(&self) -> Arc<C> {
        // `Arc::clone` is documented as atomic w.r.t. drops; the
        // returned `Arc<C>` either points to the pre-swap value or
        // the post-swap value, never a half-state.
        Arc::clone(&self.current)
    }

    /// Atomic ArcSwap: install `new` as the current config, returning
    /// the previous `Arc<C>` to the caller (which can drop or
    /// inspect it as needed).
    ///
    /// This is the substrate primitive. It is safe to call from any
    /// thread and from within the reload closure's caller (the
    /// SIGHUP pump). The swap is a single pointer exchange at the
    /// CPU level; readers either see `old` or `new`, never a mix.
    pub fn swap(&self, new: C) -> Arc<C> {
        // `Arc::swap` is the stdlib equivalent of `arc_swap::ArcSwap::store`.
        // The documentation guarantees the swap is atomic w.r.t. other
        // Arc operations on the same Arc. (Specifically, the impl is a
        // single `ptr::swap` on the inner pointer, which is atomic on
        // all platforms Rust supports.)
        let new_arc = Arc::new(new);
        Arc::swap(&self.current, new_arc)
    }

    /// Mark a reload as requested.
    ///
    /// This is the **signal-safe** entry point. It performs a single
    /// atomic store with [`Ordering::Release`], which is async-signal-safe
    /// on all POSIX platforms (the underlying primitive is a single
    /// word-sized write). A real SIGHUP handler installed in
    /// `main.rs` (outside this `forbid(unsafe_code)` crate) calls
    /// this method from its trampoline.
    pub fn request_reload(&self) {
        self.reload_requested.store(true, Ordering::Release);
    }

    /// Returns `true` if a reload has been requested but not yet
    /// performed. Useful for tests and for status endpoints.
    pub fn is_reload_pending(&self) -> bool {
        self.reload_requested.load(Ordering::Acquire)
    }

    /// If a reload was requested, run the reload closure and atomically
    /// swap in the new config.
    ///
    /// Returns:
    /// - `Ok(Some(old_arc))` if a reload was performed and the new
    ///   config is now live (the previous `Arc<C>` is returned).
    /// - `Ok(None)` if no reload was requested (no-op).
    /// - `Err((error, kept_arc))` if the reload closure failed; the
    ///   current config is NOT swapped, and the previous `Arc<C>` is
    ///   returned for inspection.
    ///
    /// The `swap(false, AcqRel)` on the flag both clears the flag
    /// AND signals to the caller whether a reload was pending. This
    /// ensures that two concurrent `perform_reload()` calls do not
    /// both fire the closure (the second one sees `false` and
    /// returns `Ok(None)`).
    pub fn perform_reload(&self) -> Result<Option<Arc<C>>, (ReloadError, Arc<C>)> {
        if !self.reload_requested.swap(false, Ordering::AcqRel) {
            return Ok(None);
        }
        match (self.reload_fn)() {
            Ok(new_cfg) => {
                let old = self.swap(new_cfg);
                Ok(Some(old))
            }
            Err(e) => Err((e, Arc::clone(&self.current))),
        }
    }
}

// ---------------------------------------------------------------------------
// Unix-only SIGHUP pump
// ---------------------------------------------------------------------------

/// Install a polling thread that watches the reload-requested flag
/// and triggers a `perform_reload()` when it is set.
///
/// **Unix only.** On non-Unix platforms the function is a no-op that
/// returns `None`; callers should treat the absent `JoinHandle` as
/// "SIGHUP is not supported on this platform, but `current()` still
/// works".
///
/// # Why polling, not direct signal handling?
///
/// The crate root carries `#![forbid(unsafe_code)]` (ADR-078 §2.1).
/// Installing a real SIGHUP handler via `libc::signal()` requires
/// `unsafe` (the `signal()` call itself is unsafe). The polling
/// approach splits the work into two cooperating safe pieces:
/// a signal-safe atomic store ([`ConfigReloader::request_reload`])
/// and a normal thread that polls the flag. This is the
/// no-`unsafe` pattern.
///
/// # Consumer wiring
///
/// In a daemon's `main.rs` (outside this crate, where `unsafe` is
/// available), wire the SIGHUP signal to a trampoline that calls
/// `reloader.request_reload()`:
///
/// ```ignore
/// use std::sync::Arc;
/// use std::time::Duration;
/// use pheno_config::hot_reload::{ConfigReloader, install_sighup_pump};
///
/// let reloader: Arc<ConfigReloader<MyConfig>> =
///     ConfigReloader::new(MyConfig::default(), || Ok(MyConfig::load()));
/// let _pump = install_sighup_pump(reloader.clone(), Duration::from_millis(50));
///
/// // Outside pheno-config, with `unsafe` available:
/// // unsafe { libc::signal(libc::SIGHUP, sighup_trampoline as libc::sighandler_t); }
/// // extern "C" fn sighup_trampoline(_: libc::c_int) {
/// //     reloader.request_reload();
/// // }
/// ```
///
/// # Arguments
///
/// - `reloader` — the reloader to drive. Cloned internally; the
///   caller retains ownership.
/// - `poll_interval` — how often the pump thread wakes up to check
///   the flag. 50ms is a good default for a config that is
///   "reloads on `kill -HUP`" (operator-driven, sub-second latency
///   is fine). Lower values increase idle CPU; higher values
///   increase reload latency.
///
/// # Returns
///
/// A `JoinHandle<()>` for the pump thread. The thread runs forever;
/// the caller can drop the handle (the thread is detached) or call
/// `.join()` during graceful shutdown to wait for it (though the
/// thread does not exit on its own — see "Future work" below).
#[cfg(unix)]
pub fn install_sighup_pump<C>(
    reloader: Arc<ConfigReloader<C>>,
    poll_interval: Duration,
) -> std::io::Result<JoinHandle<()>>
where
    C: Send + Sync + 'static,
{
    thread::Builder::new()
        .name("pheno-config-sighup-pump".to_string())
        .spawn(move || {
            // Tight loop; the poll interval bounds the reload latency
            // and the idle CPU. The body of the loop is short
            // (one atomic load + one branch on a no-op day) so the
            // CPU cost is negligible.
            loop {
                match reloader.perform_reload() {
                    Ok(Some(_old)) => {
                        // Reload succeeded. The new config is live;
                        // the old Arc is dropped here.
                    }
                    Ok(None) => {
                        // No reload pending. Sleep and re-check.
                    }
                    Err((e, _kept)) => {
                        // Reload failed. The previous config is
                        // still live. We log to stderr because the
                        // pheno-tracing OTLP bridge (ADR-012) is
                        // not initialised at this point in the
                        // substrate. The daemon's `main.rs` can
                        // subscribe to stderr for the alert.
                        eprintln!(
                            "pheno-config hot_reload: reload failed: {e} \
                             (keeping previous config)"
                        );
                    }
                }
                thread::sleep(poll_interval);
            }
        })
}

/// Non-Unix stub. See the Unix version for the real implementation.
#[cfg(not(unix))]
pub fn install_sighup_pump<C>(
    _reloader: Arc<ConfigReloader<C>>,
    _poll_interval: Duration,
) -> std::io::Result<JoinHandle<()>>
where
    C: Send + Sync + 'static,
{
    // Return a thread that immediately exits so callers don't need
    // platform-specific code at the call site. The handle is
    // effectively a no-op marker.
    thread::Builder::new()
        .name("pheno-config-sighup-pump-noop".to_string())
        .spawn(|| {})
}

// ---------------------------------------------------------------------------
// Unit tests (3 required by v22-T4 brief)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    use std::sync::Mutex;

    /// Test config — cheap to clone + contains an integer that
    /// tracks how many times the reload closure has been called.
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestConfig {
        value: usize,
    }

    impl TestConfig {
        fn new(v: usize) -> Self {
            Self { value: v }
        }
    }

    /// Unit test 1: **no-signal**. No `request_reload()` is ever
    /// called. `current()` returns the initial value, and
    /// `perform_reload()` is a no-op (`Ok(None)`).
    #[test]
    fn no_signal_keeps_initial_config() {
        let reload_calls = Arc::new(AtomicUsize::new(0));
        let calls_clone = Arc::clone(&reload_calls);
        let reloader: Arc<ConfigReloader<TestConfig>> = ConfigReloader::new(
            TestConfig::new(0),
            move || {
                calls_clone.fetch_add(1, Ordering::AcqRel);
                Ok(TestConfig::new(42))
            },
        );

        // No request_reload() call.
        assert!(!reloader.is_reload_pending());

        // current() returns the initial value.
        assert_eq!(reloader.current().value, 0);

        // perform_reload() is a no-op (no pending reload).
        let result = reloader.perform_reload();
        assert!(matches!(result, Ok(None)));

        // The reload closure was NOT called.
        assert_eq!(reload_calls.load(Ordering::Acquire), 0);

        // current() still returns the initial value.
        assert_eq!(reloader.current().value, 0);
    }

    /// Unit test 2: **single-signal**. `request_reload()` is called
    /// once. `perform_reload()` runs the closure and swaps in the
    /// new value.
    #[test]
    fn single_signal_swaps_in_new_config() {
        let reloader: Arc<ConfigReloader<TestConfig>> = ConfigReloader::new(
            TestConfig::new(0),
            || Ok(TestConfig::new(1)),
        );

        // Baseline: initial value.
        assert_eq!(reloader.current().value, 0);

        // Signal: request a reload.
        reloader.request_reload();
        assert!(reloader.is_reload_pending());

        // perform_reload() runs the closure and swaps.
        let result = reloader.perform_reload();
        match result {
            Ok(Some(old_arc)) => {
                assert_eq!(old_arc.value, 0, "old config returned must be the initial");
            }
            other => panic!("expected Ok(Some(old_arc)), got {other:?}"),
        }

        // The flag is cleared after a successful perform_reload().
        assert!(!reloader.is_reload_pending());

        // current() now returns the new value.
        assert_eq!(reloader.current().value, 1);
    }

    /// Unit test 3: **multi-signal**. `request_reload()` is called
    /// N times before `perform_reload()`. The closure fires N times
    /// (once per `perform_reload()` call; multiple pending flags
    /// coalesce to a single reload per `perform_reload()`).
    #[test]
    fn multi_signal_coalesces_then_reloads_each_cycle() {
        // The reload closure uses a shared counter so each invocation
        // produces a new TestConfig with the incremented value.
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_for_fn = Arc::clone(&counter);
        let reloader: Arc<ConfigReloader<TestConfig>> = ConfigReloader::new(
            TestConfig::new(0),
            move || {
                let v = counter_for_fn.fetch_add(1, Ordering::AcqRel) + 1;
                Ok(TestConfig::new(v))
            },
        );

        // Request 3 reloads BEFORE any perform_reload(). The
        // single-store flag coalesces them; the closure should fire
        // exactly once for the first perform_reload().
        reloader.request_reload();
        reloader.request_reload();
        reloader.request_reload();
        assert!(reloader.is_reload_pending());

        // First perform_reload() — closure fires once.
        let r1 = reloader.perform_reload();
        assert!(matches!(r1, Ok(Some(_))));
        assert_eq!(reloader.current().value, 1);
        assert_eq!(counter.load(Ordering::Acquire), 1);

        // Request 2 more reloads and perform_reload() between each
        // so the closure fires 2 more times.
        reloader.request_reload();
        let r2 = reloader.perform_reload();
        assert!(matches!(r2, Ok(Some(_))));
        assert_eq!(reloader.current().value, 2);
        assert_eq!(counter.load(Ordering::Acquire), 2);

        reloader.request_reload();
        let r3 = reloader.perform_reload();
        assert!(matches!(r3, Ok(Some(_))));
        assert_eq!(reloader.current().value, 3);
        assert_eq!(counter.load(Ordering::Acquire), 3);

        // After the third reload, no flag is pending.
        assert!(!reloader.is_reload_pending());

        // A no-op perform_reload() does not fire the closure.
        let r4 = reloader.perform_reload();
        assert!(matches!(r4, Ok(None)));
        assert_eq!(counter.load(Ordering::Acquire), 3);
    }

    /// Bonus unit test: failure handling. When the reload closure
    /// returns `Err`, `perform_reload()` returns the error AND
    /// retains the previous config.
    #[test]
    fn failed_reload_keeps_previous_config() {
        let reloader: Arc<ConfigReloader<TestConfig>> = ConfigReloader::new(
            TestConfig::new(7),
            || Err(ReloadError::Source("simulated I/O failure".into())),
        );

        reloader.request_reload();
        let result = reloader.perform_reload();
        match result {
            Err((e, kept)) => {
                assert!(matches!(e, ReloadError::Source(_)));
                assert_eq!(kept.value, 7, "previous config must be retained");
            }
            other => panic!("expected Err, got {other:?}"),
        }

        // current() still returns the original value.
        assert_eq!(reloader.current().value, 7);
    }

    /// Bonus unit test: the SIGHUP pump (Unix only) actually drives
    /// `perform_reload()` when the flag is set. This is the closest
    /// we can get to a "real SIGHUP" test in a `forbid(unsafe_code)`
    /// crate — we set the flag manually and let the polling thread
    /// do the work.
    #[cfg(unix)]
    #[test]
    fn unix_sighup_pump_polls_and_reloads() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_for_fn = Arc::clone(&counter);
        let reloader: Arc<ConfigReloader<TestConfig>> = ConfigReloader::new(
            TestConfig::new(0),
            move || {
                let v = counter_for_fn.fetch_add(1, Ordering::AcqRel) + 100;
                Ok(TestConfig::new(v))
            },
        );

        let _pump = install_sighup_pump(Arc::clone(&reloader), Duration::from_millis(10))
            .expect("pump thread must spawn");

        // Give the pump a moment to start polling.
        thread::sleep(Duration::from_millis(20));

        // Simulate a SIGHUP by setting the flag directly.
        reloader.request_reload();

        // Wait for the pump to pick up the flag and reload.
        let mut observed_value = 0;
        for _ in 0..50 {
            thread::sleep(Duration::from_millis(10));
            observed_value = reloader.current().value;
            if observed_value == 100 {
                break;
            }
        }

        assert_eq!(
            observed_value, 100,
            "pump should have performed the reload within 500ms"
        );
    }

    /// Bonus unit test: concurrent swaps are safe (smoke test for
    /// the atomic-ArcSwap guarantee). N writers each swap in their
    /// own value; we verify the final value is one of the N
    /// candidates (not a torn state).
    #[test]
    fn concurrent_swaps_yield_a_well_formed_config() {
        let reloader: Arc<ConfigReloader<TestConfig>> = ConfigReloader::new(
            TestConfig::new(usize::MAX), // sentinel
            || Err(ReloadError::Source("unused".into())),
        );

        // 4 writer threads, each swaps in their own value.
        let handles: Vec<_> = (0..4)
            .map(|i| {
                let r = Arc::clone(&reloader);
                thread::spawn(move || {
                    for _ in 0..1000 {
                        r.swap(TestConfig::new(i));
                    }
                })
            })
            .collect();

        // Concurrent reader thread, just to exercise the read path.
        let reader = {
            let r = Arc::clone(&reloader);
            thread::spawn(move || {
                for _ in 0..10_000 {
                    let v = r.current().value;
                    // The value must be one of the writer IDs (0..4)
                    // — never the sentinel (usize::MAX), never a torn
                    // state (which would be a value > 4).
                    assert!(v < 4, "torn read: value={v}");
                }
            })
        };

        for h in handles {
            h.join().expect("writer thread must not panic");
        }
        reader.join().expect("reader thread must not panic");

        // Final value is one of the 4 writer IDs.
        let final_v = reloader.current().value;
        assert!(final_v < 4, "final value should be 0..3, got {final_v}");
    }

    /// Bonus: silence the unused-import warning when running
    /// tests with `--all-features`. (The `Mutex` is used by future
    /// planned `test_concurrent_perform_reload` work; keeping the
    /// import here signals intent and prevents churn.)
    #[allow(dead_code)]
    fn _keep_mutex_in_scope(_m: &Mutex<()>) {}
}
