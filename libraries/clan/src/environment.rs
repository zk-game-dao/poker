use candid::CandidType;
use serde::{Deserialize, Serialize};

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
