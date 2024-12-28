use super::{principal_to_blob, User, UserError};
use crate::{USERS, USER_ETH_ADDRESS_INDEX};
use alloy::primitives::Address;
use candid::Principal;
use ic_stable_structures::storable::Blob;

pub fn create(principal: Principal, eth_address: &Address) -> Result<User, UserError> {
    let principal_bytes = principal_to_blob(principal)?;

    USERS.with_borrow_mut(|users| {
        if users.contains_key(&principal_bytes) {
            return Err(UserError::AlreadyExists);
        }
        let user = User::new(eth_address.into_array());
        users.insert(principal_bytes, user.clone());

        USER_ETH_ADDRESS_INDEX.with_borrow_mut(|index| {
            index.insert(eth_address.into_array(), principal_bytes);
        });

        Ok(user)
    })
}

pub fn get_by_principal_bytes(principal_bytes: &Blob<29>) -> Result<User, UserError> {
    USERS.with_borrow(|users| users.get(principal_bytes).ok_or(UserError::NotFound))
}

pub fn get_by_principal(principal: Principal) -> Result<User, UserError> {
    let principal_bytes = principal_to_blob(principal)?;
    get_by_principal_bytes(&principal_bytes)
}

pub fn get_by_eth_address(eth_address: &Address) -> Result<User, UserError> {
    let eth_address_bytes = eth_address.into_array();
    let principal_bytes = USER_ETH_ADDRESS_INDEX
        .with_borrow(|index| index.get(&eth_address_bytes).ok_or(UserError::NotFound))?;
    get_by_principal_bytes(&principal_bytes)
}

pub fn set_eth_address(principal: Principal, address: Address) -> Result<User, UserError> {
    let principal_bytes = principal_to_blob(principal)?;

    USERS.with_borrow_mut(|users| match get_by_principal_bytes(&principal_bytes) {
        Ok(mut user) => {
            user.eth_address = address.into_array();
            users.insert(principal_bytes, user.clone());
            return Ok(user);
        }
        Err(_) => {
            let user = User::new(address.into_array());
            users.insert(principal_bytes, user.clone());
            Ok(user)
        }
    })
}
