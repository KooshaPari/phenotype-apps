//! Criterion benchmark — `pheno-config` cascade load + parse.
//!
//! Exercises the [`pheno_config::cascade`] module: each timed iteration
//! builds the full provider cascade (defaults → optional `config.toml`
//! file on disk → `PHENO_*` env vars) and then extracts a typed value
//! from the resulting [`Figment`].
//!
//! ## Why this matters
//!
//! The cascade is on the cold-start path of every binary that uses
//! `pheno-config`. A regression in `Env::prefixed` lookup or in
//! `Toml::file` parsing will surface as slower process startup
//! across the fleet. Keeping this benchmark in tree (and run on CI
//! for label-stable crates) catches those regressions before they
//! ship.
//!
//! ## Two scenarios
//!
//! 1. `cascade_load_from_disk` — uses [`build_cascade`], which
//!    reads `config.toml` from the current working directory if
//!    present. Mirrors a normal production cold start.
//! 2. `cascade_load_from_str` — uses [`build_cascade_from_str`],
//!    which substitutes an in-memory TOML string for the on-disk
//!    file. Mirrors an embedded-defaults scenario and removes disk
//!    variance from the measurement.
//!
//! ## Env determinism
//!
//! Both benchmarks set a known `PHENO_*` env var so the
//! `Env::prefixed("PHENO_")` provider does meaningful work on
//! every iteration (otherwise the env layer would scan zero vars
//! and the timings would not reflect a typical container start).
//!
//! Per-iteration: **1000 cascade load + parse cycles**.

use std::env;

use criterion::{criterion_group, criterion_main, Criterion};
use figment::Figment;

use pheno_config::cascade::{build_cascade, build_cascade_from_str};

/// Helper: build the cascade and parse a typed value out of it.
///
/// Returns the parsed `u16` so the compiler cannot eliminate the
/// `find_value` call as dead code.
#[inline]
fn load_and_parse(cascade: Figment) -> u16 {
    let value = cascade
        .find_value("server.port")
        .expect("server.port must resolve from defaults");
    // `figment::value::Value::to_u128` is the typed-extraction path
    // used in the crate's own tests; we downcast to `u16` here.
    // Saturate on overflow so the bench never panics — the metric we
    // care about is the cascade-load time, not the parsed-port value.
    let n = value.to_u128().unwrap_or(0);
    u16::try_from(n).unwrap_or(u16::MAX)
}

/// 1000 iterations per sample — matches the SIDE-23 brief.
const ITERS: usize = 1000;

fn bench_cascade_load_from_disk(c: &mut Criterion) {
    // Set a deterministic env var so the Env layer is exercised on
    // every iteration. Cleared after the sample collection ends via
    // the scoped guard below.
    env::set_var("PHENO_SERVER_PORT", "9090");
    let restore = env::var("PHENO_SERVER_PORT");

    c.bench_function("pheno_config::cascade_load_from_disk_1000x", |b| {
        b.iter(|| {
            let mut acc: u32 = 0;
            for _ in 0..ITERS {
                let cascade = build_cascade();
                acc = acc.wrapping_add(load_and_parse(cascade) as u32);
            }
            // Black-box so the compiler cannot elide the loop.
            std::hint::black_box(acc);
        });
    });

    // Restore prior env state to avoid leaking into other benches.
    match restore {
        Ok(v) => env::set_var("PHENO_SERVER_PORT", v),
        Err(_) => env::remove_var("PHENO_SERVER_PORT"),
    }
}

fn bench_cascade_load_from_str(c: &mut Criterion) {
    // Same env setup as the disk variant.
    env::set_var("PHENO_SERVER_PORT", "9090");
    let restore = env::var("PHENO_SERVER_PORT");

    c.bench_function("pheno_config::cascade_load_from_str_1000x", |b| {
        b.iter(|| {
            let mut acc: u32 = 0;
            for _ in 0..ITERS {
                let cascade = build_cascade_from_str(
                    r#"
                    [server]
                    port = 8081
                    "#,
                );
                acc = acc.wrapping_add(load_and_parse(cascade) as u32);
            }
            std::hint::black_box(acc);
        });
    });

    match restore {
        Ok(v) => env::set_var("PHENO_SERVER_PORT", v),
        Err(_) => env::remove_var("PHENO_SERVER_PORT"),
    }
}

criterion_group!(
    name = pheno_config_cascade;
    config = Criterion::default()
        // Quick profile for the macbook device-fit gate (ADR-023
        // Rule 3.1: heavy work goes to heavy-runner). Override with
        // `--profile-time` for the full measurement.
        .warm_up_time(std::time::Duration::from_millis(500))
        .measurement_time(std::time::Duration::from_secs(2));
    targets = bench_cascade_load_from_disk, bench_cascade_load_from_str
);
criterion_main!(pheno_config_cascade);
