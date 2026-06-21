# SPEC — pheno-cli-base

## Public API

### `ConfigArg`
```rust
#[derive(Args, Debug, Clone)]
pub struct ConfigArg {
    /// Path to a config file (also reads PHENOTYPE_CONFIG)
    #[arg(short, long, env = "PHENOTYPE_CONFIG")]
    pub config: Option<PathBuf>,
}

impl ConfigArg {
    pub fn path(&self) -> Option<&Path> { self.config.as_deref() }
}
```

### `Verbosity`
```rust
#[derive(Args, Debug, Clone, Copy)]
pub struct Verbosity {
    /// Increase log verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,
    /// Suppress non-error logs
    #[arg(short, long, conflicts_with = "verbose", global = true)]
    pub quiet: bool,
}

impl Verbosity {
    pub fn to_filter(&self) -> EnvFilter {
        if self.quiet { EnvFilter::new("error") }
        else {
            let level = match self.verbose { 0 => "info", 1 => "debug", _ => "trace" };
            EnvFilter::new(level)
        }
    }
}
```

### `setup_tracing`
```rust
pub fn setup_tracing(filter: EnvFilter) {
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();
}
```

## Design decisions
- **No async runtime** — pure synchronous, no tokio
- **`clap` derive only** — no builder pattern
- **`tracing-subscriber::fmt`** — single subscriber, no OTLP
- **`global = true` on flags** — verbose/quiet apply to subcommands
- **Tier 0 substrate** — meta-bundle required (per ADR-023)
- **Mandatory `tracing` dep** — unlike `pheno-errors`/`pheno-config`, CLI base legitimately needs tracing at the heart of its API

## Versioning
- `0.1.0` — initial release (2026-06-18)
- Backward-compatible within `0.x` per Cargo semver
