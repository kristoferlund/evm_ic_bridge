use ic_cdk::update;

use crate::{
    http_error::HttpError,
    user::{user_utils::auth_guard_no_anon, UserDto, UserError, UserManager},
};

#[update]
async fn user_create() -> Result<UserDto, HttpError> {
    auth_guard_no_anon()?;
    let caller = ic_cdk::api::caller();
    match UserManager::create(caller) {
        Ok(user) => Ok(UserDto {
            principal: user.principal,
            eth_address: None,
        }),
        Err(err) => match err {
            UserError::AlreadyExists => Err(HttpError::conflict("User already exists")),
            UserError::InvalidPrincipal => Err(HttpError::bad_request("Invalid principal")),
            _ => Err(HttpError::internal_server_error("Internal error")),
        },
    }
}
