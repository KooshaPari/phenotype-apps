//! Unit tests for `pheno-context`.
//!
//! Covers: from_headers (W3C trace-context), current_span construction,
//! snapshot determinism, empty-headers edge case.

use pheno_context::Context;
use http::HeaderMap;

fn make_headers() -> HeaderMap {
    let mut h = HeaderMap::new();
    h.insert("x-request-id", "req-test-001".parse().unwrap());
    h.insert("x-trace-id", "0af7651916cd43dd8448eb211c80319c".parse().unwrap());
    h.insert("x-span-id", "b7ad6b7169203331".parse().unwrap());
    h.insert("x-user-id", "user-42".parse().unwrap());
    h
}

#[test]
fn from_headers_extracts_all() {
    let ctx = Context::from_headers(&make_headers()).expect("valid headers");
    assert_eq!(ctx.request_id.as_deref(), Some("req-test-001"));
    assert_eq!(
        ctx.trace_id.as_deref(),
        Some("0af7651916cd43dd8448eb211c80319c")
    );
    assert_eq!(ctx.span_id.as_deref(), Some("b7ad6b7169203331"));
    assert_eq!(ctx.user_id.as_deref(), Some("user-42"));
}

#[test]
fn from_headers_empty_is_ok() {
    let empty = HeaderMap::new();
    let ctx = Context::from_headers(&empty).expect("empty is valid");
    assert!(ctx.request_id.is_none());
    assert!(ctx.trace_id.is_none());
    assert!(ctx.span_id.is_none());
    assert!(ctx.user_id.is_none());
}

#[test]
fn from_headers_partial() {
    let mut h = HeaderMap::new();
    h.insert("x-trace-id", "0af7651916cd43dd8448eb211c80319c".parse().unwrap());
    let ctx = Context::from_headers(&h).expect("partial is valid");
    assert!(ctx.trace_id.is_some());
    assert!(ctx.span_id.is_none());
    assert!(ctx.request_id.is_none());
}

#[test]
fn current_span_constructs() {
    let ctx = Context::from_headers(&make_headers()).unwrap();
    let span = ctx.current_span();
    // Span can be entered without panicking
    let _enter = span.enter();
}

#[test]
fn snapshot_is_deterministic() {
    let ctx1 = Context::from_headers(&make_headers()).unwrap();
    let ctx2 = Context::from_headers(&make_headers()).unwrap();
    let snap1 = ctx1.snapshot();
    let snap2 = ctx2.snapshot();
    assert_eq!(snap1, snap2, "two identical contexts should produce identical snapshots");
}

#[test]
fn snapshot_serializable_to_json() {
    let ctx = Context::from_headers(&make_headers()).unwrap();
    let snap = ctx.snapshot();
    let json = serde_json::to_string(&snap).expect("serialize");
    assert!(json.contains("req-test-001"));
    assert!(json.contains("0af7651916cd43dd8448eb211c80319c"));
}

#[test]
fn merge_prefers_existing() {
    let a = Context::from_headers(&make_headers()).unwrap();
    let mut b = HeaderMap::new();
    b.insert("x-trace-id", "ffffffffffffffffffffffffffffffff".parse().unwrap());
    let b = Context::from_headers(&b).unwrap();
    let merged = a.merge(&b);
    // a wins because it already has a trace_id
    assert_eq!(
        merged.trace_id.as_deref(),
        Some("0af7651916cd43dd8448eb211c80319c")
    );
}
