use alloy::primitives::U256;
use candid::CandidType;
use thiserror::Error;

use crate::user::{user_types::EthTxHashBytes, UserError};

#[derive(Error, Debug)]
pub enum EthPoolError {
    #[error("User error: {0}")]
    UserError(#[from] UserError),
    #[error("Transaction error: {0}")]
    TransactionError(#[from] anyhow::Error),
    #[error("Transport Error: {0}")]
    TransportError(#[from] alloy::transports::TransportError),
}

#[derive(Clone, Debug)]
pub struct EthPoolLiquidityPosition {
    pub amount: U256,
    pub last_claimed_fee_per_token: U256,
    pub tx_hash: EthTxHashBytes,
    pub timestamp: u64,
}

#[derive(CandidType)]
pub struct EthPoolLiquidityPositionDto {
    pub amount: String,
    pub last_claimed_fee_per_token: String,
    pub tx_hash: String,
    pub timestamp: u64,
}
