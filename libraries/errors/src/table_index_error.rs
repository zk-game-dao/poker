use candid::CandidType;
use currency::currency_error::CurrencyError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{canister_management_error::CanisterManagementError, table_error::TableError};

// Define a new encompassing error type that includes GameError and LockError
#[derive(Error, Debug, CandidType, Serialize, Deserialize)]
pub enum TableIndexError {
    #[error("Table error: {0}")]
    TableError(#[from] TableError),

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

    #[error("Block query failed")]
    BlockQueryFailed,

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

    #[error("Canister management error: {0}")]
    ManagementCanisterError(#[from] CanisterManagementError),

    #[error("Currency error: {0}")]
    CurrencyError(#[from] CurrencyError),

    #[error("No table available")]
    NoTableAvailable,

    #[error("Inter canister call error: {0}")]
    CanisterCallError(String),

    #[error("No available tables: {0}")]
    NoAvailableTables(String),
}
