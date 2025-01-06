use bridge_tests::{
    common::create_basic_identity,
    context::Context,
    types::{RpcResult, UserDto},
};
use candid::{encode_one, Principal};
use ic_agent::Identity;

// Register an ethereum address with the anonymous principal should fail
#[tokio::test]
async fn anon() {
    let mut context = Context::new();
    let context = context.setup_default().await;
    let response: RpcResult<UserDto> = context
        .update_call_unwrap(
            context.bridge_canister,
            Principal::anonymous(),
            "user_register_eth_address",
            encode_one(()).unwrap(),
        )
        .await;
    assert!(response.is_err());
    assert!(matches!(response.unwrap_err().code, 401));
    context.teardown_default().await;
}

// Register an ethereum address with a non anonymous principal that has not
// been created and not logged in using SIWE should fail, user not found
#[tokio::test]
async fn not_created_and_not_logged_in() {
    let mut context = Context::new();
    let context = context.setup_default().await;
    let identity = create_basic_identity();
    let response: RpcResult<UserDto> = context
        .update_call_unwrap(
            context.bridge_canister,
            identity.sender().unwrap(),
            "user_register_eth_address",
            encode_one(()).unwrap(),
        )
        .await;
    assert!(response.is_err());
    let response = response.unwrap_err();
    assert!(matches!(response.code, 404));
    assert_eq!(response.details.as_ref().unwrap(), "User not found.");

    context.teardown_default().await;
}

// Register an ethereum address with a non anonymous principal that *has
// been* created and not logged in using SIWE should fail,
// "No Ethereum address found for caller"
#[tokio::test]
async fn not_logged_in() {
    let mut context = Context::new();
    let context = context.setup_default().await;
    let identity = create_basic_identity();
    // Create a user
    let response: RpcResult<UserDto> = context
        .update_call_unwrap(
            context.bridge_canister,
            identity.sender().unwrap(),
            "user_create",
            encode_one(()).unwrap(),
        )
        .await;
    assert!(response.is_ok());
    // Register an Ethereum address without logging in using SIWE first
    let response: RpcResult<UserDto> = context
        .update_call_unwrap(
            context.bridge_canister,
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

    context.teardown_default().await;
}

// Register an ethereum address after logging in using SIWE should succeed
#[tokio::test]
async fn siwe_logged_in() {
    let mut context = Context::new();
    let context = context.setup_default().await;
    let (_, _, identity) = context.full_login(0, None).await;
    let response: RpcResult<UserDto> = context
        .update_call_unwrap(
            context.bridge_canister,
            identity.sender().unwrap(),
            "user_register_eth_address",
            encode_one(()).unwrap(),
        )
        .await;
    assert!(response.is_ok());

    context.teardown_default().await;
}

// Register an ethereum address twice for the same SIWE logged in user should fail
#[tokio::test]
async fn twice_for_same_user() {
    let mut context = Context::new();
    let context = context.setup_default().await;
    let (_, _, identity) = context.full_login(0, None).await;

    // First attempt to register an Ethereum address
    let first_response: RpcResult<UserDto> = context
        .update_call_unwrap(
            context.bridge_canister,
            identity.sender().unwrap(),
            "user_register_eth_address",
            encode_one(()).unwrap(),
        )
        .await;
    assert!(first_response.is_ok());

    // Second attempt to register an Ethereum address
    let second_response: RpcResult<UserDto> = context
        .update_call_unwrap(
            context.bridge_canister,
            identity.sender().unwrap(),
            "user_register_eth_address",
            encode_one(()).unwrap(),
        )
        .await;
    assert!(second_response.is_err());
    assert!(matches!(second_response.unwrap_err().code, 409));

    context.teardown_default().await;
}
