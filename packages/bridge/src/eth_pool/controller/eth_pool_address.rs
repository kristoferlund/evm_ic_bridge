use alloy::primitives::Address;
use ic_cdk::query;

use crate::{http_error::HttpError, STATE};

#[query]
fn eth_pool_address() -> Result<String, HttpError> {
    STATE.with_borrow(|state| {
        let eth_pool_address = state
            .eth_pool_address
            .ok_or(HttpError::not_found("ETH eth_pool_address not initialized"))?;

        Ok(Address::from_slice(&eth_pool_address).to_checksum(None))
    })
}
