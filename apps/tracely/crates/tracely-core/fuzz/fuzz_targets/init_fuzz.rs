#![no_main]

use libfuzzer_sys::fuzz_target;
use tracely::tracing::{TraceContext, TracingConfig};

fuzz_target!(|data: &[u8]| {
    // Ensure TraceContext + TracingConfig can handle arbitrary data
    // without panicking or UB.
    let _ctx = TraceContext::new();
    let _cfg = TracingConfig::default();
    // Fuzz the logging path with random string data
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = tracely::logging::LoggerConfig::default();
    }
});
