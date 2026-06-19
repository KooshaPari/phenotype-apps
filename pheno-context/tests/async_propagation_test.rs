//! Integration test: Context propagates across async tasks via tracing spans.
//!
//! Spawns 3 tasks; each inherits the parent's `trace_id` via span context.

use pheno_context::Context;
use std::sync::Arc;
use std::sync::Mutex;
use http::HeaderMap;

fn make_ctx() -> Context {
    let mut h = HeaderMap::new();
    h.insert("x-trace-id", "11111111111111111111111111111111".parse().unwrap());
    h.insert("x-span-id", "2222222222222222".parse().unwrap());
    Context::from_http_headers(&h).unwrap()
}

#[tokio::test]
async fn trace_id_propagates_across_spawns() {
    let parent = make_ctx();
    let collected: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

    let mut handles = vec![];
    for i in 0..3 {
        let p = parent.clone();
        let c = Arc::clone(&collected);
        let h = tokio::spawn(async move {
            let span = p.current_span();
            let _enter = span.enter();
            let trace = p.trace_id.clone().unwrap_or_default();
            c.lock().unwrap().push(format!("task-{}:{}", i, trace));
        });
        handles.push(h);
    }

    for h in handles {
        h.await.unwrap();
    }

    let traces = collected.lock().unwrap();
    assert_eq!(traces.len(), 3);
    for t in traces.iter() {
        assert!(t.contains("11111111111111111111111111111111"));
    }
}
