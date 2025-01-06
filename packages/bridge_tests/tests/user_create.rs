use bridge_tests::{
    common::create_basic_identity,
    context::Context,
    types::{RpcResult, UserDto},
};
use candid::{encode_one, Principal};
use ic_agent::Identity;

// Create a new user with the anonymous principal should fail
#[tokio::test]
async fn create_anon() {
    let mut context = Context::new();
    let context = context.setup_default().await;
    context.setup_default().await;
    let response: RpcResult<UserDto> = context
        .update_call_unwrap(
            context.bridge_canister,
            Principal::anonymous(),
            "user_create",
            encode_one(()).unwrap(),
        )
        .await;
    assert!(response.is_err());
    assert!(matches!(response.unwrap_err().code, 401));

    context.teardown_default().await;
}

// Create a new user with a non anonymous principal should succeed
#[tokio::test]
async fn create() {
    let mut context = Context::new();
    let context = context.setup_default().await;
    let identity = create_basic_identity();
    let response: RpcResult<UserDto> = context
        .update_call_unwrap(
            context.bridge_canister,
            identity.sender().unwrap(),
            "user_create",
            encode_one(()).unwrap(),
        )
        .await;
    assert!(response.is_ok());

    context.teardown_default().await;
}
