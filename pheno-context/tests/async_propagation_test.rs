//! Integration test: Context values can be cloned and moved across work units.

use http::HeaderMap;
use pheno_context::Context;

fn make_ctx() -> Context {
    let mut h = HeaderMap::new();
    h.insert("x-request-id", "req-async-001".parse().unwrap());
    h.insert("x-trace-id", "11111111111111111111111111111111".parse().unwrap());
    h.insert("x-span-id", "2222222222222222".parse().unwrap());
    Context::from_headers(&h).unwrap()
}

#[test]
fn trace_id_survives_cloned_work_items() {
    let parent = make_ctx();
    let traces: Vec<String> = (0..3)
        .map(|i| {
            let ctx = parent.clone();
            format!("task-{}:{}", i, ctx.trace_id)
        })
        .collect();

    assert_eq!(traces.len(), 3);
    for trace in traces {
        assert!(trace.contains("11111111111111111111111111111111"));
    }
}
