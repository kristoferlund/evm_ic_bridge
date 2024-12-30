use eth_pool::EthPoolLiquidityPositionDto;
use http_error::HttpError;
use ic_cdk::export_candid;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, Log,
};
use init::InitArgs;
use state::state_types::State;
use std::cell::RefCell;
use user::user_types::UserDto;

pub mod declarations;
pub mod eth_pool;
pub mod event;
pub mod evm;
pub mod http_error;
pub mod init;
pub mod siwe;
pub mod state;
pub mod user;
pub mod utils;

type Memory = VirtualMemory<DefaultMemoryImpl>;

const EVENT_LOG_INDEX_MEMORY_ID: MemoryId = MemoryId::new(3);
const EVENT_LOG_DATA_MEMORY_ID: MemoryId = MemoryId::new(4);

type EventLog = Log<Vec<u8>, Memory, Memory>;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static EVENT_LOG: RefCell<EventLog> = RefCell::new(
        Log::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(EVENT_LOG_INDEX_MEMORY_ID)),
            MEMORY_MANAGER.with(|m| m.borrow().get(EVENT_LOG_DATA_MEMORY_ID)),
        ).expect("Failed to initialize change log.")
    );

}

export_candid!();
