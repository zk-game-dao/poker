use std::collections::{HashMap, HashSet};

use candid::{CandidType, Principal};
use errors::tournament_error::TournamentError;
use macros::impl_principal_traits;
use serde::{Deserialize, Serialize};
use table::poker::game::table_functions::{table::{TableConfig, TableId}, types::CurrencyType};
use user::user::{UsersCanisterId, WalletPrincipalId};

use crate::tournaments::payouts::{calculate_dynamic_payout_structure, PayoutStructure};

use super::{
    blind_level::{BlindLevel, SpeedType},
    spin_and_go::{SpinGoMultiplier, SpinGoMultiplierDistribution},
    table_balancing::TableBalancer,
    tournament_type::{BuyInOptions, TournamentSizeType, TournamentType},
};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum UserTournamentAction {
    Join(WalletPrincipalId),
    Leave(WalletPrincipalId),
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq, Hash, Copy)]
pub struct TournamentId(pub Principal);

impl_principal_traits!(TournamentId);

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct TournamentData {
    pub id: TournamentId,
    pub name: String,
    pub description: String,
    pub hero_picture: String,

    pub currency: CurrencyType,
    pub buy_in: u64,
    pub guaranteed_prize_pool: Option<u64>, // For guaranteed tournaments
    pub starting_chips: u64,
    pub speed_type: SpeedType,

    pub min_players: u8,
    pub max_players: u32,

    pub late_registration_duration_ns: u64,
    pub payout_structure: PayoutStructure,
    pub tournament_type: TournamentType,

    pub current_players: HashMap<WalletPrincipalId, UserTournamentData>,
    pub all_players: HashMap<WalletPrincipalId, UserTournamentData>,
    pub state: TournamentState,
    pub start_time: u64,

    pub table_config: TableConfig,
    pub tables: HashMap<TableId, TableInfo>, // Key is canister id of table and value is list of players
    pub sorted_users: Option<Vec<WalletPrincipalId>>,

    pub require_proof_of_humanity: bool,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct TableInfo {
    pub players: HashSet<WalletPrincipalId>,
    pub last_balance_time: Option<u64>,
}

impl Default for TableInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl TableInfo {
    pub fn new() -> Self {
        Self {
            players: HashSet::new(),
            last_balance_time: None,
        }
    }
}

impl Default for TournamentData {
    fn default() -> Self {
        Self {
            id: TournamentId::default(),
            name: "".to_string(),
            description: "".to_string(),
            hero_picture: "".to_string(),
            currency: CurrencyType::Fake,
            buy_in: 0,
            guaranteed_prize_pool: None,
            starting_chips: 0,
            speed_type: SpeedType::new_default(0, 0),
            min_players: 0,
            max_players: 0,
            late_registration_duration_ns: 0,
            payout_structure: PayoutStructure::default(),
            tournament_type: TournamentType::BuyIn(TournamentSizeType::SingleTable(
                BuyInOptions::new_freezout(),
            )),
            current_players: HashMap::new(),
            all_players: HashMap::new(),
            state: TournamentState::Registration,
            start_time: u64::MAX,
            table_config: TableConfig::default(),
            tables: HashMap::new(),
            sorted_users: None,
            require_proof_of_humanity: false,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UserTournamentData {
    pub users_canister_principal: UsersCanisterId,
    pub chips: u64,
    pub position: u32,
    pub reentries: u32,
    pub addons: u32,
    pub rebuys: u32,
}

impl UserTournamentData {
    pub fn new(users_canister_principal: UsersCanisterId, chips: u64, position: u32) -> Self {
        Self {
            users_canister_principal,
            chips,
            position,
            reentries: 0,
            addons: 0,
            rebuys: 0,
        }
    }
}

impl Default for UserTournamentData {
    fn default() -> Self {
        Self {
            users_canister_principal: UsersCanisterId::default(),
            chips: 0,
            position: 0,
            reentries: 0,
            addons: 0,
            rebuys: 0,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TournamentState {
    Registration,
    LateRegistration,
    Running,
    FinalTable,
    Completed,
    Cancelled,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CustomTournamentSpeedType {
    max_levels: u8,
    level_duration_ns: u64,
    blind_multiplier: f64,
    ante_start_level: u8,
    ante_percentage: u8,
    initial_blind_percentage: u8,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum NewTournamentSpeedType {
    Regular(u8),
    Turbo(u8),
    HyperTurbo(u8),
    Custom(CustomTournamentSpeedType),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct NewTournament {
    pub name: String,
    pub description: String,
    pub hero_picture: String,
    pub currency: CurrencyType,
    pub buy_in: u64,
    pub guaranteed_prize_pool: Option<u64>, // For guaranteed tournaments
    pub starting_chips: u64,
    pub speed_type: NewTournamentSpeedType,
    pub min_players: u8,
    pub max_players: u32,
    pub late_registration_duration_ns: u64,
    pub tournament_type: TournamentType,
    pub start_time: u64,
    pub require_proof_of_humanity: bool,
}

impl TournamentData {
    pub fn new(
        id: TournamentId,
        mut new_tournament_data: NewTournament,
        table_config: TableConfig,
    ) -> Result<Self, TournamentError> {
        let speed_type = match &new_tournament_data.speed_type {
            NewTournamentSpeedType::Regular(levels) => {
                if *levels < 10 {
                    return Err(TournamentError::InvalidConfiguration(
                        "Regular tournaments must have at least 10 levels".to_string(),
                    ));
                }
                SpeedType::new_regular(new_tournament_data.starting_chips, *levels)
            }
            NewTournamentSpeedType::Turbo(levels) => {
                if *levels < 15 {
                    return Err(TournamentError::InvalidConfiguration(
                        "Turbo tournaments must have at least 15 levels".to_string(),
                    ));
                }
                SpeedType::new_turbo(new_tournament_data.starting_chips, *levels)
            }
            NewTournamentSpeedType::HyperTurbo(levels) => {
                if *levels < 20 {
                    return Err(TournamentError::InvalidConfiguration(
                        "Hyper-turbo tournaments must have at least 20 levels".to_string(),
                    ));
                }
                SpeedType::new_hyper_turbo(new_tournament_data.starting_chips, *levels)
            }
            NewTournamentSpeedType::Custom(custom) => {
                if custom.level_duration_ns == 0 {
                    return Err(TournamentError::InvalidConfiguration(
                        "Custom tournaments must have a non-zero level duration".to_string(),
                    ));
                }
                SpeedType::new_custom(
                    new_tournament_data.starting_chips,
                    custom.max_levels,
                    custom.level_duration_ns,
                    custom.blind_multiplier,
                    custom.ante_start_level,
                    custom.ante_percentage,
                    custom.initial_blind_percentage,
                )
            }
        };

        let tournament_type = match &mut new_tournament_data.tournament_type {
            TournamentType::BuyIn(size_type) => match size_type {
                TournamentSizeType::SingleTable(options) => {
                    TournamentType::BuyIn(TournamentSizeType::SingleTable(options.clone()))
                }
                TournamentSizeType::MultiTable(options, table_balancer) => {
                    let balancer = TableBalancer::new(
                        table_balancer.min_players_per_table,
                        table_balancer.max_players_per_table,
                        &speed_type,
                    );
                    TournamentType::BuyIn(TournamentSizeType::MultiTable(options.clone(), balancer))
                }
            },
            TournamentType::Freeroll(size_type) => match size_type {
                TournamentSizeType::SingleTable(options) => {
                    TournamentType::Freeroll(TournamentSizeType::SingleTable(options.clone()))
                }
                TournamentSizeType::MultiTable(options, table_balancer) => {
                    let balancer = TableBalancer::new(
                        table_balancer.min_players_per_table,
                        table_balancer.max_players_per_table,
                        &speed_type,
                    );
                    TournamentType::Freeroll(TournamentSizeType::MultiTable(
                        options.clone(),
                        balancer,
                    ))
                }
            },
            TournamentType::SitAndGo(size_type) => match size_type {
                TournamentSizeType::SingleTable(options) => {
                    TournamentType::SitAndGo(TournamentSizeType::SingleTable(options.clone()))
                }
                _ => {
                    return Err(TournamentError::InvalidConfiguration(
                        "Invalid Sit and Go options".to_string(),
                    ));
                }
            },
            TournamentType::SpinAndGo(size_type, multiplier) => match size_type {
                TournamentSizeType::SingleTable(options) => TournamentType::SpinAndGo(
                    TournamentSizeType::SingleTable(options.clone()),
                    multiplier.clone(),
                ),
                _ => {
                    return Err(TournamentError::InvalidConfiguration(
                        "Invalid Spin and Go options".to_string(),
                    ));
                }
            },
        };

        ic_cdk::println!("Tournament type: {:?}", tournament_type);

        let tournament = Self {
            id,
            name: new_tournament_data.name,
            description: new_tournament_data.description,
            hero_picture: new_tournament_data.hero_picture,
            currency: new_tournament_data.currency,
            buy_in: new_tournament_data.buy_in,
            guaranteed_prize_pool: new_tournament_data.guaranteed_prize_pool,
            starting_chips: new_tournament_data.starting_chips,
            speed_type,
            min_players: new_tournament_data.min_players,
            max_players: new_tournament_data.max_players,
            late_registration_duration_ns: new_tournament_data.late_registration_duration_ns,
            payout_structure: PayoutStructure::default(),
            tournament_type,
            current_players: HashMap::new(),
            all_players: HashMap::new(),
            state: TournamentState::Registration,
            start_time: new_tournament_data.start_time,
            table_config,
            tables: HashMap::new(),
            sorted_users: None,
            require_proof_of_humanity: new_tournament_data.require_proof_of_humanity,
        };

        Ok(tournament)
    }

    pub async fn new_spin_and_go(
        id: TournamentId,
        new_tournament_data: NewTournament,
        table_config: TableConfig,
    ) -> Result<(Self, u64), TournamentError> {
        // Create a base tournament first
        let mut tournament = Self::new(id, new_tournament_data.clone(), table_config)?;

        // Override with Spin and Go specific settings
        // Force 3 players for standard Spin and Go
        tournament.min_players = 3;
        tournament.max_players = 3;

        // Use hyper-turbo blind structure
        tournament.speed_type = SpeedType::new_spin_and_go(
            new_tournament_data.starting_chips,
            20, // max levels, adjust as needed
        );

        // Get a random multiplier from the distribution
        let distribution = SpinGoMultiplierDistribution::standard();
        let selected_multiplier = distribution.select_random_multiplier().await?;

        // Calculate prize pool based on buy-in and multiplier
        let prize_pool = new_tournament_data.buy_in * selected_multiplier.multiplier;

        // Update tournament type and payout structure
        tournament.tournament_type = TournamentType::SpinAndGo(
            TournamentSizeType::SingleTable(BuyInOptions::new_freezout()),
            SpinGoMultiplier::from(selected_multiplier.clone()),
        );

        // Set payout structure from the selected multiplier
        tournament.payout_structure = PayoutStructure { payouts: selected_multiplier.payout_structure };

        // No late registration for Spin and Go
        tournament.late_registration_duration_ns = 0;

        Ok((tournament, prize_pool))
    }

    pub fn is_full(&self) -> bool {
        self.current_players.len() >= self.max_players as usize
    }

    pub fn get_current_blinds(&self) -> (u64, u64, u64) {
        self.speed_type.get_blind_level().get_blinds()
    }

    pub fn should_increase_blinds(&self) -> bool {
        let blind_params = self.speed_type.get_params();
        if (blind_params.current_level + 1) as usize >= blind_params.blind_levels.len() {
            return false;
        }
        if let Some(next_level_time) = blind_params.next_level_time {
            ic_cdk::api::time() >= next_level_time
        } else {
            false
        }
    }

    pub fn increase_blinds(&mut self) -> Option<BlindLevel> {
        let blind_params = self.speed_type.get_params_mut();
        if (blind_params.current_level + 1) as usize >= blind_params.blind_levels.len() {
            return None;
        }

        blind_params.current_level += 1;
        let new_level = blind_params.blind_levels[blind_params.current_level as usize].clone();
        blind_params.next_level_time = Some(ic_cdk::api::time() + new_level.duration_ns);
        Some(new_level)
    }

    pub fn validate(&self) -> Result<(), TournamentError> {
        // Validate blind structure
        if self.speed_type.get_params().blind_levels.is_empty() {
            return Err(TournamentError::InvalidConfiguration(
                "Blind structure cannot be empty".to_string(),
            ));
        }

        if self.max_players < 2 {
            return Err(TournamentError::InvalidConfiguration(
                "Max players per table must be at least 2".to_string(),
            ));
        } else if self.max_players > 8
            && !matches!(
                self.tournament_type,
                TournamentType::Freeroll(TournamentSizeType::MultiTable(_, _))
                    | TournamentType::BuyIn(TournamentSizeType::MultiTable(_, _))
            )
        {
            return Err(TournamentError::InvalidConfiguration(
                "Max players must be less than or equal to 8 for single table tournaments"
                    .to_string(),
            ));
        };

        match &self.tournament_type {
            TournamentType::BuyIn(TournamentSizeType::SingleTable(option))
            | TournamentType::BuyIn(TournamentSizeType::MultiTable(option, _))
            | TournamentType::Freeroll(TournamentSizeType::SingleTable(option))
            | TournamentType::Freeroll(TournamentSizeType::MultiTable(option, _))
            | TournamentType::SitAndGo(TournamentSizeType::SingleTable(option))
            | TournamentType::SpinAndGo(TournamentSizeType::SingleTable(option), _) => {
                if option.addon.enabled
                    && option.addon.addon_start_time <= self.start_time + 3e11 as u64
                {
                    return Err(TournamentError::InvalidConfiguration(
                        "Addon start time must be over 5 minutes after the tournament starts"
                            .to_string(),
                    ));
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn get_user_tournament_data(
        &self,
        user_principal: &WalletPrincipalId,
    ) -> Result<&UserTournamentData, TournamentError> {
        match self
            .current_players
            .get(user_principal)
            .ok_or(TournamentError::Other(
                "Could not get user tournament data".to_string(),
            )) {
            Ok(data) => Ok(data),
            Err(_) => self
                .all_players
                .get(user_principal)
                .ok_or(TournamentError::Other(
                    "Could not get user tournament data".to_string(),
                )),
        }
    }

    pub fn get_user_tournament_data_mut(
        &mut self,
        user_principal: &WalletPrincipalId,
    ) -> Result<&mut UserTournamentData, TournamentError> {
        match self
            .current_players
            .get_mut(user_principal)
            .ok_or(TournamentError::Other(
                "Could not get user tournament data".to_string(),
            )) {
            Ok(data) => Ok(data),
            Err(_) => self
                .all_players
                .get_mut(user_principal)
                .ok_or(TournamentError::Other(
                    "Could not get user tournament data".to_string(),
                )),
        }
    }

    pub fn calculate_payouts(&mut self) -> Result<(), TournamentError> {
        let total_players = self.all_players.len() as u32;
        let payout_structure = calculate_dynamic_payout_structure(
            total_players,
            &self.tournament_type,
        );

        self.payout_structure = payout_structure;

        Ok(())
    }
}

/// Calculates the current blind level at a specific timestamp
///
/// # Arguments
/// * `timestamp` - The timestamp (in nanoseconds) to check the blind level for
///
/// # Returns
/// * `Option<BlindLevel>` - The blind level at that time, or None if before start or after all levels
pub fn get_blind_level_at_time(
    speed_type: NewTournamentSpeedType,
    timestamp: u64,
    tournament_start: u64,
    starting_chips: u64,
) -> Option<BlindLevel> {
    // Check if timestamp is before tournament start
    if timestamp < tournament_start {
        return None;
    }

    let params = match speed_type {
        NewTournamentSpeedType::Regular(levels) => SpeedType::new_regular(starting_chips, levels),
        NewTournamentSpeedType::Turbo(levels) => SpeedType::new_turbo(starting_chips, levels),
        NewTournamentSpeedType::HyperTurbo(levels) => {
            SpeedType::new_hyper_turbo(starting_chips, levels)
        }
        NewTournamentSpeedType::Custom(custom) => SpeedType::new_custom(
            starting_chips,
            custom.max_levels,
            custom.level_duration_ns,
            custom.blind_multiplier,
            custom.ante_start_level,
            custom.ante_percentage,
            custom.initial_blind_percentage,
        ),
    };
    let params = params.get_params();

    // Track the cumulative time to determine which level we're in
    let mut level_start_time: u64 = tournament_start;

    for level in &params.blind_levels {
        let level_end_time = level_start_time + level.duration_ns;
        // Check if the timestamp falls within this level's time window
        if timestamp >= level_start_time && timestamp < level_end_time {
            return Some(level.clone());
        }

        level_start_time = level_end_time;
    }

    // If elapsed time exceeds total duration, return the last level
    if !params.blind_levels.is_empty() && timestamp >= level_start_time {
        return Some(params.blind_levels.last().unwrap().clone());
    }

    None
}

#[cfg(test)]
mod tests {
    use table::poker::game::table_functions::ante::AnteType;

    use crate::tournaments::{
        blind_level::SpeedType,
        types::{get_blind_level_at_time, CustomTournamentSpeedType, NewTournamentSpeedType},
    };

    #[test]
    fn test_get_blind_level_at_time_before_start() {
        // Test when timestamp is before tournament start
        let timestamp = 1000;
        let tournament_start = 2000;
        let starting_chips = 10000;
        let speed_type = NewTournamentSpeedType::Regular(10);

        let result =
            get_blind_level_at_time(speed_type, timestamp, tournament_start, starting_chips);

        assert!(
            result.is_none(),
            "Should return None when timestamp is before tournament start"
        );
    }

    #[test]
    fn test_get_blind_level_at_time_regular_first_level() {
        // Test first blind level for Regular speed type
        let tournament_start = 1_000_000_000_000;
        let timestamp = tournament_start + 100; // Just after start
        let starting_chips = 10000;
        let speed_type = NewTournamentSpeedType::Regular(10);

        let result =
            get_blind_level_at_time(speed_type, timestamp, tournament_start, starting_chips);

        assert!(result.is_some(), "Should return a blind level");
        let level = result.unwrap();

        // For regular tournaments, initial small blind is 1% of starting stack
        let expected_small_blind = ((starting_chips as f64 * 0.01).round() / 2.0).round() as u64;
        let expected_big_blind = expected_small_blind * 2;

        assert_eq!(level.small_blind, expected_small_blind);
        assert_eq!(level.big_blind, expected_big_blind);
        assert!(
            matches!(level.ante_type, AnteType::None),
            "First level should have no ante"
        );
    }

    #[test]
    fn test_get_blind_level_at_time_regular_second_level() {
        // Test second blind level for Regular speed type
        let tournament_start = 1_000_000_000_000;
        let level_duration = 9e11 as u64; // 15 minutes
        let timestamp = tournament_start + level_duration + 100; // Just after first level
        let starting_chips = 10000;
        let speed_type = NewTournamentSpeedType::Regular(10);

        let result =
            get_blind_level_at_time(speed_type, timestamp, tournament_start, starting_chips);

        assert!(result.is_some(), "Should return a blind level");
        let level = result.unwrap();

        // Create a regular tournament to compare
        let regular = SpeedType::new_regular(starting_chips, 10);
        let params = match regular {
            SpeedType::Regular(params) => params,
            _ => panic!("Expected Regular variant"),
        };

        // Second level should match the second entry in the blind levels
        let expected_level = &params.blind_levels[1];

        assert_eq!(level.small_blind, expected_level.small_blind);
        assert_eq!(level.big_blind, expected_level.big_blind);
        assert_eq!(
            level.ante_type,
            expected_level.ante_type.clone(),
            "Should match ante type of second level"
        );
    }

    #[test]
    fn test_get_blind_level_at_time_turbo() {
        // Test blind level for Turbo speed type
        let tournament_start = 1_000_000_000_000;
        let level_duration = 6e11 as u64; // 10 minutes
        let timestamp = tournament_start + level_duration + 100; // Just after first level
        let starting_chips = 10000;
        let speed_type = NewTournamentSpeedType::Turbo(15);

        let result =
            get_blind_level_at_time(speed_type, timestamp, tournament_start, starting_chips);

        assert!(result.is_some(), "Should return a blind level");
        let level = result.unwrap();

        // Create a turbo tournament to compare
        let turbo = SpeedType::new_turbo(starting_chips, 15);
        let params = match turbo {
            SpeedType::Turbo(params) => params,
            _ => panic!("Expected Turbo variant"),
        };

        // Second level should match the second entry in the blind levels
        let expected_level = &params.blind_levels[1];

        assert_eq!(level.small_blind, expected_level.small_blind);
        assert_eq!(level.big_blind, expected_level.big_blind);
        assert_eq!(
            level.ante_type,
            expected_level.ante_type.clone(),
            "Should match ante type of second level for turbo"
        );
    }

    #[test]
    fn test_get_blind_level_at_time_hyper_turbo() {
        // Test blind level for HyperTurbo speed type
        let tournament_start = 1_000_000_000_000;
        let level_duration = 1.8e11 as u64; // 3 minutes
        let timestamp = tournament_start + level_duration * 2 + 100; // During third level
        let starting_chips = 10000;
        let speed_type = NewTournamentSpeedType::HyperTurbo(20);

        let result =
            get_blind_level_at_time(speed_type, timestamp, tournament_start, starting_chips);

        assert!(result.is_some(), "Should return a blind level");
        let level = result.unwrap();

        // Create a hyper turbo tournament to compare
        let hyper_turbo = SpeedType::new_hyper_turbo(starting_chips, 20);
        let params = match hyper_turbo {
            SpeedType::HyperTurbo(params) => params,
            _ => panic!("Expected HyperTurbo variant"),
        };

        // Third level should match the third entry in the blind levels
        let expected_level = &params.blind_levels[2];

        assert_eq!(level.small_blind, expected_level.small_blind);
        assert_eq!(level.big_blind, expected_level.big_blind);
        assert_eq!(
            level.ante_type,
            expected_level.ante_type.clone(),
            "Should match ante type of third level for hyper turbo"
        );
    }

    #[test]
    fn test_get_blind_level_at_time_custom() {
        // Test blind level for Custom speed type
        let tournament_start = 1_000_000_000_000;
        let level_duration = 3e11 as u64; // 5 minutes
        let timestamp = tournament_start + level_duration * 3 + 100; // During fourth level
        let starting_chips = 10000;

        let custom_params = CustomTournamentSpeedType {
            max_levels: 12,
            level_duration_ns: level_duration,
            blind_multiplier: 1.8,
            ante_start_level: 3,
            ante_percentage: 12,
            initial_blind_percentage: 3,
        };

        let speed_type = NewTournamentSpeedType::Custom(custom_params);

        let result = get_blind_level_at_time(
            speed_type.clone(),
            timestamp,
            tournament_start,
            starting_chips,
        );

        assert!(result.is_some(), "Should return a blind level");
        let level = result.unwrap();

        // Create a custom tournament to compare
        let custom = match &speed_type {
            NewTournamentSpeedType::Custom(params) => SpeedType::new_custom(
                starting_chips,
                params.max_levels,
                params.level_duration_ns,
                params.blind_multiplier,
                params.ante_start_level,
                params.ante_percentage,
                params.initial_blind_percentage,
            ),
            _ => panic!("Expected Custom variant"),
        };

        let params = match custom {
            SpeedType::Custom(params) => params,
            _ => panic!("Expected Custom variant"),
        };

        // Fourth level should match the fourth entry in the blind levels
        let expected_level = &params.blind_levels[3];

        assert_eq!(level.small_blind, expected_level.small_blind);
        assert_eq!(level.big_blind, expected_level.big_blind);
        assert_eq!(
            level.ante_type,
            expected_level.ante_type.clone(),
            "Should match ante type of fourth level for custom"
        );
    }

    #[test]
    fn test_get_blind_level_at_time_beyond_last_level() {
        // Test when timestamp is beyond all defined levels
        let tournament_start = 1_000_000_000_000;
        let starting_chips = 10000;
        let max_levels = 10;
        let speed_type = NewTournamentSpeedType::Regular(max_levels);

        // Create a regular tournament to get the total duration
        let regular = SpeedType::new_regular(starting_chips, max_levels);
        let params = match regular {
            SpeedType::Regular(params) => params,
            _ => panic!("Expected Regular variant"),
        };

        // Calculate total duration of all levels
        let total_duration: u64 = params
            .blind_levels
            .iter()
            .map(|level| level.duration_ns)
            .sum();

        // Set timestamp beyond all levels
        let timestamp = tournament_start + total_duration + 1_000_000_000;

        let result =
            get_blind_level_at_time(speed_type, timestamp, tournament_start, starting_chips);

        assert!(
            result.is_some(),
            "Should return the last blind level when beyond all levels"
        );
        let level = result.unwrap();

        // Should match the last blind level
        let expected_level = params.blind_levels.last().unwrap();

        assert_eq!(level.small_blind, expected_level.small_blind);
        assert_eq!(level.big_blind, expected_level.big_blind);
        assert_eq!(
            level.ante_type,
            expected_level.ante_type.clone(),
            "Should match ante type of last level"
        );
    }

    #[test]
    fn test_get_blind_level_at_time_exact_level_boundary() {
        // Test when timestamp is exactly at a level boundary
        let tournament_start = 1_000_000_000_000;
        let level_duration = 9e11 as u64; // 15 minutes for Regular
        let timestamp = tournament_start + level_duration; // Exactly at end of first level
        let starting_chips = 10000;
        let speed_type = NewTournamentSpeedType::Regular(10);

        let result =
            get_blind_level_at_time(speed_type, timestamp, tournament_start, starting_chips);

        assert!(result.is_some(), "Should return a blind level");
        let level = result.unwrap();

        // Create a regular tournament to compare
        let regular = SpeedType::new_regular(starting_chips, 10);
        let params = match regular {
            SpeedType::Regular(params) => params,
            _ => panic!("Expected Regular variant"),
        };

        // At exact boundary, should be in the second level
        let expected_level = &params.blind_levels[1];

        assert_eq!(level.small_blind, expected_level.small_blind);
        assert_eq!(level.big_blind, expected_level.big_blind);
    }
}
