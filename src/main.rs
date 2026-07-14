mod cli;
mod commands;
mod config;
mod error;
mod logger;
mod rpc;

use anyhow::{Context, Result};
use clap::Parser;

use crate::{cli::Cli, config::AppConfig, rpc::BitcoinRpcClient};

fn main() -> Result<()> {
    // A missing .env file is valid: environment variables or config.toml may
    // supply the settings instead.
    dotenvy::dotenv().ok();
    logger::init();

    let cli = Cli::parse();
    tracing::debug!(config_path = %cli.config.display(), "loading application configuration");
    let config = AppConfig::load(&cli).context("failed to load application configuration")?;
    let client = BitcoinRpcClient::new(config.rpc_connection())
        .context("failed to initialize the Bitcoin Core RPC client")?;

    commands::execute(cli.command, &client, &config)
}
