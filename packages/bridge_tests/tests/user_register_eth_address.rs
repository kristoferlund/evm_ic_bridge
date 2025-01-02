use bridge_tests::{
    common::{bridge_update, setup},
    siwe::{create_basic_identity, full_login},
    types::{RpcResult, UserDto},
};
use candid::{encode_one, Principal};
use ic_agent::Identity;

// Register an ethereum address with the anonymous principal should fail
#[tokio::test]
async fn anon() {
    let (ic, _, bridge) = setup().await;
    let response: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        Principal::anonymous(),
        "user_register_eth_address",
        encode_one(()).unwrap(),
    )
    .await;
    assert!(response.is_err());
    assert!(matches!(response.unwrap_err().code, 401));

    ic.drop().await;
}

// Register an ethereum address with a non anonymous principal that has not
// been created and not logged in using SIWE should fail, user not found
#[tokio::test]
async fn not_created_and_not_logged_in() {
    let (ic, _, bridge) = setup().await;
    let identity = create_basic_identity();
    let response: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        identity.sender().unwrap(),
        "user_register_eth_address",
        encode_one(()).unwrap(),
    )
    .await;
    assert!(response.is_err());
    let response = response.unwrap_err();
    assert!(matches!(response.code, 404));
    assert_eq!(response.details.as_ref().unwrap(), "User not found.");

    ic.drop().await;
}

// Register an ethereum address with a non anonymous principal that *has
// been* created and not logged in using SIWE should fail,
// "No Ethereum address found for caller"
#[tokio::test]
async fn not_logged_in() {
    let (ic, _, bridge) = setup().await;
    let identity = create_basic_identity();
    // Create a user
    let response: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        identity.sender().unwrap(),
        "user_create",
        encode_one(()).unwrap(),
    )
    .await;
    assert!(response.is_ok());
    // Register an Ethereum address without logging in using SIWE first
    let response: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        identity.sender().unwrap(),
        "user_register_eth_address",
        encode_one(()).unwrap(),
    )
    .await;
    assert!(response.is_err());
    let response = response.unwrap_err();
    assert!(matches!(response.code, 404));
    assert_eq!(
        response.details.as_ref().unwrap(),
        "No Ethereum address found for caller."
    );

    ic.drop().await;
}

// Register an ethereum address after logging in using SIWE should succeed
#[tokio::test]
async fn siwe_logged_in() {
    let (ic, siwe, bridge) = setup().await;
    let (_, _, _, identity) = full_login(&ic, siwe, bridge, None).await;
    let response: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        identity.sender().unwrap(),
        "user_register_eth_address",
        encode_one(()).unwrap(),
    )
    .await;
    assert!(response.is_ok());

    ic.drop().await;
}

// Register an ethereum address twice for the same SIWE logged in user should fail
#[tokio::test]
async fn twice_for_same_user() {
    let (ic, siwe, bridge) = setup().await;
    let (_, _, _, identity) = full_login(&ic, siwe, bridge, None).await;

    // First attempt to register an Ethereum address
    let first_response: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        identity.sender().unwrap(),
        "user_register_eth_address",
        encode_one(()).unwrap(),
    )
    .await;
    assert!(first_response.is_ok());

    // Second attempt to register an Ethereum address
    let second_response: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        identity.sender().unwrap(),
        "user_register_eth_address",
        encode_one(()).unwrap(),
    )
    .await;
    assert!(second_response.is_err());
    assert!(matches!(second_response.unwrap_err().code, 409));

    ic.drop().await;
}
