use alloy::{
    network::TransactionBuilder,
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use bridge_tests::{
    common::create_basic_identity,
    context::Context,
    types::{EthPoolLiquidityPositionDto, RpcResult},
};
use candid::{encode_one, Principal};
use ic_agent::Identity;

const TX_NOT_FOUND: &str = "0x63899cc622fc27128fab8b5b36aecfe963119432acaf629a5f6bb38487a6a528";
const INVALID_HASH: &str = "0x1234";

// Anon call should fail
#[tokio::test]
async fn anon() {
    let mut context = Context::new();
    let context = context.setup_default().await;
    let response: RpcResult<EthPoolLiquidityPositionDto> = context
        .update_call_unwrap(
            context.bridge_canister,
            Principal::anonymous(),
            "eth_pool_create_position",
            encode_one(TX_NOT_FOUND).unwrap(),
        )
        .await;

    assert!(response.is_err());
    assert!(matches!(response.unwrap_err().code, 401));

    context.teardown_default().await;
}

// Non SIWE identity should fail
#[tokio::test]
async fn non_siwe_id() {
    let mut context = Context::new();
    let context = context.setup_default().await;
    let identity = create_basic_identity();
    let response: RpcResult<EthPoolLiquidityPositionDto> = context
        .update_call_unwrap(
            context.bridge_canister,
            identity.sender().unwrap(),
            "eth_pool_create_position",
            encode_one(TX_NOT_FOUND).unwrap(),
        )
        .await;
    assert!(response.is_err());
    assert!(matches!(response.unwrap_err().code, 401));

    context.teardown_default().await;
}

// Creating position with invalid hash should fail
#[tokio::test]
async fn invalid_hash() {
    let mut context = Context::new();
    let context = context.setup_default().await;
    let (_, _, identity) = context.full_login_with_eth_registered(0, None).await;
    let response: RpcResult<EthPoolLiquidityPositionDto> = context
        .update_call_unwrap(
            context.bridge_canister,
            identity.sender().unwrap(),
            "eth_pool_create_position",
            encode_one(INVALID_HASH).unwrap(),
        )
        .await;
    assert!(response.is_err());
    let response = response.unwrap_err();
    assert!(matches!(response.code, 400));
    assert_eq!(response.details.as_ref().unwrap(), "Invalid hex string");

    context.teardown_default().await;
}

// Creating position with a transaction that does not exist should fail
#[tokio::test]
async fn tx_not_found() {
    let mut context = Context::new();
    let context = context.setup_default().await;
    let (_, _, identity) = context.full_login_with_eth_registered(0, None).await;

    let ic = context.ic.as_ref().unwrap();
    let call_id = ic
        .submit_call(
            context.bridge_canister,
            identity.sender().unwrap(),
            "eth_pool_create_position",
            encode_one(TX_NOT_FOUND).unwrap(),
        )
        .await
        .unwrap();

    context.tick(2).await;

    context.proxy_one_https_outcall_to_anvil().await; // Transaction referenced by hash

    let response: RpcResult<EthPoolLiquidityPositionDto> =
        context.await_call_and_decode(call_id).await;

    assert!(response.is_err());
    let response = response.unwrap_err();
    assert!(matches!(response.code, 400));
    assert_eq!(
        response.details.as_ref().unwrap(),
        "Transaction error: Transaction not found"
    );

    context.teardown_default().await;
}

// Only the currently logged in user can create a position related to a transaction
// they sent
#[tokio::test]
async fn tx_wrong_sender() {
    let mut context = Context::new();
    let context = context.setup_default().await;
    let (_, _, identity) = context.full_login_with_eth_registered(0, None).await;

    // Make transfer from an account that is not the currently logged in user
    let tx_hash = context.send_eth_to_pool_address(1, 100).await;

    // Create a position for a transaction that was not sent by the currently logged in user
    let ic = context.ic.as_ref().unwrap();
    let call_id = ic
        .submit_call(
            context.bridge_canister,
            identity.sender().unwrap(),
            "eth_pool_create_position",
            encode_one(tx_hash).unwrap(),
        )
        .await
        .unwrap();
    context.tick(2).await;
    context.proxy_one_https_outcall_to_anvil().await; // Transaction referenced by hash

    let response: RpcResult<EthPoolLiquidityPositionDto> =
        context.await_call_and_decode(call_id).await;

    assert!(response.is_err());
    let response = response.unwrap_err();
    assert!(matches!(response.code, 400));
    assert_eq!(
        response.details.as_ref().unwrap(),
        "Transaction error: Transaction not sent by caller"
    );

    context.teardown_default().await;
}

// Make sure transaction was sent to the canister address
#[tokio::test]
async fn tx_wrong_recipient() {
    let mut context = Context::new();
    let context = context.setup_default().await;
    let (_, _, identity) = context.full_login_with_eth_registered(0, None).await;
    let anvil = context.anvil.as_ref().unwrap();
    let provider = ProviderBuilder::new().on_http(anvil.endpoint_url());

    let signer1: PrivateKeySigner = anvil.keys()[1].clone().into();

    // Send to signer1 instead of the eth pool address
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

    // Attempt to create a position for a transaction that was not sent to the eth pool address
    let ic = context.ic.as_ref().unwrap();
    let call_id = ic
        .submit_call(
            context.bridge_canister,
            identity.sender().unwrap(),
            "eth_pool_create_position",
            encode_one(tx_hash).unwrap(),
        )
        .await
        .unwrap();
    context.tick(2).await;
    context.proxy_one_https_outcall_to_anvil().await; // Transaction referenced by hash

    let response: RpcResult<EthPoolLiquidityPositionDto> =
        context.await_call_and_decode(call_id).await;

    assert!(response.is_err());
    let response = response.unwrap_err();
    assert!(matches!(response.code, 400));
    assert_eq!(
        response.details.as_ref().unwrap(),
        "Transaction error: Transaction not sent to canister address"
    );

    context.teardown_default().await;
}

// Transaction needs to have enough confirmations to be accepted
#[tokio::test]
async fn tx_not_enough_confirmations() {
    let mut context = Context::new();
    let context = context.setup_default().await;
    let (_, _, identity) = context.full_login_with_eth_registered(0, None).await;

    // Send some ETH to the eth pool address
    let tx_hash = context.send_eth_to_pool_address(0, 100).await;

    // Create a position withouth waiting for the transaction to be confirmed
    let ic = context.ic.as_ref().unwrap();
    let call_id = ic
        .submit_call(
            context.bridge_canister,
            identity.sender().unwrap(),
            "eth_pool_create_position",
            encode_one(tx_hash).unwrap(),
        )
        .await
        .unwrap();
    context.tick(2).await;
    context.proxy_one_https_outcall_to_anvil().await; // Transaction referenced by hash
    context.tick(5).await;
    context.proxy_one_https_outcall_to_anvil().await; // Latests block
    let response: RpcResult<EthPoolLiquidityPositionDto> =
        context.await_call_and_decode(call_id).await;

    assert!(response.is_err());
    let response = response.unwrap_err();
    assert!(matches!(response.code, 400));
    assert_eq!(
        response.details.as_ref().unwrap(),
        "Transaction error: Transaction has not enough confirmations"
    );

    context.teardown_default().await;
}

#[tokio::test]
async fn create_position() {
    let mut context = Context::new();
    let context = context.setup_default().await;

    // Send some ETH to the eth pool address
    let amount = 100;
    let tx_hash = context.send_eth_to_pool_address(0, amount).await;

    // Mine enough blocks to have the transaction confirmed
    let _ = context.anvil_mine_blocks(15);

    let ic = context.ic.as_ref().unwrap();
    let (_, _, identity) = context.full_login_with_eth_registered(0, None).await;
    let call_id = ic
        .submit_call(
            context.bridge_canister,
            identity.sender().unwrap(),
            "eth_pool_create_position",
            encode_one(tx_hash.clone()).unwrap(),
        )
        .await
        .unwrap();
    context.tick(2).await;
    context.proxy_one_https_outcall_to_anvil().await; // Transaction referenced by hash
    context.tick(5).await;
    context.proxy_one_https_outcall_to_anvil().await; // Latests block
    let response: RpcResult<EthPoolLiquidityPositionDto> =
        context.await_call_and_decode(call_id).await;

    assert!(response.is_ok());
    let response = response.unwrap_ok();
    assert_eq!(response.tx_hash, tx_hash);
    assert_eq!(response.amount, amount.to_string());

    context.teardown_default().await;
}
