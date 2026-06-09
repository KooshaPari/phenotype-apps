# FocalPoint Justfile
set shell := ["bash", "-euo", "pipefail", "-c"]

# Show available commands
default:
    @just --list

# Build the workspace
build:
    cargo build --workspace

# Run all tests
test:
    cargo test --workspace

# Run linting (clippy + fmt check)
lint:
    cargo fmt -- --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Auto-format code
fmt:
    cargo fmt

# Run cargo-deny audit
audit:
    cargo deny check

# CI-like run (build + test + lint + audit)
ci: build test lint audit

# Clean artifacts
clean:
    cargo clean
