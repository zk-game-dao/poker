use candid::CandidType;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Define a new encompassing error type that includes GameError and LockError
#[derive(Error, Debug, CandidType, Serialize, Deserialize)]
pub enum LogStoreError {
    #[error("Failed to serialize data: {0}")]
    SerializationError(String),

    #[error("Failed to deserialize data: {0}")]
    DeserializationError(String),
}
