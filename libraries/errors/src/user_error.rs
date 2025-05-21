use candid::{types::principal::PrincipalError, CandidType};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::canister_management_error::CanisterManagementError;

// Define a new encompassing error type that includes GameError and LockError
#[derive(Error, Debug, CandidType, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserError {
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("failed to acquire lock")]
    LockError,

    #[error("serialization error: {0}")]
    SerializationError(String),

    #[error("Authorization error")]
    AuthorizationError,

    #[error("User not found")]
    UserNotFound,

    #[error("User already exists")]
    UserAlreadyExists,

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

    #[error("Canister management error: {0}")]
    ManagementCanisterError(#[from] CanisterManagementError),

    #[error("Canister call failed")]
    CanisterCallFailed(String),

    #[error("Minimum verification date does not match requested date")]
    MinimumVerificationDateMismatch,

    #[error("Invalid issuer")]
    InvalidIssuer,

    #[error("Invalid credential type: {0}")]
    InvalidCredentialType(String),

    #[error("Invalid credential structure")]
    InvalidCredentialStructure,

    #[error("Principal error: {0}")]
    PrincipalError(String),

    #[error("Error upgrading canister: {0}")]
    UpgradeError(String),
}

impl From<PrincipalError> for UserError {
    fn from(value: PrincipalError) -> Self {
        Self::PrincipalError(value.to_string())
    }
}
