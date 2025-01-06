use crate::{
    common::{create_basic_identity, create_delegated_identity, get_response_headers},
    types::{
        BridgeSettings, EthPoolLiquidityPositionDto, EthTxHash, EvmRpcSettings,
        PrepareLoginOkResponse, RpcError, RpcResult, RuntimeFeature, SettingsInput, UserDto,
    },
};
use alloy::{
    network::TransactionBuilder,
    node_bindings::{Anvil, AnvilInstance},
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::{local::PrivateKeySigner, Signer},
};
use candid::{decode_one, encode_args, encode_one, CandidType, Principal};
use ic_agent::{identity::DelegatedIdentity, Identity};
use ic_siwe::{delegation::SignedDelegation, login::LoginDetails};
use pocket_ic::{
    common::rest::{
        CanisterHttpMethod, CanisterHttpReply, CanisterHttpRequest, CanisterHttpResponse,
        MockCanisterHttpResponse, RawMessageId,
    },
    nonblocking::PocketIc,
    PocketIcBuilder, WasmResult,
};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::json;
use std::{fs, str::FromStr, time::Duration};
use ureq::Agent;

pub const BRIDGE_ENGINE_WASM: &str = "../../target/wasm32-unknown-unknown/release/bridge.wasm.gz";
pub const IC_SIWE_WASM: &str = "../ic_siwe_provider/ic_siwe_provider.wasm.gz";
pub const EVM_RPC_WASM: &str = "../evm_rpc/evm_rpc.wasm.gz";

pub struct Context {
    pub ic: Option<PocketIc>,
    pub ic_siwe_canister: Principal,
    pub bridge_canister: Principal,
    pub evm_rpc_canister: Principal,
    pub anvil: Option<AnvilInstance>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            ic: None,
            ic_siwe_canister: Principal::anonymous(),
            bridge_canister: Principal::anonymous(),
            evm_rpc_canister: Principal::anonymous(),
            anvil: None,
        }
    }

    pub async fn setup_ic(&mut self) -> &mut Context {
        self.ic = Some(
            PocketIcBuilder::new()
                .with_ii_subnet() // to have tECDSA keys available
                .with_application_subnet()
                .with_log_level(slog::Level::Error)
                .build_async()
                .await,
        );
        self
    }

    pub async fn setup_ic_siwe(&mut self) -> &mut Context {
        let ic = self.ic.as_ref().unwrap();
        let ic_siwe_canister = ic.create_canister().await;
        ic.add_cycles(ic_siwe_canister, 2_000_000_000_000).await; // 2T Cycles
        let ic_siwe_wasm = fs::read(IC_SIWE_WASM).expect("IC_SIWE_WASM not found");
        let ic_siwe_settings = SettingsInput {
            domain: "127.0.0.1".to_string(),
            uri: "http://127.0.0.1".to_string(),
            salt: "dummy-salt".to_string(),
            chain_id: None,
            scheme: Some("http".to_string()),
            statement: Some("Login to the app".to_string()),
            sign_in_expires_in: Some(Duration::from_secs(3).as_nanos() as u64), // 3 seconds
            session_expires_in: Some(Duration::from_secs(60 * 60 * 24 * 7).as_nanos() as u64), // 1 week
            targets: None,
            runtime_features: Some(vec![RuntimeFeature::IncludeUriInSeed]),
        };
        let args = encode_one(ic_siwe_settings).unwrap();
        ic.install_canister(ic_siwe_canister, ic_siwe_wasm, args, None)
            .await;
        self.ic_siwe_canister = ic_siwe_canister;
        self
    }

    pub async fn setup_bridge(&mut self) -> &mut Context {
        let ic = self.ic.as_ref().unwrap();
        let bridge_canister = ic.create_canister().await;
        ic.add_cycles(bridge_canister, 2_000_000_000_000).await; // 2T Cycles
        let bridge_wasm = fs::read(BRIDGE_ENGINE_WASM).expect("BRIDGE_ENGINE_WASM not found");
        let bridge_settings = BridgeSettings {
            ecdsa_key_id: "dfx_test_key".to_string(),
            siwe_provider_canister: self.ic_siwe_canister.to_string(),
            evm_rpc_url: "http://127.0.0.1:8545".to_string(),
            eth_min_confirmations: 12,
        };
        let args = encode_one(bridge_settings).unwrap();
        ic.install_canister(bridge_canister, bridge_wasm, args, None)
            .await;
        self.bridge_canister = bridge_canister;
        self
    }

    pub async fn setup_evm_rpc(&mut self) -> &mut Context {
        let ic = self.ic.as_ref().unwrap();
        let evm_rpc_canister = ic
            .create_canister_with_id(
                None,
                None,
                Principal::from_str("7hfb6-caaaa-aaaar-qadga-cai").unwrap(),
            )
            .await
            .unwrap();
        ic.add_cycles(evm_rpc_canister, 2_000_000_000_000).await; // 2T Cycles
        let evm_rpc_wasm = fs::read(EVM_RPC_WASM).expect("EVM_RPC_WASM not found");
        let evm_rpc_settings = EvmRpcSettings { nodes_in_subnet: 1 };
        let args = encode_one(evm_rpc_settings).unwrap();
        ic.install_canister(evm_rpc_canister, evm_rpc_wasm, args, None)
            .await;
        self.evm_rpc_canister = evm_rpc_canister;
        self
    }

    pub async fn setup_anvil(&mut self) -> &mut Context {
        let anvil = Anvil::new()
            .try_spawn()
            .expect("Failed to spawn Anvil instance. Ensure `anvil` is available in $PATH.");
        self.anvil = Some(anvil);
        self
    }

    pub async fn setup_default(&mut self) -> &mut Context {
        self.setup_ic().await;
        self.setup_ic_siwe().await;
        self.setup_bridge().await;
        self.setup_evm_rpc().await;
        self.setup_anvil().await;
        self.tick(5).await;
        self
    }

    pub async fn teardown_default(&mut self) -> &mut Context {
        if let Some(ic) = self.ic.take() {
            ic.drop().await;
        }
        self
    }

    pub async fn tick(&self, times: u32) {
        let ic = self.ic.as_ref().unwrap();
        for _ in 0..times {
            ic.tick().await;
        }
    }

    pub async fn update_call<T: CandidType + for<'de> Deserialize<'de>>(
        &self,
        canister: Principal,
        sender: Principal,
        method: &str,
        args: Vec<u8>,
    ) -> Result<T, String> {
        let ic = self.ic.as_ref().unwrap();
        match ic.update_call(canister, sender, method, args).await {
            Ok(WasmResult::Reply(data)) => decode_one(&data).unwrap(),
            Ok(WasmResult::Reject(error_message)) => Err(error_message.to_string()),
            Err(user_error) => Err(user_error.to_string()),
        }
    }

    pub async fn update_call_unwrap<T: CandidType + for<'de> Deserialize<'de>>(
        &self,
        canister: Principal,
        sender: Principal,
        method: &str,
        args: Vec<u8>,
    ) -> RpcResult<T> {
        let ic = self.ic.as_ref().unwrap();
        match ic.update_call(canister, sender, method, args).await {
            Ok(WasmResult::Reply(data)) => decode_one(&data).unwrap(),
            Ok(WasmResult::Reject(error_message)) => RpcResult::Err(RpcError {
                code: 500,
                message: error_message.to_string(),
                details: None,
            }),
            Err(err) => RpcResult::Err(RpcError {
                code: 500,
                message: err.to_string(),
                details: None,
            }),
        }
    }

    pub async fn query_call<T: CandidType + for<'de> Deserialize<'de>>(
        &self,
        canister: Principal,
        sender: Principal,
        method: &str,
        args: Vec<u8>,
    ) -> Result<T, String> {
        let ic = self.ic.as_ref().unwrap();
        match ic.query_call(canister, sender, method, args).await {
            Ok(WasmResult::Reply(data)) => decode_one(&data).unwrap(),
            Ok(WasmResult::Reject(error_message)) => Err(error_message.to_string()),
            Err(user_error) => Err(user_error.to_string()),
        }
    }
    pub async fn query_call_unwrap<T: CandidType + for<'de> Deserialize<'de>>(
        &self,
        canister: Principal,
        sender: Principal,
        method: &str,
        args: Vec<u8>,
    ) -> RpcResult<T> {
        let ic = self.ic.as_ref().unwrap();
        match ic.query_call(canister, sender, method, args).await {
            Ok(WasmResult::Reply(data)) => decode_one(&data).unwrap(),
            Ok(WasmResult::Reject(error_message)) => RpcResult::Err(RpcError {
                code: 500,
                message: error_message.to_string(),
                details: None,
            }),
            Err(err) => RpcResult::Err(RpcError {
                code: 500,
                message: err.to_string(),
                details: None,
            }),
        }
    }

    pub async fn await_call_and_decode<T>(&self, call_id: RawMessageId) -> T
    where
        T: CandidType + DeserializeOwned,
    {
        let ic = self.ic.as_ref().unwrap();
        let wasm_result = ic.await_call(call_id).await;
        match wasm_result.unwrap() {
            WasmResult::Reply(data) => decode_one(&data).unwrap(),
            WasmResult::Reject(msg) => panic!("Unexpected reject {}", msg),
        }
    }

    pub async fn full_login(
        &self,
        key_index: usize,
        targets: Option<Vec<Principal>>,
    ) -> (PrivateKeySigner, String, DelegatedIdentity) {
        let anvil = self.anvil.as_ref().unwrap();
        let signer: PrivateKeySigner = anvil.keys()[key_index].clone().into();
        let address = signer.address().to_checksum(None);
        let (signature, _, nonce) = self.prepare_login_and_sign_message(&signer).await;

        // Create a session identity
        let session_identity = create_basic_identity();
        let session_pubkey = session_identity.public_key().unwrap();

        // Login
        let login_args = encode_args((
            signature,
            address.clone(),
            session_pubkey.clone(),
            nonce.clone(),
        ))
        .unwrap();
        let login_response: LoginDetails = self
            .update_call(
                self.ic_siwe_canister,
                Principal::anonymous(),
                "siwe_login",
                login_args,
            )
            .await
            .unwrap();

        // Get the delegation
        let get_delegation_args = encode_args((
            address.clone(),
            session_pubkey.clone(),
            login_response.expiration,
        ))
        .unwrap();

        let get_delegation_response: SignedDelegation = self
            .query_call(
                self.ic_siwe_canister,
                Principal::anonymous(),
                "siwe_get_delegation",
                get_delegation_args,
            )
            .await
            .unwrap();

        // Create a delegated identity
        let delegated_identity = create_delegated_identity(
            session_identity,
            &login_response,
            get_delegation_response.signature.as_ref().to_vec(),
            targets,
        );

        // Create a user in the bridge canister
        let _: UserDto = self
            .update_call(
                self.bridge_canister,
                delegated_identity.sender().unwrap(),
                "user_create",
                encode_one(()).unwrap(),
            )
            .await
            .unwrap();

        (signer, address, delegated_identity)
    }

    pub async fn full_login_with_eth_registered(
        &self,
        key_index: usize,
        targets: Option<Vec<Principal>>,
    ) -> (PrivateKeySigner, String, DelegatedIdentity) {
        let (signer, address, delegated_identity) = self.full_login(key_index, targets).await;

        let _: UserDto = self
            .update_call(
                self.bridge_canister,
                delegated_identity.sender().unwrap(),
                "user_register_eth_address",
                encode_one(()).unwrap(),
            )
            .await
            .unwrap();

        (signer, address, delegated_identity)
    }

    pub async fn prepare_login_and_sign_message(
        &self,
        signer: &PrivateKeySigner,
    ) -> (String, String, String) {
        let args = encode_one(signer.address().to_checksum(None)).unwrap();
        let response: PrepareLoginOkResponse = self
            .update_call(
                self.ic_siwe_canister,
                Principal::anonymous(),
                "siwe_prepare_login",
                args,
            )
            .await
            .unwrap();

        let signature = signer
            .sign_message(response.siwe_message.as_bytes())
            .await
            .unwrap();

        (
            format!("0x{}", hex::encode(signature.as_bytes())),
            response.siwe_message,
            response.nonce,
        )
    }

    pub async fn get_eth_pool_address(&self) -> String {
        let response: RpcResult<String> = self
            .query_call_unwrap(
                self.bridge_canister,
                Principal::anonymous(),
                "eth_pool_address",
                encode_one(()).unwrap(),
            )
            .await;
        assert!(response.is_ok());
        response.unwrap_ok().clone()
    }

    // Forward an IC https outcall to local Anvil server
    pub fn anvil_request(&self, canister_req: &CanisterHttpRequest) -> MockCanisterHttpResponse {
        let agent = Agent::new();
        let method = match canister_req.http_method {
            CanisterHttpMethod::GET => "GET",
            CanisterHttpMethod::POST => "POST",
            CanisterHttpMethod::HEAD => "HEAD",
        };
        let anvil = self.anvil.as_ref().unwrap();
        let mut request = agent.request(method, anvil.endpoint_url().as_str());

        for header in &canister_req.headers {
            request = request.set(&header.name, &header.value);
        }

        let response = request.send_bytes(&canister_req.body).unwrap();

        let status = response.status();
        let headers = get_response_headers(&response);

        let mut reader = response.into_reader();
        let mut body = Vec::new();
        reader.read_to_end(&mut body).unwrap();

        MockCanisterHttpResponse {
            subnet_id: canister_req.subnet_id,
            request_id: canister_req.request_id,
            response: CanisterHttpResponse::CanisterHttpReply(CanisterHttpReply {
                status,
                headers,
                body,
            }),
            additional_responses: vec![],
        }
    }

    // Mine blocks to fast forward the local Anvil blockchain
    pub fn anvil_mine_blocks(&self, num_blocks: u64) -> Result<(), Box<dyn std::error::Error>> {
        let agent = Agent::new();

        let anvil = self.anvil.as_ref().unwrap();
        let request = agent.request("POST", anvil.endpoint_url().as_str());

        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "anvil_mine",
            "params": [format!("0x{:x}", num_blocks)]
        });

        let _ = request
            .set("Content-Type", "application/json")
            .send_string(&payload.to_string())
            .unwrap();

        Ok(())
    }

    // Get the pending HTTPS outcalls from the IC and forward the first one to the local Anvil server
    // Response is mocked back to the IC
    pub async fn proxy_one_https_outcall_to_anvil(&self) {
        let ic = self.ic.as_ref().unwrap();
        let canister_http_requests = ic.get_canister_http().await;
        if canister_http_requests.is_empty() {
            return;
        }
        let canister_http_request = &canister_http_requests[0];
        let canister_http_response = self.anvil_request(canister_http_request);
        ic.mock_canister_http_response(canister_http_response).await;
    }

    // Send some ETH to the eth pool address from the
    pub async fn send_eth_to_pool_address(&self, from: usize, amount: u32) -> EthTxHash {
        let anvil = self.anvil.as_ref().unwrap();
        let provider = ProviderBuilder::new().on_http(anvil.endpoint_url());
        let signer: PrivateKeySigner = anvil.keys()[from].clone().into();
        let eth_pool_address = self.get_eth_pool_address().await;
        let tx = TransactionRequest::default()
            .with_from(signer.address())
            .with_to(alloy::primitives::Address::parse_checksummed(eth_pool_address, None).unwrap())
            .with_nonce(0)
            .with_value(U256::from(amount))
            .with_gas_limit(21_000)
            .with_max_priority_fee_per_gas(1_000_000_000)
            .with_max_fee_per_gas(20_000_000_000);
        let pending_tx = provider.send_transaction(tx).await.unwrap();
        let tx_receipt = pending_tx.get_receipt().await.unwrap();
        format!("0x{}", hex::encode(tx_receipt.transaction_hash))
    }

    pub async fn create_eth_position(
        &self,
        principal: &Principal,
        tx_hash: &str,
    ) -> RpcResult<EthPoolLiquidityPositionDto> {
        let ic = self.ic.as_ref().unwrap();
        let call_id = ic
            .submit_call(
                self.bridge_canister,
                *principal,
                "eth_pool_create_position",
                encode_one(tx_hash).unwrap(),
            )
            .await
            .unwrap();
        self.tick(2).await;
        self.proxy_one_https_outcall_to_anvil().await; // Transaction referenced by hash
        self.tick(5).await;
        self.proxy_one_https_outcall_to_anvil().await; // Latests block
        self.await_call_and_decode(call_id).await
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
