use ic_cdk::update;

use crate::{
    http_error::HttpError,
    user::{user_utils::auth_guard_no_anon, User, UserError, UserManager},
};

#[update]
async fn user_create() -> Result<User, HttpError> {
    auth_guard_no_anon()?;
    let caller = ic_cdk::api::caller();
    match UserManager::create(caller) {
        Ok(user) => Ok(user),
        Err(err) => match err {
            UserError::AlreadyExists => Err(HttpError::conflict("User already exists")),
            UserError::InvalidPrincipal => Err(HttpError::bad_request("Invalid principal")),
            _ => Err(HttpError::internal_server_error("Internal error")),
        },
    }
}
