use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, CandidType, Clone)]
pub struct RpcError {
    pub code: u16,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, CandidType, Clone)]
pub enum RpcResult<T> {
    Ok(T),
    Err(RpcError),
}

impl<T> RpcResult<T> {
    pub fn is_ok(&self) -> bool {
        matches!(self, RpcResult::Ok(_))
    }

    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    pub fn unwrap_ok(&self) -> &T {
        match self {
            RpcResult::Ok(value) => value,
            RpcResult::Err(_) => panic!("called `RpcResult::unwrap_ok()` on an `Err` value"),
        }
    }

    pub fn unwrap_err(&self) -> &RpcError {
        match self {
            RpcResult::Ok(_) => panic!("called `RpcResult::unwrap_err()` on an `Ok` value"),
            RpcResult::Err(value) => value,
        }
    }
}

#[derive(CandidType, Deserialize, Debug)]
pub struct UserDto {
    pub principal: Principal,
    pub eth_address: Option<String>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SiweUser {
    pub eth_address: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Event {
    Init(InitArgs),
    PostUpgrade(InitArgs),
    CreateUser(Principal),
    RegisterEthAddress(Principal, EthAddressBytes),
    EThPoolCreatePosition(Principal, String),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct InitArgs {
    pub ecdsa_key_id: String,
    pub siwe_provider_canister: String,
    pub evm_rpc_url: String,
    pub eth_min_confirmations: u64,
}

pub type EthAddressBytes = [u8; 20];

#[derive(CandidType, Deserialize, Debug)]
pub struct EthPoolLiquidityPositionDto {
    pub amount: String,
    pub last_claimed_fee_per_token: String,
    pub tx_hash: String,
    pub timestamp: u64,
}

#[derive(CandidType, Debug, Clone, PartialEq, Deserialize)]
pub enum RuntimeFeature {
    IncludeUriInSeed,
    DisableEthToPrincipalMapping,
    DisablePrincipalToEthMapping,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct SettingsInput {
    pub domain: String,
    pub uri: String,
    pub salt: String,
    pub chain_id: Option<u32>,
    pub scheme: Option<String>,
    pub statement: Option<String>,
    pub sign_in_expires_in: Option<u64>,
    pub session_expires_in: Option<u64>,
    pub targets: Option<Vec<String>>,
    pub runtime_features: Option<Vec<RuntimeFeature>>,
}

#[derive(Serialize, Deserialize, CandidType)]
pub struct BridgeSettings {
    pub ecdsa_key_id: String,
    pub siwe_provider_canister: String,
    pub evm_rpc_url: String,
    pub eth_min_confirmations: u64,
}

#[derive(Serialize, Deserialize, CandidType)]
pub struct EvmRpcSettings {
    #[serde(rename = "nodesInSubnet")]
    pub nodes_in_subnet: u32,
}

#[derive(CandidType, Deserialize)]
pub struct PrepareLoginOkResponse {
    pub siwe_message: String,
    pub nonce: String,
}

pub type EthTxHash = String;
