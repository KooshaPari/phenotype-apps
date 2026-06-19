//! Quickstart example for pheno-cli-base.
//!
//! Run with:
//!   cargo run --example quickstart -- -v -c config.toml
//!
//! Or use the PHENOTYPE_CONFIG / RUST_LOG env vars:
//!   RUST_LOG=debug cargo run --example quickstart -- -v
//!
//! Note: this is a no-network example, so `-c` will not actually load a file.

use clap::Parser;
use pheno_cli_base::{ConfigArg, Verbosity, setup_tracing};

#[derive(Debug, Parser)]
#[command(name = "quickstart", about = "pheno-cli-base demo")]
struct Cli {
    #[command(flatten)]
    config: ConfigArg,

    #[command(flatten)]
    verbosity: Verbosity,
}

fn main() {
    let cli = Cli::parse();
    setup_tracing(cli.verbosity.to_filter());

    tracing::info!("starting quickstart");
    if let Some(path) = cli.config.path() {
        tracing::info!(?path, "would load config");
    } else {
        tracing::info!("no config path given; using defaults");
    }
    tracing::info!("done");
}
