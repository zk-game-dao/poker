use candid::CandidType;
use serde::{Deserialize, Serialize};
use table::poker::game::table_functions::ante::AnteType;

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
use ic_cdk::api::time as get_time;

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
fn get_time() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("System time before UNIX epoch")
        .as_nanos() as u64
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct SpeedTypeParams {
    pub level_duration_ns: u64,
    pub blind_multiplier: f64,        // e.g., 1.5 for 50% increase
    pub ante_start_level: u8,         // level at which antes begin
    pub ante_percentage: u8,          // percentage of big blind for ante
    pub initial_blind_percentage: u8, // percentage of starting stack for initial small blind. Needs to be divided by 100.
    pub blind_levels: Vec<BlindLevel>,
    pub current_level: u8,
    pub next_level_time: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum SpeedType {
    Regular(SpeedTypeParams),
    Turbo(SpeedTypeParams),
    HyperTurbo(SpeedTypeParams),
    Custom(SpeedTypeParams),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct BlindLevel {
    pub small_blind: u64,
    pub big_blind: u64,
    pub ante_type: AnteType,
    pub duration_ns: u64,
}

fn generate_blind_levels(
    starting_stack: u64,
    level_duration_ns: u64,
    blind_multiplier: f64,
    ante_start_level: u8,
    ante_percentage: u8,
    initial_blind_percentage: u8,
    max_levels: u8,
) -> Vec<BlindLevel> {
    let initial_small_blind =
        ((starting_stack as f64 * (initial_blind_percentage as f64 / 100.0)).round() / 2.0).round()
            as u64;
    let mut levels = Vec::new();
    let mut current_small_blind = initial_small_blind;

    for level in 0..max_levels {
        let big_blind = current_small_blind * 2;

        let ante_type = if level >= ante_start_level {
            AnteType::PercentageOfBigBlind(ante_percentage)
        } else {
            AnteType::None
        };

        levels.push(BlindLevel {
            small_blind: current_small_blind,
            big_blind,
            ante_type,
            duration_ns: level_duration_ns,
        });

        current_small_blind = (current_small_blind as f64 * blind_multiplier).round() as u64;

        if big_blind * 2 >= starting_stack {
            break;
        }
    }

    levels
}

impl SpeedType {
    pub fn new_default(starting_stack: u64, max_levels: u8) -> Self {
        let params = SpeedTypeParams {
            level_duration_ns: 9e11 as u64, // 15 minutes
            blind_multiplier: 1.5,          // 50% increase
            ante_start_level: 4,            // antes start at level 5
            ante_percentage: 10,            // 10% of big blind
            initial_blind_percentage: 2,
            blind_levels: generate_blind_levels(
                starting_stack,
                9e11 as u64,
                1.5,
                4,
                10,
                1,
                max_levels,
            ),
            current_level: 0,
            next_level_time: Some(u64::MAX),
        };
        SpeedType::Regular(params)
    }
    pub fn new_regular(starting_stack: u64, max_levels: u8) -> Self {
        let params = SpeedTypeParams {
            level_duration_ns: 9e11 as u64, // 15 minutes
            blind_multiplier: 1.5,          // 50% increase
            ante_start_level: 4,            // antes start at level 5
            ante_percentage: 10,            // 10% of big blind
            initial_blind_percentage: 2,
            blind_levels: generate_blind_levels(
                starting_stack,
                9e11 as u64,
                1.5,
                4,
                10,
                1,
                max_levels,
            ),
            current_level: 0,
            next_level_time: Some(get_time() + 9e11 as u64),
        };
        SpeedType::Regular(params)
    }

    pub fn new_turbo(starting_stack: u64, max_levels: u8) -> Self {
        let params = SpeedTypeParams {
            level_duration_ns: 6e11 as u64, // 10 minutes
            blind_multiplier: 1.75,         // 75% increase
            ante_start_level: 3,            // antes start at level 4
            ante_percentage: 12,            // 12% of big blind
            initial_blind_percentage: 2,
            blind_levels: generate_blind_levels(
                starting_stack,
                6e11 as u64,
                1.75,
                3,
                12,
                1,
                max_levels,
            ),
            current_level: 0,
            next_level_time: Some(get_time() + 6e11 as u64),
        };
        SpeedType::Turbo(params)
    }

    pub fn new_hyper_turbo(starting_stack: u64, max_levels: u8) -> Self {
        let params = SpeedTypeParams {
            level_duration_ns: 1.8e11 as u64, // 3 minutes
            blind_multiplier: 2.0,            // 100% increase
            ante_start_level: 2,              // antes start at level 3
            ante_percentage: 15,              // 15% of big blind
            initial_blind_percentage: 4,
            blind_levels: generate_blind_levels(
                starting_stack,
                1.8e11 as u64,
                2.0,
                2,
                15,
                1,
                max_levels,
            ),
            current_level: 0,
            next_level_time: Some(get_time() + 1.8e11 as u64),
        };
        SpeedType::HyperTurbo(params)
    }

    pub fn new_custom(
        starting_stack: u64,
        max_levels: u8,
        level_duration_ns: u64,
        blind_multiplier: f64,
        ante_start_level: u8,
        ante_percentage: u8,
        initial_blind_percentage: u8,
    ) -> Self {
        let params = SpeedTypeParams {
            level_duration_ns,
            blind_multiplier,
            ante_start_level,
            ante_percentage,
            initial_blind_percentage,
            blind_levels: generate_blind_levels(
                starting_stack,
                level_duration_ns,
                blind_multiplier,
                ante_start_level,
                ante_percentage,
                initial_blind_percentage,
                max_levels,
            ),
            current_level: 0,
            next_level_time: Some(get_time() + level_duration_ns),
        };
        SpeedType::Custom(params)
    }

    pub fn new_spin_and_go(starting_stack: u64, max_levels: u8) -> Self {
        let params = SpeedTypeParams {
            level_duration_ns: 1.2e11 as u64, // 2 minutes
            blind_multiplier: 2.0,            // 100% increase
            ante_start_level: 1,              // antes start at level 2
            ante_percentage: 15,              // 15% of big blind
            initial_blind_percentage: 4,
            blind_levels: generate_blind_levels(
                starting_stack,
                1.2e11 as u64,
                2.0,
                1,
                15,
                4,
                max_levels,
            ),
            current_level: 0,
            next_level_time: Some(get_time() + 1.2e11 as u64),
        };
        SpeedType::HyperTurbo(params)
    }

    pub fn get_params(&self) -> &SpeedTypeParams {
        match self {
            SpeedType::Regular(params) => params,
            SpeedType::Turbo(params) => params,
            SpeedType::HyperTurbo(params) => params,
            SpeedType::Custom(params) => params,
        }
    }

    pub fn get_params_mut(&mut self) -> &mut SpeedTypeParams {
        match self {
            SpeedType::Regular(params) => params,
            SpeedType::Turbo(params) => params,
            SpeedType::HyperTurbo(params) => params,
            SpeedType::Custom(params) => params,
        }
    }

    // Helper to get estimated duration in seconds
    pub fn get_estimated_duration(&self) -> u64 {
        self.get_params()
            .blind_levels
            .iter()
            .map(|level| level.duration_ns)
            .sum()
    }

    pub fn get_blind_level(&self) -> BlindLevel {
        let params = self.get_params();
        params.blind_levels[params.current_level as usize].clone()
    }
}

impl BlindLevel {
    pub fn get_ante_amount(&self) -> u64 {
        match self.ante_type {
            AnteType::None => 0,
            AnteType::Fixed(amount) => amount,
            AnteType::BigBlindAnte => self.big_blind,
            AnteType::PercentageOfBigBlind(percentage) => {
                (self.big_blind * percentage as u64) / 100
            }
        }
    }

    pub fn is_big_blind_ante(&self) -> bool {
        matches!(self.ante_type, AnteType::BigBlindAnte)
    }

    pub fn get_blinds(&self) -> (u64, u64, u64) {
        (self.small_blind, self.big_blind, self.get_ante_amount())
    }
}
