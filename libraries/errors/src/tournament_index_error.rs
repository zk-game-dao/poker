use candid::CandidType;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    canister_management_error::CanisterManagementError, tournament_error::TournamentError,
};

// Define a new encompassing error type that includes GameError and LockError
#[derive(Error, Debug, CandidType, Serialize, Deserialize)]
pub enum TournamentIndexError {
    #[error("Tournament not found")]
    TournamentNotFound,

    #[error("Invalid currency")]
    InvalidCurrency,

    #[error("Invalid tournament config: {0}")]
    InvalidTournamentConfig(String),

    #[error("Canister management error: {0}")]
    ManagementCanisterError(#[from] CanisterManagementError),

    #[error("Tournament creation error: {0}")]
    TournamentCreationError(#[from] TournamentError),

    #[error("Insufficient cycles")]
    InsufficientCycles,

    #[error("Insufficient liquidity")]
    InsufficientLiquidity,

    #[error("Not authorized")]
    NotAuthorized,

    #[error("failed to acquire lock")]
    LockError,

    #[error("Canister call failed: {0}")]
    CanisterCallFailed(String),

    #[error("Failed to add to table pool: {0}")]
    FailedToAddToTablePool(String),

    #[error("Spin and go user pool not found")]
    PoolNotFound,

    #[error("Failed to add to user pool: {0}")]
    FailedToAddToUserPool(String),

    #[error("Failed to make canister call: {0}")]
    CanisterCallError(String),
}
