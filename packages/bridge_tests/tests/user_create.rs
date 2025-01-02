use bridge_tests::{
    common::{bridge_update, setup},
    siwe::create_basic_identity,
    types::{RpcResult, UserDto},
};
use candid::{encode_one, Principal};
use ic_agent::Identity;

// Create a new user with the anonymous principal should fail
#[tokio::test]
async fn create_anon() {
    let (ic, _, bridge) = setup().await;
    let response: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        Principal::anonymous(),
        "user_create",
        encode_one(()).unwrap(),
    )
    .await;
    assert!(response.is_err());
    assert!(matches!(response.unwrap_err().code, 401));

    ic.drop().await;
}

// Create a new user with a non anonymous principal should succeed
#[tokio::test]
async fn create() {
    let (ic, _, bridge) = setup().await;
    let identity = create_basic_identity();
    let response: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        identity.sender().unwrap(),
        "user_create",
        encode_one(()).unwrap(),
    )
    .await;
    assert!(response.is_ok());

    ic.drop().await;
}
