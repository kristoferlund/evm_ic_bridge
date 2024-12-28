use candid::CandidType;
use serde::Deserialize;
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

#[derive(CandidType, Deserialize, Debug, Clone, Default)]
pub struct User {
    pub eth_address: EthAddressBytes,
}

impl User {
    pub fn new() -> Self {
        Self::default()
    }
}
