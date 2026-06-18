# Changelog

All notable changes to pheno-fastapi-base are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.1.0] - 2026-06-18

### Added
- `create_app` factory with health/ready probes + structlog middleware.
- `AppError` exception type with stable wire-code mapping.
- `AsyncTestClient` for in-process testing.
- 5 contract tests in `tests/test_app.py`.

### Notes
- Tier 0 substrate per ADR-023.
- See SPEC.md for public API.