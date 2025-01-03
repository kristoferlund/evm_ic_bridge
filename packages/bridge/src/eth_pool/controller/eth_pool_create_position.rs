use crate::{
    eth_pool::{
        eth_pool_manager::EthPoolManager, eth_pool_types::EthPoolLiquidityPositionDto, EthPoolError,
    },
    user::user_types::EthTxHashBytes,
    utils::alloy::fixed_bytes_array_from_hex_str,
};
use crate::{http_error::HttpError, user::user_utils::auth_guard_eth};
use ic_cdk::update;

#[update]
pub async fn eth_pool_create_position(
    hash: String,
) -> Result<EthPoolLiquidityPositionDto, HttpError> {
    auth_guard_eth()?;

    let hash_bytes: EthTxHashBytes =
        fixed_bytes_array_from_hex_str::<32>(&hash).map_err(HttpError::bad_request)?;

    let caller = ic_cdk::caller();
    match EthPoolManager::create_position(caller, hash_bytes).await {
        Ok(position) => Ok(EthPoolLiquidityPositionDto {
            tx_hash: hash,
            amount: position.amount.to_string(),
            last_claimed_fee_per_token: position.last_claimed_fee_per_token.to_string(),
            timestamp: position.timestamp,
        }),
        Err(e) => match e {
            EthPoolError::TransportError(_) => {
                Err(HttpError::internal_server_error("Transport error"))
            }
            _ => Err(HttpError::bad_request(e.to_string())),
        },
    }
}
