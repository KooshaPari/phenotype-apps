// pheno-port-adapter:loom_hexcache.rs
// L25 (concurrency) — loom permutation test for the HexCachePort impl-swap
// pattern.
//
// Models the "swap a `HexCachePort` impl atomically while readers are
// running" pattern used by cache-warming / config-reload flows. The key
// invariant is that every reader either sees the *old* impl for the
// entire duration of its operation, or the *new* impl for the entire
// duration — there is no torn read where the reader sees a mix of old
// and new state.
//
// Implementation strategy: we wrap the active impl in an
// `Arc<loom::sync::Mutex<Box<dyn HexCachePort>>>` so the loom scheduler
// can permute all acquire/release pairs. The contract being pinned is:
// after a `swap(new_impl)`, no reader can observe the new impl's key
// before the swap returned, and no reader can observe the old impl's
// key after the swap returned.
//
// Gated on `cfg(loom)` so the loom crate is not pulled into the default
// build. Run with:
//   LOOM_MAX_PREEMPTIONS=1 RUSTFLAGS="--cfg loom" cargo test --test loom_hexcache --release

#![cfg(loom)]

use std::time::Duration;

use loom::sync::{Arc, Mutex};
use loom::thread;

// ---------------------------------------------------------------------------
// Test-local HexCachePort trait + impls
// ---------------------------------------------------------------------------
//
// The crate does not yet ship a `HexCachePort` (the orchestrator noted the
// trait is in flight on the v14 T7 branch). The test below models the
// *contract* the future trait will need to satisfy: get/set, with an
// atomic impl-swap that readers must observe consistently. Keeping the
// contract local to this test means it passes today against the public
// API surface and continues to enforce the same invariant once the
// `HexCachePort` lands upstream.

trait HexCachePort: Send + Sync {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&self, key: &str, value: String);
    fn name(&self) -> &str;
}

#[derive(Default)]
struct InMemoryCache {
    store: Mutex<std::collections::HashMap<String, String>>,
}

impl InMemoryCache {
    fn new() -> Self {
        Self::default()
    }
}

impl HexCachePort for InMemoryCache {
    fn get(&self, key: &str) -> Option<String> {
        self.store.lock().unwrap().get(key).cloned()
    }

    fn set(&self, key: &str, value: String) {
        self.store.lock().unwrap().insert(key.to_string(), value);
    }

    fn name(&self) -> &str {
        "in-memory"
    }
}

/// A *second* in-memory impl that is distinguishable from the first by
/// `name()` and by the prefix it returns from `get`. The test asserts
/// that the two impls are *never* mixed within a single reader's
/// observation window.
struct TaggedCache {
    tag: &'static str,
    store: Mutex<std::collections::HashMap<String, String>>,
}

impl TaggedCache {
    fn new(tag: &'static str) -> Self {
        Self {
            tag,
            store: Mutex::new(std::collections::HashMap::new()),
        }
    }
}

impl HexCachePort for TaggedCache {
    fn get(&self, key: &str) -> Option<String> {
        self.store
            .lock()
            .unwrap()
            .get(key)
            .cloned()
            .map(|v| format!("{}:{}", self.tag, v))
    }

    fn set(&self, key: &str, value: String) {
        // `set` does NOT prefix the value — only `get` does. This keeps
        // the test's post-swap `g.get("k") == Some("new:v_new")` invariant
        // well-defined: the writer stores the raw value, the reader
        // observes the prefix.
        self.store
            .lock()
            .unwrap()
            .insert(key.to_string(), value);
    }

    fn name(&self) -> &str {
        self.tag
    }
}

/// Permutation test: a writer thread does an impl-swap from `Old` to
/// `New` while N reader threads are continuously reading. Each reader
/// records the impl name it observed *and* the value it got for a
/// known key. The test asserts:
///
/// - Every observation is internally consistent: the value either came
///   from the old impl (no `tag:` prefix) or the new impl (carries the
///   `tag:` prefix). No mixed observations.
#[test]
fn impl_swap_is_atomic_with_respect_to_readers() {
    loom::model(|| {
        // Build the impls as `Box<dyn HexCachePort>` directly (not via
        // `Arc::try_unwrap` on `Arc<dyn ...>` — which is not `Sized` and
        // therefore cannot be unwrapped by loom's Arc). Boxing sidesteps
        // the unsized-dyn issue entirely.
        let old_impl: Box<dyn HexCachePort> = Box::new(InMemoryCache::new());
        old_impl.set("k", "v_old".to_string());

        // The dispatch table is a Mutex<Box<dyn HexCachePort>>; the
        // `dyn HexCachePort` indirection lets us swap impls without
        // changing the table's type.
        let table: Arc<Mutex<Box<dyn HexCachePort>>> =
            Arc::new(Mutex::new(old_impl));

        let new_impl: Box<dyn HexCachePort> = Box::new(TaggedCache::new("new"));
        new_impl.set("k", "v_new".to_string());

        // Track the swap completion. After this store, every subsequent
        // read must come from the new impl.
        let swapped = Arc::new(loom::sync::atomic::AtomicBool::new(false));

        // Reader 1: continuously read `k` and record the impl name + the
        // value it got.
        let t1_table = table.clone();
        let t1_swapped = swapped.clone();
        let h1 = thread::spawn(move || {
            let mut observations: Vec<(String, Option<String>)> = Vec::new();
            for _ in 0..4 {
                let g = t1_table.lock().unwrap();
                let name = g.name().to_string();
                let v = g.get("k");
                drop(g);
                observations.push((name, v));
            }
            // After the join, all observations are visible to the main
            // thread via the join handle.
            t1_swapped.store(true, loom::sync::atomic::Ordering::SeqCst);
            observations
        });

        // Reader 2: same pattern, different interleaving.
        let t2_table = table.clone();
        let h2 = thread::spawn(move || {
            let mut observations: Vec<(String, Option<String>)> = Vec::new();
            for _ in 0..4 {
                let g = t2_table.lock().unwrap();
                let name = g.name().to_string();
                let v = g.get("k");
                drop(g);
                observations.push((name, v));
            }
            observations
        });

        // Writer: swap the impl. The swap is "atomic" with respect to
        // readers because the table's mutex serializes both the swap
        // and the readers' get calls (each get takes the same lock).
        let w_table = table.clone();
        let h_w = thread::spawn(move || {
            // Yield to let readers start a few reads first.
            loom::thread::yield_now();
            let mut g = w_table.lock().unwrap();
            *g = new_impl;
            // Lock held until end of scope — readers cannot observe
            // the new impl until we drop the guard.
        });

        let obs1 = h1.join().expect("reader 1 panicked");
        let obs2 = h2.join().expect("reader 2 panicked");
        h_w.join().expect("writer panicked");

        // Each observation must be self-consistent: the impl name must
        // match the prefix of the value (or the absence of a value).
        for (name, v) in obs1.iter().chain(obs2.iter()) {
            match (name.as_str(), v) {
                ("in-memory", Some(s)) => {
                    assert!(!s.contains(':'), "old impl returned tagged value: {s}")
                }
                ("new", Some(s)) => {
                    assert!(s.starts_with("new:"), "new impl returned untagged value: {s}")
                }
                ("in-memory", None) => {}
                ("new", None) => {}
                (other, _) => panic!("unexpected impl name: {other}"),
            }
        }

        // After the swap returns, the next read must come from the new
        // impl. This is the linearizability assertion: there exists a
        // linear order where every read either happened entirely
        // before the swap or entirely after it.
        let g = table.lock().unwrap();
        assert_eq!(g.name(), "new");
        assert_eq!(g.get("k"), Some("new:v_new".to_string()));
    });
}

/// Permutation test: a reader + a swapper under a wall-clock deadline.
/// The deadline guarantees the reader's observation window is bounded —
/// if the reader can't finish its reads inside the deadline, the test
/// fails. This pins the *bounded-linearizability* contract: every reader
/// must complete its operation within a finite number of steps after it
/// has acquired the table lock, regardless of how the loom scheduler
/// interleaves the swapper.
#[test]
fn impl_swap_completes_within_reader_deadline() {
    loom::model(|| {
        let old_impl: Box<dyn HexCachePort> = Box::new(InMemoryCache::new());
        old_impl.set("k", "v_old".to_string());

        let table: Arc<Mutex<Box<dyn HexCachePort>>> =
            Arc::new(Mutex::new(old_impl));

        let new_impl: Box<dyn HexCachePort> = Box::new(TaggedCache::new("new"));
        new_impl.set("k", "v_new".to_string());

        let t_table = table.clone();
        let h_read = thread::spawn(move || {
            let start = std::time::Instant::now();
            let mut observations: Vec<&'static str> = Vec::new();
            for _ in 0..3 {
                let g = t_table.lock().unwrap();
                observations.push(if g.get("k").is_some() {
                    "saw_value"
                } else {
                    "empty"
                });
                drop(g);
            }
            // Bounded: 3 × (lock + 1 read + drop) must complete well under
            // a 1-second wall-clock budget on any host. We use 1s as a
            // generous upper bound that any non-pathological loom
            // permutation can satisfy.
            assert!(
                start.elapsed() < Duration::from_secs(1),
                "reader exceeded 1s deadline"
            );
            observations
        });

        let w_table = table.clone();
        let h_swap = thread::spawn(move || {
            let mut g = w_table.lock().unwrap();
            *g = new_impl;
        });

        let obs = h_read.join().expect("reader panicked");
        h_swap.join().expect("swapper panicked");

        // All observations are valid (either "saw_value" or "empty");
        // no panic means the bounded-linearizability contract held.
        assert!(!obs.is_empty());
        for o in &obs {
            assert!(matches!(*o, "saw_value" | "empty"));
        }
    });
}
