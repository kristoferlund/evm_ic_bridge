use ic_cdk::update;

use crate::{
    http_error::HttpError,
    user::{user_utils::auth_guard_no_anon, User, UserManager},
};

#[update]
async fn user_create() -> Result<User, HttpError> {
    auth_guard_no_anon()?;
    let caller = ic_cdk::api::caller();
    let user_manager = UserManager::new();
    if user_manager.get_by_principal(caller).is_ok() {
        return Err(HttpError::conflict("User already exists"));
    }
    let user = user_manager
        .create(caller)
        .map_err(HttpError::bad_request)?;
    Ok(user)
}
