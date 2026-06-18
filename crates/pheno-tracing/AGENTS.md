# pheno-tracing

**Date:** 2026-06-18
**Status:** ACTIVE
**MSRV:** edition 2021 (see `Cargo.toml`)

## Purpose

Port-driven distributed tracing crate. Provides a clean port/adapter
boundary for telemetry integration (per ADR-014 hexagonal L4). Defines
`TracePort`, `TraceOperation`, `TraceResult`, `SpanId`, `TraceId`,
`SpanKind` and concrete adapters for the fleet. Canonical tracing
substrate per ADR-012.

## Build

```bash
cargo build --release
cargo test --workspace
cargo clippy --all-targets -- -D warnings
```

## Substrate Placement

`pheno-*-lib` (ADR-023) — pure reusable Rust library. Hexagonal
pattern primitive (ADR-014). Fleet-critical substrate — **canonical
across the entire pheno-* fleet** (ADR-012 supersedes any prior
duplicate crates).

## Authority

phenotype-org-governance/SUPERSEDED.md
