use alloy::primitives::U256;

use super::Event;
use crate::{
    eth_pool::EthPoolStateTransitions, init::init_state_transitions::InitStateTransitions,
    user::user_state_transitions::UserStateTransitions,
};

pub struct EventProcessor {}

impl EventProcessor {
    pub fn process(event: Event) {
        match event {
            Event::Init(args) => {
                InitStateTransitions::init_and_upgrade(&args);
            }
            Event::PostUpgrade(args) => {
                InitStateTransitions::init_and_upgrade(&args);
            }
            Event::CreateUser(principal) => {
                UserStateTransitions::create(principal);
            }
            Event::RegisterEthAddress(principal, eth_address) => {
                UserStateTransitions::set_eth_address(principal, eth_address);
            }
            Event::EThPoolCreatePosition(principal, value, tx_hash, timestamp) => {
                EthPoolStateTransitions::create_position(
                    principal,
                    U256::from_str_radix(&value, 16).unwrap(), //TODO: error handling + make sure radix is correct
                    tx_hash,
                    timestamp,
                );
            }
        }
    }
}
