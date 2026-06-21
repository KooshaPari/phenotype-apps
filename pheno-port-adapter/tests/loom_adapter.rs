//! Loom concurrency permutation tests for `pheno-port-adapter` (v16 T6 L25).
//!
//! `pheno-port-adapter`'s TCP / Unix adapters wrap their active
//! stream in an interior `Mutex<Option<…>>` so the synchronous
//! `PortAdapter` trait methods (which take `&self`, not `&mut self`)
//! can mutate the connection state from multiple threads.
//!
//! These loom permutations verify that the mutex is doing its job:
//! under every interleaving loom explores, the adapter's observable
//! state remains consistent.
//!
//! Enable with:
//!
//! ```bash
//! RUSTFLAGS="--cfg loom" cargo test --test loom_adapter --features loom
//! ```
//!
//! The default `cargo test` path does NOT compile this file; the
//! `cfg(loom)` guard keeps it out of the regular CI run.

#![cfg(loom)]

use loom::sync::Arc;
use loom::thread;

use pheno_port_adapter::{AdapterError, PortAdapter};

/// Invariant: any number of `disconnect()` calls on a fresh
/// adapter must all observe `Ok(())`, regardless of thread
/// interleaving. This pins the `Mutex` contract: every thread
/// either takes the lock, clears the `stream`, and returns `Ok`,
/// or takes the lock after another thread already cleared and
/// also returns `Ok`.
#[test]
fn disconnect_is_idempotent_under_loom() {
    loom::model(|| {
        let adapter = Arc::new(pheno_port_adapter::adapters::tcp::TcpAdapter::new());
        let mut handles = Vec::new();

        for _ in 0..2 {
            let a = Arc::clone(&adapter);
            handles.push(thread::spawn(move || a.disconnect().is_ok()));
        }

        let mut all_ok = true;
        for h in handles {
            if !h.join().expect("thread panicked") {
                all_ok = false;
            }
        }
        assert!(all_ok, "disconnect must always be Ok");
    });
}

/// Invariant: every concurrent `health()` call on a fresh adapter
/// observes the same `HealthCheckFailed` error — there is no
/// interleaving under which `health()` can spuriously return `Ok`
/// on an unconnected adapter.
#[test]
fn health_on_unconnected_returns_health_check_failed_under_loom() {
    loom::model(|| {
        let adapter = Arc::new(pheno_port_adapter::adapters::tcp::TcpAdapter::new());
        let mut handles = Vec::new();

        for _ in 0..3 {
            let a = Arc::clone(&adapter);
            handles.push(thread::spawn(move || {
                matches!(a.health(), Err(AdapterError::HealthCheckFailed(_)))
            }));
        }

        let mut all_failed = true;
        for h in handles {
            if !h.join().expect("thread panicked") {
                all_failed = false;
            }
        }
        assert!(all_failed, "every health() on unconnected adapter must fail");
    });
}

/// Invariant: 2 threads racing to `name()` on the same adapter both
/// observe the stable `"tcp"` literal. This is the trivial case
/// (no shared mutation), but pinning it under loom catches
/// accidental future introduction of interior state behind `name()`.
#[test]
fn name_is_stable_under_loom() {
    loom::model(|| {
        let adapter = Arc::new(pheno_port_adapter::adapters::tcp::TcpAdapter::new());
        let mut handles = Vec::new();

        for _ in 0..2 {
            let a = Arc::clone(&adapter);
            handles.push(thread::spawn(move || a.name().to_string()));
        }

        let mut names = Vec::new();
        for h in handles {
            names.push(h.join().expect("thread panicked"));
        }
        for n in &names {
            assert_eq!(n, "tcp");
        }
    });
}

/// Invariant: 2 threads concurrently call `connect()` against the
/// same unroutable address. Both must observe `ConnectFailed`,
/// and the failed-connection bookkeeping must leave the adapter
/// in a clean unconnected state.
#[test]
fn concurrent_failed_connects_leave_adapter_clean_under_loom() {
    loom::model(|| {
        let adapter = Arc::new(pheno_port_adapter::adapters::tcp::TcpAdapter::new());
        let mut handles = Vec::new();

        for _ in 0..2 {
            let a = Arc::clone(&adapter);
            handles.push(thread::spawn(move || {
                matches!(a.connect("127.0.0.1:1"), Err(AdapterError::ConnectFailed(_)))
            }));
        }

        let mut all_failed = true;
        for h in handles {
            if !h.join().expect("thread panicked") {
                all_failed = false;
            }
        }
        assert!(all_failed, "both connects must fail with ConnectFailed");
        // Adapter is still unconnected after the racing failures.
        assert!(matches!(
            adapter.health(),
            Err(AdapterError::HealthCheckFailed(_))
        ));
    });
}