use ic_cdk::query;

use crate::{event::Event, http_error::HttpError, EVENT_LOG};

#[query]
pub fn event_log() -> Result<Vec<Event>, HttpError> {
    Ok(EVENT_LOG.with_borrow(|log| {
        log.iter()
            .map(|e| bincode::deserialize(&e).unwrap())
            .collect()
    }))
}
