mod cli;
mod commands;
mod config;
mod error;
mod rpc;

use anyhow::{Context, Result};
use clap::Parser;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{cli::Cli, config::AppConfig, rpc::BitcoinRpcClient};

fn main() -> Result<()> {
    // A missing .env file is valid: environment variables or config.toml may
    // supply the settings instead.
    dotenvy::dotenv().ok();
    init_logging();

    let cli = Cli::parse();
    tracing::debug!(config_path = %cli.config.display(), "loading application configuration");
    let config = AppConfig::load(&cli).context("failed to load application configuration")?;
    let client = BitcoinRpcClient::new(config.rpc_connection())
        .context("failed to initialize the Bitcoin Core RPC client")?;

    commands::execute(cli.command, &client, &config)
}

fn init_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
    let formatting = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_level(true)
        .with_thread_ids(false)
        .with_file(false)
        .with_writer(std::io::stderr);

    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(formatting)
        .try_init();
}
