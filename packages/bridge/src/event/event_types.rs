use crate::{
    init::InitArgs,
    user::user_types::{EthAddressBytes, EthTxHashBytes},
};
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

pub type PositionAmount = String;
pub type Timestamp = u64;

#[derive(Serialize, Clone, CandidType, Deserialize)]
pub enum Event {
    Init(InitArgs),
    PostUpgrade(InitArgs),
    CreateUser(Principal),
    RegisterEthAddress(Principal, EthAddressBytes),
    EThPoolCreatePosition(Principal, PositionAmount, EthTxHashBytes, Timestamp),
}
