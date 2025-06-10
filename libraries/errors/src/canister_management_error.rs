use candid::CandidType;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, CandidType, Deserialize, Serialize, PartialEq, Eq)]
pub enum CanisterManagementError {
    #[error("Failed to create canister: {0}")]
    CreateCanisterError(String),

    #[error("Failed to install code: {0}")]
    InstallCodeError(String),

    #[error("Failed to stop canister: {0}")]
    StopCanisterError(String),

    #[error("Failed to delete canister: {0}")]
    DeleteCanisterError(String),

    #[error("Failed to get canister status: {0}")]
    ManagementCanisterError(String),

    #[error("Failed to deposit cycles: {0}")]
    Transfer(String),

    #[error("Insufficient cycles")]
    InsufficientCycles,

    #[error("{0}")]
    LedgerError(String),

    #[error("{0}")]
    QueryError(String),

    #[error("Failed to call canister: {0}")]
    CanisterCallError(String),
}
