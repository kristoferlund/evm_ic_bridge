use crate::init::{InitArgs, InitManager};
use ic_cdk::post_upgrade;

#[post_upgrade]
fn post_upgrade(args: InitArgs) {
    // Init arguments are never replayed but applied "fresh" on each upgrade
    InitManager::new().init(args);
}
