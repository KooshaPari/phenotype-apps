# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- `src/lib.rs`: replace blanket `pub use module::*` glob re-exports with explicit
  named re-exports, giving downstream crates a stable, documented public API surface
  (audit finding L0/L5).
- `.github/workflows/release.yml`: switch trigger from `push: branches: [main]` to
  `push: tags: v*`, remove broken `promote` job referencing a 404 placeholder action,
  add `--locked` flag to `cargo build`/`cargo publish`, add Cargo.toml↔tag version
  verification step, add SBOM generation via `cargo-cyclonedx`, and fix garbled
  `${{ }}` template expressions (audit finding L9/L17).
- `.github/workflows/quality-gate.yml`: remove `continue-on-error: true` from the
  coverage threshold check so a coverage regression actually fails the gate
  (audit finding L11).
- `CODEOWNERS`: tombstone root file with a redirect comment; `.github/CODEOWNERS` is
  the single authoritative source (audit finding L37).
- `ADR.md`: add canonical-source header noting `docs/adr/` wins on conflict
  (audit finding L37).
- `src/endpoints.rs`: add `HealthzEndpoint` and `ReadyzEndpoint` so any
  service built on `apisync` can mount liveness/readiness probes via the
  standard router without pulling in extra dependencies (audit finding L5/L27).
- `src/domain/middleware/request_id.rs`: add `RequestIdMiddleware` that echoes
  the inbound `X-Request-Id` header or generates a fresh id and stamps it on
  the response, closing the request-id propagation gap noted in the audit
  (audit finding L5/L27).
- `README.md`: document the expected `tokio::time::timeout` wrapper around
  adapter boundaries so downstream callers fail closed instead of waiting
  forever on transport work (audit finding L26).
- `LICENSE`: include both MIT and Apache-2.0 license texts to match the
  crate's `MIT OR Apache-2.0` declaration (audit finding L17).
- `fuzz/fuzz_targets/router_dispatch.rs`: populate the previously empty fuzz
  harness with a smoke-test that drives `ItemCrudEndpoint::handle` with random
  bytes so future regressions in the dispatch path surface during fuzzing
  (audit finding L11/L25).
- `fuzz/Cargo.toml`: declare the local `apisync` + `serde_json` + `futures`
  dependencies required by the new fuzz target.
- `AGENTS.md`: add a one-line backlog pointer so autonomous agents know where
  to look for the next round of audit findings (audit finding L30/L38).

[Unreleased]: https://github.com/KooshaPari/Apisync/compare/main...HEAD
