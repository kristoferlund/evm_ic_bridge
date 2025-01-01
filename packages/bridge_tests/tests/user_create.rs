use bridge_tests::{
    common::{bridge_update, setup},
    siwe::{create_basic_identity, full_login},
    types::{RpcResult, UserDto},
};
use candid::{encode_one, Principal};
use ic_agent::Identity;

// Create a new user with the anonymous principal should fail
#[test]
fn test_user_create_anon() {
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
fn test_user_create() {
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

// Register an ethereum address with the anonymous principal should fail
#[test]
fn test_user_register_eth_address_anon() {
    let (ic, _, bridge) = setup();
    let response: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        Principal::anonymous(),
        "user_register_eth_address",
        encode_one(()).unwrap(),
    );
    assert!(response.is_err());
    assert!(matches!(response.unwrap_err().code, 401));
}

// Register an ethereum address with a non anonymous principal that has not
// been created and not logged in using SIWE should fail, user not found
#[test]
fn test_user_register_eth_address_not_created_and_not_logged_in() {
    let (ic, _, bridge) = setup();
    let identity = create_basic_identity();
    let response: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        identity.sender().unwrap(),
        "user_register_eth_address",
        encode_one(()).unwrap(),
    );
    assert!(response.is_err());
    let response = response.unwrap_err();
    assert!(matches!(response.code, 404));
    assert_eq!(response.details.as_ref().unwrap(), "User not found.");
}

// Register an ethereum address with a non anonymous principal that *has
// been* created and not logged in using SIWE should fail,
// "No Ethereum address found for caller"
#[test]
fn test_user_register_eth_address_not_logged_in() {
    let (ic, _, bridge) = setup();
    let identity = create_basic_identity();
    // Create a user
    let response: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        identity.sender().unwrap(),
        "user_create",
        encode_one(()).unwrap(),
    );
    assert!(response.is_ok());
    // Register an Ethereum address without logging in using SIWE first
    let response: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        identity.sender().unwrap(),
        "user_register_eth_address",
        encode_one(()).unwrap(),
    );
    assert!(response.is_err());
    let response = response.unwrap_err();
    assert!(matches!(response.code, 404));
    assert_eq!(
        response.details.as_ref().unwrap(),
        "No Ethereum address found for caller."
    );
}

// Register an ethereum address after logging in using SIWE should succeed
#[test]
fn test_user_register_eth_address_siwe_logged_in() {
    let (ic, siwe, bridge) = setup();
    let (_, _, identity) = full_login(&ic, siwe, bridge, None);
    let response: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        identity.sender().unwrap(),
        "user_register_eth_address",
        encode_one(()).unwrap(),
    );
    assert!(response.is_ok());
}

// Register an ethereum address twice for the same SIWE logged in user should fail
#[test]
fn test_user_register_eth_address_twice_for_same_user() {
    let (ic, siwe, bridge) = setup();
    let (_, _, identity) = full_login(&ic, siwe, bridge, None);

    // First attempt to register an Ethereum address
    let first_response: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        identity.sender().unwrap(),
        "user_register_eth_address",
        encode_one(()).unwrap(),
    );
    assert!(first_response.is_ok());

    // Second attempt to register an Ethereum address
    let second_response: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        identity.sender().unwrap(),
        "user_register_eth_address",
        encode_one(()).unwrap(),
    );
    assert!(second_response.is_err());
    assert!(matches!(second_response.unwrap_err().code, 409));
}
