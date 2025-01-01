use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, CandidType, Clone)]
pub struct InitArgs {
    pub ecdsa_key_id: String,
    pub siwe_provider_canister: String,
    pub evm_rpc_url: String,
    pub eth_min_confirmations: u64,
}
