use std::collections::BTreeMap;

use alloy::{
    primitives::{Address, U256},
    signers::icp::IcpSigner,
};
use candid::Principal;

use crate::user::user_types::User;

#[derive(Default)]
pub struct State {
    // Settings
    pub eth_min_confirmations: u64,

    // Runtime
    pub signer: Option<IcpSigner>,
    pub canister_eth_address: Option<Address>,

    pub users: BTreeMap<Principal, User>,

    pub total_liquidity: U256,
    pub last_claimed_fee_per_token: u32,
}
