use crate::{
    eth_pool::EthPoolLiquidityPosition,
    user::user_types::{EthAddressBytes, User},
};
use alloy::{primitives::U256, signers::icp::IcpSigner};
use candid::Principal;
use std::collections::HashMap;

#[derive(Default)]
pub struct State {
    // Settings
    pub ecdsa_key_id: String,
    pub siwe_provider_canister: String,
    pub evm_rpc_url: String,
    pub eth_min_confirmations: u64,

    // Runtime
    pub canister_eth_address: Option<EthAddressBytes>,
    pub signer: Option<IcpSigner>,

    pub users: HashMap<Principal, User>,
    pub users_by_eth_address: HashMap<EthAddressBytes, Principal>,

    pub total_liquidity: U256,
    pub last_claimed_fee_per_token: U256,

    pub eth_pool_liquidity_positions: HashMap<Principal, Vec<EthPoolLiquidityPosition>>,
}
