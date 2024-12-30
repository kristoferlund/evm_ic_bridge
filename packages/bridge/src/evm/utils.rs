use alloy::{
    signers::icp::IcpSigner,
    transports::icp::{RpcApi, RpcService},
};

use crate::STATE;

pub fn get_rpc_service() -> RpcService {
    RpcService::Custom(RpcApi {
        url: "https://ic-alloy-evm-rpc-proxy.kristofer-977.workers.dev/eth-sepolia".to_string(),
        headers: None,
    })
}

pub async fn create_signer() -> IcpSigner {
    let ecdsa_key_id = STATE.with_borrow(|state| state.ecdsa_key_id.clone());
    IcpSigner::new(vec![], &ecdsa_key_id, None).await.unwrap()
}
