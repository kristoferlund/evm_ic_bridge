use alloy::primitives::U256;
use candid::Principal;

use crate::{state::state_types::EthLiquidityPosition, STATE};

pub struct EthPoolStateTransitions {}

impl EthPoolStateTransitions {
    pub fn create_position(user_principal: Principal, amount: U256) -> EthLiquidityPosition {
        STATE.with_borrow_mut(|state| {
            state.total_liquidity = state.total_liquidity + amount;

            let liquidity_position = EthLiquidityPosition {
                amount,
                last_claimed_fee_per_token: state.last_claimed_fee_per_token,
            };

            state
                .eth_liquidity_positions
                .entry(user_principal)
                .or_insert_with(Vec::new)
                .push(liquidity_position.clone());

            liquidity_position
        })
    }
}
