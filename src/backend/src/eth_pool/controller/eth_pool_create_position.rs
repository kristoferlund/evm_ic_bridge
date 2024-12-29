use crate::{
    eth_pool::eth_pool_manager::EthPoolManager, user::UserManager,
    utils::alloy::fixed_bytes_from_hex_str,
};
use ic_cdk::query;

use crate::{http_error::HttpError, user::user_utils::auth_guard_eth};

#[query]
pub fn eth_pool_create_position(hash: String) -> Result<(), HttpError> {
    auth_guard_eth()?;

    let caller = ic_cdk::caller();
    let hash_bytes = fixed_bytes_from_hex_str::<32>(&hash).map_err(HttpError::bad_request)?;

    EthPoolManager::create_position(caller, hash_bytes).map_err(HttpError::bad_request)?;

    Ok(())
}
