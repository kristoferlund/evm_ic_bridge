use ic_cdk::update;

use crate::{
    http_error::HttpError,
    siwe,
    user::{user_utils::auth_guard_no_anon, UserDto, UserManager},
};

#[update]
async fn user_register_eth_address() -> Result<UserDto, HttpError> {
    auth_guard_no_anon()?;

    let caller = ic_cdk::caller();

    // Make sure user exists and doesn't already have an Ethereum address
    match UserManager::get_by_principal(caller) {
        Ok(user) => {
            if user.eth_address.is_some() {
                return Err(HttpError::conflict("Ethereum address already registered."));
            }
            Ok(())
        }
        Err(_) => {
            return Err(HttpError::not_found("User not found."));
        }
    }?;

    // Get Ethereum address from the SIWE provider canister
    // Sign in with Ethereum needs to be done before this call
    let address = siwe::get_eth_address(caller)
        .await
        .map_err(|e| HttpError::bad_request(e.to_string()))?
        .ok_or_else(|| HttpError::not_found("No Ethereum address found for caller."))?;

    // Save Ethereum address to the user record so we don't have to call the SIWE canister every time
    let user = UserManager::set_eth_address(caller, address.into_array())
        .map_err(|e| HttpError::internal_server_error(e.to_string()))?;

    Ok(UserDto {
        principal: user.principal,
        eth_address: Some(address.to_checksum(None)),
    })
}
