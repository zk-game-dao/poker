use candid::CandidType;
use errors::tournament_error::TournamentError;
use serde::{Deserialize, Serialize};

use super::spin_and_go::SpinGoMultiplier;

use super::table_balancing::TableBalancer;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum TournamentType {
    BuyIn(TournamentSizeType),
    SitAndGo(TournamentSizeType),
    SpinAndGo(TournamentSizeType, SpinGoMultiplier),
    Freeroll(TournamentSizeType),
}

impl TournamentType {
    pub fn get_type_id(&self) -> u8 {
        match self {
            TournamentType::BuyIn(_) => 0,
            TournamentType::SitAndGo(_) => 1,
            TournamentType::Freeroll(_) => 2,
            TournamentType::SpinAndGo(_, _) => 3,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum TournamentSizeType {
    SingleTable(BuyInOptions),
    MultiTable(BuyInOptions, TableBalancer),
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct BuyInOptions {
    pub freezout: bool,
    pub reentry: ReentryOptions,
    pub rebuy: RebuyOptions,
    pub addon: AddonOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ReentryOptions {
    pub enabled: bool,
    pub max_reentries: u32, // Maximum number of reentrys allowed per player
    pub reentry_end_timestamp: u64, // Duration of reentry period in seconds
    pub reentry_price: u64, // Cost of each reentry
    pub reentry_chips: u64, // Chips received for reentry
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct RebuyOptions {
    pub enabled: bool,
    pub max_rebuys: u32,           // Maximum number of rebuys allowed per player
    pub rebuy_window_seconds: u64, // Time window (in seconds) to make rebuy decision
    pub rebuy_end_timestamp: u64,  // Duration of rebuy period in seconds
    pub rebuy_price: u64,          // Cost of each rebuy
    pub rebuy_chips: u64,          // Chips received for rebuy
    pub min_chips_for_rebuy: u64,  // Minimum chip threshold to allow rebuy
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct AddonOptions {
    pub enabled: bool,
    pub max_addons: u32,       // Maximum number of addons allowed per player
    pub addon_start_time: u64, // When addon becomes available (usually after reentry period)
    pub addon_end_time: u64,   // When addon is no longer available
    pub addon_price: u64,      // Cost of addon
    pub addon_chips: u64,      // Chips received for addon
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct NewTournamentOptions {
    // Rebuy options
    pub enable_rebuy: bool,
    pub max_rebuys: u32,
    pub rebuy_window_seconds: u64,
    pub rebuy_end_timestamp: u64,
    pub rebuy_price: u64,
    pub rebuy_chips: u64,
    pub min_chips_for_rebuy: u64,

    // Reentry options
    pub enable_reentry: bool,
    pub max_reentries: u32,
    pub reentry_end_timestamp: u64,
    pub reentry_price: u64,
    pub reentry_chips: u64,

    // Addon options
    pub max_addons: u32,
    pub addon_price: u64,
    pub addon_chips: u64,
    pub addon_start_time: u64,
    pub addon_end_time: u64,
}
// Example usage:
impl BuyInOptions {
    pub fn new_freezout() -> Self {
        Self {
            freezout: true,
            reentry: ReentryOptions {
                enabled: false,
                max_reentries: 0,
                reentry_end_timestamp: 0,
                reentry_price: 0,
                reentry_chips: 0,
            },
            addon: AddonOptions {
                enabled: false,
                max_addons: 0,
                addon_start_time: 0,
                addon_end_time: 0,
                addon_price: 0,
                addon_chips: 0,
            },
            rebuy: RebuyOptions {
                enabled: false,
                max_rebuys: 0,
                rebuy_window_seconds: 0,
                rebuy_end_timestamp: 0,
                rebuy_price: 0,
                rebuy_chips: 0,
                min_chips_for_rebuy: 0,
            },
        }
    }

    pub fn new_reentry(
        new_tournament_options: NewTournamentOptions,
    ) -> Result<Self, TournamentError> {
        let addon = if new_tournament_options.max_addons > 0 {
            AddonOptions {
                enabled: true,
                max_addons: new_tournament_options.max_addons,
                addon_start_time: new_tournament_options.addon_start_time, // Addon typically available after reentry period
                addon_end_time: new_tournament_options.addon_end_time,
                addon_price: new_tournament_options.addon_price,
                addon_chips: new_tournament_options.addon_chips,
            }
        } else {
            AddonOptions {
                enabled: false,
                max_addons: 0,
                addon_start_time: 0,
                addon_end_time: 0,
                addon_price: 0,
                addon_chips: 0,
            }
        };
        let reentry = if new_tournament_options.max_reentries > 0 {
            ReentryOptions {
                enabled: true,
                max_reentries: new_tournament_options.max_reentries,
                reentry_end_timestamp: new_tournament_options.reentry_end_timestamp,
                reentry_price: new_tournament_options.reentry_price,
                reentry_chips: new_tournament_options.reentry_chips,
            }
        } else {
            ReentryOptions {
                enabled: false,
                max_reentries: 0,
                reentry_end_timestamp: 0,
                reentry_price: 0,
                reentry_chips: 0,
            }
        };
        let rebuy = if new_tournament_options.enable_rebuy {
            RebuyOptions {
                enabled: true,
                max_rebuys: new_tournament_options.max_rebuys,
                rebuy_window_seconds: new_tournament_options.rebuy_window_seconds,
                rebuy_end_timestamp: new_tournament_options.rebuy_end_timestamp,
                rebuy_price: new_tournament_options.rebuy_price,
                rebuy_chips: new_tournament_options.rebuy_chips,
                min_chips_for_rebuy: new_tournament_options.min_chips_for_rebuy,
            }
        } else {
            RebuyOptions {
                enabled: false,
                max_rebuys: 0,
                rebuy_window_seconds: 0,
                rebuy_end_timestamp: 0,
                rebuy_price: 0,
                rebuy_chips: 0,
                min_chips_for_rebuy: 0,
            }
        };
        let addon_period =
            new_tournament_options.addon_end_time - new_tournament_options.addon_start_time;
        if reentry.enabled
            && addon.enabled
            && addon.addon_start_time < reentry.reentry_end_timestamp
        {
            return Err(TournamentError::AddonNotAllowed(
                "Addon start time must be after reentry period".to_string(),
            ));
        } else if new_tournament_options.addon_end_time < new_tournament_options.addon_start_time {
            return Err(TournamentError::AddonNotAllowed(
                "Addon end time must be after addon start time".to_string(),
            ));
        } else if addon.enabled && addon_period < 180_000_000_000 {
            return Err(TournamentError::AddonNotAllowed(
                "Addon period must be at least 3 minutes".to_string(),
            ));
        } else if addon.enabled && addon_period > 600_000_000_000 {
            return Err(TournamentError::AddonNotAllowed(
                "Addon period must be at most 10 minutes".to_string(),
            ));
        }

        Ok(Self {
            freezout: false,
            reentry,
            addon,
            rebuy,
        })
    }

    pub fn can_reentry(&self, reentrys_used: u32) -> bool {
        self.reentry.enabled && reentrys_used < self.reentry.max_reentries
    }

    pub fn can_addon(&self, addons_used: u32) -> bool {
        self.addon.enabled && addons_used < self.addon.max_addons
    }
}
