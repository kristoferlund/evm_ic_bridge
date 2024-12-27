use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User already exists")]
    AlreadyExists,
    #[error("Invalid user principal")]
    InvalidPrincipal,
    #[error("User not found")]
    NotFound,
}

type EthAddressBytes = [u8; 20];

const MAX_USER_SIZE: u32 = 20;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct User {
    pub eth_address: EthAddressBytes,
}

impl User {
    pub fn new(eth_address: EthAddressBytes) -> Self {
        Self { eth_address }
    }
}

impl Storable for User {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_USER_SIZE,
        is_fixed_size: true,
    };
}
