mod address;
mod blockchain;
mod rpc;
mod wallet;

use anyhow::{Context, Result};

use crate::{cli::Command, config::AppConfig, error::RpcError, rpc::BitcoinRpcClient};

pub fn execute(command: Command, client: &BitcoinRpcClient, config: &AppConfig) -> Result<()> {
    match command {
        Command::BlockchainInfo => blockchain::run(client),
        Command::WalletInfo => wallet::run_info(client, &config.wallet),
        Command::Balance => wallet::run_balance(client, &config.wallet),
        Command::NewAddress => address::run(client, &config.wallet),
        Command::Rpc { method, params } => rpc::run(client, method, params),
    }
}

fn handle_wallet_result<T>(result: Result<T, RpcError>, wallet: &str) -> Result<T> {
    match result {
        Ok(value) => Ok(value),
        Err(error @ RpcError::WalletUnavailable { .. }) => Err(error).with_context(|| {
            format!(
                "wallet `{wallet}` is unavailable; create it with `bitcoin-cli createwallet \"{wallet}\"` or load it with `bitcoin-cli loadwallet \"{wallet}\"`"
            )
        }),
        Err(error) => Err(error.into()),
    }
}
