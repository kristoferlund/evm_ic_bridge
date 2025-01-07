use alloy::primitives::U256;
use candid::CandidType;
use thiserror::Error;

use crate::{declarations::icrc1_ledger::BlockIndex, user::UserError};

#[derive(Error, Debug)]
pub enum CkPoolError {
    #[error("User error: {0}")]
    UserError(#[from] UserError),
    #[error("Transaction error: {0}")]
    TransactionError(#[from] anyhow::Error),
}

#[derive(Clone, Debug)]
pub struct CkPoolLiquidityPosition {
    pub amount: U256,
    pub last_claimed_fee_per_token: U256,
    pub block_index: BlockIndex,
    pub timestamp: u64,
}

#[derive(CandidType)]
pub struct CkPoolLiquidityPositionDto {
    pub amount: String,
    pub last_claimed_fee_per_token: String,
    pub block_index: String,
    pub timestamp: u64,
}
