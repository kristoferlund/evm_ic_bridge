use ic_cdk::update;

use crate::{auth_guard, eth::liquidity, http_error::HttpError, user};

#[update]
async fn eth_liquidity_create_position(tx_hash: String) -> Result<bool, HttpError> {
    auth_guard()?;

    let caller = ic_cdk::caller();

    let user = user::get_by_principal(caller).map_err(|e| HttpError::bad_request(e.to_string()))?;

    liquidity::

    // 1. Get eth address of caller
    // 2. Look up transaction hash
    // 3. Register liquidity position
    //

    Ok(true)
}
