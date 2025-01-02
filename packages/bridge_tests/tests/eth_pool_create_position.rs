use bridge_tests::{
    anvil::{anvil_request, await_call_and_decode},
    common::{bridge_update, setup, tick},
    siwe::{create_basic_identity, full_login_with_eth_registered},
    types::{EthPoolLiquidityPositionDto, RpcResult},
};
use candid::{encode_one, Principal};

use ic_agent::Identity;

const TX_NOT_FOUND: &str = "0x63899cc622fc27128fab8b5b36aecfe963119432acaf629a5f6bb38487a6a528";
const INVALID_HASH: &str = "0x1234";

// Anon call should fail
#[tokio::test]
async fn anon() {
    let (ic, _, bridge) = setup().await;
    let response: RpcResult<EthPoolLiquidityPositionDto> = bridge_update(
        &ic,
        bridge,
        Principal::anonymous(),
        "eth_pool_create_position",
        encode_one(TX_NOT_FOUND).unwrap(),
    )
    .await;

    dbg!(&response);

    assert!(response.is_err());
    assert!(matches!(response.unwrap_err().code, 401));

    ic.drop().await;
}

// Non SIWE identity should fail
#[tokio::test]
async fn non_siwe_id() {
    let (ic, _, bridge) = setup().await;
    let identity = create_basic_identity();
    let response: RpcResult<EthPoolLiquidityPositionDto> = bridge_update(
        &ic,
        bridge,
        identity.sender().unwrap(),
        "eth_pool_create_position",
        encode_one(TX_NOT_FOUND).unwrap(),
    )
    .await;
    assert!(response.is_err());
    assert!(matches!(response.unwrap_err().code, 401));

    ic.drop().await;
}

// Creating position with invalid hash should fail
#[tokio::test]
async fn invalid_hash() {
    let (ic, siwe, bridge) = setup().await;
    let (_, _, _, identity) = full_login_with_eth_registered(&ic, siwe, bridge, None).await;
    let response: RpcResult<EthPoolLiquidityPositionDto> = bridge_update(
        &ic,
        bridge,
        identity.sender().unwrap(),
        "eth_pool_create_position",
        encode_one(INVALID_HASH).unwrap(),
    )
    .await;
    assert!(response.is_err());
    let response = response.unwrap_err();
    assert!(matches!(response.code, 400));
    assert_eq!(response.details.as_ref().unwrap(), "Invalid hex string");

    ic.drop().await;
}

// Creating position with a transaction that does not exist should fail
#[tokio::test]
async fn tx_not_found() {
    let (ic, siwe, bridge) = setup().await;
    let (anvil, _, _, identity) = full_login_with_eth_registered(&ic, siwe, bridge, None).await;

    let call_id = ic
        .submit_call(
            bridge,
            identity.sender().unwrap(),
            "eth_pool_create_position",
            encode_one(TX_NOT_FOUND).unwrap(),
        )
        .await
        .unwrap();

    tick(&ic, 2).await;

    let canister_http_requests = ic.get_canister_http().await;
    assert_eq!(canister_http_requests.len(), 1);
    let canister_http_request = &canister_http_requests[0];
    let canister_http_response = anvil_request(canister_http_request, anvil);

    let response: RpcResult<EthPoolLiquidityPositionDto> =
        await_call_and_decode(&ic, canister_http_response, call_id).await;

    assert!(response.is_err());
    let response = response.unwrap_err();
    assert!(matches!(response.code, 400));
    assert_eq!(
        response.details.as_ref().unwrap(),
        "Transaction error: Transaction not found"
    );

    ic.drop().await;
}

//#[test]
// fn tx_not_mined() {
//     let (ic, siwe, bridge) = setup();
//     let (wallet, _, identity) = full_login_with_eth_registered(&ic, siwe, bridge, None);
//
//     let response: RpcResult<String> = bridge_query(
//         &ic,
//         bridge,
//         identity.sender().unwrap(),
//         "eth_pool_address",
//         encode_one(()).unwrap(),
//     );
//     assert!(response.is_ok());
//     let eth_pool_address = response.unwrap_ok();
//     let eth_pool_address = Address::from_str(eth_pool_address.as_str()).unwrap();
//
//     // Build the transaction
//     let tx = TransactionRequest::new()
//         .to(eth_pool_address)
//         .value(U256::exp10(18) / 10)
//         .from(wallet.address());
//
//     wallet.sign_transaction_sync(tx).unwrap();
//     let call_id = ic
//         .submit_call(
//             bridge,
//             identity.sender().unwrap(),
//             "eth_pool_create_position",
//             encode_one(TX_NOT_FOUND).unwrap(),
//         )
//         .unwrap();
//
//     tick(&ic, 2);
//
//     let canister_http_requests = ic.get_canister_http();
//     assert_eq!(canister_http_requests.len(), 1);
//     let canister_http_request = &canister_http_requests[0];
//     let canister_http_response = anvil_request(canister_http_request);
//
//     let response: RpcResult<EthPoolLiquidityPositionDto> =
//         await_call_and_decode(&ic, canister_http_response, call_id);
//
//     assert!(response.is_err());
//     let response = response.unwrap_err();
//     assert!(matches!(response.code, 400));
//     assert_eq!(
//         response.details.as_ref().unwrap(),
//         "Transaction error: Transaction not found"
//     );
// }
