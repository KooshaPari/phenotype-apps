//! Quickstart example for pheno-context.
//!
//! Demonstrates building a `Context` from W3C trace-context headers
//! and using it to create a `tracing::Span` with structured fields.
//!
//! Run with:
//!   cargo run --example quickstart

use http::HeaderMap;
use pheno_context::Context;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simulate incoming HTTP request headers (W3C trace-context)
    let mut headers = HeaderMap::new();
    headers.insert("x-request-id", "req-abc-123".parse()?);
    headers.insert("x-trace-id", "0af7651916cd43dd8448eb211c80319c".parse()?);
    headers.insert("x-span-id", "b7ad6b7169203331".parse()?);
    headers.insert("x-user-id", "user-42".parse()?);

    // Build context from headers
    let ctx = Context::from_http_headers(&headers)?;
    println!("✓ Built context: {:#?}", ctx);

    // Create a tracing::Span pre-populated with context fields
    let span = ctx.current_span();
    let _enter = span.enter();

    tracing::info!("processing request");
    tracing::info!(user_id = %ctx.user_id.as_deref().unwrap_or("anonymous"), "user identified");

    Ok(())
}
