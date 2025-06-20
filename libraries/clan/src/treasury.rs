use std::collections::HashMap;

use candid::{CandidType, Principal};
use errors::clan_error::ClanError;
use serde::{Deserialize, Serialize};

use crate::Clan;

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

impl Clan {
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
}
