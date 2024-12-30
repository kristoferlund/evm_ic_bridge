use candid::{CandidType, Principal};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    NotFound,
    #[error("User already exists")]
    AlreadyExists,
    #[error("Invalid user principal")]
    InvalidPrincipal,
    #[error("User has no Ethereum address")]
    NoEthAddress,
}

pub type EthAddressBytes = [u8; 20];

#[derive(Clone)]
pub struct User {
    pub principal: Principal,
    pub eth_address: Option<EthAddressBytes>,
}

impl User {
    pub fn new(principal: Principal) -> Self {
        Self {
            principal,
            eth_address: None,
        }
    }
}

#[derive(CandidType)]
pub struct UserDto {
    pub principal: Principal,
    pub eth_address: Option<String>,
}
