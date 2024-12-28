use super::Event;
use crate::{init::init_manager::InitManager, user::user_manager::UserManager};

pub struct EventProcessor {}

impl EventProcessor {
    pub fn process(event: Event) {
        match event {
            Event::Init(args) => {
                InitManager::init(&args);
            }
            Event::CreateUser(principal) => {
                UserManager::create(principal);
            }
            Event::RegisterEthAddress(principal, eth_address) => {
                UserManager::set_eth_address(principal, eth_address);
            }
        }
    }
}
