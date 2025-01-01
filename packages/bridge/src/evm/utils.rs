use alloy::{
    signers::icp::IcpSigner,
    transports::icp::{RpcApi, RpcService},
};

use crate::STATE;

pub fn get_rpc_service() -> RpcService {
    let evm_rpc_url = STATE.with_borrow(|state| state.evm_rpc_url.clone());
    RpcService::Custom(RpcApi {
        url: evm_rpc_url,
        headers: None,
    })
}

pub async fn create_signer() -> IcpSigner {
    let ecdsa_key_id = STATE.with_borrow(|state| state.ecdsa_key_id.clone());
    IcpSigner::new(vec![], &ecdsa_key_id, None).await.unwrap()
}
