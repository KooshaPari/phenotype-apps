# FocalPoint

Connector-first screen-time management platform. Native iOS enforcement built on a portable Rust core: rules engine, connector runtime, reward/penalty ledger, audit chain, mascot state machine.

## Stack
| Layer | Technology |
|-------|------------|
| Core | Rust (cargo workspace, 56 crates) |
| Mobile | Swift/SwiftUI (iOS native app) |
| Backend | Go (services/) |
| DB | SQLite, PostgreSQL, SurrealDB |
| Config | deny.toml (cargo-deny), clippy.toml, rust-toolchain.toml |
| Testing | Rust tests, cargo test, pytest |

## Key Commands
```bash
# Build
cargo build --workspace

# Test
cargo test --workspace

# Lint
cargo clippy --all -- -D warnings

# Format
cargo fmt --all

# Quality gate
cargo test --workspace && cargo clippy --all -- -D warnings && cargo fmt --all

# Install iOS dependencies (macOS)
xcrun simctl list devices
```

## Key Files
- `crates/` — 56 Rust workspace crates
- `apps/` — Application entry points (iOS, CLI, etc.)
- `services/` — Go backend services
- `tooling/` — Build and developer tooling
- `fuzz/` — Fuzzing harnesses
- `docs/` — Documentation
- `docs-site/` — Published VitePress site
- `deny.toml` — cargo-deny security config
- `rust-toolchain.toml` — MSRV pin

## Reference
Global Phenotype rules: see `~/.claude/CLAUDE.md` or `/Users/kooshapari/CodeProjects/Phenotype/repos/CLAUDE.md`
