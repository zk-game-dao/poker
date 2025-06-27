use candid::{CandidType, Principal};
use clan::{
    search::ClanSearchFilters, subscriptions::{ClanRole, SubscriptionTierId}, tags::{ClanTag, TagCategory}, Clan, ClanId, ClanPrivacy
};
use currency::Currency;
use errors::clan_index_error::ClanIndexError;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use user::user::WalletPrincipalId;

#[derive(Debug, Clone, PartialEq, CandidType, Serialize, Deserialize)]
pub struct ClanIndex {
    /// Map of clan ID to clan data
    pub clans: HashMap<ClanId, Clan>,

    /// Map of individual clan tags to clan IDs for fast tag-based lookup
    pub tag_to_clans: HashMap<ClanTag, HashSet<ClanId>>,

    /// Map of user principals to clan IDs they're members of
    pub user_to_clans: HashMap<WalletPrincipalId, HashSet<ClanId>>,

    /// Map of currency types to clan IDs using that currency
    pub currency_to_clans: HashMap<Currency, HashSet<ClanId>>,

    /// Statistics
    pub total_clans: usize,
    pub total_members: usize,
}

impl Default for ClanIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl ClanIndex {
    pub fn new() -> Self {
        Self {
            clans: HashMap::new(),
            tag_to_clans: HashMap::new(),
            user_to_clans: HashMap::new(),
            currency_to_clans: HashMap::new(),
            total_clans: 0,
            total_members: 0,
        }
    }

    /// Add a new clan to the index
    pub fn add_clan(&mut self, clan: Clan) -> Result<(), ClanIndexError> {
        let clan_id = clan.id;
        let currency = clan.supported_currency.clone();
        let tags = clan.tags.clone();

        // Add to various indexes
        self.clans.insert(clan_id, clan.clone());

        // Add to tag indexes
        for tag in &tags {
            self.tag_to_clans
                .entry(tag.clone())
                .or_insert_with(HashSet::new)
                .insert(clan_id);
        }

        // Add to currency index
        self.currency_to_clans
            .entry(currency)
            .or_insert_with(HashSet::new)
            .insert(clan_id);

        // Add members to user index
        for member_principal in clan.members.keys() {
            self.user_to_clans
                .entry(*member_principal)
                .or_insert_with(HashSet::new)
                .insert(clan_id);
        }

        // Update statistics
        self.total_clans += 1;
        self.total_members += clan.members.len();

        Ok(())
    }

    /// Remove a clan from the index
    pub fn remove_clan(&mut self, clan_id: ClanId) -> Result<(), ClanIndexError> {
        let clan = self
            .clans
            .remove(&clan_id)
            .ok_or(ClanIndexError::ClanNotFound)?;

        // Remove from tag indexes
        for tag in &clan.tags {
            if let Some(clan_set) = self.tag_to_clans.get_mut(tag) {
                clan_set.remove(&clan_id);
                if clan_set.is_empty() {
                    self.tag_to_clans.remove(tag);
                }
            }
        }

        // Remove from currency index
        if let Some(clan_set) = self.currency_to_clans.get_mut(&clan.supported_currency) {
            clan_set.remove(&clan_id);
            if clan_set.is_empty() {
                self.currency_to_clans.remove(&clan.supported_currency);
            }
        }

        // Remove from user index
        for member_principal in clan.members.keys() {
            if let Some(user_clans) = self.user_to_clans.get_mut(member_principal) {
                user_clans.remove(&clan_id);
                if user_clans.is_empty() {
                    self.user_to_clans.remove(member_principal);
                }
            }
        }

        // Update statistics
        self.total_clans -= 1;
        self.total_members -= clan.members.len();

        Ok(())
    }

    /// Update clan data in the index
    pub fn update_clan(&mut self, clan: Clan) -> Result<(), ClanIndexError> {
        let clan_id = clan.id;

        if let Some(old_clan) = self.clans.get(&clan_id) {
            // Handle tag changes
            let old_tags = &old_clan.tags;
            let new_tags = &clan.tags;

            // Remove old tags that are no longer present
            for old_tag in old_tags {
                if !new_tags.contains(old_tag) {
                    if let Some(clan_set) = self.tag_to_clans.get_mut(old_tag) {
                        clan_set.remove(&clan_id);
                        if clan_set.is_empty() {
                            self.tag_to_clans.remove(old_tag);
                        }
                    }
                }
            }

            // Add new tags
            for new_tag in new_tags {
                if !old_tags.contains(new_tag) {
                    self.tag_to_clans
                        .entry(new_tag.clone())
                        .or_insert_with(HashSet::new)
                        .insert(clan_id);
                }
            }

            // Handle currency change
            if old_clan.supported_currency != clan.supported_currency {
                // Remove from old currency index
                if let Some(old_currency_set) =
                    self.currency_to_clans.get_mut(&old_clan.supported_currency)
                {
                    old_currency_set.remove(&clan_id);
                    if old_currency_set.is_empty() {
                        self.currency_to_clans.remove(&old_clan.supported_currency);
                    }
                }

                // Add to new currency index
                self.currency_to_clans
                    .entry(clan.supported_currency.clone())
                    .or_insert_with(HashSet::new)
                    .insert(clan_id);
            }

            // Handle member changes
            let old_member_count = old_clan.members.len();
            let new_member_count = clan.members.len();

            // Update total members count
            self.total_members = self
                .total_members
                .saturating_sub(old_member_count)
                .saturating_add(new_member_count);

            // Update user index for removed members
            for old_member in old_clan.members.keys() {
                if !clan.members.contains_key(old_member) {
                    if let Some(user_clans) = self.user_to_clans.get_mut(old_member) {
                        user_clans.remove(&clan_id);
                        if user_clans.is_empty() {
                            self.user_to_clans.remove(old_member);
                        }
                    }
                }
            }

            // Update user index for new members
            for new_member in clan.members.keys() {
                if !old_clan.members.contains_key(new_member) {
                    self.user_to_clans
                        .entry(*new_member)
                        .or_insert_with(HashSet::new)
                        .insert(clan_id);
                }
            }
        }

        // Update the clan data
        self.clans.insert(clan_id, clan);
        Ok(())
    }

    /// Add a member to a clan
    pub fn add_member_to_clan(
        &mut self,
        clan_id: ClanId,
        user_principal: WalletPrincipalId,
    ) -> Result<(), ClanIndexError> {
        if self.clans.contains_key(&clan_id) {
            self.user_to_clans
                .entry(user_principal)
                .or_insert_with(HashSet::new)
                .insert(clan_id);
            self.total_members += 1;
            Ok(())
        } else {
            Err(ClanIndexError::ClanNotFound)
        }
    }

    /// Remove a member from a clan
    pub fn remove_member_from_clan(
        &mut self,
        clan_id: ClanId,
        user_principal: WalletPrincipalId,
    ) -> Result<(), ClanIndexError> {
        if self.clans.contains_key(&clan_id) {
            if let Some(user_clans) = self.user_to_clans.get_mut(&user_principal) {
                if user_clans.remove(&clan_id) {
                    self.total_members -= 1;
                    if user_clans.is_empty() {
                        self.user_to_clans.remove(&user_principal);
                    }
                }
            }
            Ok(())
        } else {
            Err(ClanIndexError::ClanNotFound)
        }
    }

    /// Get all clans
    pub fn get_all_clans(&self) -> Vec<Clan> {
        self.clans.values().cloned().collect()
    }

    /// Get clans by currency
    pub fn get_clans_by_currency(&self, currency: &Currency) -> Vec<Clan> {
        self.currency_to_clans
            .get(currency)
            .map(|clan_ids| {
                clan_ids
                    .iter()
                    .filter_map(|clan_id| self.clans.get(clan_id))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get clans a user is a member of
    pub fn get_user_clans(&self, user_principal: &WalletPrincipalId) -> Vec<Clan> {
        self.user_to_clans
            .get(user_principal)
            .map(|clan_ids| {
                clan_ids
                    .iter()
                    .filter_map(|clan_id| self.clans.get(clan_id))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Search clans with filters
    pub fn search_clans(
        &self,
        filters: Option<ClanSearchFilters>,
        page: u64,
        page_size: u64,
    ) -> Vec<Clan> {
        let mut clan_ids: Vec<ClanId> = self.clans.keys().copied().collect();

        // Apply tag filters first if provided
        if let Some(ref search_filters) = filters {
            if let Some(ref tag_filters) = search_filters.tag_filters {
                clan_ids = self.search_by_tags(tag_filters.clone());
            }
        }

        // Get clans from IDs
        let mut clans: Vec<Clan> = clan_ids
            .iter()
            .filter_map(|clan_id| self.clans.get(clan_id))
            .cloned()
            .collect();

        // Apply other filters
        if let Some(filters) = filters {
            clans = clans
                .into_iter()
                .filter(|clan| {
                    // Name filter
                    if let Some(ref name_filter) = filters.name_contains {
                        if !clan
                            .name
                            .to_lowercase()
                            .contains(&name_filter.to_lowercase())
                        {
                            return false;
                        }
                    }

                    // Currency filter
                    if let Some(ref currency_filter) = filters.currency {
                        if clan.supported_currency != *currency_filter {
                            return false;
                        }
                    }

                    // Privacy filter
                    if let Some(ref privacy_filter) = filters.privacy {
                        if clan.privacy != *privacy_filter {
                            return false;
                        }
                    }

                    // Member count filters
                    let member_count = clan.members.len() as u32;
                    if let Some(min_members) = filters.min_members {
                        if member_count < min_members {
                            return false;
                        }
                    }
                    if let Some(max_members) = filters.max_members {
                        if member_count > max_members {
                            return false;
                        }
                    }

                    // Joining fee filter
                    if let Some(has_joining_fee) = filters.has_joining_fee {
                        let clan_has_fee = clan.joining_fee > 0;
                        if has_joining_fee != clan_has_fee {
                            return false;
                        }
                    }

                    // Subscription enabled filter
                    if let Some(subscription_enabled) = filters.subscription_enabled {
                        if clan.subscription_enabled != subscription_enabled {
                            return false;
                        }
                    }

                    // Proof of humanity filter
                    if let Some(require_poh) = filters.require_proof_of_humanity {
                        if clan.require_proof_of_humanity != require_poh {
                            return false;
                        }
                    }

                    true
                })
                .collect();
        }

        // Sort by member count (most popular first)
        clans.sort_by(|a, b| b.members.len().cmp(&a.members.len()));

        // Apply pagination
        let start = (page * page_size) as usize;
        let end = std::cmp::min(start + page_size as usize, clans.len());

        if start >= clans.len() {
            Vec::new()
        } else {
            clans[start..end].to_vec()
        }
    }

    /// Get clans by member count range
    pub fn get_clans_by_member_count(
        &self,
        min_members: usize,
        max_members: Option<usize>,
    ) -> Vec<Clan> {
        self.clans
            .values()
            .filter(|clan| {
                let member_count = clan.members.len();
                member_count >= min_members && max_members.map_or(true, |max| member_count <= max)
            })
            .cloned()
            .collect()
    }

    /// Get most popular clans (by member count)
    pub fn get_popular_clans(&self, limit: usize) -> Vec<Clan> {
        let mut clans: Vec<Clan> = self.clans.values().cloned().collect();
        clans.sort_by(|a, b| b.members.len().cmp(&a.members.len()));
        clans.truncate(limit);
        clans
    }

    /// Get clan count
    pub fn get_clan_count(&self) -> usize {
        self.total_clans
    }

    /// Get total member count across all clans
    pub fn get_total_members(&self) -> usize {
        self.total_members
    }

    /// Get all clan IDs
    pub fn get_all_clan_ids(&self) -> Vec<ClanId> {
        self.clans.keys().copied().collect()
    }

    /// Check if a principal is a valid clan canister
    pub fn is_valid_clan_canister(&self, principal: &ClanId) -> bool {
        self.clans.contains_key(principal)
    }

    /// Get clans created by a specific user
    pub fn get_clans_created_by(&self, creator: &Principal) -> Vec<Clan> {
        self.clans
            .values()
            .filter(|clan| clan.created_by == *creator)
            .cloned()
            .collect()
    }

    /// Get clans where user has a specific role
    pub fn get_clans_by_user_role(
        &self,
        user_principal: &WalletPrincipalId,
        role: &ClanRole,
    ) -> Vec<Clan> {
        self.clans
            .values()
            .filter(|clan| {
                clan.members
                    .get(user_principal)
                    .map_or(false, |member| member.role == *role)
            })
            .cloned()
            .collect()
    }

    /// Get clans by subscription tier
    pub fn get_clans_with_subscription_tier(&self, tier: &SubscriptionTierId) -> Vec<Clan> {
        self.clans
            .values()
            .filter(|clan| clan.subscription_tiers.contains_key(tier))
            .cloned()
            .collect()
    }

    /// Get statistics about clans
    pub fn get_clan_statistics(&self) -> ClanStatistics {
        let mut stats = ClanStatistics::default();

        stats.total_clans = self.total_clans;
        stats.total_members = self.total_members;

        for clan in self.clans.values() {
            // Privacy distribution
            match clan.privacy {
                ClanPrivacy::Public => stats.public_clans += 1,
                ClanPrivacy::InviteOnly => stats.invite_only_clans += 1,
                ClanPrivacy::Application => stats.application_clans += 1,
            }

            // Currency distribution
            *stats
                .currency_distribution
                .entry(clan.supported_currency.clone())
                .or_insert(0) += 1;

            // Subscription enabled
            if clan.subscription_enabled {
                stats.subscription_enabled_clans += 1;
            }

            // With joining fee
            if clan.joining_fee > 0 {
                stats.clans_with_joining_fee += 1;
            }

            // Average member count
            stats.average_members_per_clan += clan.members.len() as f64;
        }

        if self.total_clans > 0 {
            stats.average_members_per_clan /= self.total_clans as f64;
        }

        stats
    }
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct ClanStatistics {
    pub total_clans: usize,
    pub total_members: usize,
    pub public_clans: usize,
    pub invite_only_clans: usize,
    pub application_clans: usize,
    pub subscription_enabled_clans: usize,
    pub clans_with_joining_fee: usize,
    pub average_members_per_clan: f64,
    pub currency_distribution: HashMap<Currency, usize>,
    pub tag_statistics: HashMap<TagCategory, usize>,
}

impl Default for ClanStatistics {
    fn default() -> Self {
        Self {
            total_clans: 0,
            total_members: 0,
            public_clans: 0,
            invite_only_clans: 0,
            application_clans: 0,
            subscription_enabled_clans: 0,
            clans_with_joining_fee: 0,
            average_members_per_clan: 0.0,
            currency_distribution: HashMap::new(),
            tag_statistics: HashMap::new(),
        }
    }
}
