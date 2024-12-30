use crate::init::{init_utils::validate_init_args, InitArgs, InitManager};
use ic_cdk::post_upgrade;

#[post_upgrade]
fn post_upgrade(args: InitArgs) {
    validate_init_args(&args);
    InitManager::post_upgrade(args);
}
