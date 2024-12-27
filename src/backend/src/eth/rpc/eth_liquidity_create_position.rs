use ic_cdk::update;

use crate::{auth_guard, http_error::HttpError, user};

#[update]
async fn eth_liquidity_create_position() -> Result<bool, HttpError> {
    auth_guard()?;

    let caller = ic_cdk::caller();

    let user = user::get_by_principal(caller).map_err(|e| HttpError::bad_request(e.to_string()))?;

    // 1. Get eth address of caller
    // 2. Look up transaction hash
    // 3. Register liquidity position
    //

    Ok(true)
}
