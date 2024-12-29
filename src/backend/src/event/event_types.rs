use crate::{init::InitArgs, user::user_types::EthAddressBytes};
use alloy::primitives::U256;
use candid::Principal;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub enum Event {
    Init(InitArgs),
    PostUpgrade(InitArgs),
    CreateUser(Principal),
    RegisterEthAddress(Principal, EthAddressBytes),
    EThPoolCreatePosition(Principal, U256),
}
