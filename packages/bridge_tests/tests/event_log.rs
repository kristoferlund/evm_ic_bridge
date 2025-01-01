use bridge_tests::{
    common::{bridge_update, setup},
    siwe::full_login,
    types::{Event, RpcResult, UserDto},
};
use candid::encode_one;
use ic_agent::Identity;

// Basic first testing if the event_log function is working, jyst checking that
// the response is ok and that the correct number of events are returned.
#[test]
fn test_event_log() {
    let (ic, siwe, bridge) = setup();
    let (_, _, identity) = full_login(&ic, siwe, bridge, None);
    let _: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        identity.sender().unwrap(),
        "user_register_eth_address",
        encode_one(()).unwrap(),
    );

    let response: RpcResult<Vec<Event>> = bridge_update(
        &ic,
        bridge,
        identity.sender().unwrap(),
        "event_log",
        encode_one(()).unwrap(),
    );

    assert!(response.is_ok());
    assert_eq!(response.unwrap_ok().len(), 3);
}

//TODO: More / real tests
