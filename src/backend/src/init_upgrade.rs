use crate::{evm::utils::create_signer, CanisterSettingsDto, State, STATE};
use alloy::signers::Signer;
use std::time::Duration;

fn save_settings(settings: CanisterSettingsDto) {
    let CanisterSettingsDto {
        eth_min_confirmations,
    } = settings;

    STATE.with_borrow_mut(|state| {
        *state = State {
            eth_min_confirmations,
            ..State::default()
        };
    });
}

fn init_signer() {
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

#[ic_cdk::init]
fn init(settings: CanisterSettingsDto) {
    save_settings(settings);
    init_signer();
}

#[ic_cdk::post_upgrade]
fn post_upgrade(settings: CanisterSettingsDto) {
    save_settings(settings);
    init_signer();
}
