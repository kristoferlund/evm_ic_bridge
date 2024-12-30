use candid::Principal;
use serde_bytes::ByteBuf;
use thiserror::Error;

use crate::declarations::ic_siwe_provider::{ic_siwe_provider, GetAddressResponse};
use alloy::primitives::Address;

#[derive(Error, Debug)]
pub enum GetAuthenticatedEthAddressError {
    #[error("Anonymous caller is not allowed to call this method.")]
    AnonymousCaller,
    #[error("Invalid SIWE provider canister ID.")]
    InvalidSiweProviderCanisterId,
    #[error("SIWE canister returned an error: {0}")]
    SiweCanisterError(String),
    #[error("SIWE canister returned an invalid address.")]
    InvalidAddress,
}

pub async fn get_eth_address(
    principal: Principal,
) -> Result<Option<Address>, GetAuthenticatedEthAddressError> {
    let response = ic_siwe_provider
        .get_address(ByteBuf::from(principal.as_slice()))
        .await
        .map_err(|e| {
            GetAuthenticatedEthAddressError::SiweCanisterError(format!(
                "Code: {:?}, message: {}",
                e.0, e.1
            ))
        })?;

    match response.0 {
        GetAddressResponse::Ok(address) => Ok(Some(
            Address::parse_checksummed(&address, None)
                .map_err(|_| GetAuthenticatedEthAddressError::InvalidAddress)?,
        )),
        GetAddressResponse::Err(_) => Ok(None),
    }
}
