//! Main library entry point for apisync

//! Main library entry point for apisync

pub mod adapters;
pub mod application;
pub mod domain;
pub mod endpoints;
pub mod infrastructure;

// Selective re-exports — NOT `pub use *` — so consumers depend on a stable
// public API surface rather than every internal module detail. This is the
// audit's L0/L14 finding: flat `pub use *` at crate root couples consumers
// to internal layering and makes every module change a potential breaking
// change.
pub use domain::{Endpoint, Request, Response};
pub use domain::{CreateItem, Item, ItemStore, UpdateItem};
pub use domain::{ApiError, HealthStatus};
pub use endpoints::ItemCrudEndpoint;
pub use application::Router;
pub use infrastructure::logging;

/// Main library entry point that initializes shared infrastructure.
pub struct ApiKit;

impl Default for ApiKit {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiKit {
    /// Create a new `ApiKit` and initialize tracing/logging.
    ///
    /// Call this once at application start to ensure structured logging is
    /// wired before any adapter or handler runs. This is the audit's L5
    /// observability fix: the `infrastructure::logging::init()` existed but
    /// was never called from the production code path.
    pub fn new() -> Self {
        logging::init();
        ApiKit
    }
}
