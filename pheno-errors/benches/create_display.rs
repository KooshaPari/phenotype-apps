//! Criterion benchmark — `pheno-errors` create + display.
//!
//! Exercises the canonical [`pheno_errors::AppError`] type. Each
//! timed iteration constructs a fresh [`AppError::Validation`] with
//! a dynamic message and then renders it via `Display::fmt` (via
//! `to_string`). The benchmark uses [`std::hint::black_box`] on the
//! rendered `String` length so the optimizer cannot elide either
//! the construction or the formatting.
//!
//! ## Why this matters
//!
//! `AppError` is on the hot path of every fallible call site in
//! the fleet. A regression in `Display` formatting or in the
//! `thiserror`-derived source-chain walking will surface as more
//! time spent in error propagation, which is amplified in
//! request/response loops. Keeping this benchmark in tree catches
//! regressions before they ship.
//!
//! Per-iteration: **10000 create + display cycles**.

use criterion::{criterion_group, criterion_main, Criterion};
use pheno_errors::AppError;

/// 10000 iterations per sample — matches the SIDE-23 brief.
const ITERS: usize = 10_000;

fn bench_create_and_display_10_000x(c: &mut Criterion) {
    c.bench_function("pheno_errors::create_and_display_10000x", |b| {
        b.iter(|| {
            // Accumulate the rendered byte count so the compiler
            // can't elide the inner loop. The accumulator is
            // outside the hot path so it doesn't perturb timings.
            let mut total_chars: usize = 0;
            for i in 0..ITERS {
                let err = AppError::validation(format!(
                    "validation failed for field #{i} (expected non-empty string)"
                ));
                let rendered = err.to_string();
                total_chars = total_chars.wrapping_add(rendered.len());
            }
            std::hint::black_box(total_chars);
        });
    });
}

criterion_group!(
    name = pheno_errors_create_display;
    config = Criterion::default()
        // Quick profile for the macbook device-fit gate (ADR-023
        // Rule 3.1). Override with `--profile-time` for the full
        // measurement.
        .warm_up_time(std::time::Duration::from_millis(500))
        .measurement_time(std::time::Duration::from_secs(2));
    targets = bench_create_and_display_10_000x
);
criterion_main!(pheno_errors_create_display);
