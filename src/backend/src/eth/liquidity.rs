use crate::{evm::utils::get_rpc_service, STATE};
use alloy::{
    primitives::{Address, FixedBytes},
    providers::{Provider, ProviderBuilder},
    transports::icp::IcpConfig,
};
use anyhow::{anyhow, bail, Result};

async fn create_position(address: Address, hash: FixedBytes<32>) -> Result<u64> {
    let rpc_service = get_rpc_service();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new().on_icp(config);
    let tx = provider
        .get_transaction_by_hash(hash)
        .await?
        .ok_or_else(|| anyhow!("Transaction not found"))?;

    // Verify that the transaction has been mined
    let tx_block_number = tx
        .block_number
        .ok_or_else(|| anyhow!("Transaction not mined"))?;

    // Verify that the transaction was sent by the caller
    if tx.from != address {
        bail!("Transaction not sent by caller");
    }

    // Verify that the transaction was sent to the canister address
    let tx_to = tx
        .to
        .ok_or_else(|| anyhow!("Transaction has no recipient"))?;
    let canister_eth_address = STATE
        .with_borrow(|state| state.canister_eth_address)
        .ok_or_else(|| anyhow!("Canister address not set"))?;
    if tx_to != canister_eth_address {
        bail!("Transaction not sent to canister address");
    }

    // Verify that the transaction has enough confirmations
    let latest_block = provider.get_block_number().await?;
    let nr_confirmations = latest_block - tx_block_number;
    let eth_min_confirmations = STATE.with_borrow(|state| state.eth_min_confirmations);
    if nr_confirmations < eth_min_confirmations {
        bail!("Transaction has not enough confirmations");
    }

    Ok(1)
}

async fn get_latest_block() -> Result<u64> {
    let rpc_service = get_rpc_service();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new().on_icp(config);
    let result = provider.get_block_number().await;

    match result {
        Ok(block) => Ok(block),
        Err(e) => bail!(e.to_string()),
    }
}
