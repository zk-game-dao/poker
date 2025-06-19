use candid::CandidType;
use serde::Deserialize;
use table::poker::game::{
    table_functions::{table::{TableConfig, TableId}, types::CurrencyType},
    types::GameType,
};

#[derive(Debug, Clone, PartialEq, CandidType, Deserialize)]
pub struct FilterOptions {
    // Inclusion filters
    game_type: Option<GameType>,
    timer_duration: Option<u16>,
    seats: Option<u8>,
    currency_type: Option<CurrencyType>,

    // Exclusion filters
    exclude_game_type: Option<GameType>,
    exclude_timer_duration: Option<u16>,
    exclude_seats: Option<u8>,
    exclude_currency_type: Option<CurrencyType>,
}

impl Default for FilterOptions {
    fn default() -> Self {
        Self::new(None, None, None, None, None, None, None, None)
    }
}

impl FilterOptions {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        game_type: Option<GameType>,
        timer_duration: Option<u16>,
        seats: Option<u8>,
        currency_type: Option<CurrencyType>,
        exclude_game_type: Option<GameType>,
        exclude_timer_duration: Option<u16>,
        exclude_seats: Option<u8>,
        exclude_currency_type: Option<CurrencyType>,
    ) -> Self {
        FilterOptions {
            game_type,
            timer_duration,
            seats,
            currency_type,
            exclude_game_type,
            exclude_timer_duration,
            exclude_seats,
            exclude_currency_type,
        }
    }

    // Helper to create a filter with only inclusion options
    pub fn with_inclusions(
        game_type: Option<GameType>,
        timer_duration: Option<u16>,
        seats: Option<u8>,
        currency_type: Option<CurrencyType>,
    ) -> Self {
        Self::new(
            game_type,
            timer_duration,
            seats,
            currency_type,
            None,
            None,
            None,
            None,
        )
    }

    // Helper to create a filter with only exclusion options
    pub fn with_exclusions(
        exclude_game_type: Option<GameType>,
        exclude_timer_duration: Option<u16>,
        exclude_seats: Option<u8>,
        exclude_currency_type: Option<CurrencyType>,
    ) -> Self {
        Self::new(
            None,
            None,
            None,
            None,
            exclude_game_type,
            exclude_timer_duration,
            exclude_seats,
            exclude_currency_type,
        )
    }

    pub fn filter_tables(
        &self,
        tables: Vec<(TableId, TableConfig)>,
    ) -> Vec<(TableId, TableConfig)> {
        tables
            .into_iter()
            .filter(|(_, table_config)| {
                // Check inclusion filters
                let mut result = true;
                if let Some(game_type) = &self.game_type {
                    result = result && table_config.game_type == *game_type;
                }
                if let Some(timer_duration) = &self.timer_duration {
                    result = result && table_config.timer_duration == *timer_duration;
                }
                if let Some(seats) = &self.seats {
                    result = result && table_config.seats == *seats;
                }
                if let Some(currency_type) = &self.currency_type {
                    result = result && table_config.currency_type == *currency_type;
                }

                // Check exclusion filters
                if let Some(exclude_game_type) = &self.exclude_game_type {
                    result = result && table_config.game_type != *exclude_game_type;
                }
                if let Some(exclude_timer_duration) = &self.exclude_timer_duration {
                    result = result && table_config.timer_duration != *exclude_timer_duration;
                }
                if let Some(exclude_seats) = &self.exclude_seats {
                    result = result && table_config.seats != *exclude_seats;
                }
                if let Some(exclude_currency_type) = &self.exclude_currency_type {
                    result = result && table_config.currency_type != *exclude_currency_type;
                }

                result
            })
            .collect()
    }
}
