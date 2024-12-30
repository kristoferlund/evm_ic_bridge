use super::User;
use crate::STATE;
use candid::Principal;

pub struct UserStateTransitions {}

impl UserStateTransitions {
    pub fn create(principal: Principal) -> User {
        STATE.with_borrow_mut(|state| {
            let user = User::new(principal);
            state.users.insert(principal, user.clone());
            user
        })
    }

    pub fn set_eth_address(principal: Principal, eth_address: [u8; 20]) -> User {
        STATE.with_borrow_mut(|state| {
            let user = state.users.get_mut(&principal).unwrap();
            user.eth_address = Some(eth_address);
            state.users_by_eth_address.insert(eth_address, principal);
            user.clone()
        })
    }
}
