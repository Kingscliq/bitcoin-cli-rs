use anyhow::Result;

use crate::rpc::BitcoinRpcClient;

use super::handle_wallet_result;

pub(super) fn run(client: &BitcoinRpcClient, wallet: &str) -> Result<()> {
    tracing::info!(command = "new-address", wallet, "executing CLI command");
    let address = handle_wallet_result(client.get_new_address(wallet), wallet)?;
    println!("{address}");
    Ok(())
}
