use anyhow::Result;

use crate::rpc::BitcoinRpcClient;

pub(super) fn run(client: &BitcoinRpcClient) -> Result<()> {
    tracing::info!(command = "blockchain-info", "executing CLI command");
    let info = client.get_blockchain_info()?;

    println!("Chain:                 {}", info.chain);
    println!("Blocks:                {}", info.blocks);
    println!("Headers:               {}", info.headers);
    println!("Best block hash:       {}", info.bestblockhash);
    println!("Difficulty:            {}", info.difficulty);
    println!(
        "Verification progress: {:.2}%",
        info.verificationprogress * 100.0
    );
    println!("Initial block download: {}", info.initialblockdownload);
    println!("Pruned:                {}", info.pruned);

    Ok(())
}
