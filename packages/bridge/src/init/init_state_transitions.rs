use super::InitArgs;
use crate::{state::state_types::State, user::user_types::EthAddressBytes, STATE};
use alloy::signers::icp::IcpSigner;

pub struct InitStateTransitions {}

impl InitStateTransitions {
    pub fn init_and_upgrade(args: &InitArgs) {
        let InitArgs {
            ecdsa_key_id,
            siwe_provider_canister,
            evm_rpc_url,
            eth_min_confirmations,
        } = args;

        STATE.with_borrow_mut(|state| {
            *state = State {
                ecdsa_key_id: ecdsa_key_id.clone(),
                siwe_provider_canister: siwe_provider_canister.clone(),
                evm_rpc_url: evm_rpc_url.clone(),
                eth_min_confirmations: *eth_min_confirmations,
                ..State::default()
            };
        });
    }

    pub fn init_signer(address: EthAddressBytes, signer: IcpSigner) {
        STATE.with_borrow_mut(|state| {
            state.eth_pool_address = Some(address);
            state.signer = Some(signer);
        });
    }
}
