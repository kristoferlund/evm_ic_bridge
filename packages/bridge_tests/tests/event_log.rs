use bridge_tests::{
    context::Context,
    types::{Event, RpcResult, UserDto},
};
use candid::encode_one;
use ic_agent::Identity;

// Basic first testing if the event_log function is working, jyst checking that
// the response is ok and that the correct number of events are returned.

#[tokio::test]
async fn event_log() {
    let mut context = Context::new();
    let context = context.setup_default().await;
    let (_, _, identity) = context.full_login(0, None).await;
    let _: RpcResult<UserDto> = context
        .update_call_unwrap(
            context.bridge_canister,
            identity.sender().unwrap(),
            "user_register_eth_address",
            encode_one(()).unwrap(),
        )
        .await;

    let response: RpcResult<Vec<Event>> = context
        .update_call_unwrap(
            context.bridge_canister,
            identity.sender().unwrap(),
            "event_log",
            encode_one(()).unwrap(),
        )
        .await;

    assert!(response.is_ok());
    assert_eq!(response.unwrap_ok().len(), 3);

    context.teardown_default().await;
}

//TODO: More / real tests
