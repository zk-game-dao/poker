use candid::CandidType;
use currency::currency_error::CurrencyError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    canister_management_error::CanisterManagementError, chat_error::ChatError,
    game_error::GameError, user_error::UserError,
};

// Define a new encompassing error type that includes GameError and LockError
#[derive(Error, Debug, CandidType, Serialize, Deserialize, PartialEq, Eq)]
pub enum TableError {
    // Should technically be from GameError but the Candid idl is being a bitch.
    #[error("game error: {0}")]
    Game(#[from] GameError),

    #[error("Chat error: {0}")]
    Chat(#[from] ChatError),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("failed to acquire lock")]
    LockError,

    #[error("serialization error: {0}")]
    SerializationError(String),

    #[error("Table not found")]
    TableNotFound,

    #[error("Authorization error")]
    AuthorizationError,

    #[error("User not found")]
    UserNotFound,

    #[error("No winner")]
    NoWinner,

    #[error("Invalid blinds")]
    InvalidBlinds,

    #[error("Invalid table status")]
    StateNotInitialized,

    #[error("Block query failed: {0}")]
    BlockQueryFailed(String),

    #[error("Block not found")]
    BlockNotFound,

    #[error("Invalid transaction details")]
    InvalidTransactionDetails,

    #[error("Invalid transaction type")]
    InvalidTransactionType,

    #[error("Duplicate transaction")]
    DuplicateTransaction,

    #[error("Ledger error: {0}")]
    LedgerError(String),

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Query error: {0}")]
    QueryError(String),

    #[error("Canister call error: {0}")]
    CanisterCallError(String),

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("User already in game")]
    UserAlreadyInGame,

    #[error("Player not found")]
    PlayerNotFound,

    #[error("Seat not found")]
    SeatNotFound,

    #[error("User error: {0}")]
    UserError(#[from] UserError),

    #[error("Canister management error: {0}")]
    ManagementCanisterError(#[from] CanisterManagementError),

    #[error("Currency error: {0}")]
    CurrencyError(#[from] CurrencyError),

    #[error("User not verified")]
    UserNotVerified,
}
