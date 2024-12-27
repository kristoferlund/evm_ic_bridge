use std::cell::RefCell;

use alloy::{primitives::Address, signers::icp::IcpSigner};
use candid::{CandidType, Nat, Principal};
use http_error::HttpError;
use ic_cdk::export_candid;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Blob,
    DefaultMemoryImpl, StableBTreeMap,
};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use user::User;

pub mod declarations;
pub mod eth;
pub mod evm;
pub mod http_error;
pub mod init_upgrade;
pub mod service;
pub mod siwe;
pub mod user;

type Memory = VirtualMemory<DefaultMemoryImpl>;

const USERS_MEMORY_ID: MemoryId = MemoryId::new(1);
const USER_ETH_ADDRESS_MEMORY_ID: MemoryId = MemoryId::new(2);

fn auth_guard() -> Result<(), HttpError> {
    match ic_cdk::caller() {
        caller if caller == Principal::anonymous() => Err(HttpError::unauthorized(
            "Calls with the anonymous principal are not allowed.".to_string(),
        )),
        _ => Ok(()),
    }
}

pub type AddressBytes = [u8; 20];

#[derive(Serialize, Deserialize, CandidType)]
pub struct CanisterSettingsDto {
    pub eth_min_confirmations: u64,
}

#[derive(Default)]
pub struct State {
    // Settings
    eth_min_confirmations: u64,

    // Runtime
    signer: Option<IcpSigner>,
    canister_eth_address: Option<Address>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // USERS
    static USERS: RefCell<StableBTreeMap<Blob<29>, User, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(USERS_MEMORY_ID)),
        )
    );

     static USER_ETH_ADDRESS_INDEX: RefCell<StableBTreeMap<AddressBytes, Blob<29>, Memory>> = RefCell::new(
         StableBTreeMap::init(
             MEMORY_MANAGER.with(|m| m.borrow().get(USER_ETH_ADDRESS_MEMORY_ID)),
         )
     );
}

export_candid!();
