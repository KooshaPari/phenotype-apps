//! Quickstart example for pheno-context.
//!
//! Demonstrates building a `Context` from request headers.
//!
//! Run with:
//!   cargo run --example quickstart

use http::HeaderMap;
use pheno_context::Context;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert("x-request-id", "req-abc-123".parse()?);
    headers.insert("x-trace-id", "0af7651916cd43dd8448eb211c80319c".parse()?);
    headers.insert("x-span-id", "b7ad6b7169203331".parse()?);
    headers.insert("x-user-id", "user-42".parse()?);

    let ctx = Context::from_headers(&headers)?;
    println!("Built context: {ctx}");

    Ok(())
}
