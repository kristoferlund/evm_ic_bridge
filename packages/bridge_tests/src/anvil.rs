use alloy::node_bindings::AnvilInstance;
use candid::{decode_one, CandidType};
use pocket_ic::{
    common::rest::{
        CanisterHttpHeader, CanisterHttpMethod, CanisterHttpReply, CanisterHttpRequest,
        CanisterHttpResponse, MockCanisterHttpResponse, RawMessageId,
    },
    nonblocking::PocketIc,
    WasmResult,
};
use serde::de::DeserializeOwned;
use serde_json::json;
use ureq::{Agent, Response};

pub fn get_response_headers(response: &Response) -> Vec<CanisterHttpHeader> {
    let mut headers = vec![];
    response.headers_names().into_iter().for_each(|name| {
        let value = response.header(&name).unwrap().to_string();
        headers.push(CanisterHttpHeader { name, value });
    });
    headers
}

// Forward an IC https outcall to local Anvil server
pub fn anvil_request(
    canister_req: &CanisterHttpRequest,
    anvil: &AnvilInstance,
) -> MockCanisterHttpResponse {
    let agent = Agent::new();
    let method = match canister_req.http_method {
        CanisterHttpMethod::GET => "GET",
        CanisterHttpMethod::POST => "POST",
        CanisterHttpMethod::HEAD => "HEAD",
    };
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

pub async fn proxy_one_https_outcall(ic: &PocketIc, anvil: &AnvilInstance) {
    let canister_http_requests = ic.get_canister_http().await;
    assert_eq!(canister_http_requests.len(), 1);
    let canister_http_request = &canister_http_requests[0];
    let canister_http_response = anvil_request(canister_http_request, anvil);
    ic.mock_canister_http_response(canister_http_response).await;
}

pub async fn await_call_and_decode<T>(ic: &PocketIc, call_id: RawMessageId) -> T
where
    T: CandidType + DeserializeOwned,
{
    let wasm_result = ic.await_call(call_id).await;
    match wasm_result.unwrap() {
        WasmResult::Reply(data) => decode_one(&data).unwrap(),
        WasmResult::Reject(msg) => panic!("Unexpected reject {}", msg),
    }
}

pub fn mine_blocks(
    anvil: &AnvilInstance,
    num_blocks: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let agent = Agent::new();

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
