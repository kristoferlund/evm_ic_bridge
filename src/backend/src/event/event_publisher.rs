use super::Event;
use crate::EVENT_LOG;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PublishEventError {
    #[error("Failed to serialize the event: {0}")]
    SerializationError(String),
    #[error("Failed to append the event to the log.")]
    LogAppendError,
    #[error("Failed to process the event: {0}")]
    EventProcessingError(String),
}

pub struct EventPublisher {}

impl EventPublisher {
    pub fn publish(event: Event) -> Result<(), PublishEventError> {
        let encoded_event = bincode::serialize(&event)
            .map_err(|e| e.to_string())
            .map_err(|e| PublishEventError::SerializationError(e))?;

        EVENT_LOG
            .with_borrow_mut(|log| log.append(&encoded_event))
            .map_err(|_| PublishEventError::LogAppendError)?;

        Ok(())
    }
}
