use std::collections::{HashMap, HashSet};

use candid::CandidType;
use clan::{tags::{ClanTag, TagCategory}, Clan, ClanId};
use serde::{Deserialize, Serialize};

use crate::clans_index::ClanIndex;

/// Tag-related search filters for the clan index
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct TagSearchFilters {
    /// Tags that must be present (AND logic)
    pub required_tags: Option<Vec<ClanTag>>,
    
    /// Tags where at least one must be present (OR logic)
    pub any_of_tags: Option<Vec<ClanTag>>,
    
    /// Tags that must NOT be present (NOT logic)
    pub excluded_tags: Option<Vec<ClanTag>>,
    
    /// Filter by specific tag categories
    pub categories: Option<Vec<TagCategory>>,
}

/// Updated clan index with tag support
impl ClanIndex {
    /// Add tag-to-clans mapping for efficient tag-based searches
    pub fn add_tag_index(&mut self, clan_id: ClanId, tags: &HashSet<ClanTag>) {        
        for tag in tags {
            self.tag_to_clans
                .entry(tag.clone())
                .or_insert_with(HashSet::new)
                .insert(clan_id);
        }
    }
    
    /// Remove tag index for a clan
    pub fn remove_tag_index(&mut self, clan_id: ClanId, tags: &HashSet<ClanTag>) {
        for tag in tags {
            if let Some(clan_set) = self.tag_to_clans.get_mut(tag) {
                clan_set.remove(&clan_id);
                if clan_set.is_empty() {
                    self.tag_to_clans.remove(tag);
                }
            }
        }
    }
    
    /// Search clans by tags
    pub fn search_by_tags(&self, filters: TagSearchFilters) -> Vec<ClanId> {
        let all_clan_ids: HashSet<ClanId> = self.clans.keys().copied().collect();
        let mut result_ids = all_clan_ids;
        
        // Apply required tags (AND logic)
        if let Some(required_tags) = filters.required_tags {
            for tag in required_tags {
                if let Some(clan_ids) = self.tag_to_clans.get(&tag) {
                    result_ids = result_ids.intersection(clan_ids).copied().collect();
                } else {
                    // If any required tag has no clans, return empty result
                    return Vec::new();
                }
            }
        }
        
        // Apply any-of tags (OR logic)
        if let Some(any_of_tags) = filters.any_of_tags {
            let mut any_of_ids = HashSet::new();
            for tag in any_of_tags {
                if let Some(clan_ids) = self.tag_to_clans.get(&tag) {
                    any_of_ids.extend(clan_ids);
                }
            }
            result_ids = result_ids.intersection(&any_of_ids).copied().collect();
        }
        
        // Apply excluded tags (NOT logic)
        if let Some(excluded_tags) = filters.excluded_tags {
            for tag in excluded_tags {
                if let Some(clan_ids) = self.tag_to_clans.get(&tag) {
                    result_ids = result_ids.difference(clan_ids).copied().collect();
                }
            }
        }
        
        // Filter by categories if specified
        if let Some(categories) = filters.categories {
            let category_clan_ids: HashSet<ClanId> = self.clans
                .iter()
                .filter(|(_, clan)| {
                    clan.tags.iter().any(|tag| categories.contains(&tag.category()))
                })
                .map(|(id, _)| *id)
                .collect();
            
            result_ids = result_ids.intersection(&category_clan_ids).copied().collect();
        }
        
        result_ids.into_iter().collect()
    }

    /// Get clans by specific tag
    pub fn get_clans_by_tag(&self, tag: &ClanTag) -> Vec<Clan> {
        self.tag_to_clans
            .get(tag)
            .map(|clan_ids| {
                clan_ids.iter()
                    .filter_map(|clan_id| self.clans.get(clan_id))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Get most popular tags across all clans
    pub fn get_popular_tags(&self, limit: usize) -> Vec<(ClanTag, usize)> {
        let mut tag_counts: HashMap<ClanTag, usize> = HashMap::new();
        
        for clan in self.clans.values() {
            for tag in &clan.tags {
                *tag_counts.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        
        let mut sorted_tags: Vec<(ClanTag, usize)> = tag_counts.into_iter().collect();
        sorted_tags.sort_by(|a, b| b.1.cmp(&a.1));
        sorted_tags.truncate(limit);
        sorted_tags
    }
    
    /// Get tag statistics by category
    pub fn get_tag_statistics_by_category(&self) -> HashMap<TagCategory, usize> {
        let mut category_counts: HashMap<TagCategory, usize> = HashMap::new();
        
        for clan in self.clans.values() {
            for tag in &clan.tags {
                *category_counts.entry(tag.category()).or_insert(0) += 1;
            }
        }
        
        category_counts
    }
}

/// Validation functions for clan tags
pub fn validate_clan_tags(tags: &HashSet<ClanTag>) -> Result<(), String> {
    const MAX_TAGS: usize = 10;
    const MAX_CUSTOM_TAGS: usize = 3;
    
    if tags.len() > MAX_TAGS {
        return Err(format!("Maximum {} tags allowed", MAX_TAGS));
    }
    
    let custom_tag_count = tags.iter()
        .filter(|tag| matches!(tag, ClanTag::Custom(_)))
        .count();
    
    if custom_tag_count > MAX_CUSTOM_TAGS {
        return Err(format!("Maximum {} custom tags allowed", MAX_CUSTOM_TAGS));
    }
    
    // Validate mutually exclusive tags
    let skill_tags: Vec<&ClanTag> = tags.iter()
        .filter(|tag| matches!(tag.category(), TagCategory::SkillLevel))
        .collect();
    
    if skill_tags.len() > 2 {
        return Err("Cannot have more than 2 skill level tags".to_string());
    }
    
    Ok(())
}

/// Helper function to suggest tags based on clan characteristics
pub fn suggest_tags_for_clan(
    joining_fee: u64,
    subscription_enabled: bool,
    require_proof_of_humanity: bool,
    member_count: usize,
) -> Vec<ClanTag> {
    let mut suggested_tags = Vec::new();
    
    // Suggest based on joining fee (stakes)
    match joining_fee {
        0 => {}, // No specific stakes tag for free clans
        1..=10000 => suggested_tags.push(ClanTag::Microstakes),
        10001..=100000 => suggested_tags.push(ClanTag::LowStakes),
        100001..=1000000 => suggested_tags.push(ClanTag::MidStakes),
        1000001..=10000000 => suggested_tags.push(ClanTag::HighStakes),
        _ => suggested_tags.push(ClanTag::Nosebleeds),
    }
    
    // Suggest based on features
    if subscription_enabled {
        suggested_tags.push(ClanTag::VIP);
    }
    
    if require_proof_of_humanity {
        suggested_tags.push(ClanTag::Verified);
    }
    
    // Suggest based on size
    match member_count {
        0..=10 => suggested_tags.push(ClanTag::Exclusive),
        50.. => suggested_tags.push(ClanTag::Social),
        _ => {},
    }
    
    suggested_tags
}
