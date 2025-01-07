use crate::{
    ck_pool::{ck_pool_manager::CkPoolManager, ck_pool_types::CkPoolLiquidityPositionDto},
    declarations::icrc1_ledger::BlockIndex,
    http_error::HttpError,
    user::user_utils::auth_guard_no_anon,
};
use ic_cdk::update;
use std::str::FromStr;

#[update]
pub async fn ck_pool_create_position(
    block_index: String,
) -> Result<CkPoolLiquidityPositionDto, HttpError> {
    auth_guard_no_anon()?;

    let block_index = BlockIndex::from_str(&block_index)
        .map_err(|_| HttpError::bad_request("Invalid block index"))?;

    let caller = ic_cdk::caller();
    match CkPoolManager::create_position(caller, block_index).await {
        Ok(position) => Ok(CkPoolLiquidityPositionDto {
            block_index: position.block_index.to_string(),
            amount: position.amount.to_string(),
            last_claimed_fee_per_token: position.last_claimed_fee_per_token.to_string(),
            timestamp: position.timestamp,
        }),
        Err(e) => Err(HttpError::bad_request(e.to_string())),
    }
}
