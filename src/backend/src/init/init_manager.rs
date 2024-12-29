use super::InitArgs;
use crate::{
    event::{Event, EventPublisher},
    evm::utils::create_signer,
    state::state_types::State,
    STATE,
};
use alloy::signers::Signer;
use std::time::Duration;

pub enum InitManagerMode {
    Normal, // Emit and store events
    Replay, // Replay mode: no event emission
}

pub struct InitManager {
    mode: InitManagerMode,
}

pub fn init_signer() {
    ic_cdk_timers::set_timer(Duration::from_secs(0), || {
        ic_cdk::spawn(async move {
            let signer = create_signer().await;
            let address = signer.address();

            STATE.with_borrow_mut(|state| {
                state.signer = Some(signer);
                state.canister_eth_address = Some(address);
            });

            ic_cdk::println!("Initialising signer for address: {}", address);
        });
    });
}

impl InitManager {
    pub fn new() -> Self {
        InitManager {
            mode: InitManagerMode::Normal,
        }
    }

    pub fn replay() -> Self {
        InitManager {
            mode: InitManagerMode::Replay,
        }
    }

    pub fn init(&self, args: InitArgs) {
        let InitArgs {
            eth_min_confirmations,
        } = args;

        STATE.with_borrow_mut(|state| {
            *state = State {
                eth_min_confirmations,
                ..State::default()
            };
        });

        EventPublisher::publish(Event::Init(args));

        init_signer();
    }
}
