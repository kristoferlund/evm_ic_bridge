use bridge_tests::{
    common::{bridge_update, setup},
    siwe::full_login,
    types::{Event, RpcResult, UserDto},
};
use candid::encode_one;
use ic_agent::Identity;

// Basic first testing if the event_log function is working, jyst checking that
// the response is ok and that the correct number of events are returned.

#[tokio::test]
async fn event_log() {
    let (ic, siwe, bridge) = setup().await;
    let (_, _, _, identity) = full_login(&ic, siwe, bridge, None).await;
    let _: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        identity.sender().unwrap(),
        "user_register_eth_address",
        encode_one(()).unwrap(),
    )
    .await;

    let response: RpcResult<Vec<Event>> = bridge_update(
        &ic,
        bridge,
        identity.sender().unwrap(),
        "event_log",
        encode_one(()).unwrap(),
    )
    .await;

    assert!(response.is_ok());
    assert_eq!(response.unwrap_ok().len(), 3);

    ic.drop().await;
}

//TODO: More / real tests
