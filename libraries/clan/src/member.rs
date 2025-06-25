use candid::CandidType;
use serde::{Deserialize, Serialize};
use user::user::WalletPrincipalId;

use crate::subscriptions::{ClanRole, SubscriptionTierId};

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
    pub principal_id: WalletPrincipalId,
    pub role: ClanRole,
    pub status: MemberStatus,
    pub subscription_tier: SubscriptionTierId,
    pub subscription_expires_at: Option<u64>, // None for lifetime/free tiers
    pub subscription_auto_renew: bool,
    pub joined_at: u64,           // Timestamp when member joined
    pub contribution_points: u64, // Points earned for clan activities
    pub games_played: u64,
    pub tournaments_won: u64,
    pub xp: u64,                      // Experience points earned
    pub total_winnings: u64,          // In smallest currency unit
    pub total_subscription_paid: u64, // Total amount paid for subscriptions
    pub last_active: u64,
}

impl ClanMember {
    pub fn new(principal_id: WalletPrincipalId, role: ClanRole) -> Self {
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
        matches!(
            self.role,
            ClanRole::Owner | ClanRole::Admin | ClanRole::Moderator
        )
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
