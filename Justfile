# Justfile — task runner for the FocalPoint project
# See https://just.systems/man/en/

set dotenv-load

_default:
    @just --list

# Run all CI checks locally
ci: fmt-check lint test build
    @echo "✓ CI checks pass"

# One-command root verify: fmt + clippy + tests + deny + journey-manifest gate.
# Exposed in CLAUDE.md and CI. Fails fast on first error.
# Usage: just verify
verify: fmt-check lint test deny docs-check
    @echo "✓ verify: all gates passed"

# cargo-deny: advisories + licenses + bans
deny:
    cargo deny check advisories
    cargo deny check licenses
    cargo deny check bans

# Docs integrity: check that threat-model and journey docs exist and are non-empty
docs-check:
    #!/usr/bin/env bash
    set -euo pipefail
    required=(
        "docs/security/threat-model.md"
        "docs/operations/journey-traceability.md"
        "SECURITY.md"
    )
    for f in "${required[@]}"; do
        if [[ ! -s "$f" ]]; then
            echo "docs-check FAIL: $f is missing or empty" >&2
            exit 1
        fi
    done
    echo "docs-check: OK"

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
# Grade targets (strictest checks — no caching)
grade:
    @echo "=== Running full grade ==="
    ./grade.sh

grade-fast:
    @echo "=== Running fast grade ==="
    ./grade.sh --fast

grade-json:
    @echo "=== Running grade (JSON) ==="
    ./grade.sh --json

grade-html:
    @echo "=== Running grade (HTML) ==="
    ./grade.sh --html

