//! Tracing integration test for pheno-errors.
//!
//! Run with:
//!   OTEL_SDK_DISABLED=true cargo test --features tracing --test tracing_test

#![cfg(feature = "tracing")]

use pheno_errors::AppError;
use tracing_test::traced_test;

#[traced_test]
#[test]
fn validation_error_logs() {
    let err = AppError::Validation {
        field: "email".into(),
        message: "must be valid".into(),
    };
    tracing::error!(error = %err, "validation failed");
    assert!(logs_contain("validation failed"));
    assert!(logs_contain("email"));
}

#[traced_test]
#[test]
fn notfound_error_logs() {
    let err = AppError::NotFound {
        entity: "user".into(),
        id: "42".into(),
    };
    tracing::warn!(error = %err, "user not found");
    assert!(logs_contain("user not found"));
}
