use alloy::{
    network::TransactionBuilder,
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use bridge_tests::{
    anvil::{await_call_and_decode, proxy_one_https_outcall},
    common::{bridge_update, get_eth_pool_address, setup, tick},
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

    proxy_one_https_outcall(&ic, &anvil).await; // Transaction referenced by hash

    let response: RpcResult<EthPoolLiquidityPositionDto> =
        await_call_and_decode(&ic, call_id).await;

    assert!(response.is_err());
    let response = response.unwrap_err();
    assert!(matches!(response.code, 400));
    assert_eq!(
        response.details.as_ref().unwrap(),
        "Transaction error: Transaction not found"
    );

    ic.drop().await;
}

// Only the currently logged in user can create a position related to a transaction
// they sent
#[tokio::test]
async fn tx_wrong_sender() {
    let (ic, siwe, bridge) = setup().await;
    let (anvil, _, _, identity) = full_login_with_eth_registered(&ic, siwe, bridge, None).await;
    let eth_pool_address = get_eth_pool_address(&ic, bridge, &identity).await;
    let provider = ProviderBuilder::new().on_http(anvil.endpoint_url());

    let signer1: PrivateKeySigner = anvil.keys()[1].clone().into();

    let tx = TransactionRequest::default()
        .with_from(signer1.address())
        .with_to(alloy::primitives::Address::parse_checksummed(eth_pool_address, None).unwrap())
        .with_nonce(0)
        .with_value(U256::from(100))
        .with_gas_limit(21_000)
        .with_max_priority_fee_per_gas(1_000_000_000)
        .with_max_fee_per_gas(20_000_000_000);
    let pending_tx = provider.send_transaction(tx).await.unwrap();
    let tx_receipt = pending_tx.get_receipt().await.unwrap();
    let tx_hash = format!("0x{}", hex::encode(tx_receipt.transaction_hash));

    let call_id = ic
        .submit_call(
            bridge,
            identity.sender().unwrap(),
            "eth_pool_create_position",
            encode_one(tx_hash).unwrap(),
        )
        .await
        .unwrap();
    tick(&ic, 2).await;
    proxy_one_https_outcall(&ic, &anvil).await; // Transaction referenced by hash

    let response: RpcResult<EthPoolLiquidityPositionDto> =
        await_call_and_decode(&ic, call_id).await;

    assert!(response.is_err());
    let response = response.unwrap_err();
    assert!(matches!(response.code, 400));
    assert_eq!(
        response.details.as_ref().unwrap(),
        "Transaction error: Transaction not sent by caller"
    );

    ic.drop().await;
}

// Make sure transaction was sent to the canister address
#[tokio::test]
async fn tx_wrong_recipient() {
    let (ic, siwe, bridge) = setup().await;
    let (anvil, _, _, identity) = full_login_with_eth_registered(&ic, siwe, bridge, None).await;
    let provider = ProviderBuilder::new().on_http(anvil.endpoint_url());

    let signer1: PrivateKeySigner = anvil.keys()[1].clone().into();

    let tx = TransactionRequest::default()
        .with_to(signer1.address())
        .with_nonce(0)
        .with_value(U256::from(100))
        .with_gas_limit(21_000)
        .with_max_priority_fee_per_gas(1_000_000_000)
        .with_max_fee_per_gas(20_000_000_000);
    let pending_tx = provider.send_transaction(tx).await.unwrap();
    let tx_receipt = pending_tx.get_receipt().await.unwrap();
    let tx_hash = format!("0x{}", hex::encode(tx_receipt.transaction_hash));

    let call_id = ic
        .submit_call(
            bridge,
            identity.sender().unwrap(),
            "eth_pool_create_position",
            encode_one(tx_hash).unwrap(),
        )
        .await
        .unwrap();
    tick(&ic, 2).await;
    proxy_one_https_outcall(&ic, &anvil).await; // Transaction referenced by hash

    let response: RpcResult<EthPoolLiquidityPositionDto> =
        await_call_and_decode(&ic, call_id).await;

    assert!(response.is_err());
    let response = response.unwrap_err();
    assert!(matches!(response.code, 400));
    assert_eq!(
        response.details.as_ref().unwrap(),
        "Transaction error: Transaction not sent to canister address"
    );

    ic.drop().await;
}

// Transaction needs to have enough confirmations to be accepted
#[tokio::test]
async fn tx_not_enough_confirmations() {
    let (ic, siwe, bridge) = setup().await;
    let (anvil, _, _, identity) = full_login_with_eth_registered(&ic, siwe, bridge, None).await;
    let eth_pool_address = get_eth_pool_address(&ic, bridge, &identity).await;
    let provider = ProviderBuilder::new().on_http(anvil.endpoint_url());

    let tx = TransactionRequest::default()
        .with_to(alloy::primitives::Address::parse_checksummed(eth_pool_address, None).unwrap())
        .with_nonce(0)
        .with_value(U256::from(100))
        .with_gas_limit(21_000)
        .with_max_priority_fee_per_gas(1_000_000_000)
        .with_max_fee_per_gas(20_000_000_000);
    let pending_tx = provider.send_transaction(tx).await.unwrap();
    let tx_receipt = pending_tx.get_receipt().await.unwrap();
    let tx_hash = format!("0x{}", hex::encode(tx_receipt.transaction_hash));

    let call_id = ic
        .submit_call(
            bridge,
            identity.sender().unwrap(),
            "eth_pool_create_position",
            encode_one(tx_hash).unwrap(),
        )
        .await
        .unwrap();
    tick(&ic, 2).await;
    proxy_one_https_outcall(&ic, &anvil).await; // Transaction referenced by hash
    tick(&ic, 5).await;
    proxy_one_https_outcall(&ic, &anvil).await; // Latests block
    let response: RpcResult<EthPoolLiquidityPositionDto> =
        await_call_and_decode(&ic, call_id).await;

    assert!(response.is_err());
    let response = response.unwrap_err();
    assert!(matches!(response.code, 400));
    assert_eq!(
        response.details.as_ref().unwrap(),
        "Transaction error: Transaction has not enough confirmations"
    );

    ic.drop().await;
}
