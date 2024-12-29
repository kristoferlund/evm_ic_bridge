use super::{
    user_state_transitions::UserStateTransitions, user_types::EthAddressBytes, User, UserError,
};
use crate::{
    event::{Event, EventPublisher},
    STATE,
};
use candid::Principal;

pub struct UserManager {}

impl UserManager {
    pub fn create(principal: Principal) -> Result<User, UserError> {
        if principal == Principal::anonymous() {
            return Err(UserError::InvalidPrincipal);
        }
        STATE.with_borrow(|state| {
            if state.users.contains_key(&principal) {
                return Err(UserError::AlreadyExists);
            }
            let user = UserStateTransitions::create(principal);
            EventPublisher::publish(Event::CreateUser(principal));
            Ok(user)
        })
    }

    pub fn set_eth_address(
        principal: Principal,
        eth_address: EthAddressBytes,
    ) -> Result<User, UserError> {
        STATE.with_borrow(|state| {
            if !state.users.contains_key(&principal) {
                return Err(UserError::NotFound);
            }
            let user = UserStateTransitions::set_eth_address(principal, eth_address);
            EventPublisher::publish(Event::RegisterEthAddress(principal, eth_address));
            Ok(user)
        })
    }

    pub fn get_by_principal(principal: Principal) -> Result<User, UserError> {
        STATE.with_borrow(|state| {
            state
                .users
                .get(&principal)
                .cloned()
                .ok_or(UserError::NotFound)
        })
    }

    pub fn exists(principal: Principal) -> bool {
        STATE.with_borrow(|state| state.users.contains_key(&principal))
    }
}
