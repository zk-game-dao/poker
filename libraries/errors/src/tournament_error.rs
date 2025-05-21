use candid::CandidType;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    canister_management_error::CanisterManagementError, table_error::TableError,
    user_error::UserError,
};

// Define a new encompassing error type that includes GameError and LockError
#[derive(Error, Debug, CandidType, Serialize, Deserialize, PartialEq, Eq)]
pub enum TournamentError {
    #[error("Tournament template not found")]
    TemplateNotFound,

    #[error("Tournament not found")]
    TournamentNotFound,

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Registration closed")]
    RegistrationClosed,

    #[error("Tournament full")]
    TournamentFull,

    #[error("Not registered")]
    NotRegistered,

    #[error("Already registered")]
    AlreadyRegistered,

    #[error("Canister call error: {0}")]
    CanisterCallError(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Not authorized")]
    NotAuthorized,

    #[error("Table error: {0}")]
    TableError(#[from] TableError),

    #[error("Canister management error: {0}")]
    ManagementCanisterError(#[from] CanisterManagementError),

    #[error("User error: {0}")]
    UserError(#[from] UserError),

    #[error("failed to acquire lock")]
    LockError,

    #[error("Table not found")]
    TableNotFound,

    #[error("Rebuy not allowed: {0}")]
    RebuyNotAllowed(String),

    #[error("Reentry not allowed: {0}")]
    ReentryNotAllowed(String),

    #[error("Addon not allowed: {0}")]
    AddonNotAllowed(String),

    #[error("Transfer failed: {0}")]
    TransferFailed(String),

    #[error("User not verified")]
    UserNotVerified,

    #[error("Error: {0}")]
    Other(String),
}
