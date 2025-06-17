use candid::{CandidType, Decode, Encode, Principal};
use serde::{Deserialize, Serialize};
use user::user::{User, UserAvatar};
use std::collections::HashMap;
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

use table::poker::game::table_functions::types::CurrencyType;

const MAX_CLAN_SIZE: u32 = 50_000_000; // 50MB max size for clan data

/// Represents the role a member has within a clan
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum ClanRole {
    Owner,
    Admin,
    Moderator,
    Member,
}

/// Represents the status of a clan member
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum MemberStatus {
    Active,
    Inactive,
    Suspended { until: Option<u64> }, // Timestamp when suspension ends
}

/// Individual clan member data
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct ClanMember {
    pub principal_id: Principal,
    pub role: ClanRole,
    pub status: MemberStatus,
    pub joined_at: u64, // Timestamp when member joined
    pub contribution_points: u64, // Points earned for clan activities
    pub games_played: u64,
    pub tournaments_won: u64,
    pub xp: u64, // Experience points earned
    pub total_winnings: u64, // In smallest currency unit
    pub last_active: u64,
}

impl ClanMember {
    pub fn new(principal_id: Principal, role: ClanRole) -> Self {
        let now = ic_cdk::api::time();
        Self {
            principal_id,
            role,
            status: MemberStatus::Active,
            joined_at: now,
            contribution_points: 0,
            games_played: 0,
            tournaments_won: 0,
            total_winnings: 0,
            xp: 0,
            last_active: now,
        }
    }

    pub fn is_admin_or_higher(&self) -> bool {
        matches!(self.role, ClanRole::Owner | ClanRole::Admin)
    }

    pub fn can_moderate(&self) -> bool {
        matches!(self.role, ClanRole::Owner | ClanRole::Admin | ClanRole::Moderator)
    }
}

/// Reward distribution configuration
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum RewardDistributionType {
    Percentage(u8), // 0-100, percentage of treasury balance
    FixedAmount(u64), // Fixed amount in clan's supported currency
}

impl Default for RewardDistributionType {
    fn default() -> Self {
        RewardDistributionType::Percentage(80) // Default 80% of treasury
    }
}

/// Clan treasury configuration and balances
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct ClanTreasury {
    pub balance: u64, // Balance in the clan's single supported currency
    pub revenue_share_percentage: u8, // Fixed at 50% for MVP - 50% to platform, 50% to clan
    pub reward_distribution: RewardDistributionType, // How rewards are calculated
    pub total_revenue_generated: u64,
    pub total_rewards_distributed: u64,
    pub total_joining_fees_collected: u64,
}

impl Default for ClanTreasury {
    fn default() -> Self {
        Self {
            balance: 0,
            revenue_share_percentage: 50, // Fixed 50/50 split for MVP
            reward_distribution: RewardDistributionType::default(),
            total_revenue_generated: 0,
            total_rewards_distributed: 0,
            total_joining_fees_collected: 0,
        }
    }
}

/// Custom table and environment settings for clan games
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct ClanEnvironmentSettings {
    pub default_table_color: u64,
    pub default_card_color: u64,
    pub default_environment_color: u64,
    pub custom_logo_url: Option<String>,
    pub custom_background_url: Option<String>,
    pub table_name_prefix: Option<String>, // e.g., "[CLAN] Table Name"
    pub welcome_message: Option<String>,
}

impl Default for ClanEnvironmentSettings {
    fn default() -> Self {
        Self {
            default_table_color: 0,
            default_card_color: 0,
            default_environment_color: 0,
            custom_logo_url: None,
            custom_background_url: None,
            table_name_prefix: None,
            welcome_message: None,
        }
    }
}

/// Privacy and access control settings
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum ClanPrivacy {
    Public,    // Anyone can join
    InviteOnly, // Only invited users can join
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
    pub applicant: Principal,
    pub message: Option<String>,
    pub requested_at: u64,
    pub referred_by: Option<Principal>, // If referred by existing member
}

/// Main clan data structure
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct Clan {
    pub id: Principal, // Unique clan identifier (could be canister ID)
    pub name: String,
    pub description: String,
    pub tag: String, // Short clan tag (e.g., "ZKGM")
    pub avatar: Option<UserAvatar>, // Reuse the existing avatar system
    
    // Currency - Each clan supports exactly one token
    pub supported_currency: CurrencyType,
    
    // Membership
    pub members: HashMap<Principal, ClanMember>,
    pub member_limit: u32,
    pub pending_requests: Vec<JoinRequest>,
    pub invited_users: HashMap<Principal, u64>, // Principal -> invitation timestamp
    
    // Settings
    pub privacy: ClanPrivacy,
    pub require_proof_of_humanity: bool,
    pub joining_fee: u64, // Fee required to join clan in supported_currency
    
    // Treasury and Economics
    pub treasury: ClanTreasury,
    
    // Customization
    pub environment_settings: ClanEnvironmentSettings,
    
    // Statistics
    pub stats: ClanStats,
    
    // Active Tables and Tournaments
    pub active_tables: Vec<Principal>, // Table canister IDs
    pub hosted_tournaments: Vec<Principal>, // Tournament canister IDs
    
    // Metadata
    pub created_at: u64,
    pub created_by: Principal,
    pub website: Option<String>,
    pub discord: Option<String>,
    pub twitter: Option<String>,
}

impl Clan {
    pub fn new(
        id: Principal,
        name: String,
        description: String,
        tag: String,
        creator: Principal,
        privacy: ClanPrivacy,
        supported_currency: CurrencyType,
        joining_fee: u64,
    ) -> Result<Self, String> {
        // Validate inputs
        if name.is_empty() || name.len() > 50 {
            return Err("Clan name must be 1-50 characters".to_string());
        }
        
        if tag.is_empty() || tag.len() > 10 {
            return Err("Clan tag must be 1-10 characters".to_string());
        }
        
        if description.len() > 500 {
            return Err("Description must be under 500 characters".to_string());
        }

        let now = ic_cdk::api::time();
        let mut members = HashMap::new();
        
        // Add creator as owner
        members.insert(creator, ClanMember::new(creator, ClanRole::Owner));

        Ok(Self {
            id,
            name,
            description,
            tag: tag.to_uppercase(), // Standardize tags to uppercase
            avatar: None,
            supported_currency,
            members,
            member_limit: 100, // Default limit
            pending_requests: Vec::new(),
            invited_users: HashMap::new(),
            privacy,
            require_proof_of_humanity: false,
            joining_fee,
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
    pub fn is_member(&self, principal: &Principal) -> bool {
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
    pub fn add_member(&mut self, principal: Principal, role: ClanRole) -> Result<(), String> {
        if self.is_full() {
            return Err("Clan is at maximum capacity".to_string());
        }

        if self.is_member(&principal) {
            return Err("User is already a member".to_string());
        }

        self.members.insert(principal, ClanMember::new(principal, role));
        
        // Remove from pending requests if applicable
        self.pending_requests.retain(|req| req.applicant != principal);
        
        // Remove from invited users if applicable
        self.invited_users.remove(&principal);

        Ok(())
    }

    /// Process joining fee payment and add member
    pub fn join_with_fee(&mut self, principal: Principal, paid_amount: u64) -> Result<(), String> {
        if paid_amount < self.joining_fee {
            return Err(format!("Insufficient joining fee. Required: {}, Paid: {}", self.joining_fee, paid_amount));
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
    pub fn remove_member(&mut self, principal: &Principal) -> Result<ClanMember, String> {
        // Can't remove the owner
        if let Some(member) = self.members.get(principal) {
            if member.role == ClanRole::Owner {
                return Err("Cannot remove clan owner".to_string());
            }
        }

        self.members.remove(principal)
            .ok_or_else(|| "Member not found".to_string())
    }

    /// Get members by role
    pub fn get_members_by_role(&self, role: ClanRole) -> Vec<&ClanMember> {
        self.members.values()
            .filter(|member| member.role == role)
            .collect()
    }

    /// Get active members (not suspended)
    pub fn get_active_members(&self) -> Vec<&ClanMember> {
        self.members.values()
            .filter(|member| member.status == MemberStatus::Active)
            .collect()
    }

    /// Update member role
    pub fn update_member_role(&mut self, principal: &Principal, new_role: ClanRole, updater: &Principal) -> Result<(), String> {
        let updater_member = self.members.get(updater)
            .ok_or("Updater is not a clan member")?;

        let target_member = self.members.get(principal)
            .ok_or("Target member not found")?;

        // Only owners can promote to admin, only admin+ can change roles
        match new_role {
            ClanRole::Owner | ClanRole::Admin => {
                if updater_member.role != ClanRole::Owner {
                    return Err("Only clan owner can promote to admin".to_string());
                }
            },
            _ => {
                if !updater_member.is_admin_or_higher() {
                    return Err("Insufficient permissions to change roles".to_string());
                }
            }
        }

        // Can't demote the owner
        if target_member.role == ClanRole::Owner {
            return Err("Cannot change owner role".to_string());
        }

        if let Some(member) = self.members.get_mut(principal) {
            member.role = new_role;
        }

        Ok(())
    }

    /// Update joining fee (admin+ only)
    pub fn update_joining_fee(&mut self, new_fee: u64, updater: &Principal) -> Result<(), String> {
        let updater_member = self.members.get(updater)
            .ok_or("Updater is not a clan member")?;

        if !updater_member.is_admin_or_higher() {
            return Err("Only admins and owners can update joining fee".to_string());
        }

        self.joining_fee = new_fee;
        Ok(())
    }

    /// Update reward distribution method (admin+ only)
    pub fn update_reward_distribution(&mut self, new_distribution: RewardDistributionType, updater: &Principal) -> Result<(), String> {
        let updater_member = self.members.get(updater)
            .ok_or("Updater is not a clan member")?;

        if !updater_member.is_admin_or_higher() {
            return Err("Only admins and owners can update reward distribution".to_string());
        }

        self.treasury.reward_distribution = new_distribution;
        Ok(())
    }

    /// Calculate available reward amount based on current distribution type
    pub fn calculate_available_reward_amount(&self) -> u64 {
        match &self.treasury.reward_distribution {
            RewardDistributionType::Percentage(percentage) => {
                (self.treasury.balance * (*percentage as u64)) / 100
            },
            RewardDistributionType::FixedAmount(amount) => {
                // Return the fixed amount or treasury balance, whichever is smaller
                std::cmp::min(*amount, self.treasury.balance)
            }
        }
    }

    /// Distribute rewards to members from treasury
    pub fn distribute_rewards(&mut self, distribution: HashMap<Principal, u64>) -> Result<(), String> {
        let distribution_total: u64 = distribution.values().sum();
        let available_amount = self.calculate_available_reward_amount();
        
        if distribution_total > available_amount {
            return Err(format!("Distribution total ({}) exceeds available reward amount ({})", distribution_total, available_amount));
        }

        if distribution_total > self.treasury.balance {
            return Err("Insufficient treasury balance".to_string());
        }

        // Deduct from treasury
        self.treasury.balance -= distribution_total;
        self.treasury.total_rewards_distributed += distribution_total;

        Ok(())
    }

    /// Get clan leaderboard by contribution points
    pub fn get_contribution_leaderboard(&self) -> Vec<(&Principal, u64)> {
        let mut members: Vec<_> = self.members.iter()
            .map(|(principal, member)| (principal, member.contribution_points))
            .collect();
        
        members.sort_by(|a, b| b.1.cmp(&a.1));
        members
    }

    /// Get clan leaderboard by games played
    pub fn get_games_leaderboard(&self) -> Vec<(&Principal, u64)> {
        let mut members: Vec<_> = self.members.iter()
            .map(|(principal, member)| (principal, member.games_played))
            .collect();
        
        members.sort_by(|a, b| b.1.cmp(&a.1));
        members
    }

    /// Get clan leaderboard by total winnings
    pub fn get_winnings_leaderboard(&self) -> Vec<(&Principal, u64)> {
        let mut members: Vec<_> = self.members.iter()
            .map(|(principal, member)| (principal, member.total_winnings))
            .collect();
        
        members.sort_by(|a, b| b.1.cmp(&a.1));
        members
    }

    /// Check if user meets joining requirements (excluding fee)
    pub fn meets_joining_requirements(&self, user: &User) -> Result<(), String> {
        if self.require_proof_of_humanity && !user.is_verified.unwrap_or(false) {
            return Err("Proof of humanity verification required".to_string());
        }

        Ok(())
    }
}

// Implement Storable for Clan to work with ic_stable_structures
impl Storable for Clan {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap_or_else(|e| {
            ic_cdk::println!("Clan serialization error: {:?}", e);
            vec![]
        }))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_else(|e| {
            ic_cdk::println!("Clan deserialization error: {:?}", e);
            // Return a default clan that should not be used
            Clan {
                id: Principal::anonymous(),
                name: "ERROR".to_string(),
                description: "".to_string(),
                tag: "ERR".to_string(),
                avatar: None,
                supported_currency: CurrencyType::Fake,
                members: HashMap::new(),
                member_limit: 0,
                pending_requests: Vec::new(),
                invited_users: HashMap::new(),
                privacy: ClanPrivacy::Public,
                require_proof_of_humanity: false,
                joining_fee: 0,
                treasury: ClanTreasury::default(),
                environment_settings: ClanEnvironmentSettings::default(),
                stats: ClanStats::default(),
                active_tables: Vec::new(),
                hosted_tournaments: Vec::new(),
                created_at: 0,
                created_by: Principal::anonymous(),
                website: None,
                discord: None,
                twitter: None,
            }
        })
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_CLAN_SIZE,
        is_fixed_size: false,
    };
}

/// Events that can occur in a clan (for audit trail and notifications)
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum ClanEvent {
    MemberJoined { member: Principal, joining_fee_paid: u64, timestamp: u64 },
    MemberLeft { member: Principal, timestamp: u64 },
    MemberPromoted { member: Principal, new_role: ClanRole, by: Principal, timestamp: u64 },
    MemberSuspended { member: Principal, by: Principal, until: Option<u64>, timestamp: u64 },
    TournamentHosted { tournament_id: Principal, timestamp: u64 },
    RevenueGenerated { amount: u64, timestamp: u64 },
    RewardDistributed { amount: u64, to: Principal, timestamp: u64 },
    JoiningFeeUpdated { old_fee: u64, new_fee: u64, by: Principal, timestamp: u64 },
    SettingsUpdated { by: Principal, timestamp: u64 },
}

/// Clan invitation structure
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct ClanInvitation {
    pub clan_id: Principal,
    pub clan_name: String,
    pub clan_tag: String,
    pub invited_by: Principal,
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
    pub supported_currency: CurrencyType,
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
