use super::InitArgs;
use crate::{
    event::{Event, EventPublisher},
    evm::utils::create_signer,
    init::init_state_transitions::InitStateTransitions,
};
use alloy::signers::Signer;
use std::time::Duration;

pub struct InitManager {}

pub fn init_signer() {
    ic_cdk_timers::set_timer(Duration::from_secs(0), || {
        ic_cdk::spawn(async move {
            let signer = create_signer().await;
            let address = signer.address().into_array();
            InitStateTransitions::init_signer(address, signer);
        });
    });
}

impl InitManager {
    pub fn init(args: InitArgs) {
        init_signer();
        InitStateTransitions::init_and_upgrade(&args);
        EventPublisher::publish(Event::Init(args));
    }

    pub fn post_upgrade(args: InitArgs) {
        InitStateTransitions::init_and_upgrade(&args);
        EventPublisher::publish(Event::PostUpgrade(args));
    }
}
