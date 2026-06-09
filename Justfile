# Justfile — task runner for the FocalPoint project
# See https://just.systems/man/en/

set dotenv-load

_default:
    @just --list

# Run all CI checks locally
ci: fmt-check lint test build
    @echo "✓ CI checks pass"

# Format code
fmt:
    cargo fmt --all

# Check formatting
fmt-check:
    cargo fmt --all -- --check

# Lint
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Run tests
test:
    cargo test --all-features

# Build release
build:
    cargo build --release

# Audit dependencies for security advisories
audit:
    cargo deny check advisories

# Check licenses
licenses:
    cargo deny check licenses

# Clean build artifacts
clean:
    cargo clean
