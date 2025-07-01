use candid::CandidType;
use errors::clan_error::ClanError;
use serde::{Deserialize, Serialize};
use user::user::{User, WalletPrincipalId};

use crate::{Clan, member::ClanMember};

/// Represents the role a member has within a clan
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum ClanRole {
    Owner,
    Admin,
    Moderator,
    Member,
}

/// Subscription tier identifier - clans can create custom tiers
#[derive(
    Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
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

    /// Can create custom tables for clan members
    pub can_create_tables: bool,

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
            can_create_tables: false,
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
    pub fn new_custom(
        id: SubscriptionTierId,
        name: String,
        requirements: SubscriptionRequirements,
        benefits: SubscriptionBenefits,
        is_active: bool,
        tier_order: u32,
    ) -> Self {
        Self {
            id,
            name,
            requirements,
            benefits,
            is_active,
            tier_order,
        }
    }

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
                minimum_contribution_points: Some(1000),
                ..Default::default()
            },
            benefits: SubscriptionBenefits {
                description: "Full clan access with all privileges".to_string(),
                max_table_stakes: None, // Unlimited
                tournament_access: true,
                can_create_tournaments: true,
                can_create_tables: true,
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

impl Clan {
    /// Check if user meets subscription tier requirements
    pub fn meets_subscription_requirements(
        &self,
        user: &User,
        member: &ClanMember,
        tier_id: &SubscriptionTierId,
    ) -> Result<(), ClanError> {
        let tier = self
            .subscription_tiers
            .get(tier_id)
            .ok_or(ClanError::SubscriptionTierNotFound(tier_id.0.clone()))?;

        if !tier.is_active {
            return Err(ClanError::SubscriptionTierInactive(tier_id.0.clone()));
        }

        let req = &tier.requirements;

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
        member_principal: &WalletPrincipalId,
        new_tier_id: &SubscriptionTierId,
        paid_amount: u64,
        months: u32,
    ) -> Result<(), ClanError> {
        let member = self
            .members
            .get_mut(member_principal)
            .ok_or(ClanError::MemberNotFound)?;

        let tier = self
            .subscription_tiers
            .get(new_tier_id)
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
    pub fn has_tier_access(
        &self,
        member_principal: &WalletPrincipalId,
        required_benefit: &str,
    ) -> Result<bool, ClanError> {
        let member = self
            .members
            .get(member_principal)
            .ok_or(ClanError::MemberNotFound)?;

        if member.is_admin_or_higher() {
            return Ok(true); // Admins have all access
        }

        if self.subscription_enabled && !member.is_subscription_active() {
            return Ok(false);
        } else if !self.subscription_enabled {
            return Ok(true); // No subscription required
        }

        let tier = self
            .subscription_tiers
            .get(&member.subscription_tier)
            .ok_or(ClanError::SubscriptionTierNotFound(
                member.subscription_tier.0.clone(),
            ))?;

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
    pub fn can_access_table_stakes(
        &self,
        member_principal: &WalletPrincipalId,
        table_stakes: u64,
    ) -> Result<bool, ClanError> {
        let member = self
            .members
            .get(member_principal)
            .ok_or(ClanError::MemberNotFound)?;

        if self.subscription_enabled && !member.is_subscription_active() {
            return Ok(false);
        } else if !self.subscription_enabled {
            return Ok(true);
        }

        let tier = self
            .subscription_tiers
            .get(&member.subscription_tier)
            .ok_or(ClanError::SubscriptionTierNotFound(
                member.subscription_tier.0.clone(),
            ))?;

        match tier.benefits.max_table_stakes {
            Some(max_stakes) => Ok(table_stakes <= max_stakes),
            None => Ok(true), // No limit
        }
    }

    pub fn create_custom_subscription_tier(
        &mut self,
        id: SubscriptionTierId,
        name: String,
        requirements: SubscriptionRequirements,
        benefits: SubscriptionBenefits,
        is_active: bool,
        tier_order: u32,
        creator: &WalletPrincipalId,
    ) -> Result<SubscriptionTier, ClanError> {
        let creator_member = self.members.get(creator).ok_or(ClanError::MemberNotFound)?;

        if !creator_member.is_admin_or_higher() {
            return Err(ClanError::InsufficientPermissions);
        }

        // Validate tier configuration
        if name.is_empty() || name.len() > 50 {
            return Err(ClanError::InvalidTierName);
        }

        let tier = SubscriptionTier::new_custom(id.clone(), name, requirements, benefits, is_active, tier_order);

        self.subscription_tiers.insert(
            id.clone(),
            tier.clone(),
        );
        Ok(tier)
    }

    /// Create or update a custom subscription tier (admin+ only)
    pub fn update_subscription_tier(
        &mut self,
        tier: SubscriptionTier,
        updater: &WalletPrincipalId,
    ) -> Result<(), ClanError> {
        let updater_member = self.members.get(updater).ok_or(ClanError::MemberNotFound)?;

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
        updater: &WalletPrincipalId,
    ) -> Result<(), ClanError> {
        let updater_member = self.members.get(updater).ok_or(ClanError::MemberNotFound)?;

        if !updater_member.is_admin_or_higher() {
            return Err(ClanError::InsufficientPermissions);
        }

        // Can't remove basic tier
        if tier_id == &SubscriptionTierId::basic() {
            return Err(ClanError::CannotRemoveBasicTier);
        }

        // Check if any members are using this tier
        let members_using_tier = self
            .members
            .values()
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
        self.members
            .values()
            .filter(|member| {
                member.subscription_tier == *tier_id && member.is_subscription_active()
            })
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
    pub fn process_subscription_renewals(&mut self) -> Vec<(WalletPrincipalId, String)> {
        let mut renewal_results = Vec::new();
        let now = ic_cdk::api::time();
        let members_to_process: Vec<WalletPrincipalId> = self.members.keys().cloned().collect();

        for member_principal in members_to_process {
            if let Some(member) = self.members.get_mut(&member_principal) {
                if member.subscription_auto_renew {
                    if let Some(expiry) = member.subscription_expires_at {
                        // Check if subscription expires within 7 days
                        let seven_days = 7 * 24 * 60 * 60 * 1_000_000_000;
                        if expiry <= now + seven_days {
                            // In a real implementation, you'd charge their payment method here
                            renewal_results
                                .push((member_principal, "Renewal required".to_string()));
                        }
                    }
                }
            }
        }

        renewal_results
    }
}
