use anyhow::Result;
use clap::Parser;
use tracing::info;

use kvirtualstage::KVirtualStageCommand;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let cmd = KVirtualStageCommand::parse();

    match cmd.execute().await {
        Ok(_) => {
            info!("KVirtualStage completed successfully");
            Ok(())
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
