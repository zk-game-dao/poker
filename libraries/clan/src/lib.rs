use candid::{CandidType, Principal};
use currency::Currency;
use errors::clan_error::ClanError;
use macros::impl_principal_traits;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use table::poker::game::table_functions::table::TableId;
use tournaments::tournaments::types::TournamentId;
use user::user::{User, UserAvatar, WalletPrincipalId};

use crate::{
    environment::ClanEnvironmentSettings,
    member::{ClanMember, MemberStatus},
    subscriptions::{ClanRole, SubscriptionTier, SubscriptionTierId},
    tags::ClanTag,
    treasury::ClanTreasury,
};

pub mod environment;
pub mod member;
pub mod search;
pub mod storable;
pub mod subscriptions;
pub mod tags;
pub mod treasury;

/// Privacy and access control settings
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum ClanPrivacy {
    Public,      // Anyone can join
    InviteOnly,  // Only invited users can join
    Application, // Users can apply, admins approve
}

/// Clan statistics and performance metrics
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct ClanStats {
    pub total_games_played: u64,
    pub total_tournaments_hosted: u64,
    pub total_revenue_generated: u64,
    pub most_active_day: Option<u64>, // Day of week (0-6)
    pub creation_date: u64,
    pub last_activity: u64,
}

impl Default for ClanStats {
    fn default() -> Self {
        let now = ic_cdk::api::time();
        Self {
            total_games_played: 0,
            total_tournaments_hosted: 0,
            total_revenue_generated: 0,
            most_active_day: None,
            creation_date: now,
            last_activity: now,
        }
    }
}

/// Pending join requests for clans with application-based entry
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct JoinRequest {
    pub applicant: WalletPrincipalId,
    pub message: Option<String>,
    pub requested_at: u64,
    pub referred_by: Option<WalletPrincipalId>, // If referred by existing member
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq, Hash, Copy)]
pub struct ClanId(pub Principal);

impl_principal_traits!(ClanId);

/// Main clan data structure
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct Clan {
    pub id: ClanId, // Unique clan identifier (could be canister ID)
    pub name: String,
    pub description: String,
    pub avatar: Option<UserAvatar>, // Reuse the existing avatar system

    // Currency - Each clan supports exactly one token
    pub supported_currency: Currency,

    // Membership
    pub members: HashMap<WalletPrincipalId, ClanMember>,
    pub member_limit: u32,
    pub pending_requests: Vec<JoinRequest>,
    pub invited_users: HashMap<WalletPrincipalId, u64>, // Principal -> invitation timestamp

    // Settings
    pub tags: HashSet<ClanTag>,
    pub privacy: ClanPrivacy,
    pub require_proof_of_humanity: bool,
    pub joining_fee: u64, // Fee required to join clan in supported_currency

    // Subscription System - Fully customizable by clan admins
    pub subscription_tiers: HashMap<SubscriptionTierId, SubscriptionTier>,
    pub subscription_enabled: bool,

    // Treasury and Economics
    pub treasury: ClanTreasury,

    // Customization
    pub environment_settings: ClanEnvironmentSettings,

    // Statistics
    pub stats: ClanStats,

    // Active Tables and Tournaments
    pub active_tables: Vec<TableId>,           // Table canister IDs
    pub hosted_tournaments: Vec<TournamentId>, // Tournament canister IDs

    // Metadata
    pub created_at: u64,
    pub created_by: WalletPrincipalId,
    pub website: Option<String>,
    pub discord: Option<String>,
    pub twitter: Option<String>,
}

impl Clan {
    pub fn new(
        id: ClanId,
        name: String,
        description: String,
        tags: Vec<ClanTag>,
        creator: WalletPrincipalId,
        privacy: ClanPrivacy,
        supported_currency: Currency,
        joining_fee: u64,
    ) -> Result<Self, ClanError> {
        // Validate inputs
        if name.is_empty() || name.len() > 50 {
            return Err(ClanError::InvalidClanName);
        }

        if description.len() > 500 {
            return Err(ClanError::InvalidDescription);
        }

        let now = ic_cdk::api::time();
        let mut members = HashMap::new();

        // Add creator as owner with elite tier
        let mut creator_member = ClanMember::new(creator, ClanRole::Owner);
        creator_member.subscription_tier = SubscriptionTierId::elite();
        members.insert(creator, creator_member);

        // Initialize default subscription tiers
        let mut subscription_tiers = HashMap::new();
        subscription_tiers.insert(SubscriptionTierId::basic(), SubscriptionTier::new_basic());
        subscription_tiers.insert(
            SubscriptionTierId::premium(),
            SubscriptionTier::new_premium(),
        );
        subscription_tiers.insert(SubscriptionTierId::elite(), SubscriptionTier::new_elite());

        Ok(Self {
            id,
            name,
            description,
            tags: tags.into_iter().collect(),
            avatar: None,
            supported_currency,
            members,
            member_limit: 100, // Default limit
            pending_requests: Vec::new(),
            invited_users: HashMap::new(),
            privacy,
            require_proof_of_humanity: false,
            joining_fee,
            subscription_tiers,
            subscription_enabled: true, // Enable subscriptions by default
            treasury: ClanTreasury::default(),
            environment_settings: ClanEnvironmentSettings::default(),
            stats: ClanStats::default(),
            active_tables: Vec::new(),
            hosted_tournaments: Vec::new(),
            created_at: now,
            created_by: creator,
            website: None,
            discord: None,
            twitter: None,
        })
    }

    /// Check if a user is a member of the clan
    pub fn is_member(&self, principal: &WalletPrincipalId) -> bool {
        self.members.contains_key(principal)
    }

    /// Get member count
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Check if clan is at capacity
    pub fn is_full(&self) -> bool {
        self.member_count() >= self.member_limit as usize
    }

    /// Add a new member to the clan
    pub fn add_member(
        &mut self,
        principal: WalletPrincipalId,
        role: ClanRole,
    ) -> Result<(), ClanError> {
        if self.is_full() {
            return Err(ClanError::ClanFull(self.member_limit));
        }

        if self.is_member(&principal) {
            return Err(ClanError::UserAlreadyMember);
        }

        self.members
            .insert(principal, ClanMember::new(principal, role));

        // Remove from pending requests if applicable
        self.pending_requests
            .retain(|req| req.applicant != principal);

        // Remove from invited users if applicable
        self.invited_users.remove(&principal);

        Ok(())
    }

    /// Process joining fee payment and add member
    pub fn join_with_fee(
        &mut self,
        principal: WalletPrincipalId,
        paid_amount: u64,
    ) -> Result<(), ClanError> {
        if paid_amount < self.joining_fee {
            return Err(ClanError::InsufficientJoiningFee {
                required: self.joining_fee,
                paid: paid_amount,
            });
        }

        // Add the joining fee to treasury
        self.treasury.balance += self.joining_fee;
        self.treasury.total_joining_fees_collected += self.joining_fee;

        // If they paid more than required, the excess goes to treasury as donation
        if paid_amount > self.joining_fee {
            self.treasury.balance += paid_amount - self.joining_fee;
        }

        // Add them as a regular member
        self.add_member(principal, ClanRole::Member)?;

        Ok(())
    }

    /// Remove a member from the clan
    pub fn remove_member(
        &mut self,
        principal: &WalletPrincipalId,
    ) -> Result<ClanMember, ClanError> {
        // Can't remove the owner
        if let Some(member) = self.members.get(principal) {
            if member.role == ClanRole::Owner {
                return Err(ClanError::CannotRemoveOwner);
            }
        }

        self.members
            .remove(principal)
            .ok_or(ClanError::MemberNotFound)
    }

    /// Get members by role
    pub fn get_members_by_role(&self, role: ClanRole) -> Vec<&ClanMember> {
        self.members
            .values()
            .filter(|member| member.role == role)
            .collect()
    }

    /// Get active members (not suspended)
    pub fn get_active_members(&self) -> Vec<&ClanMember> {
        self.members
            .values()
            .filter(|member| member.status == MemberStatus::Active)
            .collect()
    }

    /// Update member role
    pub fn update_member_role(
        &mut self,
        principal: &WalletPrincipalId,
        new_role: ClanRole,
        updater: &WalletPrincipalId,
    ) -> Result<(), ClanError> {
        let updater_member = self.members.get(updater).ok_or(ClanError::MemberNotFound)?;

        let target_member = self
            .members
            .get(principal)
            .ok_or(ClanError::MemberNotFound)?;

        // Only owners can promote to admin, only admin+ can change roles
        match new_role {
            ClanRole::Owner | ClanRole::Admin => {
                if updater_member.role != ClanRole::Owner {
                    return Err(ClanError::InsufficientPermissions);
                }
            }
            _ => {
                if !updater_member.is_admin_or_higher() {
                    return Err(ClanError::InsufficientPermissions);
                }
            }
        }

        // Can't demote the owner
        if target_member.role == ClanRole::Owner {
            return Err(ClanError::CannotRemoveOwner);
        }

        if let Some(member) = self.members.get_mut(principal) {
            member.role = new_role;
        }

        Ok(())
    }

    /// Update joining fee (admin+ only)
    pub fn update_joining_fee(
        &mut self,
        new_fee: u64,
        updater: &WalletPrincipalId,
    ) -> Result<(), ClanError> {
        let updater_member = self.members.get(updater).ok_or(ClanError::MemberNotFound)?;

        if !updater_member.is_admin_or_higher() {
            return Err(ClanError::InsufficientPermissions);
        }

        self.joining_fee = new_fee;
        Ok(())
    }

    /// Get clan leaderboard by contribution points
    pub fn get_contribution_leaderboard(&self) -> Vec<(&WalletPrincipalId, u64)> {
        let mut members: Vec<_> = self
            .members
            .iter()
            .map(|(principal, member)| (principal, member.contribution_points))
            .collect();

        members.sort_by(|a, b| b.1.cmp(&a.1));
        members
    }

    /// Get clan leaderboard by games played
    pub fn get_games_leaderboard(&self) -> Vec<(&WalletPrincipalId, u64)> {
        let mut members: Vec<_> = self
            .members
            .iter()
            .map(|(principal, member)| (principal, member.games_played))
            .collect();

        members.sort_by(|a, b| b.1.cmp(&a.1));
        members
    }

    /// Get clan leaderboard by total winnings
    pub fn get_winnings_leaderboard(&self) -> Vec<(&WalletPrincipalId, u64)> {
        let mut members: Vec<_> = self
            .members
            .iter()
            .map(|(principal, member)| (principal, member.total_winnings))
            .collect();

        members.sort_by(|a, b| b.1.cmp(&a.1));
        members
    }

    /// Check if user meets joining requirements (excluding fee)
    pub fn meets_joining_requirements(&self, user: &User) -> Result<(), ClanError> {
        if self.require_proof_of_humanity && !user.is_verified.unwrap_or(false) {
            return Err(ClanError::VerificationRequired);
        }

        Ok(())
    }
}

/// Events that can occur in a clan (for audit trail and notifications)
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum ClanEvent {
    MemberJoined {
        member: WalletPrincipalId,
        joining_fee_paid: u64,
        timestamp: u64,
    },
    MemberLeft {
        member: WalletPrincipalId,
        timestamp: u64,
    },
    MemberPromoted {
        member: WalletPrincipalId,
        new_role: ClanRole,
        by: WalletPrincipalId,
        timestamp: u64,
    },
    MemberSuspended {
        member: WalletPrincipalId,
        by: WalletPrincipalId,
        until: Option<u64>,
        timestamp: u64,
    },
    TournamentHosted {
        tournament_id: WalletPrincipalId,
        timestamp: u64,
    },
    RevenueGenerated {
        amount: u64,
        timestamp: u64,
    },
    RewardDistributed {
        amount: u64,
        to: WalletPrincipalId,
        timestamp: u64,
    },
    JoiningFeeUpdated {
        old_fee: u64,
        new_fee: u64,
        by: WalletPrincipalId,
        timestamp: u64,
    },
    SettingsUpdated {
        by: WalletPrincipalId,
        timestamp: u64,
    },
}

/// Clan invitation structure
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct ClanInvitation {
    pub clan_id: ClanId,
    pub clan_name: String,
    pub clan_tag: String,
    pub invited_by: WalletPrincipalId,
    pub invited_at: u64,
    pub expires_at: Option<u64>,
    pub message: Option<String>,
}

/// Create/Update clan request structure
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CreateClanRequest {
    pub name: String,
    pub description: String,
    pub tag: String,
    pub privacy: ClanPrivacy,
    pub supported_currency: Currency,
    pub joining_fee: u64,
    pub require_proof_of_humanity: bool,
    pub minimum_level_required: Option<f64>,
    pub minimum_experience_points: Option<u64>,
    pub member_limit: Option<u32>,
    pub avatar: Option<UserAvatar>,
    pub website: Option<String>,
    pub discord: Option<String>,
    pub twitter: Option<String>,
}
