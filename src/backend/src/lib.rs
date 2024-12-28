use std::cell::RefCell;

use http_error::HttpError;
use ic_cdk::export_candid;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, Log,
};
use serde_bytes::ByteBuf;
use state::state_types::State;

pub mod declarations;
pub mod eth;
pub mod eth_pool;
pub mod event;
pub mod evm;
pub mod http_error;
pub mod init;
pub mod siwe;
pub mod state;
pub mod user;

type Memory = VirtualMemory<DefaultMemoryImpl>;

// const USERS_MEMORY_ID: MemoryId = MemoryId::new(1);
// const USER_ETH_ADDRESS_MEMORY_ID: MemoryId = MemoryId::new(2);
const EVENT_LOG_INDEX_MEMORY_ID: MemoryId = MemoryId::new(3);
const EVENT_LOG_DATA_MEMORY_ID: MemoryId = MemoryId::new(4);

pub type AddressBytes = [u8; 20];

type EventLog = Log<Vec<u8>, Memory, Memory>;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // // USERS
    // static USERS: RefCell<StableBTreeMap<Blob<29>, User, Memory>> = RefCell::new(
    //     StableBTreeMap::init(
    //         MEMORY_MANAGER.with(|m| m.borrow().get(USERS_MEMORY_ID)),
    //     )
    // );
    //
    //  static USER_ETH_ADDRESS_INDEX: RefCell<StableBTreeMap<AddressBytes, Blob<29>, Memory>> = RefCell::new(
    //      StableBTreeMap::init(
    //          MEMORY_MANAGER.with(|m| m.borrow().get(USER_ETH_ADDRESS_MEMORY_ID)),
    //      )
    //  );

    static EVENT_LOG: RefCell<EventLog> = RefCell::new(
        Log::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(EVENT_LOG_INDEX_MEMORY_ID)),
            MEMORY_MANAGER.with(|m| m.borrow().get(EVENT_LOG_DATA_MEMORY_ID)),
        ).expect("Failed to initialize change log.")
    );

}

export_candid!();
