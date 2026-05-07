# Contributing

## Overview

Kwality is an archived experimental framework. This repository is preserved for historical and research reference. Active development has migrated to successor projects (Benchora, Tracera, phenotype-shared).

For new work, please use the current Phenotype ecosystem tooling.

## Development Setup

### Prerequisites

- Go 1.21+
- Rust 1.75+ (for `engines/runtime-validator/`)
- Docker and Docker Compose
- `golangci-lint`, `goimports`, `gosec`, `cargo`, `clippy`

### Installation

```bash
# Download Go dependencies
go mod download

# Build Rust runtime validator
cd engines/runtime-validator && cargo build

# Or build everything via Makefile
make setup
```

## Testing

```bash
# Run all tests (Go + Rust)
make test

# Run Go tests only
make test-go

# Run Rust tests only
make test-rust

# Run integration tests
make test-integration

# Run end-to-end tests (requires Docker)
make test-e2e
```

## Code Style

```bash
# Format all code
make fmt

# Lint all code
make lint

# Format Go code only
make fmt-go

# Lint Go code only
make lint-go
```

Go code uses `go fmt` and `goimports`. Rust code uses `cargo fmt` and `cargo clippy` with `-D warnings`.

## Security

```bash
# Run security checks
make security

# Check for vulnerabilities
make vuln-check
```

## Submitting Changes

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `make check` to verify linting and tests pass
5. Submit a pull request
