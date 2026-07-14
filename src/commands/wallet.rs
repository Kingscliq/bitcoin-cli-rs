use anyhow::Result;

use crate::rpc::BitcoinRpcClient;

use super::handle_wallet_result;

pub(super) fn run_info(client: &BitcoinRpcClient, wallet: &str) -> Result<()> {
    tracing::info!(command = "wallet-info", wallet, "executing CLI command");
    let info = handle_wallet_result(client.get_wallet_info(wallet), wallet)?;

    println!("Wallet:              {}", info.walletname);
    println!("Trusted balance:     {} BTC", info.trusted_balance);
    println!("Unconfirmed balance: {} BTC", info.unconfirmed_balance);
    println!("Immature balance:    {} BTC", info.immature_balance);
    println!("Transactions:        {}", info.txcount);

    Ok(())
}

pub(super) fn run_balance(client: &BitcoinRpcClient, wallet: &str) -> Result<()> {
    tracing::info!(command = "balance", wallet, "executing CLI command");
    let balance = handle_wallet_result(client.get_balance(wallet), wallet)?;
    println!("{balance} BTC");
    Ok(())
}
