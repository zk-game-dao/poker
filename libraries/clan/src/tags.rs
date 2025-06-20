use candid::CandidType;
use serde::{Deserialize, Serialize};

/// Predefined clan tags that highlight clan characteristics
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq, Hash)]
pub enum ClanTag {
    // Skill Level Tags
    Beginner,
    Intermediate,
    Advanced,
    Professional,
    MixedSkill,
    
    // Stakes Tags
    Microstakes,     // Very low stakes
    LowStakes,       // Low stakes
    MidStakes,       // Medium stakes
    HighStakes,      // High stakes
    Nosebleeds,      // Highest stakes
    
    // Game Type Tags
    CashGames,
    Tournaments,
    SitAndGo,
    SpinAndGo,
    MixedGameTypes,
    HeadsUp,
    
    // Activity Schedule Tags
    Weekdays,
    Weekends,
    EarlyMorning,    // 6am-12pm
    Afternoon,       // 12pm-6pm
    Evening,         // 6pm-12am
    LateNight,       // 12am-6am
    TwentyFourSeven, // Always active
    
    // Geographic/Timezone Tags - Comprehensive timezone coverage
    // Major Regions
    NorthAmerica,
    SouthAmerica, 
    Europe,
    Asia,
    Africa,
    Oceania,
    
    // Specific Timezones - UTC offsets
    UTCMinus12,     // Baker Island Time
    UTCMinus11,     // Samoa Standard Time
    UTCMinus10,     // Hawaii Standard Time
    UTCMinus9,      // Alaska Standard Time
    UTCMinus8,      // Pacific Standard Time (PST)
    UTCMinus7,      // Mountain Standard Time (MST)
    UTCMinus6,      // Central Standard Time (CST)
    UTCMinus5,      // Eastern Standard Time (EST)
    UTCMinus4,      // Atlantic Standard Time
    UTCMinus3,      // Argentina Time, Brazil Time
    UTCMinus2,      // South Georgia Time
    UTCMinus1,      // Azores Time
    UTC,            // Coordinated Universal Time
    UTCPlus1,       // Central European Time (CET)
    UTCPlus2,       // Eastern European Time (EET)
    UTCPlus3,       // Moscow Time
    UTCPlus4,       // Gulf Standard Time
    UTCPlus5,       // Pakistan Standard Time
    UTCPlus6,       // Bangladesh Standard Time
    UTCPlus7,       // Indochina Time
    UTCPlus8,       // China Standard Time
    UTCPlus9,       // Japan Standard Time
    UTCPlus10,      // Australian Eastern Standard Time
    UTCPlus11,      // Solomon Islands Time
    UTCPlus12,      // Fiji Time
    
    // Language Tags
    English,
    Spanish,
    French,
    German,
    Chinese,
    Japanese,
    Russian,
    Portuguese,
    Italian,
    Korean,
    Multilingual,
    
    // Community Style Tags
    Casual,
    Competitive,
    Educational,
    Social,
    Serious,
    Fun,
    StudyGroup,
    
    // Special Features Tags
    Coaching,
    HandReviews,
    Strategy,
    Streaming,
    VoiceChat,
    TextChat,
    DiscordIntegration,
    
    // Member Requirements Tags
    InviteOnly,
    Verified,        // Requires proof of humanity
    Application,
    
    // Clan Benefits Tags
    Rakeback,
    Freerolls,
    Rewards,
    Leaderboards,
    Exclusive,
    VIP,
    
    // Custom tag for user-defined tags
    Custom(String),
}

impl ClanTag {
    /// Get the display name for a tag
    pub fn display_name(&self) -> String {
        match self {
            ClanTag::Beginner => "Beginner Friendly".to_string(),
            ClanTag::Intermediate => "Intermediate".to_string(),
            ClanTag::Advanced => "Advanced".to_string(),
            ClanTag::Professional => "Professional".to_string(),
            ClanTag::MixedSkill => "Mixed Skill".to_string(),
            
            ClanTag::Microstakes => "Micro Stakes".to_string(),
            ClanTag::LowStakes => "Low Stakes".to_string(),
            ClanTag::MidStakes => "Mid Stakes".to_string(),
            ClanTag::HighStakes => "High Stakes".to_string(),
            ClanTag::Nosebleeds => "Nosebleeds".to_string(),
            
            ClanTag::CashGames => "Cash Games".to_string(),
            ClanTag::Tournaments => "Tournaments".to_string(),
            ClanTag::SitAndGo => "Sit & Go".to_string(),
            ClanTag::SpinAndGo => "Spin & Go".to_string(),
            ClanTag::HeadsUp => "Heads Up".to_string(),
            ClanTag::MixedGameTypes => "Mixed Game Types".to_string(),
            
            ClanTag::Weekdays => "Weekdays".to_string(),
            ClanTag::Weekends => "Weekends".to_string(),
            ClanTag::EarlyMorning => "Early Morning".to_string(),
            ClanTag::Afternoon => "Afternoon".to_string(),
            ClanTag::Evening => "Evening".to_string(),
            ClanTag::LateNight => "Late Night".to_string(),
            ClanTag::TwentyFourSeven => "24/7 Active".to_string(),
            
            ClanTag::NorthAmerica => "North America".to_string(),
            ClanTag::SouthAmerica => "South America".to_string(),
            ClanTag::Europe => "Europe".to_string(),
            ClanTag::Asia => "Asia".to_string(),
            ClanTag::Africa => "Africa".to_string(),
            ClanTag::Oceania => "Oceania".to_string(),
            
            ClanTag::UTCMinus12 => "UTC-12".to_string(),
            ClanTag::UTCMinus11 => "UTC-11".to_string(),
            ClanTag::UTCMinus10 => "UTC-10 (HST)".to_string(),
            ClanTag::UTCMinus9 => "UTC-9 (AKST)".to_string(),
            ClanTag::UTCMinus8 => "UTC-8 (PST)".to_string(),
            ClanTag::UTCMinus7 => "UTC-7 (MST)".to_string(),
            ClanTag::UTCMinus6 => "UTC-6 (CST)".to_string(),
            ClanTag::UTCMinus5 => "UTC-5 (EST)".to_string(),
            ClanTag::UTCMinus4 => "UTC-4 (AST)".to_string(),
            ClanTag::UTCMinus3 => "UTC-3".to_string(),
            ClanTag::UTCMinus2 => "UTC-2".to_string(),
            ClanTag::UTCMinus1 => "UTC-1".to_string(),
            ClanTag::UTC => "UTC".to_string(),
            ClanTag::UTCPlus1 => "UTC+1 (CET)".to_string(),
            ClanTag::UTCPlus2 => "UTC+2 (EET)".to_string(),
            ClanTag::UTCPlus3 => "UTC+3 (MSK)".to_string(),
            ClanTag::UTCPlus4 => "UTC+4 (GST)".to_string(),
            ClanTag::UTCPlus5 => "UTC+5 (PKT)".to_string(),
            ClanTag::UTCPlus6 => "UTC+6 (BST)".to_string(),
            ClanTag::UTCPlus7 => "UTC+7 (ICT)".to_string(),
            ClanTag::UTCPlus8 => "UTC+8 (CST)".to_string(),
            ClanTag::UTCPlus9 => "UTC+9 (JST)".to_string(),
            ClanTag::UTCPlus10 => "UTC+10 (AEST)".to_string(),
            ClanTag::UTCPlus11 => "UTC+11".to_string(),
            ClanTag::UTCPlus12 => "UTC+12".to_string(),
            
            ClanTag::English => "English".to_string(),
            ClanTag::Spanish => "Spanish".to_string(),
            ClanTag::French => "French".to_string(),
            ClanTag::German => "German".to_string(),
            ClanTag::Chinese => "Chinese".to_string(),
            ClanTag::Japanese => "Japanese".to_string(),
            ClanTag::Russian => "Russian".to_string(),
            ClanTag::Portuguese => "Portuguese".to_string(),
            ClanTag::Italian => "Italian".to_string(),
            ClanTag::Korean => "Korean".to_string(),
            ClanTag::Multilingual => "Multilingual".to_string(),
            
            ClanTag::Casual => "Casual".to_string(),
            ClanTag::Competitive => "Competitive".to_string(),
            ClanTag::Educational => "Educational".to_string(),
            ClanTag::Social => "Social".to_string(),
            ClanTag::Serious => "Serious".to_string(),
            ClanTag::Fun => "Fun".to_string(),
            ClanTag::StudyGroup => "Study Group".to_string(),
            
            ClanTag::Coaching => "Coaching".to_string(),
            ClanTag::HandReviews => "Hand Reviews".to_string(),
            ClanTag::Strategy => "Strategy".to_string(),
            ClanTag::Streaming => "Streaming".to_string(),
            ClanTag::VoiceChat => "Voice Chat".to_string(),
            ClanTag::TextChat => "Text Chat".to_string(),
            ClanTag::DiscordIntegration => "Discord".to_string(),
            
            ClanTag::InviteOnly => "Invite Only".to_string(),
            ClanTag::Verified => "Verified Members".to_string(),
            ClanTag::Application => "Application Required".to_string(),
            
            ClanTag::Rakeback => "Rakeback".to_string(),
            ClanTag::Freerolls => "Freerolls".to_string(),
            ClanTag::Rewards => "Rewards".to_string(),
            ClanTag::Leaderboards => "Leaderboards".to_string(),
            ClanTag::Exclusive => "Exclusive".to_string(),
            ClanTag::VIP => "VIP".to_string(),
            
            ClanTag::Custom(tag) => tag.clone(),
        }
    }

    /// Get the category this tag belongs to
    pub fn category(&self) -> TagCategory {
        match self {
            ClanTag::Beginner | ClanTag::Intermediate | ClanTag::Advanced | 
            ClanTag::Professional | ClanTag::MixedSkill => TagCategory::SkillLevel,
            
            ClanTag::Microstakes | ClanTag::LowStakes | ClanTag::MidStakes | 
            ClanTag::HighStakes | ClanTag::Nosebleeds => TagCategory::Stakes,
            
            ClanTag::CashGames | ClanTag::Tournaments | ClanTag::SitAndGo | 
            ClanTag::SpinAndGo | ClanTag::HeadsUp | ClanTag::MixedGameTypes => TagCategory::GameType,
            
            ClanTag::Weekdays | ClanTag::Weekends | ClanTag::EarlyMorning | 
            ClanTag::Afternoon | ClanTag::Evening | ClanTag::LateNight | 
            ClanTag::TwentyFourSeven => TagCategory::Schedule,
            
            ClanTag::NorthAmerica | ClanTag::SouthAmerica | ClanTag::Europe | 
            ClanTag::Asia | ClanTag::Africa | ClanTag::Oceania |
            ClanTag::UTCMinus12 | ClanTag::UTCMinus11 | ClanTag::UTCMinus10 | 
            ClanTag::UTCMinus9 | ClanTag::UTCMinus8 | ClanTag::UTCMinus7 | 
            ClanTag::UTCMinus6 | ClanTag::UTCMinus5 | ClanTag::UTCMinus4 | 
            ClanTag::UTCMinus3 | ClanTag::UTCMinus2 | ClanTag::UTCMinus1 | 
            ClanTag::UTC | ClanTag::UTCPlus1 | ClanTag::UTCPlus2 | 
            ClanTag::UTCPlus3 | ClanTag::UTCPlus4 | ClanTag::UTCPlus5 | 
            ClanTag::UTCPlus6 | ClanTag::UTCPlus7 | ClanTag::UTCPlus8 | 
            ClanTag::UTCPlus9 | ClanTag::UTCPlus10 | ClanTag::UTCPlus11 | 
            ClanTag::UTCPlus12 => TagCategory::Geographic,
            
            ClanTag::English | ClanTag::Spanish | ClanTag::French | ClanTag::German |
            ClanTag::Chinese | ClanTag::Japanese | ClanTag::Russian | ClanTag::Portuguese |
            ClanTag::Italian | ClanTag::Korean | ClanTag::Multilingual => TagCategory::Language,
            
            ClanTag::Casual | ClanTag::Competitive | ClanTag::Educational | 
            ClanTag::Social | ClanTag::Serious | ClanTag::Fun | 
            ClanTag::StudyGroup => TagCategory::CommunityStyle,
            
            ClanTag::Coaching | ClanTag::HandReviews | ClanTag::Strategy | 
            ClanTag::Streaming | ClanTag::VoiceChat | ClanTag::TextChat | 
            ClanTag::DiscordIntegration => TagCategory::Features,
            
            ClanTag::InviteOnly | ClanTag::Verified | ClanTag::Application => TagCategory::Requirements,
            
            ClanTag::Rakeback | ClanTag::Freerolls | ClanTag::Rewards | 
            ClanTag::Leaderboards | ClanTag::Exclusive | ClanTag::VIP => TagCategory::Benefits,
            
            ClanTag::Custom(_) => TagCategory::Custom,
        }
    }

    /// Create a custom tag with validation
    pub fn custom(tag: &str) -> Result<Self, String> {
        if tag.is_empty() || tag.len() > 20 {
            return Err("Custom tag must be 1-20 characters".to_string());
        }
        
        if tag.chars().any(|c| !c.is_alphanumeric() && c != ' ' && c != '-' && c != '_') {
            return Err("Custom tag can only contain letters, numbers, spaces, hyphens, and underscores".to_string());
        }
        
        Ok(ClanTag::Custom(tag.to_string()))
    }

    /// Get all predefined tags by category
    pub fn get_tags_by_category(category: TagCategory) -> Vec<ClanTag> {
        match category {
            TagCategory::SkillLevel => vec![
                ClanTag::Beginner, ClanTag::Intermediate, ClanTag::Advanced,
                ClanTag::Professional, ClanTag::MixedSkill,
            ],
            TagCategory::Stakes => vec![
                ClanTag::Microstakes, ClanTag::LowStakes, ClanTag::MidStakes,
                ClanTag::HighStakes, ClanTag::Nosebleeds,
            ],
            TagCategory::GameType => vec![
                ClanTag::CashGames, ClanTag::Tournaments, ClanTag::SitAndGo,
                ClanTag::SpinAndGo, ClanTag::HeadsUp, ClanTag::MixedGameTypes,
            ],
            TagCategory::Schedule => vec![
                ClanTag::Weekdays, ClanTag::Weekends, ClanTag::EarlyMorning,
                ClanTag::Afternoon, ClanTag::Evening, ClanTag::LateNight,
                ClanTag::TwentyFourSeven,
            ],
            TagCategory::Geographic => vec![
                ClanTag::NorthAmerica, ClanTag::SouthAmerica, ClanTag::Europe, 
                ClanTag::Asia, ClanTag::Africa, ClanTag::Oceania,
                ClanTag::UTCMinus12, ClanTag::UTCMinus11, ClanTag::UTCMinus10,
                ClanTag::UTCMinus9, ClanTag::UTCMinus8, ClanTag::UTCMinus7,
                ClanTag::UTCMinus6, ClanTag::UTCMinus5, ClanTag::UTCMinus4,
                ClanTag::UTCMinus3, ClanTag::UTCMinus2, ClanTag::UTCMinus1,
                ClanTag::UTC, ClanTag::UTCPlus1, ClanTag::UTCPlus2,
                ClanTag::UTCPlus3, ClanTag::UTCPlus4, ClanTag::UTCPlus5,
                ClanTag::UTCPlus6, ClanTag::UTCPlus7, ClanTag::UTCPlus8,
                ClanTag::UTCPlus9, ClanTag::UTCPlus10, ClanTag::UTCPlus11,
                ClanTag::UTCPlus12,
            ],
            TagCategory::Language => vec![
                ClanTag::English, ClanTag::Spanish, ClanTag::French, ClanTag::German,
                ClanTag::Chinese, ClanTag::Japanese, ClanTag::Russian, ClanTag::Portuguese,
                ClanTag::Italian, ClanTag::Korean, ClanTag::Multilingual,
            ],
            TagCategory::CommunityStyle => vec![
                ClanTag::Casual, ClanTag::Competitive, ClanTag::Educational,
                ClanTag::Social, ClanTag::Serious, ClanTag::Fun, ClanTag::StudyGroup,
            ],
            TagCategory::Features => vec![
                ClanTag::Coaching, ClanTag::HandReviews, ClanTag::Strategy,
                ClanTag::Streaming, ClanTag::VoiceChat, ClanTag::TextChat,
                ClanTag::DiscordIntegration,
            ],
            TagCategory::Requirements => vec![
                ClanTag::InviteOnly, ClanTag::Verified,
                ClanTag::Application,
            ],
            TagCategory::Benefits => vec![
                ClanTag::Rakeback, ClanTag::Freerolls, ClanTag::Rewards,
                ClanTag::Leaderboards, ClanTag::Exclusive, ClanTag::VIP,
            ],
            TagCategory::Custom => vec![], // Custom tags are user-defined
        }
    }
}

/// Categories for organizing tags
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq, Hash)]
pub enum TagCategory {
    SkillLevel,
    Stakes,
    GameType,
    Schedule,
    Geographic,
    Language,
    CommunityStyle,
    Features,
    Requirements,
    Benefits,
    Custom,
}

impl TagCategory {
    pub fn display_name(&self) -> String {
        match self {
            TagCategory::SkillLevel => "Skill Level".to_string(),
            TagCategory::Stakes => "Stakes".to_string(),
            TagCategory::GameType => "Game Type".to_string(),
            TagCategory::Schedule => "Schedule".to_string(),
            TagCategory::Geographic => "Region/Timezone".to_string(),
            TagCategory::Language => "Language".to_string(),
            TagCategory::CommunityStyle => "Community Style".to_string(),
            TagCategory::Features => "Features".to_string(),
            TagCategory::Requirements => "Requirements".to_string(),
            TagCategory::Benefits => "Benefits".to_string(),
            TagCategory::Custom => "Custom".to_string(),
        }
    }
}
