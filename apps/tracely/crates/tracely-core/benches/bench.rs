/// Criterion benchmark harness for tracely-core
///
/// Measures latency of the core tracing and logging paths.
/// Run: `cargo bench --package tracely-core`

use criterion::{criterion_group, criterion_main, Criterion};
use tracely_core::tracing::{TraceContext, TracingConfig};

fn bench_trace_context_new(c: &mut Criterion) {
    c.bench_function("trace_context_new", |b| {
        b.iter(|| {
            let _ctx = TraceContext::new();
        });
    });
}

fn bench_tracing_init(c: &mut Criterion) {
    // Only measure the config creation, not actual subscriber init
    // (which has global side-effects)
    c.bench_function("tracing_config_default", |b| {
        b.iter(|| {
            let _cfg = TracingConfig::default();
        });
    });
}

criterion_group!(benches, bench_trace_context_new, bench_tracing_init);
criterion_main!(benches);
