use crate::{
    common::{query, update},
    types::UserDto,
};
use alloy::{
    node_bindings::{Anvil, AnvilInstance},
    signers::{local::PrivateKeySigner, Signer},
};
use candid::{encode_args, encode_one, CandidType, Principal};
use ic_agent::{
    identity::{
        BasicIdentity, DelegatedIdentity, Delegation as AgentDelegation,
        SignedDelegation as AgentSignedDelegation,
    },
    Identity,
};
use ic_siwe::{delegation::SignedDelegation, login::LoginDetails};
use pocket_ic::nonblocking::PocketIc;
use rand::Rng;
use serde::Deserialize;

pub fn create_signer(anvil: &AnvilInstance) -> PrivateKeySigner {
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    signer
}

#[derive(CandidType, Deserialize)]
struct PrepareLoginOkResponse {
    siwe_message: String,
    nonce: String,
}

pub async fn prepare_login_and_sign_message(
    ic: &PocketIc,
    ic_siwe_provider_canister: Principal,
    signer: &PrivateKeySigner,
) -> (String, String, String) {
    let args = encode_one(signer.address().to_checksum(None)).unwrap();
    let response: PrepareLoginOkResponse = update(
        ic,
        ic_siwe_provider_canister,
        Principal::anonymous(),
        "siwe_prepare_login",
        args,
    )
    .await
    .unwrap();

    let signature = signer
        .sign_message(response.siwe_message.as_bytes())
        .await
        .unwrap();

    (
        format!("0x{}", hex::encode(signature.as_bytes())),
        response.siwe_message,
        response.nonce,
    )
}

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

pub async fn full_login(
    ic: &PocketIc,
    ic_siwe_provider_canister: Principal,
    bridge_canister: Principal,
    targets: Option<Vec<Principal>>,
) -> (AnvilInstance, PrivateKeySigner, String, DelegatedIdentity) {
    let anvil = Anvil::new()
        .block_time(1)
        .try_spawn()
        .expect("Failed to spawn Anvil instance. Ensure `anvil` is available in $PATH.");

    let signer = create_signer(&anvil);
    let address = signer.address().to_checksum(None);
    let (signature, _, nonce) =
        prepare_login_and_sign_message(ic, ic_siwe_provider_canister, &signer).await;

    // Create a session identity
    let session_identity = create_basic_identity();
    let session_pubkey = session_identity.public_key().unwrap();

    // Login
    let login_args = encode_args((
        signature,
        address.clone(),
        session_pubkey.clone(),
        nonce.clone(),
    ))
    .unwrap();
    let login_response: LoginDetails = update(
        ic,
        ic_siwe_provider_canister,
        Principal::anonymous(),
        "siwe_login",
        login_args,
    )
    .await
    .unwrap();

    // Get the delegation
    let get_delegation_args = encode_args((
        address.clone(),
        session_pubkey.clone(),
        login_response.expiration,
    ))
    .unwrap();

    let get_delegation_response: SignedDelegation = query(
        ic,
        ic_siwe_provider_canister,
        Principal::anonymous(),
        "siwe_get_delegation",
        get_delegation_args,
    )
    .await
    .unwrap();

    // Create a delegated identity
    let delegated_identity = create_delegated_identity(
        session_identity,
        &login_response,
        get_delegation_response.signature.as_ref().to_vec(),
        targets,
    );

    // Create a user in the bridge canister
    let _: UserDto = update(
        ic,
        bridge_canister,
        delegated_identity.sender().unwrap(),
        "user_create",
        encode_one(()).unwrap(),
    )
    .await
    .unwrap();

    (anvil, signer, address, delegated_identity)
}

pub async fn full_login_with_eth_registered(
    ic: &PocketIc,
    ic_siwe_provider_canister: Principal,
    bridge_canister: Principal,
    targets: Option<Vec<Principal>>,
) -> (AnvilInstance, PrivateKeySigner, String, DelegatedIdentity) {
    let (anvil, signer, address, delegated_identity) =
        full_login(ic, ic_siwe_provider_canister, bridge_canister, targets).await;

    let _: UserDto = update(
        ic,
        bridge_canister,
        delegated_identity.sender().unwrap(),
        "user_register_eth_address",
        encode_one(()).unwrap(),
    )
    .await
    .unwrap();

    (anvil, signer, address, delegated_identity)
}
