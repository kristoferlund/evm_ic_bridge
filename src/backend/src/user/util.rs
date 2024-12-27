use crate::http_error::HttpError;
use crate::user;
use alloy::primitives::Address;
use anyhow::{bail, Result};
use candid::Principal;
use ic_stable_structures::storable::Blob;

use super::UserError;

pub fn get_caller_eth_address() -> Result<Address> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        bail!("Anonymous caller is not allowed to call this method.");
    }

    let user = user::get_by_principal(caller)?;
    Ok(Address::from_slice(&user.eth_address))
}

pub fn auth_guard() -> Result<Address, HttpError> {
    get_caller_eth_address().map_err(HttpError::unauthorized)
}

pub fn principal_to_blob(principal: Principal) -> Result<Blob<29>, UserError> {
    principal.as_slice()[..29]
        .try_into()
        .map_err(|_| UserError::InvalidPrincipal)
}
