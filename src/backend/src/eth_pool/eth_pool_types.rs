use thiserror::Error;

use crate::user::UserError;

#[derive(Error, Debug)]
pub enum EthPoolError {
    #[error("User error: {0}")]
    UserError(#[from] UserError),
    #[error("Transaction error: {0}")]
    TransactionError(#[from] anyhow::Error),
    #[error("Transport Error: {0}")]
    TransportError(#[from] alloy::transports::TransportError),
}
