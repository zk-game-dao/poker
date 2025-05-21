use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, CandidType, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChatError {
    #[error("Message not found: {0}")]
    MessageNotFound(u64),

    #[error("Unauthorized: {reason}")]
    Unauthorized { reason: String },

    #[error("Invalid request: {reason}")]
    InvalidRequest { reason: String },

    #[error("Message too long: {current_size} exceeds maximum of {max_size}")]
    MessageTooLong {
        current_size: usize,
        max_size: usize,
    },

    #[error("Too many messages in short period")]
    RateLimitExceeded,

    #[error("Sender {0} is muted")]
    SenderMuted(Principal),

    #[error("Cannot edit message after {0} seconds")]
    EditTimeExpired(u64),

    #[error("Message with ID {0} already exists")]
    DuplicateMessageId(u64),

    #[error("Chat history is full")]
    ChatHistoryFull,

    #[error("Failed to acquire lock: {0}")]
    LockError(String),

    #[error("User {0} not found in table")]
    UserNotInTable(Principal),

    #[error("Internal error: {0}")]
    InternalError(String),
}
