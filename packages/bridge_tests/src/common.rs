use candid::Principal;
use ic_agent::identity::{
    BasicIdentity, DelegatedIdentity, Delegation as AgentDelegation,
    SignedDelegation as AgentSignedDelegation,
};
use ic_agent::Identity;
use ic_siwe::login::LoginDetails;
use pocket_ic::common::rest::CanisterHttpHeader;
use rand::Rng;
use ureq::Response;

pub const BRIDGE_ENGINE_WASM: &str = "../../target/wasm32-unknown-unknown/release/bridge.wasm.gz";
pub const IC_SIWE_WASM: &str = "../ic_siwe_provider/ic_siwe_provider.wasm.gz";
pub const EVM_RPC_WASM: &str = "../evm_rpc/evm_rpc.wasm.gz";

pub fn create_basic_identity() -> BasicIdentity {
    let mut ed25519_seed = [0u8; 32];
    rand::thread_rng().fill(&mut ed25519_seed);
    let ed25519_keypair =
        ring::signature::Ed25519KeyPair::from_seed_unchecked(&ed25519_seed).unwrap();
    BasicIdentity::from_key_pair(ed25519_keypair)
}

pub fn create_delegated_identity(
    identity: BasicIdentity,
    login_response: &LoginDetails,
    signature: Vec<u8>,
    targets: Option<Vec<Principal>>,
) -> DelegatedIdentity {
    // Create a delegated identity
    let signed_delegation = AgentSignedDelegation {
        delegation: AgentDelegation {
            pubkey: identity.public_key().unwrap(),
            expiration: login_response.expiration,
            targets,
        },
        signature,
    };
    DelegatedIdentity::new(
        login_response.user_canister_pubkey.to_vec(),
        Box::new(identity),
        vec![signed_delegation],
    )
}

#[macro_export]
macro_rules! assert_starts_with {
    ($left:expr, $right:expr $(,)?) => {{
        let left_val = $left;
        let right_val = $right;
        if !left_val.starts_with(&right_val) {
            panic!(
                "assertion failed: `(left starts with right)`\n  left: `{}`,\n right: `{}`",
                left_val, right_val
            );
        }
    }};
}

pub fn get_response_headers(response: &Response) -> Vec<CanisterHttpHeader> {
    let mut headers = vec![];
    response.headers_names().into_iter().for_each(|name| {
        let value = response.header(&name).unwrap().to_string();
        headers.push(CanisterHttpHeader { name, value });
    });
    headers
}
