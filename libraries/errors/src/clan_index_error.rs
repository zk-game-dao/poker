use crate::{canister_management_error::CanisterManagementError, clan_error::ClanError};
use candid::{CandidType, Deserialize};
use thiserror::Error;

#[derive(CandidType, Deserialize, Debug, Error)]
pub enum ClanIndexError {
    // General errors
    #[error("Failed to acquire lock")]
    LockError,

    #[error("Not authorized to perform this action")]
    NotAuthorized,

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Canister call error: {0}")]
    CanisterCallError(String),

    #[error("Canister call failed: {0}")]
    CanisterCallFailed(String),

    // Clan-specific errors
    #[error("Clan not found")]
    ClanNotFound,

    #[error("Clan already exists")]
    ClanAlreadyExists,

    #[error("Clan tag already exists")]
    TagAlreadyExists,

    #[error("User is not a member of this clan")]
    UserNotMember,

    #[error("User is already a member of this clan")]
    UserAlreadyMember,

    #[error("Clan is at maximum capacity")]
    ClanFull,

    #[error("Insufficient permissions")]
    InsufficientPermissions,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Invalid subscription tier")]
    InvalidSubscriptionTier,

    #[error("Subscription has expired")]
    SubscriptionExpired,

    #[error("Joining fee is required")]
    JoiningFeeRequired,

    // Management errors
    #[error("Management canister error: {0}")]
    ManagementCanisterError(#[from] CanisterManagementError),

    #[error("Clan error: {0}")]
    ClanError(#[from] ClanError),
}
