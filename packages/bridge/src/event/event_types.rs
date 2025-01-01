use crate::{init::InitArgs, user::user_types::EthAddressBytes};
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, CandidType, Deserialize)]
pub enum Event {
    Init(InitArgs),
    PostUpgrade(InitArgs),
    CreateUser(Principal),
    RegisterEthAddress(Principal, EthAddressBytes),
    EThPoolCreatePosition(Principal, String),
}
