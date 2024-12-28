use alloy::primitives::U256;
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, mem::size_of};

const MAX_POSITION_SIZE: u32 = size_of::<U256>() as u32 * 2;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EthLiquidityPosition {
    pub liquidity: U256,
    pub last_claimed_fee_per_token: U256,
}

impl EthLiquidityPosition {
    pub fn new(liquidity: U256, last_claimed_fee_per_token: U256) -> Self {
        Self {
            liquidity,
            last_claimed_fee_per_token,
        }
    }
}

impl Storable for EthLiquidityPosition {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(bincode::serialize(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        bincode::deserialize(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_POSITION_SIZE,
        is_fixed_size: true,
    };
}
