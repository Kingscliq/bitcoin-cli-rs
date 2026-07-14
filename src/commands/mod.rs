mod address;
mod blockchain;
mod wallet;

use anyhow::{Context, Result, bail};

use crate::{cli::Command, config::AppConfig, error::RpcError, rpc::BitcoinRpcClient};

pub fn execute(command: Command, client: &BitcoinRpcClient, config: &AppConfig) -> Result<()> {
    match command {
        Command::BlockchainInfo => blockchain::run(client),
        Command::WalletInfo => wallet::run_info(client, &config.wallet),
        Command::Balance => wallet::run_balance(client, &config.wallet),
        Command::NewAddress => address::run(client, &config.wallet),
        Command::Rpc { method, params } => {
            let _ = params;
            tracing::warn!(
                rpc_method = method,
                "generic RPC command is not implemented"
            );
            bail!("`rpc {method}` is defined but not implemented in this milestone")
        }
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
