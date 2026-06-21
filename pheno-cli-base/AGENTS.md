# AGENTS.md — pheno-cli-base

## Purpose
Shared CLI patterns for Phenotype CLIs: `ConfigArg`, `Verbosity`, and `setup_tracing`.

## When to use
- Building a new CLI tool in the pheno-* fleet
- Standardizing tracing + config loading across multiple CLIs
- Adding `-c/--config` + `-v/--verbose` flags consistently

## When NOT to use
- Library code (no CLI surface needed)
- HTTP services (use `pheno-otel` + `pheno-context` instead)
- Long-running daemons (use the current ADR-014 IoC substrate; do not add new dependencies on archived `phenotype-bus`)

## 5-line quickstart
```rust
use clap::Parser;
use pheno_cli_base::{ConfigArg, Verbosity, setup_tracing};

#[derive(Parser)] struct Cli { #[command(flatten)] config: ConfigArg, #[command(flatten)] verbosity: Verbosity }
let cli = Cli::parse();
setup_tracing(cli.verbosity.to_filter());
```

## Architecture
- Pure Rust, no async runtime
- 3 public types: `ConfigArg`, `Verbosity`, `setup_tracing`
- Clap derive for argument parsing
- `tracing-subscriber` for log output

## Tier
**Tier 0** (per ADR-023 Rule 3.1 — meta-bundle required)

## Build / test
```bash
cargo test
cargo test --doc         # doctest the Quick Start block
cargo run --example quickstart -- -v
```

## Compliance
- 71-pillar score: see `findings/2026-06-18-T13-x-71-pillar-audit-*.md`
- ADR-036: tracing is required (mandatory; not feature-gated for CLI substrate)
- ADR-039: pheno-flake template applied

## See also
- `README.md` — full usage
- `SPEC.md` — type definitions and design
- `WORKLOG.md` — change history
