use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "bitcoin-cli-rs",
    version,
    about = "Interact with Bitcoin Core through its JSON-RPC API"
)]
pub struct Cli {
    /// Path to the TOML configuration file.
    #[arg(long, default_value = "config.toml")]
    pub config: PathBuf,

    /// Bitcoin Core JSON-RPC URL.
    #[arg(long, env = "BITCOIN_RPC_URL")]
    pub rpc_url: Option<String>,

    /// Bitcoin Core JSON-RPC username.
    #[arg(long, env = "BITCOIN_RPC_USER", hide_env_values = true)]
    pub rpc_user: Option<String>,

    /// Bitcoin Core JSON-RPC password.
    #[arg(long, env = "BITCOIN_RPC_PASSWORD", hide_env_values = true)]
    pub rpc_password: Option<String>,

    /// Bitcoin Core wallet used by wallet-specific commands.
    #[arg(long, env = "BITCOIN_RPC_WALLET")]
    pub wallet: Option<String>,

    /// RPC request timeout in seconds.
    #[arg(long, env = "BITCOIN_RPC_TIMEOUT_SECONDS")]
    pub timeout_seconds: Option<u64>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Display information about the active blockchain.
    BlockchainInfo,

    /// Display information about the configured wallet.
    WalletInfo,

    /// Display the configured wallet's trusted balance.
    Balance,

    /// Generate a new receiving address in the configured wallet.
    NewAddress,

    /// Call an arbitrary Bitcoin Core RPC method.
    Rpc {
        /// Bitcoin Core RPC method name.
        method: String,

        /// Positional RPC parameters, expressed as JSON values or strings.
        params: Vec<String>,
    },
}
