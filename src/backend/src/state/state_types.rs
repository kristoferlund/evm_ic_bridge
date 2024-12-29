use crate::user::user_types::{EthAddressBytes, User};
use alloy::{primitives::U256, signers::icp::IcpSigner};
use candid::Principal;
use std::collections::HashMap;

#[derive(Default)]
pub struct State {
    // Settings
    pub eth_min_confirmations: u64,

    // Runtime
    pub canister_eth_address: Option<EthAddressBytes>,
    pub signer: Option<IcpSigner>,

    pub users: HashMap<Principal, User>,
    pub users_by_eth_address: HashMap<EthAddressBytes, Principal>,

    pub total_liquidity: U256,
    pub last_claimed_fee_per_token: u32,
}

pub struct EthLiquidityPosition {
    pub liquidity: U256,
    pub last_claimed_fee_per_token: U256,
}
