use crate::init::{init_utils::validate_init_args, InitArgs, InitManager};

#[ic_cdk::init]
fn init(args: InitArgs) {
    validate_init_args(&args);
    InitManager::init(args);
}
