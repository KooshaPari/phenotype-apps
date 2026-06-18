# Changelog

All notable changes to `pheno-tracing` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- AGENTS.md, llms.txt, WORKLOG.md, CHANGELOG.md, LICENSE-MIT (T1.3 of v7 DAG, 2026-06-18)

## [0.1.1] - 2026-06-15

### Fixed
- Cleaned up `pheno-tracing` warnings (`c144f58c59`): added `Default` impl where needed; resolved unused-import lint.
- Removed duplicate top-level `pheno-tracing/` (`52bae896c5`); canonical location is `crates/pheno-tracing/`.

## [0.1.0] - 2026-06-13

### Added
- Initial release of `pheno-tracing` (PR #113).
- `TracePort` trait — port-driven abstraction for tracing
- `TraceOperation`, `TraceResult`, `SpanId`, `TraceId`, `SpanKind` types
- Concrete adapters in `src/adapters.rs`
- Integration tests in `tests/port_integration.rs` and `tests/adapter_tests.rs`
- Built on `tracing` 0.1 + `tracing-subscriber` 0.3
