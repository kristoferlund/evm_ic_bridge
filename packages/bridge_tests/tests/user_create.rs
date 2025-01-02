use bridge_tests::{
    common::{bridge_update, setup},
    siwe::create_basic_identity,
    types::{RpcResult, UserDto},
};
use candid::{encode_one, Principal};
use ic_agent::Identity;

// Create a new user with the anonymous principal should fail
#[test]
fn create_anon() {
    let (ic, _, bridge) = setup();
    let response: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        Principal::anonymous(),
        "user_create",
        encode_one(()).unwrap(),
    );
    assert!(response.is_err());
    assert!(matches!(response.unwrap_err().code, 401));
}

// Create a new user with a non anonymous principal should succeed
#[test]
fn create() {
    let (ic, _, bridge) = setup();
    let identity = create_basic_identity();
    let response: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        identity.sender().unwrap(),
        "user_create",
        encode_one(()).unwrap(),
    );
    assert!(response.is_ok());
}
