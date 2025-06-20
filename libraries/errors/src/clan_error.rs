use candid::{CandidType, Deserialize};
use currency::currency_error::CurrencyError;
use thiserror::Error;

use crate::{canister_management_error::CanisterManagementError, user_error::UserError};

#[derive(CandidType, Deserialize, Debug, Error)]
pub enum ClanError {
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
    
    // Clan management errors
    #[error("Clan not found")]
    ClanNotFound,
    
    #[error("Clan already exists")]
    ClanAlreadyExists,
    
    #[error("Clan tag '{0}' already exists")]
    TagAlreadyExists(String),
    
    #[error("Clan name must be between 1 and 50 characters")]
    InvalidClanName,
    
    #[error("Clan tag must be between 1 and 10 characters")]
    InvalidClanTag,
    
    #[error("Clan description must be under 500 characters")]
    InvalidDescription,
    
    #[error("Clan is at maximum capacity ({0} members)")]
    ClanFull(u32),

    #[error("Clan is not initialized")]
    StateNotInitialized,
    
    // Member management errors
    #[error("User is not a member of this clan")]
    UserNotMember,
    
    #[error("User is already a member of this clan")]
    UserAlreadyMember,
    
    #[error("Member not found")]
    MemberNotFound,
    
    #[error("Cannot remove clan owner")]
    CannotRemoveOwner,
    
    #[error("Cannot change owner role")]
    CannotChangeOwnerRole,
    
    #[error("Member is suspended until {0}")]
    MemberSuspended(u64),
    
    // Permission errors
    #[error("Insufficient permissions to perform this action")]
    InsufficientPermissions,
    
    #[error("Only clan owner can promote to admin")]
    OnlyOwnerCanPromoteAdmin,
    
    #[error("Only admins and owners can update {0}")]
    AdminOnlyAction(String),
    
    #[error("Only moderators and above can perform this action")]
    ModeratorRequired,
    
    // Financial errors
    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: u64, available: u64 },
    
    #[error("Insufficient joining fee: required {required}, paid {paid}")]
    InsufficientJoiningFee { required: u64, paid: u64 },
    
    #[error("Insufficient treasury balance: required {required}, available {available}")]
    InsufficientTreasuryBalance { required: u64, available: u64 },
    
    #[error("Invalid reward distribution: total {total} exceeds available {available}")]
    InvalidRewardDistribution { total: u64, available: u64 },
    
    // Subscription errors
    #[error("Subscription tier '{0}' not found")]
    SubscriptionTierNotFound(String),
    
    #[error("Subscription tier '{0}' is not active")]
    SubscriptionTierInactive(String),
    
    #[error("Subscription has expired on {0}")]
    SubscriptionExpired(u64),
    
    #[error("Cannot remove basic subscription tier")]
    CannotRemoveBasicTier,
    
    #[error("Cannot remove tier: {0} members are currently using it")]
    TierInUse(usize),
    
    #[error("Insufficient subscription payment: required {required}, paid {paid}")]
    InsufficientSubscriptionPayment { required: u64, paid: u64 },
    
    #[error("Subscription tier name must be between 1 and 50 characters")]
    InvalidTierName,
    
    #[error("Revenue share bonus cannot exceed 50%")]
    InvalidRevenueShareBonus,
    
    // Requirement errors
    #[error("Minimum level {0} required")]
    MinimumLevelRequired(u64),
    
    #[error("Minimum {0} experience points required")]
    MinimumExperienceRequired(u64),
    
    #[error("Minimum {0} contribution points required")]
    MinimumContributionRequired(u64),
    
    #[error("Proof of humanity verification required")]
    VerificationRequired,
    
    #[error("Minimum membership duration not met")]
    MinimumMembershipDurationNotMet,
    
    #[error("Minimum {0} games played required")]
    MinimumGamesRequired(u64),
    
    #[error("Invitation required for this tier")]
    InvitationRequired,
    
    #[error("Table stakes {stakes} exceed tier limit {limit}")]
    TableStakesExceedLimit { stakes: u64, limit: u64 },
    
    // Access control errors
    #[error("Feature '{0}' not available for your subscription tier")]
    FeatureNotAvailable(String),
    
    #[error("Tournament access not available for your subscription tier")]
    TournamentAccessDenied,
    
    #[error("Cannot create tournaments with current subscription tier")]
    CannotCreateTournaments,
    
    #[error("Analytics access not available for your subscription tier")]
    AnalyticsAccessDenied,
    
    #[error("Exclusive access required")]
    ExclusiveAccessRequired,
    
    #[error("Cannot invite members with current subscription tier")]
    CannotInviteMembers,
    
    // Join request errors
    #[error("Join request not found")]
    JoinRequestNotFound,
    
    #[error("User has already applied to join")]
    AlreadyApplied,
    
    #[error("User is already invited")]
    AlreadyInvited,
    
    #[error("Invitation has expired")]
    InvitationExpired,
    
    #[error("Join request message is too long (max 500 characters)")]
    JoinRequestMessageTooLong,
    
    // Privacy errors
    #[error("Clan is invite-only")]
    InviteOnlyAccess,
    
    #[error("Clan requires application approval")]
    ApplicationRequired,
    
    // Environment/Customization errors
    #[error("Custom styling not available for your subscription tier")]
    CustomStylingNotAvailable,
    
    #[error("Invalid URL format: {0}")]
    InvalidUrl(String),
    
    #[error("Logo URL is too long (max 500 characters)")]
    LogoUrlTooLong,
    
    #[error("Background URL is too long (max 500 characters)")]
    BackgroundUrlTooLong,
    
    #[error("Welcome message is too long (max 1000 characters)")]
    WelcomeMessageTooLong,
    
    // Statistics/Analytics errors
    #[error("Insufficient data for analytics")]
    InsufficientAnalyticsData,
    
    #[error("Analytics period too large")]
    AnalyticsPeriodTooLarge,
    
    // Tournament/Table errors
    #[error("Table not found")]
    TableNotFound,
    
    #[error("Tournament not found")]
    TournamentNotFound,
    
    #[error("Table is not owned by this clan")]
    TableNotOwnedByClan,
    
    #[error("Tournament is not hosted by this clan")]
    TournamentNotHostedByClan,
    
    // Validation errors
    #[error("Invalid currency for this clan")]
    InvalidCurrency,
    
    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(u64),
    
    #[error("Invalid member limit: must be between 1 and 1000")]
    InvalidMemberLimit,
    
    #[error("Invalid percentage: must be between 0 and 100")]
    InvalidPercentage,
    
    // System errors
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Internal system error: {0}")]
    InternalError(String),

    #[error("User is banned")]
    UserBanned,

    #[error("User error: {0}")]
    UserError(#[from] UserError),

    #[error("Currency error: {0}")]
    CurrencyError(#[from] CurrencyError),

    #[error("Canister management error: {0}")]
    ManagementCanisterError(#[from] CanisterManagementError),
}

impl ClanError {
    /// Helper method to create an insufficient funds error
    pub fn insufficient_funds(required: u64, available: u64) -> Self {
        Self::InsufficientFunds { required, available }
    }
    
    /// Helper method to create an insufficient joining fee error
    pub fn insufficient_joining_fee(required: u64, paid: u64) -> Self {
        Self::InsufficientJoiningFee { required, paid }
    }
    
    /// Helper method to create an insufficient subscription payment error
    pub fn insufficient_subscription_payment(required: u64, paid: u64) -> Self {
        Self::InsufficientSubscriptionPayment { required, paid }
    }
    
    /// Helper method to create a table stakes exceed limit error
    pub fn table_stakes_exceed_limit(stakes: u64, limit: u64) -> Self {
        Self::TableStakesExceedLimit { stakes, limit }
    }
    
    /// Helper method to create a clan full error
    pub fn clan_full(member_limit: u32) -> Self {
        Self::ClanFull(member_limit)
    }
    
    /// Helper method to create a tier in use error
    pub fn tier_in_use(member_count: usize) -> Self {
        Self::TierInUse(member_count)
    }
}
