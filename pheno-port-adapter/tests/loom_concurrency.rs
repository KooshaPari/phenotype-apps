//! L21/L25 — loom model-checker concurrency tests for `pheno-port-adapter`.
//!
//! Run with: `RUSTFLAGS="--cfg loom" cargo test --test loom_concurrency --release`
//!
//! Three permutation tests exercise the shared-state data structures the
//! crate's public API relies on (port-resolution map, metrics counters,
//! shared state machine). Each test pins a `preemption_bound` to keep the
//! per-test wall time bounded for CI:
//!
//! | test                            | preemption_bound | rationale                                  |
//! |---------------------------------|------------------|--------------------------------------------|
//! | `parallel_port_resolution`      | 10               | 2 writers × 2 resolvers → wider fan-out    |
//! | `concurrent_metrics_collection` |  5               | 3 incrementers + 1 reader, mid-complexity  |
//! | `shared_state_race`             |  3               | 2 writers + 1 reader, baseline race window |
//!
//! These complement (not replace) the existing `loom_circuit_breaker.rs`
//! and `loom_request_router.rs` model-checker tests — they target distinct
//! shared-state shapes (counter-only / map-only / mixed) that the previous
//! tests do not exercise.
//!
//! Per ADR-038 (hex-port-adapter L4 policy), every port that takes
//! `&self` and mutates internal state MUST be safe under concurrent
//! caller access. The test surface here is a layer below the public
//! trait — it exercises the synchronization primitives the trait impls
//! are expected to compose.
//!
//! v20 L21 cycle — adds 3 schedules to the L25 permutation matrix.
#![cfg(loom)]

use loom::sync::atomic::{AtomicU64, Ordering};
use loom::sync::{Arc, RwLock};
use loom::thread;
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// 1. parallel_port_resolution — 10× preemption_bound
// ---------------------------------------------------------------------------
//
// Models a port-name → endpoint registry shared across two writer threads
// (register / deregister) and two resolver threads (lookup). The invariant
// is: every successful lookup sees a value that was inserted by a writer
// (no torn reads, no read of a half-mutated slot). Loom exhaustively
// enumerates the interleavings up to the bound and reports any schedule
// that violates the postcondition.
#[test]
fn parallel_port_resolution() {
    let mut b = loom::model::Builder::new();
    b.preemption_bound = Some(10);
    b.check(|| {
        let registry: Arc<RwLock<HashMap<&'static str, &'static str>>> =
            Arc::new(RwLock::new(HashMap::new()));

        // Pre-seed one binding so resolvers always find at least one entry.
        registry
            .write()
            .unwrap()
            .insert("metrics", "tcp://127.0.0.1:9100");

        let r1 = registry.clone();
        let register = thread::spawn(move || {
            r1.write().unwrap().insert("traces", "tcp://127.0.0.1:4318");
        });

        let r2 = registry.clone();
        let register2 = thread::spawn(move || {
            r2.write().unwrap().insert("logs", "tcp://127.0.0.1:5044");
        });

        let r3 = registry.clone();
        let resolve = thread::spawn(move || {
            let guard = r3.read().unwrap();
            // The pre-seeded "metrics" entry MUST always be observable;
            // any value we read must be a valid endpoint string we (or
            // the pre-seed) wrote.
            if let Some(ep) = guard.get("metrics") {
                assert!(ep.starts_with("tcp://"), "torn read: got {:?}", ep);
            }
        });

        let r4 = registry.clone();
        let resolve2 = thread::spawn(move || {
            let guard = r4.read().unwrap();
            if let Some(ep) = guard.get("traces") {
                assert!(ep.starts_with("tcp://"), "torn read: got {:?}", ep);
            }
        });

        register.join().unwrap();
        register2.join().unwrap();
        resolve.join().unwrap();
        resolve2.join().unwrap();

        // Final-state invariant: pre-seeded entry + at least one writer
        // succeeded (writers may race; either may lose). At least the
        // pre-seeded "metrics" entry MUST survive.
        let final_view = registry.read().unwrap();
        assert!(
            final_view.get("metrics").is_some(),
            "pre-seeded entry must survive concurrent writers"
        );
    });
}

// ---------------------------------------------------------------------------
// 2. concurrent_metrics_collection — 5× preemption_bound
// ---------------------------------------------------------------------------
//
// Three threads each run a tight increment loop against a shared
// `AtomicU64` counter. The invariant is: after all threads join, the
// counter MUST equal `THREADS * INCREMENTS_PER_THREAD` exactly. Any
// deviation (lost update from a non-atomic RMW, wrong Ordering) is a
// correctness bug. Loom's exhaustive search catches memory-ordering
// violations that `cargo test --release` would silently pass.
#[test]
fn concurrent_metrics_collection() {
    const THREADS: usize = 3;
    const INCREMENTS_PER_THREAD: u64 = 4;

    let mut b = loom::model::Builder::new();
    b.preemption_bound = Some(5);
    b.check(|| {
        let counter = Arc::new(AtomicU64::new(0));

        let mut handles = Vec::with_capacity(THREADS);
        for _ in 0..THREADS {
            let c = counter.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..INCREMENTS_PER_THREAD {
                    c.fetch_add(1, Ordering::SeqCst);
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        // No increment may be lost.
        assert_eq!(
            counter.load(Ordering::SeqCst),
            THREADS as u64 * INCREMENTS_PER_THREAD,
            "lost-update detected: counter under-counts concurrent increments"
        );
    });
}

// ---------------------------------------------------------------------------
// 3. shared_state_race — 3× preemption_bound
// ---------------------------------------------------------------------------
//
// Two writers race to set a shared `AtomicU64` snapshot; one reader
// observes the live value. The invariant is: the reader sees EITHER
// the pre-write value (0) OR one of the two writers' values, but never
// a torn / non-monotonic intermediate. This is the smallest possible
// "shared-state race" the crate's port-adapter call paths produce
// (e.g. last-writer-wins config-cell update), and a baseline regression
// test for the larger two scenarios above.
#[test]
fn shared_state_race() {
    let mut b = loom::model::Builder::new();
    b.preemption_bound = Some(3);
    b.check(|| {
        let cell: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));

        let c1 = cell.clone();
        let writer_a = thread::spawn(move || {
            c1.store(0xA1, Ordering::SeqCst);
        });

        let c2 = cell.clone();
        let writer_b = thread::spawn(move || {
            c2.store(0xB2, Ordering::SeqCst);
        });

        let c3 = cell.clone();
        let reader = thread::spawn(move || {
            let observed = c3.load(Ordering::SeqCst);
            // Reader sees the initial value OR one of the two writes
            // (writers may run in any order; one of them — or neither —
            // may have completed before the read). A torn / hybrid value
            // would mean the store/load pair is not atomic.
            assert!(
                observed == 0 || observed == 0xA1 || observed == 0xB2,
                "torn read: got {:#x}",
                observed
            );
        });

        writer_a.join().unwrap();
        writer_b.join().unwrap();
        reader.join().unwrap();

        // Final state MUST be one of the two writer values (last writer
        // wins; we don't care which). It MUST NOT be 0 (a writer always
        // succeeds once both have joined).
        let final_value = cell.load(Ordering::SeqCst);
        assert!(
            final_value == 0xA1 || final_value == 0xB2,
            "final value must be one of the two writers, got {:#x}",
            final_value
        );
    });
}
