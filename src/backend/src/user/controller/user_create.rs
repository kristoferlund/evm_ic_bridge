use ic_cdk::update;

use crate::{
    event::{Event, EventPublisher},
    http_error::HttpError,
    user::{auth_guard_no_anon, User, UserManager},
};

#[update]
async fn user_create() -> Result<User, HttpError> {
    auth_guard_no_anon()?;
    let caller = ic_cdk::api::caller();
    if UserManager::get_by_principal(caller).is_ok() {
        return Err(HttpError::conflict("User already exists"));
    }
    let user = UserManager::create(caller).map_err(HttpError::bad_request)?;
    EventPublisher::publish(Event::CreateUser(caller));
    Ok(user)
}
