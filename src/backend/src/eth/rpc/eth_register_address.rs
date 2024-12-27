use ic_cdk::update;

use crate::{auth_guard, http_error::HttpError, siwe, user};

#[update]
async fn eth_register_address() -> Result<bool, HttpError> {
    // No anonymous calls allowed
    auth_guard()?;

    let caller = ic_cdk::caller();

    // Register Ethereum address for the caller is allowed only once
    if user::get_by_principal(caller).is_ok() {
        return Err(HttpError::bad_request(
            "Ethereum address already registered.",
        ));
    }

    // Get Ethereum address from the SIWE provider canister
    // Sign in with Ethereum needs to be done before this call
    let address = siwe::get_eth_address(caller)
        .await
        .map_err(|e| HttpError::bad_request(e.to_string()))?
        .ok_or_else(|| HttpError::not_found("No Ethereum address found for caller."))?;

    // Save Ethereum address to the user record so we don't have to call the SIWE canister every time
    user::set_eth_address(caller, address)
        .map_err(|e| HttpError::internal_server_error(e.to_string()))?;

    Ok(true)
}
