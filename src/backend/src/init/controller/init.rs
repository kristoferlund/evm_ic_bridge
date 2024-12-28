use crate::{
    event::{event_publisher::EventPublisher, Event},
    init::{InitArgs, InitManager},
};

#[ic_cdk::init]
fn init(args: InitArgs) {
    InitManager::init(&args);
    EventPublisher::publish(Event::Init(args));
}
