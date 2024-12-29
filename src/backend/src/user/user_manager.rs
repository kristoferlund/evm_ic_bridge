use super::{user_types::EthAddressBytes, User, UserError};
use crate::{
    event::{Event, EventPublisher},
    STATE,
};
use candid::Principal;

pub enum UserManagerMode {
    Normal, // Emit and store events
    Replay, // Replay mode: no event emission
}

pub struct UserManager {
    mode: UserManagerMode,
}

impl UserManager {
    pub fn new() -> Self {
        UserManager {
            mode: UserManagerMode::Normal,
        }
    }

    pub fn replay() -> Self {
        UserManager {
            mode: UserManagerMode::Replay,
        }
    }

    pub fn create(&self, principal: Principal) -> Result<User, UserError> {
        if principal == Principal::anonymous() {
            return Err(UserError::InvalidPrincipal);
        }
        STATE.with_borrow_mut(|state| {
            if state.users.contains_key(&principal) {
                return Err(UserError::AlreadyExists);
            }
            let user = User::new();
            state.users.insert(principal, user.clone());
            if matches!(self.mode, UserManagerMode::Normal) {
                EventPublisher::publish(Event::CreateUser(principal));
            }
            Ok(user)
        })
    }

    pub fn set_eth_address(
        &self,
        principal: Principal,
        eth_address: EthAddressBytes,
    ) -> Result<User, UserError> {
        STATE.with_borrow_mut(|state| {
            let user = state.users.get_mut(&principal).ok_or(UserError::NotFound)?;
            user.eth_address = eth_address;
            state.users_by_eth_address.insert(eth_address, principal);
            if matches!(self.mode, UserManagerMode::Normal) {
                EventPublisher::publish(Event::RegisterEthAddress(principal, eth_address));
            }
            Ok(user.clone())
        })
    }

    pub fn get_by_principal(&self, principal: Principal) -> Result<User, UserError> {
        STATE.with_borrow(|state| {
            state
                .users
                .get(&principal)
                .cloned()
                .ok_or(UserError::NotFound)
        })
    }
}
