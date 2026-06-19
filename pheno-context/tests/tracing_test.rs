//! Tracing-feature smoke test (ADR-036).
//!
//! Verifies that the `tracing` opt-in feature compiles + links
//! correctly and that `Context` construction can be captured in a
//! `tracing` span. The crate itself does not emit spans in the
//! default code path; this test exercises the dependency
//! graph end-to-end with `tracing-test::traced_test`.

#![cfg(feature = "tracing")]

use http::HeaderMap;
use pheno_context::Context;

#[tracing_test::traced_test]
#[test]
fn emits_span_on_from_headers() {
    let mut headers = HeaderMap::new();
    headers.insert("X-Request-ID", "req-1".parse().unwrap());
    headers.insert("X-Trace-ID", "trace-1".parse().unwrap());
    headers.insert("X-Span-ID", "span-1".parse().unwrap());

    let span = tracing::info_span!("from_headers");
    let _enter = span.enter();
    let ctx = Context::from_headers(&headers).unwrap();
    tracing::info!(request_id = %ctx.request_id, "built");

    assert_eq!(ctx.request_id, "req-1");
    assert_eq!(ctx.trace_id, "trace-1");
    assert_eq!(ctx.span_id, "span-1");
}

#[tracing_test::traced_test]
#[test]
fn emits_span_on_builder() {
    let span = tracing::info_span!("build");
    let _enter = span.enter();
    let ctx = Context::new()
        .with_request_id("req-1")
        .with_trace_id("trace-1")
        .with_span_id("span-1")
        .build()
        .unwrap();
    tracing::info!(request_id = %ctx.request_id, "built");

    assert_eq!(ctx.request_id, "req-1");
}
