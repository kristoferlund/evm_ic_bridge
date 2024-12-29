use super::{UserError, UserManager};
use crate::http_error::HttpError;
use candid::Principal;
use ic_stable_structures::storable::Blob;

pub fn auth_guard_no_anon() -> Result<(), HttpError> {
    match ic_cdk::caller() {
        caller if caller == Principal::anonymous() => Err(HttpError::unauthorized(
            "Calls with the anonymous principal are not allowed.".to_string(),
        )),
        _ => Ok(()),
    }
}

pub fn auth_guard_eth() -> Result<(), HttpError> {
    auth_guard_no_anon()?;

    let caller = ic_cdk::caller();
    let user_manager = UserManager::new();
    user_manager.get_by_principal(caller).map_err(|_| {
        HttpError::unauthorized("No Ethereum address registered for caller.".to_string())
    })?;

    Ok(())
}

pub fn principal_to_blob(principal: Principal) -> Result<Blob<29>, UserError> {
    principal.as_slice()[..29]
        .try_into()
        .map_err(|_| UserError::InvalidPrincipal)
}
