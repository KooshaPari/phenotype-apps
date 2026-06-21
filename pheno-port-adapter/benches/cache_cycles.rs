//! Criterion benchmark — `pheno-port-adapter` cache send/recv.
//!
//! Exercises the [`HexCachePort`] trait via the
//! [`InMemoryCache`] adapter. Each timed iteration:
//!
//! 1. Creates a fresh `InMemoryCache`.
//! 2. Runs **100 send/recv cycles** via the port — each cycle
//!    performs a `put` (send) followed by a `get` (recv) with a
//!    unique key.
//!
//! ## Why this matters
//!
//! The hex-cache port is on the hot path of every read-through
//! cache in the fleet. A regression in `tokio::sync::Mutex` lock
//! acquisition or in the lazy-expiration check will surface as
//! added tail latency under load. Keeping this benchmark in tree
//! catches those regressions before they ship.
//!
//! ## Note on async + criterion
//!
//! Criterion is synchronous; we drive the async cache on a
//! single-threaded `tokio::runtime::Runtime` so the timings
//! reflect the port's own cost without runtime overhead
//! variance. The runtime is constructed **once** outside the
//! timed region — `block_on` calls amortise the runtime
//! construction cost over the 100 cycles inside the sample.

use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;

use pheno_port_adapter::adapters::InMemoryCache;
use pheno_port_adapter::HexCachePort;

/// 100 send/recv cycles per sample — matches the SIDE-23 brief.
const CYCLES: usize = 100;

fn bench_cache_100_cycles(c: &mut Criterion) {
    // Build the runtime once, outside the timed region.
    let rt = Runtime::new().expect("tokio runtime");

    c.bench_function("pheno_port_adapter::in_memory_cache_100_cycles", |b| {
        b.iter(|| {
            rt.block_on(async {
                let cache = InMemoryCache::new();
                // Send/recv loop — each iteration is one `put` followed
                // by one `get` on the same key, so the cache is
                // exercised end-to-end.
                let mut acc: usize = 0;
                for i in 0..CYCLES {
                    let key = format!("bench-key-{i}");
                    let value = format!("bench-value-{i}").into_bytes();
                    // SEND: put the value (no expiration).
                    cache
                        .put(&key, value, Duration::ZERO)
                        .await
                        .expect("put");
                    // RECV: read it back.
                    let got = cache.get(&key).await.expect("get").expect("hit");
                    acc = acc.wrapping_add(got.len());
                }
                std::hint::black_box(acc);
            });
        });
    });
}

criterion_group!(
    name = pheno_port_adapter_cache;
    config = Criterion::default()
        // Quick profile for the macbook device-fit gate (ADR-023
        // Rule 3.1). Override with `--profile-time` for the full
        // measurement.
        .warm_up_time(std::time::Duration::from_millis(500))
        .measurement_time(std::time::Duration::from_secs(2));
    targets = bench_cache_100_cycles
);
criterion_main!(pheno_port_adapter_cache);
