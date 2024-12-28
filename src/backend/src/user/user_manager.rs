use super::user_types::{User, UserError};
use crate::STATE;
use candid::Principal;

pub struct UserManager {}

impl UserManager {
    pub fn create(principal: Principal) -> Result<User, UserError> {
        if principal == Principal::anonymous() {
            return Err(UserError::InvalidPrincipal);
        }
        STATE.with_borrow_mut(|state| {
            if state.users.contains_key(&principal) {
                return Err(UserError::AlreadyExists);
            }
            let user = User::new();
            state.users.insert(principal, user.clone());
            Ok(user)
        })
    }

    pub fn set_eth_address(principal: Principal, eth_address: [u8; 20]) -> Result<User, UserError> {
        STATE.with_borrow_mut(|state| {
            let user = state.users.get_mut(&principal).ok_or(UserError::NotFound)?;
            user.eth_address = eth_address;
            Ok(user.clone())
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
}
