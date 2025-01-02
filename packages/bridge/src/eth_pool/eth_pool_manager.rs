use super::{eth_pool_types::EthPoolLiquidityPosition, EthPoolError, EthPoolStateTransitions};
use crate::{
    event::{Event, EventPublisher},
    evm::utils::get_rpc_service,
    user::{UserError, UserManager},
    STATE,
};
use alloy::{
    primitives::{Address, FixedBytes},
    providers::{Provider, ProviderBuilder},
    transports::icp::IcpConfig,
};
use anyhow::{anyhow, Result};
use candid::Principal;

pub struct EthPoolManager {}

impl EthPoolManager {
    pub async fn create_position(
        user_principal: Principal,
        hash: FixedBytes<32>,
    ) -> Result<EthPoolLiquidityPosition, EthPoolError> {
        let user = UserManager::get_by_principal(user_principal)?;
        let user_eth_address = user.eth_address.ok_or(UserError::NoEthAddress)?;
        let user_eth_address = Address::from_slice(user_eth_address.as_slice());

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
        if tx.from != user_eth_address {
            return Err(anyhow!("Transaction not sent by caller").into());
        }

        // Verify that the transaction was sent to the canister address
        let tx_to = tx
            .to
            .ok_or_else(|| anyhow!("Transaction has no recipient"))?;
        let canister_eth_address = STATE
            .with_borrow(|state| state.eth_pool_address)
            .ok_or_else(|| anyhow!("Canister address not set"))?;
        if tx_to != canister_eth_address {
            return Err(anyhow!("Transaction not sent to canister address").into());
        }

        // Verify that the transaction has enough confirmations
        let latest_block = provider.get_block_number().await?;
        let nr_confirmations = latest_block - tx_block_number;
        let eth_min_confirmations = STATE.with_borrow(|state| state.eth_min_confirmations);
        if nr_confirmations < eth_min_confirmations {
            return Err(anyhow!("Transaction has not enough confirmations").into());
        }

        // Adding liquidity
        let position = EthPoolStateTransitions::create_position(user_principal, tx.value);
        EventPublisher::publish(Event::EThPoolCreatePosition(
            user_principal,
            tx.value.to_string(),
        ))
        .unwrap();

        Ok(position)
    }
}
