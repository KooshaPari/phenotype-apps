# Changelog — pheno-go-ctxkit

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-18

### Added
- `Inject(ctx, headers)` — write `traceparent` to outbound `http.Header`
- `Extract(headers)` — read `traceparent` from inbound `http.Header` into a new `context.Context`
- `WithTraceparent(ctx, tp)` — force a specific `traceparent` value
- `TraceparentFromContext(ctx)` — get the current `traceparent` string
- W3C Trace Context Level 2 compliance
- 100% test coverage of public API
- Golden tests cross-verified with `pheno-context` (Rust)
- Dual license (MIT OR Apache-2.0)
- Tier 0 meta-bundle (per ADR-023)
- CI: test on ubuntu + macos; gofmt + vet; codecov

### Notes
- Stdlib only — no external deps (per ADR-023 Rule 3.1)
- Go 1.22+ required
