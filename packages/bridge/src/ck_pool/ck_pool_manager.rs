use candid::{Nat, Principal};

use crate::{
    declarations::icrc1_ledger::{self, icrc1_ledger, BlockIndex, GetBlocksArgs, Icrc3Value},
    STATE,
};

use super::ck_pool_types::{CkPoolError, CkPoolLiquidityPosition};
use anyhow::{anyhow, Result};

pub struct CkPoolManager {}
// pub async fn create_position(
//     user_principal: Principal,
//     hash: EthTxHashBytes,
// ) -> Result<EthPoolLiquidityPosition, EthPoolError> {
//     if EthPoolManager::position_already_created(user_principal, hash) {
//         return Err(anyhow!("Position already created").into());
//     }
//
//     let rpc_service = get_rpc_service();
//     let config = IcpConfig::new(rpc_service);
//     let provider = ProviderBuilder::new().on_icp(config);
//     let tx = provider
//         .get_transaction_by_hash(FixedBytes::from_slice(hash.as_slice()))
//         .await?
//         .ok_or_else(|| anyhow!("Transaction not found"))?;
//
//     // Verify that the transaction has been mined
//     let tx_block_number = tx
//         .block_number
//         .ok_or_else(|| anyhow!("Transaction not mined"))?;
//
//     // Verify that the transaction was sent by the caller
//     let user = UserManager::get_by_principal(user_principal)?;
//     let user_eth_address = user.eth_address.ok_or(UserError::NoEthAddress)?;
//     let user_eth_address = Address::from_slice(user_eth_address.as_slice());
//     if tx.from != user_eth_address {
//         return Err(anyhow!("Transaction not sent by caller").into());
//     }
//
//     // Verify that the transaction was sent to the canister address
//     let tx_to = tx
//         .to
//         .ok_or_else(|| anyhow!("Transaction has no recipient"))?;
//     let canister_eth_address = STATE
//         .with_borrow(|state| state.eth_pool_address)
//         .ok_or_else(|| anyhow!("Canister address not set"))?;
//     if tx_to != canister_eth_address {
//         return Err(anyhow!("Transaction not sent to canister address").into());
//     }
//
//     // Verify that the transaction has enough confirmations
//     let latest_block = provider.get_block_number().await?;
//     let nr_confirmations = latest_block - tx_block_number;
//     let eth_min_confirmations = STATE.with_borrow(|state| state.eth_min_confirmations);
//     if nr_confirmations < eth_min_confirmations {
//         return Err(anyhow!("Transaction has not enough confirmations").into());
//     }
//
//     // Verify that the amount is greater than 0
//     if tx.value.is_zero() {
//         return Err(anyhow!("Invalid position amount").into());
//     }
//
//     let timestamp = ic_cdk::api::time();
//
//     // Adding liquidity
//     let position =
//         EthPoolStateTransitions::create_position(user_principal, tx.value, hash, timestamp);
//     EventPublisher::publish(Event::EThPoolCreatePosition(
//         user_principal,
//         tx.value.to_string(),
//         hash,
//         timestamp,
//     ))
//     .unwrap();
//
//     Ok(position)
// }

impl CkPoolManager {
    pub async fn create_position(
        user_principal: Principal,
        block_index: BlockIndex,
    ) -> Result<CkPoolLiquidityPosition, CkPoolError> {
        if CkPoolManager::position_already_created(user_principal, &block_index).await {
            return Err(anyhow!("Position already created").into());
        }

        let (response,) = icrc1_ledger
            .icrc_3_get_blocks(vec![GetBlocksArgs {
                start: block_index,
                length: Nat::from(1u32),
            }])
            .await
            .map_err(|(rejection_code, message)| {
                anyhow!(
                    "Couldn't get block, message: {}, code: {:?}",
                    message,
                    rejection_code
                )
            })?;

        if response.blocks.is_empty() {
            return Err(anyhow!("Empty block returned").into());
        }

        let tx = get_value_by_key(&response.blocks[0].block, "tx")
            .ok_or_else(|| anyhow!("Block contains no transaction"))?;

        let op = get_value_by_key(tx, "op")
            .ok_or_else(|| anyhow!("Invalid tx format, 'op' not specified"))?;

        if !is_text_equal(op, "xfer") {
            return Err(anyhow!("Transaction is not a transfer").into());
        }

        // Verify the transaction was sent by the caller
        let from = get_value_by_key(tx, "from")
            .ok_or_else(|| anyhow!("Invalid tx format, 'from' not specified"))?;

        match from {
            Icrc3Value::Array(array) => {
                if array.len() != 1 {
                    return Err(anyhow!("Invalid tx format, 'from' not a single value").into());
                }
                match &*array[0] {
                    Icrc3Value::Blob(from) => {
                        let from = Principal::from_slice(from.as_slice());
                        if from != user_principal {
                            return Err(anyhow!("Transaction not sent by caller").into());
                        }
                    }
                    _ => return Err(anyhow!("Invalid tx format, 'from' not a blob").into()),
                }
            }
            _ => return Err(anyhow!("Invalid tx format, 'from' not an array").into()),
        }

        //verify that the transaction was sent to the canister address
        let to = get_value_by_key(tx, "to")
            .ok_or_else(|| anyhow!("Invalid tx format, 'to' not specified"))?;

        //TODO: Implement the rest of the function

        unimplemented!()
    }

    pub async fn position_already_created(
        user_principal: Principal,
        block_index: &BlockIndex,
    ) -> bool {
        STATE.with_borrow(|state| {
            state
                .ck_pool_liquidity_positions
                .get(&user_principal)
                .map(|positions| {
                    positions
                        .iter()
                        .any(|position| position.block_index == *block_index)
                })
                .unwrap_or(false)
        })
    }
}

pub fn get_value_by_key<'a>(icrc3_value: &'a Icrc3Value, key: &str) -> Option<&'a Icrc3Value> {
    if let Icrc3Value::Map(entries) = icrc3_value {
        for (k, v) in entries {
            if k == key {
                return Some(v);
            } else if let Some(found) = get_value_by_key(icrc3_value, key) {
                return Some(found);
            }
        }
    }
    None
}

pub fn is_text_equal(icrc3_value: &Icrc3Value, text: &str) -> bool {
    match icrc3_value {
        Icrc3Value::Text(value) => value == text,
        _ => false,
    }
}
