use crate::init::{InitArgs, InitManager};
use ic_cdk::post_upgrade;

#[post_upgrade]
fn post_upgrade(args: InitArgs) {
    InitManager::post_upgrade(args);
}
