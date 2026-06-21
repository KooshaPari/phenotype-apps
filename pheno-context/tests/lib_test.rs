//! Unit tests for `pheno-context`.
//!
//! Covers: header extraction, required-header errors, display rendering, and clone determinism.

use http::HeaderMap;
use pheno_context::Context;

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
    assert_eq!(ctx.request_id, "req-test-001");
    assert_eq!(ctx.trace_id, "0af7651916cd43dd8448eb211c80319c");
    assert_eq!(ctx.span_id, "b7ad6b7169203331");
    assert_eq!(ctx.user_id.as_deref(), Some("user-42"));
    assert_eq!(ctx.org_id, None);
}

#[test]
fn from_headers_requires_request_id() {
    let mut h = HeaderMap::new();
    h.insert("x-trace-id", "0af7651916cd43dd8448eb211c80319c".parse().unwrap());
    h.insert("x-span-id", "b7ad6b7169203331".parse().unwrap());
    assert!(Context::from_headers(&h).is_err());
}

#[test]
fn from_headers_requires_trace_id() {
    let mut h = HeaderMap::new();
    h.insert("x-request-id", "req-test-001".parse().unwrap());
    h.insert("x-span-id", "b7ad6b7169203331".parse().unwrap());
    assert!(Context::from_headers(&h).is_err());
}

#[test]
fn from_headers_requires_span_id() {
    let mut h = HeaderMap::new();
    h.insert("x-request-id", "req-test-001".parse().unwrap());
    h.insert("x-trace-id", "0af7651916cd43dd8448eb211c80319c".parse().unwrap());
    assert!(Context::from_headers(&h).is_err());
}

#[test]
fn clone_is_deterministic() {
    let ctx = Context::from_headers(&make_headers()).unwrap();
    assert_eq!(ctx, ctx.clone());
}

#[test]
fn display_contains_core_fields() {
    let ctx = Context::from_headers(&make_headers()).unwrap();
    let rendered = ctx.to_string();
    assert!(rendered.contains("request_id=req-test-001"));
    assert!(rendered.contains("trace_id=0af7651916cd43dd8448eb211c80319c"));
    assert!(rendered.contains("span_id=b7ad6b7169203331"));
    assert!(rendered.contains("user_id=user-42"));
}
