use candid::CandidType;
use currency::Currency;
use serde::{Deserialize, Serialize};

use crate::{tags::{ClanTag, TagCategory}, ClanPrivacy};

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct ClanSearchFilters {
    pub name_contains: Option<String>,
    pub currency: Option<Currency>,
    pub privacy: Option<ClanPrivacy>,
    pub min_members: Option<u32>,
    pub max_members: Option<u32>,
    pub has_joining_fee: Option<bool>,
    pub subscription_enabled: Option<bool>,
    pub require_proof_of_humanity: Option<bool>,

    // New tag-based filters
    pub tag_filters: Option<TagSearchFilters>,
}

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
