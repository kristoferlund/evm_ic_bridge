use crate::{user::user_types::EthTxHashBytes, STATE};
use alloy::primitives::U256;
use candid::Principal;

use super::EthPoolLiquidityPosition;

pub struct EthPoolStateTransitions {}

impl EthPoolStateTransitions {
    pub fn create_position(
        user_principal: Principal,
        amount: U256,
        tx_hash: EthTxHashBytes,
        timestamp: u64,
    ) -> EthPoolLiquidityPosition {
        STATE.with_borrow_mut(|state| {
            state.total_liquidity += amount;

            let liquidity_position = EthPoolLiquidityPosition {
                amount,
                last_claimed_fee_per_token: state.last_claimed_fee_per_token,
                tx_hash,
                timestamp,
            };

            state
                .eth_pool_liquidity_positions
                .entry(user_principal)
                .or_default()
                .push(liquidity_position.clone());

            liquidity_position
        })
    }
}
