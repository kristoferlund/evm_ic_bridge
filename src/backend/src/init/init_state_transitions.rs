use alloy::signers::icp::IcpSigner;

use crate::{state::state_types::State, user::user_types::EthAddressBytes, STATE};

use super::InitArgs;

pub struct InitStateTransitions {}

impl InitStateTransitions {
    pub fn init_and_upgrade(args: &InitArgs) {
        let InitArgs {
            eth_min_confirmations,
        } = args;

        STATE.with_borrow_mut(|state| {
            *state = State {
                eth_min_confirmations: *eth_min_confirmations,
                ..State::default()
            };
        });
    }

    pub fn init_signer(address: EthAddressBytes, signer: IcpSigner) {
        STATE.with_borrow_mut(|state| {
            state.canister_eth_address = Some(address);
            state.signer = Some(signer);
        });
    }
}
