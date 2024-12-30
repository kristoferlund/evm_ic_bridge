use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, CandidType, Clone)]
pub struct InitArgs {
    pub eth_min_confirmations: u64,
}
