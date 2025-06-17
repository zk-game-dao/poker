use candid::{CandidType, Decode, Encode, Principal};
use currency::Currency;
use errors::clan_error::ClanError;
use serde::{Deserialize, Serialize};
use user::user::{User, UserAvatar};
use std::collections::HashMap;
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

const MAX_CLAN_SIZE: u32 = 50_000_000; // 50MB max size for clan data

/// Represents the role a member has within a clan
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum ClanRole {
    Owner,
    Admin,
    Moderator,
    Member,
}

/// Subscription tier identifier - clans can create custom tiers
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SubscriptionTierId(pub String);

impl SubscriptionTierId {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
    
    pub fn basic() -> Self {
        Self("Basic".to_string())
    }
    
    pub fn premium() -> Self {
        Self("Premium".to_string())
    }
    
    pub fn elite() -> Self {
        Self("Elite".to_string())
    }
}

impl Default for SubscriptionTierId {
    fn default() -> Self {
        Self::basic()
    }
}

/// Requirements that must be met to access a subscription tier
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct SubscriptionRequirements {
    /// Monthly payment required in clan's supported currency
    pub monthly_payment: Option<u64>,
    
    /// One-time payment required in clan's supported currency
    pub one_time_payment: Option<u64>,
    
    /// Minimum user level required
    pub minimum_level: Option<u64>,
    
    /// Minimum experience points required
    pub minimum_experience_points: Option<u64>,
    
    /// Minimum contribution points within the clan
    pub minimum_contribution_points: Option<u64>,
    
    /// Must be invited by an admin/owner
    pub requires_invitation: bool,
    
    /// Must be verified (proof of humanity)
    pub requires_verification: bool,
    
    /// Minimum time as clan member (in nanoseconds)
    pub minimum_membership_duration: Option<u64>,
    
    /// Minimum games played within clan
    pub minimum_games_played: Option<u64>,
}

impl Default for SubscriptionRequirements {
    fn default() -> Self {
        Self {
            monthly_payment: None,
            one_time_payment: None,
            minimum_level: None,
            minimum_experience_points: None,
            minimum_contribution_points: None,
            requires_invitation: false,
            requires_verification: false,
            minimum_membership_duration: None,
            minimum_games_played: None,
        }
    }
}

/// Benefits provided by a subscription tier
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct SubscriptionBenefits {
    /// Custom description of benefits
    pub description: String,
    
    /// Maximum stakes for tables this tier can access (None = unlimited)
    pub max_table_stakes: Option<u64>,
    
    /// Can participate in clan tournaments
    pub tournament_access: bool,
    
    /// Can create tournaments for the clan
    pub can_create_tournaments: bool,
    
    /// Gets priority in support/moderation
    pub priority_support: bool,
    
    /// Can set custom avatar/colors in clan
    pub custom_styling: bool,
    
    /// Additional percentage points for revenue sharing (0-50)
    pub revenue_share_bonus: u8,
    
    /// Can access exclusive clan channels/areas
    pub exclusive_access: bool,
    
    /// Can invite new members to clan
    pub can_invite_members: bool,
    
    /// Can see detailed clan analytics
    pub analytics_access: bool,
    
    /// Custom role name displayed in clan
    pub custom_role_name: Option<String>,
}

impl Default for SubscriptionBenefits {
    fn default() -> Self {
        Self {
            description: "Basic clan access".to_string(),
            max_table_stakes: Some(1000), // Default low stakes
            tournament_access: false,
            can_create_tournaments: false,
            priority_support: false,
            custom_styling: false,
            revenue_share_bonus: 0,
            exclusive_access: false,
            can_invite_members: false,
            analytics_access: false,
            custom_role_name: None,
        }
    }
}

/// A complete subscription tier configuration
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct SubscriptionTier {
    pub id: SubscriptionTierId,
    pub name: String,
    pub requirements: SubscriptionRequirements,
    pub benefits: SubscriptionBenefits,
    pub is_active: bool, // Admins can disable tiers
    pub tier_order: u32, // For ordering tiers (higher number = higher tier)
}

impl SubscriptionTier {
    pub fn new_basic() -> Self {
        Self {
            id: SubscriptionTierId::basic(),
            name: "Basic".to_string(),
            requirements: SubscriptionRequirements::default(),
            benefits: SubscriptionBenefits::default(),
            is_active: true,
            tier_order: 1,
        }
    }
    
    pub fn new_premium() -> Self {
        Self {
            id: SubscriptionTierId::premium(),
            name: "Premium".to_string(),
            requirements: SubscriptionRequirements {
                monthly_payment: Some(1000),
                minimum_level: Some(5),
                ..Default::default()
            },
            benefits: SubscriptionBenefits {
                description: "Enhanced clan access with tournaments".to_string(),
                max_table_stakes: Some(10000),
                tournament_access: true,
                priority_support: true,
                revenue_share_bonus: 2,
                can_invite_members: true,
                ..Default::default()
            },
            is_active: true,
            tier_order: 2,
        }
    }
    
    pub fn new_elite() -> Self {
        Self {
            id: SubscriptionTierId::elite(),
            name: "Elite".to_string(),
            requirements: SubscriptionRequirements {
                monthly_payment: Some(5000),
                minimum_level: Some(10),
                minimum_contribution_points: Some(1000),
                ..Default::default()
            },
            benefits: SubscriptionBenefits {
                description: "Full clan access with all privileges".to_string(),
                max_table_stakes: None, // Unlimited
                tournament_access: true,
                can_create_tournaments: true,
                priority_support: true,
                custom_styling: true,
                revenue_share_bonus: 5,
                exclusive_access: true,
                can_invite_members: true,
                analytics_access: true,
                custom_role_name: Some("Elite Member".to_string()),
                ..Default::default()
            },
            is_active: true,
            tier_order: 3,
        }
    }
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
    pub subscription_tier: SubscriptionTierId,
    pub subscription_expires_at: Option<u64>, // None for lifetime/free tiers
    pub subscription_auto_renew: bool,
    pub joined_at: u64, // Timestamp when member joined
    pub contribution_points: u64, // Points earned for clan activities
    pub games_played: u64,
    pub tournaments_won: u64,
    pub xp: u64, // Experience points earned
    pub total_winnings: u64, // In smallest currency unit
    pub total_subscription_paid: u64, // Total amount paid for subscriptions
    pub last_active: u64,
}

impl ClanMember {
    pub fn new(principal_id: Principal, role: ClanRole) -> Self {
        let now = ic_cdk::api::time();
        Self {
            principal_id,
            role,
            status: MemberStatus::Active,
            subscription_tier: SubscriptionTierId::basic(),
            subscription_expires_at: None, // Basic is free
            subscription_auto_renew: false,
            joined_at: now,
            contribution_points: 0,
            games_played: 0,
            tournaments_won: 0,
            total_winnings: 0,
            xp: 0,
            total_subscription_paid: 0,
            last_active: now,
        }
    }

    pub fn is_admin_or_higher(&self) -> bool {
        matches!(self.role, ClanRole::Owner | ClanRole::Admin)
    }

    pub fn can_moderate(&self) -> bool {
        matches!(self.role, ClanRole::Owner | ClanRole::Admin | ClanRole::Moderator)
    }
    
    /// Check if subscription is currently active (not expired)
    pub fn is_subscription_active(&self) -> bool {
        match self.subscription_expires_at {
            None => true, // No expiration (Basic tier or lifetime)
            Some(expiry) => ic_cdk::api::time() < expiry,
        }
    }
    
    /// Get days until subscription expires
    pub fn days_until_expiry(&self) -> Option<u64> {
        self.subscription_expires_at.map(|expiry| {
            let now = ic_cdk::api::time();
            if expiry > now {
                (expiry - now) / (24 * 60 * 60 * 1_000_000_000) // Convert nanoseconds to days
            } else {
                0
            }
        })
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
    pub total_subscription_revenue: u64, // Revenue from member subscriptions
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
            total_subscription_revenue: 0,
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
    pub supported_currency: Currency,
    
    // Membership
    pub members: HashMap<Principal, ClanMember>,
    pub member_limit: u32,
    pub pending_requests: Vec<JoinRequest>,
    pub invited_users: HashMap<Principal, u64>, // Principal -> invitation timestamp
    
    // Settings
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
        supported_currency: Currency,
        joining_fee: u64,
    ) -> Result<Self, ClanError> {
        // Validate inputs
        if name.is_empty() || name.len() > 50 {
            return Err(ClanError::InvalidClanName);
        }
        
        if tag.is_empty() || tag.len() > 10 {
            return Err(ClanError::InvalidClanTag);
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
        subscription_tiers.insert(SubscriptionTierId::premium(), SubscriptionTier::new_premium());
        subscription_tiers.insert(SubscriptionTierId::elite(), SubscriptionTier::new_elite());

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
    pub fn add_member(&mut self, principal: Principal, role: ClanRole) -> Result<(), ClanError> {
        if self.is_full() {
            return Err(ClanError::ClanFull(self.member_limit));
        }

        if self.is_member(&principal) {
            return Err(ClanError::UserAlreadyMember);
        }

        self.members.insert(principal, ClanMember::new(principal, role));
        
        // Remove from pending requests if applicable
        self.pending_requests.retain(|req| req.applicant != principal);
        
        // Remove from invited users if applicable
        self.invited_users.remove(&principal);

        Ok(())
    }

    /// Process joining fee payment and add member
    pub fn join_with_fee(&mut self, principal: Principal, paid_amount: u64) -> Result<(), ClanError> {
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

    /// Check if user meets subscription tier requirements
    pub fn meets_subscription_requirements(
        &self,
        user: &User,
        member: &ClanMember,
        tier_id: &SubscriptionTierId,
    ) -> Result<(), ClanError> {
        let tier = self.subscription_tiers.get(tier_id)
            .ok_or(ClanError::SubscriptionTierNotFound(tier_id.0.clone()))?;

        if !tier.is_active {
            return Err(ClanError::SubscriptionTierInactive(tier_id.0.clone()));
        }

        let req = &tier.requirements;

        // Check level requirement
        if let Some(min_level) = req.minimum_level {
            if user.get_level() < min_level as f64 {
                return Err(ClanError::MinimumLevelRequired(min_level));
            }
        }

        // Check experience points
        if let Some(min_xp) = req.minimum_experience_points {
            if user.get_experience_points() < min_xp {
                return Err(ClanError::MinimumExperienceRequired(min_xp));
            }
        }

        // Check contribution points
        if let Some(min_contribution) = req.minimum_contribution_points {
            if member.contribution_points < min_contribution {
                return Err(ClanError::MinimumContributionRequired(min_contribution));
            }
        }

        // Check verification requirement
        if req.requires_verification && !user.is_verified.unwrap_or(false) {
            return Err(ClanError::VerificationRequired);
        }

        // Check membership duration
        if let Some(min_duration) = req.minimum_membership_duration {
            let membership_duration = ic_cdk::api::time() - member.joined_at;
            if membership_duration < min_duration {
                return Err(ClanError::MinimumMembershipDurationNotMet);
            }
        }

        // Check games played
        if let Some(min_games) = req.minimum_games_played {
            if member.games_played < min_games {
                return Err(ClanError::MinimumGamesRequired(min_games));
            }
        }

        // Note: Payment requirements are checked in the upgrade_subscription method
        // Note: Invitation requirements would be checked during the invitation process

        Ok(())
    }

    /// Upgrade member subscription tier
    pub fn upgrade_subscription(
        &mut self,
        member_principal: &Principal,
        new_tier_id: &SubscriptionTierId,
        paid_amount: u64,
        months: u32,
    ) -> Result<(), ClanError> {
        let member = self.members.get_mut(member_principal)
            .ok_or(ClanError::MemberNotFound)?;

        let tier = self.subscription_tiers.get(new_tier_id)
            .ok_or(ClanError::SubscriptionTierNotFound(new_tier_id.0.clone()))?;

        if !tier.is_active {
            return Err(ClanError::SubscriptionTierInactive(new_tier_id.0.clone()));
        }

        // Check payment requirements
        let required_amount = if let Some(monthly_payment) = tier.requirements.monthly_payment {
            monthly_payment * months as u64
        } else if let Some(one_time_payment) = tier.requirements.one_time_payment {
            one_time_payment
        } else {
            0 // Free tier
        };

        if paid_amount < required_amount {
            return Err(ClanError::InsufficientSubscriptionPayment {
                required: required_amount,
                paid: paid_amount,
            });
        }

        // Calculate expiry date for monthly subscriptions
        let new_expiry = if tier.requirements.monthly_payment.is_some() && months > 0 {
            let now = ic_cdk::api::time();
            let month_duration = 30 * 24 * 60 * 60 * 1_000_000_000; // 30 days in nanoseconds
            Some(now + (month_duration * months as u64))
        } else {
            None // Lifetime or free tier
        };

        // Update member subscription
        member.subscription_tier = new_tier_id.clone();
        member.subscription_expires_at = new_expiry;
        member.total_subscription_paid += required_amount;

        // Add to treasury
        self.treasury.balance += required_amount;
        self.treasury.total_subscription_revenue += required_amount;

        Ok(())
    }

    /// Check if member has access to specific functionality based on tier
    pub fn has_tier_access(&self, member_principal: &Principal, required_benefit: &str) -> Result<bool, ClanError> {
        let member = self.members.get(member_principal)
            .ok_or(ClanError::MemberNotFound)?;

        if !member.is_subscription_active() {
            return Ok(false);
        }

        let tier = self.subscription_tiers.get(&member.subscription_tier)
            .ok_or(ClanError::SubscriptionTierNotFound(member.subscription_tier.0.clone()))?;

        let benefits = &tier.benefits;

        let has_access = match required_benefit {
            "tournament_access" => benefits.tournament_access,
            "can_create_tournaments" => benefits.can_create_tournaments,
            "priority_support" => benefits.priority_support,
            "custom_styling" => benefits.custom_styling,
            "exclusive_access" => benefits.exclusive_access,
            "can_invite_members" => benefits.can_invite_members,
            "analytics_access" => benefits.analytics_access,
            _ => false,
        };

        Ok(has_access)
    }

    /// Check if member can access table with specific stakes
    pub fn can_access_table_stakes(&self, member_principal: &Principal, table_stakes: u64) -> Result<bool, ClanError> {
        let member = self.members.get(member_principal)
            .ok_or(ClanError::MemberNotFound)?;

        if !member.is_subscription_active() {
            return Ok(false);
        }

        let tier = self.subscription_tiers.get(&member.subscription_tier)
            .ok_or(ClanError::SubscriptionTierNotFound(member.subscription_tier.0.clone()))?;

        match tier.benefits.max_table_stakes {
            Some(max_stakes) => Ok(table_stakes <= max_stakes),
            None => Ok(true), // No limit
        }
    }

    /// Create or update a custom subscription tier (admin+ only)
    pub fn update_subscription_tier(
        &mut self,
        tier: SubscriptionTier,
        updater: &Principal,
    ) -> Result<(), ClanError> {
        let updater_member = self.members.get(updater)
            .ok_or(ClanError::MemberNotFound)?;

        if !updater_member.is_admin_or_higher() {
            return Err(ClanError::InsufficientPermissions);
        }

        // Validate tier configuration
        if tier.name.is_empty() || tier.name.len() > 50 {
            return Err(ClanError::InvalidTierName);
        }

        if tier.benefits.revenue_share_bonus > 50 {
            return Err(ClanError::InvalidRevenueShareBonus);
        }

        self.subscription_tiers.insert(tier.id.clone(), tier);
        Ok(())
    }

    /// Remove a subscription tier (admin+ only)
    pub fn remove_subscription_tier(
        &mut self,
        tier_id: &SubscriptionTierId,
        updater: &Principal,
    ) -> Result<(), ClanError> {
        let updater_member = self.members.get(updater)
            .ok_or(ClanError::MemberNotFound)?;

        if !updater_member.is_admin_or_higher() {
            return Err(ClanError::InsufficientPermissions);
        }

        // Can't remove basic tier
        if tier_id == &SubscriptionTierId::basic() {
            return Err(ClanError::CannotRemoveBasicTier);
        }

        // Check if any members are using this tier
        let members_using_tier = self.members.values()
            .filter(|member| member.subscription_tier == *tier_id)
            .count();

        if members_using_tier > 0 {
            return Err(ClanError::TierInUse(members_using_tier));
        }

        self.subscription_tiers.remove(tier_id);
        Ok(())
    }

    /// Get members by subscription tier
    pub fn get_members_by_tier(&self, tier_id: &SubscriptionTierId) -> Vec<&ClanMember> {
        self.members.values()
            .filter(|member| member.subscription_tier == *tier_id && member.is_subscription_active())
            .collect()
    }

    /// Get subscription revenue for a specific tier
    pub fn get_tier_revenue(&self, tier_id: &SubscriptionTierId) -> u64 {
        if let Some(tier) = self.subscription_tiers.get(tier_id) {
            if let Some(monthly_payment) = tier.requirements.monthly_payment {
                let active_members = self.get_members_by_tier(tier_id).len() as u64;
                active_members * monthly_payment
            } else {
                0
            }
        } else {
            0
        }
    }

    /// Process subscription renewals for members with auto-renew enabled
    pub fn process_subscription_renewals(&mut self) -> Vec<(Principal, String)> {
        let mut renewal_results = Vec::new();
        let now = ic_cdk::api::time();
        let members_to_process: Vec<Principal> = self.members.keys().cloned().collect();

        for member_principal in members_to_process {
            if let Some(member) = self.members.get_mut(&member_principal) {
                if member.subscription_auto_renew {
                    if let Some(expiry) = member.subscription_expires_at {
                        // Check if subscription expires within 7 days
                        let seven_days = 7 * 24 * 60 * 60 * 1_000_000_000;
                        if expiry <= now + seven_days {
                            // In a real implementation, you'd charge their payment method here
                            renewal_results.push((member_principal, "Renewal required".to_string()));
                        }
                    }
                }
            }
        }

        renewal_results
    }

    /// Remove a member from the clan
    pub fn remove_member(&mut self, principal: &Principal) -> Result<ClanMember, ClanError> {
        // Can't remove the owner
        if let Some(member) = self.members.get(principal) {
            if member.role == ClanRole::Owner {
                return Err(ClanError::CannotRemoveOwner);
            }
        }

        self.members.remove(principal)
            .ok_or(ClanError::MemberNotFound)
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
    pub fn update_member_role(&mut self, principal: &Principal, new_role: ClanRole, updater: &Principal) -> Result<(), ClanError> {
        let updater_member = self.members.get(updater)
            .ok_or(ClanError::MemberNotFound)?;

        let target_member = self.members.get(principal)
            .ok_or(ClanError::MemberNotFound)?;

        // Only owners can promote to admin, only admin+ can change roles
        match new_role {
            ClanRole::Owner | ClanRole::Admin => {
                if updater_member.role != ClanRole::Owner {
                    return Err(ClanError::InsufficientPermissions);
                }
            },
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
    pub fn update_joining_fee(&mut self, new_fee: u64, updater: &Principal) -> Result<(), ClanError> {
        let updater_member = self.members.get(updater)
            .ok_or(ClanError::MemberNotFound)?;

        if !updater_member.is_admin_or_higher() {
            return Err(ClanError::InsufficientPermissions);
        }

        self.joining_fee = new_fee;
        Ok(())
    }

    /// Update reward distribution method (admin+ only)
    pub fn update_reward_distribution(&mut self, new_distribution: RewardDistributionType, updater: &Principal) -> Result<(), ClanError> {
        let updater_member = self.members.get(updater)
            .ok_or(ClanError::MemberNotFound)?;

        if !updater_member.is_admin_or_higher() {
            return Err(ClanError::InsufficientPermissions);
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
    pub fn distribute_rewards(&mut self, distribution: HashMap<Principal, u64>) -> Result<(), ClanError> {
        let distribution_total: u64 = distribution.values().sum();
        let available_amount = self.calculate_available_reward_amount();
        
        if distribution_total > available_amount {
            return Err(ClanError::InsufficientFunds { 
                available: available_amount,
                required: distribution_total,
            });
        }

        if distribution_total > self.treasury.balance {
            return Err(ClanError::InsufficientTreasuryBalance {
                available: self.treasury.balance,
                required: distribution_total,
            });
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
    pub fn meets_joining_requirements(&self, user: &User) -> Result<(), ClanError> {
        if self.require_proof_of_humanity && !user.is_verified.unwrap_or(false) {
            return Err(ClanError::VerificationRequired);
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
                supported_currency: Currency::ICP,
                members: HashMap::new(),
                member_limit: 0,
                pending_requests: Vec::new(),
                invited_users: HashMap::new(),
                privacy: ClanPrivacy::Public,
                require_proof_of_humanity: false,
                subscription_enabled: false,
                subscription_tiers: HashMap::new(),
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
