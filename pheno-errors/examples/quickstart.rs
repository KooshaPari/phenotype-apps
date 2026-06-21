//! Quickstart example for pheno-errors.
//!
//! Run with:
//!   cargo run --example quickstart

use pheno_errors::{AppError, AppResult};

fn load_user(id: u64) -> AppResult<User> {
    if id == 0 {
        return Err(AppError::Validation {
            field: "id".into(),
            message: "user id must be > 0".into(),
        });
    }
    if id > 1_000_000 {
        return Err(AppError::NotFound {
            entity: "user".into(),
            id: id.to_string(),
        });
    }
    Ok(User { id, name: format!("user-{}", id) })
}

#[derive(Debug)]
struct User {
    id: u64,
    name: String,
}

fn main() {
    // Success
    match load_user(42) {
        Ok(u) => println!("✓ Loaded: {:#?}", u),
        Err(e) => eprintln!("✗ Error: {}", e),
    }

    // Validation error
    match load_user(0) {
        Ok(u) => println!("✓ Loaded: {:#?}", u),
        Err(AppError::Validation { field, message }) => {
            eprintln!("✗ Validation on {}: {}", field, message);
        }
        Err(e) => eprintln!("✗ Other error: {}", e),
    }

    // NotFound error
    match load_user(9_999_999) {
        Ok(u) => println!("✓ Loaded: {:#?}", u),
        Err(AppError::NotFound { entity, id }) => {
            eprintln!("✗ {} with id={} not found", entity, id);
        }
        Err(e) => eprintln!("✗ Other error: {}", e),
    }
}
