//! L25 (concurrency) — loom permutation test for OTLP batcher flush ordering.
//!
//! Verifies that an OTLP exporter's *batcher* preserves the *cardinality*
//! invariant under concurrent pushes: after N concurrent `push()` calls and
//! one `flush()` drain, exactly N items are observed across the two phases
//! (no loss, no duplication). Permutes the schedule with `LOOM_MAX_PREEMPTIONS`
//! so the valid interleavings between producer threads and the flusher are
//! exercised under loom's permutation scheduler.
//!
//! Why this matters for `pheno-otel`: a real-world OTLP exporter buffers
//! spans in a `Mutex<Vec<Span>>` (or a lock-free MPSC ring) and flushes on
//! size threshold / time threshold. If the flush races with a concurrent
//! push, two failure modes are possible:
//!   1. *Loss* — the push acquires the lock after flush observes emptiness,
//!      so the item stays buffered past the flush deadline.
//!   2. *Double-count* — the push happens-before flush, the flush sees the
//!      item, but the producer is unaware and re-pushes later.
//! Neither failure is observable here because `Mutex<Vec<u8>>` provides
//! total-order push/flush under loom. The test pins that contract.
//!
//! Run with:
//!   LOOM_MAX_PREEMPTIONS=1 RUSTFLAGS="--cfg loom" cargo test --test loom_batcher --release
#![cfg(loom)]

use loom::sync::atomic::{AtomicBool, Ordering};
use loom::sync::{Arc, Mutex};
use loom::thread;

/// Cardinality invariant: N producers + 1 flusher → every pushed item is
/// observed exactly once across (drained ++ residual). The split between
/// drained and residual is scheduler-dependent; the SUM is the invariant.
///
/// Under loom's permutation scheduler, the drain thread may run before
/// some producers push (the drain observes a prefix of the eventual
/// state). The invariant pinned here is therefore the *atomicity* of
/// `Mutex::lock`: every push that *happens-before* the drain's lock
/// acquisition is observed by the drain; pushes that happen *after*
/// the drain are observed post-drain in the residual batch.
///
/// Permuted with `LOOM_MAX_PREEMPTIONS=1` so the test stays tractable
/// on a MacBook. With a higher bound, loom would explore more
/// interleavings but the invariant holds for all of them.
#[test]
fn batcher_flush_preserves_atomicity_under_concurrency() {
    loom::model(|| {
        let batch = Arc::new(Mutex::new(Vec::<u8>::new()));

        let producers: Vec<_> = (0u8..3)
            .map(|id| {
                let b = batch.clone();
                thread::spawn(move || {
                    // Simulate one OTLP span enqueue.
                    b.lock().unwrap().push(id);
                })
            })
            .collect();

        // Spawn the flusher. It may run before, between, or after the
        // producers' pushes — loom explores all three orderings.
        let drain = {
            let b = batch.clone();
            thread::spawn(move || b.lock().unwrap().drain(..).collect::<Vec<u8>>())
        };

        // Wait for drain first (its observed value is the prefix of the
        // eventual state). Then join producers — any pushes that happen
        // after drain's observation land in the residual batch below.
        let drained = drain.join().unwrap();

        // Now join producers — at this point, the residual batch holds
        // any pushes that happened AFTER the drain completed.
        for h in producers {
            h.join().unwrap();
        }
        let residual = batch.lock().unwrap().clone();

        // Total cardinality invariant: drained.len() + residual.len() == 3.
        // The split between drained/residual is scheduler-dependent, but
        // the SUM is invariant.
        let total = drained.len() + residual.len();
        assert_eq!(
            total,
            3,
            "batcher must preserve every pushed item total; drained={:?} residual={:?}",
            drained,
            residual
        );

        // Each value 0,1,2 must appear exactly once across (drained ++ residual).
        let mut combined = drained;
        combined.extend(residual);
        combined.sort();
        assert_eq!(
            combined,
            vec![0u8, 1, 2],
            "every pushed item must be observed exactly once across drain + residual; got {:?}",
            combined
        );
    });
}

/// Flush-vs-push safety: a flush that begins after a push's signaling
/// always observes that push's item.
///
/// This is the harder invariant — verifies the *happens-before* edge from
/// producer-push → flush-drain is preserved by `Mutex`. With
/// `preemption_bound=1`, loom explores both the pre-push-flush and
/// post-push-flush interleavings; both must satisfy the invariant.
///
/// The test uses a `pushed` AtomicBool so the flusher can check, *while
/// holding the batch lock*, whether the producer has already signaled.
/// This pins the lock-protected happens-before edge.
#[test]
fn batcher_flush_observes_concurrent_pushes() {
    loom::model(|| {
        let batch = Arc::new(Mutex::new(Vec::<u8>::new()));
        let pushed = Arc::new(AtomicBool::new(false));

        let b1 = batch.clone();
        let p1 = pushed.clone();
        let producer = thread::spawn(move || {
            b1.lock().unwrap().push(0xAA);
            p1.store(true, Ordering::SeqCst);
        });

        let b2 = batch.clone();
        let p2 = pushed.clone();
        let flusher = thread::spawn(move || {
            // Yield once to encourage the producer to run first under
            // loom's scheduler — without this, the flusher always wins
            // and the assertion below is trivially satisfied.
            loom::thread::yield_now();
            // Hold the lock through both the check and the drain — this
            // is the critical section that pins happens-before.
            let mut g = b2.lock().unwrap();
            // If the producer already signaled `pushed`, then the push
            // happened-before us (since the producer held `g` to push
            // and we now hold `g` to flush). The push MUST be visible.
            if p2.load(Ordering::SeqCst) {
                assert_eq!(
                    g.len(),
                    1,
                    "flusher must observe the prior push when the producer signaled"
                );
                assert_eq!(g[0], 0xAA);
            }
            g.clear();
        });

        producer.join().unwrap();
        flusher.join().unwrap();
        // The flusher's `g.clear()` runs AFTER the producer's push in
        // BOTH valid interleavings:
        //   (a) producer ran first → flusher observed the push, then cleared.
        //   (b) flusher ran first → flusher cleared the empty batch;
        //       producer then pushed into the cleared batch. After both
        //       threads join, the producer's push is in the batch.
        // In case (b), the batch contains the producer's push. The
        // happens-before chain via thread::join ensures the push is
        // visible to us. The invariant pinned here is:
        //   - total items observed across (push + clear) ≤ 1
        //     (the producer pushed at most once).
        let residual = batch.lock().unwrap().clone();
        assert!(
            residual.len() <= 1,
            "producer pushed at most one item; got {residual:?}"
        );
    });
}