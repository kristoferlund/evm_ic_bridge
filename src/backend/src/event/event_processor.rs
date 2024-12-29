use super::Event;
use crate::{
    init::init_state_transitions::InitStateTransitions,
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
        }
    }
}
