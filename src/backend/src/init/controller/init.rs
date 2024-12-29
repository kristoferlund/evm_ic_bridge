use crate::init::{InitArgs, InitManager};

#[ic_cdk::init]
fn init(args: InitArgs) {
    InitManager::new().init(args);
}
