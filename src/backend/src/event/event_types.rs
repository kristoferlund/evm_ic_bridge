use crate::init::InitArgs;
use candid::Principal;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub enum Event {
    Init(InitArgs),
    PostUpgrade(InitArgs),
    CreateUser(Principal),
    RegisterEthAddress(Principal, [u8; 20]),
}
