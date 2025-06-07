use std::collections::HashMap;

use candid::{CandidType, Principal};
use errors::game_error::GameError;
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
use errors::tournament_error::TournamentError;
use errors::trace_err;
use errors::traced_error::TracedError;
use ic_cdk_timers::TimerId;
use serde::{Deserialize, Serialize};
use user::user::User;

use crate::poker::core::{Card, FlatDeck};
use crate::poker::game::types::TableStatus;
use crate::poker::game::users::Users;

use crate::poker::game::types::{GameType, QueueItem, UserCards};
use crate::table_canister::add_experience_points_wrapper;

use super::action_log::{ActionLog, ActionType};
use super::ante::AnteType;
use super::rake::Rake;
use super::side_pot::SidePot;
use super::types::{
    BetType, CurrencyType, DealStage, Notifications, PlayerAction, SeatStatus, UserTableData,
};

#[derive(Debug, Clone, Serialize, CandidType, Deserialize, PartialEq, Eq)]
pub struct TableConfig {
    pub name: String,
    pub game_type: GameType,
    pub seats: u8,
    pub timer_duration: u16,
    pub color: u64,
    pub card_color: u64,
    pub environment_color: u64,
    pub auto_start_timer: u16,
    pub max_inactive_turns: u16,
    pub currency_type: CurrencyType,
    pub enable_rake: Option<bool>,
    pub max_seated_out_turns: Option<u16>,
    pub is_private: Option<bool>,
    pub ante_type: Option<AnteType>,
    pub table_type: Option<TableType>,
    pub is_shared_rake: Option<(Principal, String)>,
    pub require_proof_of_humanity: Option<bool>,
    pub is_paused: Option<bool>,
}

#[derive(Debug, Clone, Serialize, CandidType, Deserialize, PartialEq, Eq)]
pub enum TableType {
    Cash, // Regular cash game table
    Tournament {
        tournament_id: Principal,
        is_final_table: bool,
    },
    SitAndGo,  // Single table tournament that starts when full
    Satellite, // Tournament table where winners get entry to larger tournament
}

#[derive(Debug, Clone)]
pub struct Table {
    pub id: Principal,
    pub config: TableConfig,
    pub seats: Vec<SeatStatus>,
    pub community_cards: Vec<Card>,
    pub deck: FlatDeck,
    pub pot: u64,
    pub side_pots: Vec<SidePot>,
    pub status: TableStatus,
    pub deal_stage: DealStage,
    pub big_blind: u64,
    pub small_blind: u64,
    pub big_blind_user_principal: Principal,
    pub small_blind_user_principal: Principal,
    pub dealer_position: usize,
    pub current_player_index: usize,
    pub winners: Option<Vec<User>>,
    pub sorted_users: Option<Vec<UserCards>>,
    pub action_logs: Vec<ActionLog>,
    pub user_table_data: HashMap<Principal, UserTableData>,
    pub highest_bet: u64,
    pub highest_bet_in_pot: u64,
    pub last_raise: u64,
    pub last_raise_principal: Principal,
    pub is_side_pot_active: bool,
    pub round_ticker: u64,
    pub last_timer_started_timestamp: u64,
    pub users: Users,
    pub timer: Option<TimerId>,
    pub notifications: Notifications,
    pub queue: Vec<QueueItem>,
    pub rake_config: Option<Rake>,
    pub rake_total: Option<u64>,
}

impl Default for TableConfig {
    fn default() -> Self {
        TableConfig {
            name: "".to_string(),
            game_type: GameType::NoLimit(0),
            seats: 0,
            timer_duration: 0,
            color: 0,
            card_color: 0,
            environment_color: 0,
            auto_start_timer: 0,
            max_inactive_turns: 0,
            currency_type: CurrencyType::Real(currency::Currency::ICP),
            enable_rake: None,
            max_seated_out_turns: None,
            is_private: None,
            ante_type: None,
            table_type: None,
            is_shared_rake: None,
            require_proof_of_humanity: None,
            is_paused: None,
        }
    }
}

impl TableConfig {
    pub fn default_spin_and_go(small_blind: u64, id: Principal) -> Self {
        Self {
            name: "Spin & Go Table".to_string(),
            game_type: GameType::NoLimit(small_blind),
            seats: 3,                          // Spin and Go tournaments are 3-handed
            timer_duration: 15,                // 15 seconds per action
            color: 0,                          // Default table color
            card_color: 0,                     // Default card color
            environment_color: 0,              // Default environment
            auto_start_timer: 1,               // Auto-start timer enabled
            max_inactive_turns: 3,             // Maximum inactive turns before folding
            currency_type: CurrencyType::Fake, // Default to ICP
            enable_rake: Some(false),          // No rake in tournament play
            max_seated_out_turns: Some(3),     // Sit out for 3 hands maximum
            is_private: Some(false),           // Not a private table
            ante_type: Some(AnteType::None),   // Start with no ante
            table_type: Some(TableType::Tournament {
                tournament_id: id,
                is_final_table: true,
            }), // Tournament table
            is_shared_rake: None,              // No shared rake
            require_proof_of_humanity: Some(false), // No proof of humanity required
            is_paused: Some(false),            // Not paused initially
        }
    }
}

impl Default for Table {
    fn default() -> Self {
        Table {
            id: Principal::anonymous(),
            config: TableConfig::default(),
            seats: Vec::new(),
            community_cards: Vec::new(),
            deck: FlatDeck::new(vec![1, 2, 3, 4, 5, 6, 7, 8]),
            pot: 0,
            side_pots: Vec::new(),
            status: TableStatus::Open,
            deal_stage: DealStage::Fresh,
            big_blind: 0,
            small_blind: 0,
            big_blind_user_principal: Principal::anonymous(),
            small_blind_user_principal: Principal::anonymous(),
            dealer_position: 0,
            current_player_index: 0,
            winners: None,
            sorted_users: None,
            action_logs: Vec::new(),
            user_table_data: HashMap::new(),
            highest_bet: 0,
            highest_bet_in_pot: 0,
            last_raise: 0,
            last_raise_principal: Principal::anonymous(),
            is_side_pot_active: false,
            round_ticker: 0,
            last_timer_started_timestamp: 0,
            users: Users::default(),
            timer: None,
            notifications: Notifications::new(),
            queue: Vec::new(),
            rake_config: None,
            rake_total: None,
        }
    }
}

impl TableConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        game_type: GameType,
        seats: u8,
        timer_duration: u16,
        color: u64,
        card_color: u64,
        environment_color: u64,
        auto_start_timer: u16,
        max_inactive_turns: u16,
        currency_type: CurrencyType,
        enable_rake: Option<bool>,
        max_seated_out_turns: Option<u16>,
        is_private: Option<bool>,
        ante_type: Option<AnteType>,
        table_type: Option<TableType>,
        is_shared_rake: Option<(Principal, String)>,
        require_proof_of_humanity: Option<bool>,
        is_paused: Option<bool>,
    ) -> TableConfig {
        TableConfig {
            name,
            game_type,
            seats,
            timer_duration,
            color,
            card_color,
            environment_color,
            auto_start_timer,
            max_inactive_turns,
            currency_type,
            enable_rake,
            max_seated_out_turns,
            is_private,
            ante_type,
            table_type,
            is_shared_rake,
            require_proof_of_humanity,
            is_paused,
        }
    }
}

impl Table {
    pub fn new(id: Principal, config: TableConfig, bytes: Vec<u8>) -> Table {
        let deck = FlatDeck::new(bytes);

        let (small_blind, big_blind) = match config.game_type {
            GameType::NoLimit(small_blind) => (small_blind, small_blind * 2),
            GameType::SpreadLimit(min, _) => (min / 2, min),
            GameType::FixedLimit(small, _) => (small / 2, small),
            GameType::PotLimit(small) => (small, small * 2),
        };
        let rake = if let CurrencyType::Real(currency) = &config.currency_type {
            Rake::new(small_blind, &config.game_type, currency).ok()
        } else {
            None
        };

        let seats = vec![SeatStatus::Empty; config.seats as usize];

        Table {
            id,
            config,
            seats: seats.clone(),
            community_cards: Vec::new(),
            deck,
            pot: 0,
            side_pots: Vec::new(),
            status: TableStatus::Open,
            deal_stage: DealStage::Fresh,
            big_blind,
            small_blind,
            big_blind_user_principal: Principal::anonymous(),
            small_blind_user_principal: Principal::anonymous(),
            dealer_position: 0,
            current_player_index: 1,
            winners: None,
            sorted_users: None,
            action_logs: Vec::new(),
            user_table_data: HashMap::new(),
            highest_bet: 0,
            highest_bet_in_pot: 0,
            last_raise: 0,
            last_raise_principal: Principal::anonymous(),
            is_side_pot_active: false,
            round_ticker: 0,
            last_timer_started_timestamp: 0,
            users: Users::default(),
            timer: None,
            notifications: Notifications::new(),
            queue: Vec::new(),
            rake_config: rake,
            rake_total: Some(0),
        }
    }

    /// Set the user by `user_principal` to be all in
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user going all in
    /// - `amount` - The amount to go all in with
    /// - `is_raise` - A boolean indicating if the all in is a raise
    ///
    /// # Errors
    ///
    /// - [`GameError::PlayerNotFound`] if the player is not found
    /// - [`GameError::Other`] if the user table data cannot be retrieved
    pub fn all_in(
        &mut self,
        user_principal: Principal,
        amount: u64,
        is_raise: bool,
    ) -> Result<(), TracedError<GameError>> {
        let user = self
            .users
            .get(&user_principal)
            .ok_or_else(|| trace_err!(TracedError::new(GameError::PlayerNotFound)))?
            .clone();

        let mut action_type = ActionType::AllIn {
            amount: self.highest_bet,
        };

        if user.balance.checked_sub(amount).is_none() {
            action_type = ActionType::AllIn {
                amount: user.balance,
            };
            self.update_user_balances(user.balance, user_principal, PlayerAction::AllIn)
                .map_err(|e| trace_err!(e, "Failed to update user balance"))?;
        } else {
            self.update_user_balances(amount, user_principal, PlayerAction::AllIn)
                .map_err(|e| trace_err!(e, "Failed to update user balance"))?;
        }

        if !is_raise {
            self.log_action(Some(user_principal), action_type);
        } else {
            self.check_and_reset_players_called()
                .map_err(|e| trace_err!(e, "Failed to check and reset players called."))?;
        }

        Ok(())
    }

    /// Match the current bet or wager made by a previous player.
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user calling
    ///
    /// # Errors
    ///
    /// - [`GameError::PlayerNotFound`] if the player is not found
    /// - [`GameError::Other`] if the user table data cannot be retrieved
    pub fn call(&mut self, user_principal: Principal) -> Result<(), TracedError<GameError>> {
        // First verify the user is actually in an occupied seat
        let _ = self
            .seats
            .iter()
            .position(|status| matches!(status, SeatStatus::Occupied(p) if *p == user_principal))
            .ok_or_else(|| trace_err!(TracedError::new(GameError::PlayerNotFound)))?;

        // Get the user data
        let user = self
            .users
            .get(&user_principal)
            .ok_or_else(|| trace_err!(TracedError::new(GameError::PlayerNotFound)))?
            .clone();
        let user_table_data = self.get_user_table_data(user_principal).map_err(|e| {
            trace_err!(
                e,
                "Failed to get user table data to calculate amount to call."
            )
        })?;

        let amount_to_call = self.highest_bet - user_table_data.current_total_bet;

        // User is all in if the amount to call is equal or higher than the user's balance
        if user.balance.saturating_sub(amount_to_call) == 0 {
            self.all_in(user_principal, amount_to_call, false)
                .map_err(|e| trace_err!(e, "All in function failed in call."))?;
        } else {
            self.update_user_balances(amount_to_call, user_principal, PlayerAction::Called)
                .map_err(|e| trace_err!(e, "Update user balances failed in call."))?;
            self.log_action(Some(user_principal), ActionType::Call);
        }

        if self
            .is_user_all_in_spread_or_fixed_limit(user_principal)
            .map_err(|e| trace_err!(e, "is_user_all_in_spread_or_fixed_limit failed in call."))?
        {
            let user_table_data = self
                .get_user_table_data_mut(user_principal)
                .map_err(|e| trace_err!(e, "Failed to get user table data in call."))?;
            user_table_data.player_action = PlayerAction::AllIn;
        }

        self.next_player()
            .map_err(|e| trace_err!(e, "Next player failed in call."))?;
        self.check_next_turn_or_showdown()
            .map_err(|e| trace_err!(e, "check_next_turn_or_showdown failed in call."))?;

        Ok(())
    }

    /// Raise the current bet
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user raising
    /// - `bet_type` - The type of bet
    /// - `amount` - The amount to raise
    /// - `raw_amount` - The raw amount to raise
    ///
    /// # Errors
    ///
    /// - [`GameError::ActionNotAllowed`] if the raise amount is zero.
    /// - [`GameError::PlayerNotFound`] if the player is not found.
    pub fn raise(
        &mut self,
        user_principal: Principal,
        bet_type: BetType,
        amount: u64,
        raw_amount: u64,
    ) -> Result<(), TracedError<GameError>> {
        let mut is_user_all_in = false;
        let amount = match bet_type {
            BetType::Raised(_) => amount,
            BetType::BigBlind => self.big_blind,
            BetType::SmallBlind => self.small_blind,
            BetType::Ante(amount) => amount,
            _ => 0,
        };

        if amount == 0 {
            return Err(trace_err!(TracedError::new(GameError::ActionNotAllowed {
                reason: "Raise amount cannot be negative or zero".to_string(),
            })));
        }

        // First verify the user is actually in an occupied seat
        let _ = self
            .seats
            .iter()
            .position(|status| matches!(status, SeatStatus::Occupied(p) if *p == user_principal))
            .ok_or_else(|| trace_err!(TracedError::new(GameError::PlayerNotFound)))?;

        // Get the user data
        let user = self
            .users
            .get(&user_principal)
            .ok_or_else(|| trace_err!(TracedError::new(GameError::PlayerNotFound)))?
            .clone();

        if user.balance.saturating_sub(raw_amount) == 0 {
            self.all_in(user_principal, amount, true)
                .map_err(|e| trace_err!(e, "All in failed in raise."))?;
            is_user_all_in = true;
        } else {
            let player_action = match bet_type {
                BetType::BigBlind => PlayerAction::Raised(amount),
                BetType::SmallBlind => PlayerAction::Raised(amount),
                BetType::Raised(_) => PlayerAction::Raised(amount),
                _ => unreachable!(),
            };

            self.update_user_balances(amount, user_principal, player_action)
                .map_err(|e| trace_err!(e, "Update user balances in raise failed."))?;

            if self
                .get_user_balance(user_principal)
                .map_err(|e| trace_err!(e, "Get user balance failed in raise."))?
                == 0
            {
                self.set_player_action(user_principal, PlayerAction::AllIn)
                    .map_err(|e| {
                        trace_err!(e, "Failed to set player action to all in in raise.")
                    })?;
                self.log_action(Some(user_principal), ActionType::AllIn { amount });
                is_user_all_in = true;
            }
        }

        self.last_raise = match bet_type {
            BetType::Raised(_) => amount.saturating_sub(self.last_raise),
            BetType::BigBlind => self.big_blind,
            BetType::SmallBlind => self.small_blind,
            _ => 0,
        };
        self.last_raise_principal = user_principal;
        self.last_raise = amount;

        self.update_highest_bet(user_principal)
            .map_err(|e| trace_err!(e, "Failed to update highest bet."))?;

        self.check_and_reset_players_called()
            .map_err(|e| trace_err!(e, "Failed to check and reset players called."))?;

        if !is_user_all_in {
            self.next_player()
                .map_err(|e| trace_err!(e, "Next player failed in raise."))?;
            match bet_type {
                BetType::Raised(_) => self.log_action(
                    Some(user_principal),
                    ActionType::Raise { amount: raw_amount },
                ),
                BetType::BigBlind => self.log_action(Some(user_principal), ActionType::BigBlind),
                BetType::SmallBlind => {
                    self.log_action(Some(user_principal), ActionType::SmallBlind)
                }
                _ => {}
            }
        } else {
            self.log_action(
                Some(user_principal),
                ActionType::AllIn { amount: raw_amount },
            );

            self.next_player()
                .map_err(|e| trace_err!(e, "Next player failed in raise."))?;

            if bet_type != BetType::BigBlind && bet_type != BetType::SmallBlind {
                self.check_next_turn_or_showdown()
                    .map_err(|e| trace_err!(e, "check_next_turn_or_showdown failed in raise."))?;
            }
        }
        Ok(())
    }

    /// Check and reset players who have called if the call amount is below the highest bet
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the user table data cannot be retrieved
    /// - [`GameError::PlayerNotFound`] if the player is not found
    /// - [`GameError::InsufficientFunds`] if the player has insufficient funds
    pub fn check_and_reset_players_called(&mut self) -> Result<(), TracedError<GameError>> {
        let user_principals: Vec<Principal> = self
            .seats
            .iter()
            .filter_map(|status| match status {
                SeatStatus::Occupied(principal) => Some(*principal),
                _ => None,
            })
            .collect();
        let highest_bet = self.highest_bet;

        for user_principal in user_principals {
            let user_table_data = self.get_user_table_data_mut(user_principal).map_err(|e| {
                trace_err!(
                    e,
                    "Failed to get user table data in check_and_reset_players_called."
                )
            })?;

            if user_table_data.player_action == PlayerAction::AllIn {
                continue;
            }

            if (user_table_data.player_action == PlayerAction::Called
                || user_table_data.player_action == PlayerAction::Checked)
                && user_table_data.current_total_bet < highest_bet
            {
                self.set_player_action(user_principal, PlayerAction::None).map_err(|e| trace_err!(e, "Failed to set player action to None in check_and_reset_players_called."))?;
            }
        }
        Ok(())
    }

    /// Method to start the betting round
    ///
    /// # Errors
    ///
    /// - [`GameError::ActionNotAllowed`] if there are not enough players to start a betting round
    /// - [`GameError::Other`] if the user table data cannot be retrieved
    #[allow(clippy::type_complexity)]
    pub fn start_betting_round(
        &mut self,
        bytes: Vec<u8>,
    ) -> Result<(Vec<(Principal, u64)>, Vec<(Principal, u64)>), TracedError<GameError>> {
        #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
        {
            // ic_cdk::println!("Clearing turn timer in start betting round.");
            self.clear_turn_timer();
        }
        self.activate_queued_players()
            .map_err(|e| trace_err!(e, "Failed to activate queued players"))?;
        self.handle_queue_items()
            .map_err(|e| trace_err!(e, "Failed to handle queue items"))?;

        if self.config.is_paused.unwrap_or(false) {
            return Err(trace_err!(TracedError::new(GameError::ActionNotAllowed {
                reason: "Game is paused".to_string(),
            })));
        }

        let mut kicked_players = Vec::new();
        let mut seated_out_kicked_players = Vec::new();

        for user_principal in self.seats.clone().into_iter() {
            if let SeatStatus::Occupied(user_principal) = user_principal {
                let is_kicked = self
                    .check_and_kick_user_for_insufficient_funds(user_principal, self.big_blind)
                    .map_err(|e| trace_err!(e, "Failed to kick user for insufficient funds"))?;

                if let Some(balance) = is_kicked {
                    self.handle_kicked_player(&mut kicked_players, user_principal, balance);
                }

                let is_kicked = self
                    .check_if_seated_out_for_too_long(user_principal)
                    .map_err(|e| trace_err!(e, "Failed to check if seated out too long"))?;

                if let Some(balance) = is_kicked {
                    self.handle_kicked_player(
                        &mut seated_out_kicked_players,
                        user_principal,
                        balance,
                    );
                }
            }
        }

        for user_principal in self.seats.clone().into_iter() {
            if let SeatStatus::Occupied(user_principal) = user_principal {
                // #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
                {
                    let experience_points =
                        if let Ok(table_data) = self.get_user_table_data(user_principal) {
                            table_data.experience_points
                        } else {
                            continue;
                        };
                    
                    let users_canister_id = match self.users.get(&user_principal) {
                        Some(user) => user.users_canister_id,
                        None => continue,
                    };

                    match self.config.currency_type {
                        CurrencyType::Fake => {}
                        CurrencyType::Real(currency) => {
                            ic_cdk::futures::spawn(async move {
                                match add_experience_points_wrapper(users_canister_id, user_principal, experience_points, currency.to_string())
                                    .await
                                {
                                    Ok(res) => res,
                                    Err(_err) => {}
                                }
                            });
                        }
                    };
                }
                self.clear_user_table_data(user_principal)
                    .map_err(|e| trace_err!(e, "Failed to clear user table data"))?;
            }
        }

        if self
            .get_playing_users()
            .map_err(|e| trace_err!(e, "Failed to get playing users"))?
            < 2
        {
            if self.config.table_type.is_some() && self.config.table_type != Some(TableType::Cash) {
                match self.seats[self.dealer_position] {
                    SeatStatus::Occupied(_) => {}
                    _ => {
                        self.rotate_dealer()
                            .map_err(|e| trace_err!(e, "Failed to rotate dealer"))?;
                    }
                };
                let small_blind_user_principal = self
                    .get_small_blind_user_principal()
                    .map_err(|e| trace_err!(e, "Failed to get small blind principal"))?;
                let big_blind_user_principal = self
                    .get_big_blind_user_principal()
                    .map_err(|e| trace_err!(e, "Failed to get big blind principal"))?;
                self.handle_blind_sitting_out(small_blind_user_principal, BetType::SmallBlind)
                    .map_err(|e| trace_err!(e, "Failed to handle small blind sitting out"))?;
                self.handle_blind_sitting_out(big_blind_user_principal, BetType::BigBlind)
                    .map_err(|e| trace_err!(e, "Failed to handle big blind sitting out"))?;

                self.calculate_seated_out_pots()
                    .map_err(|e| trace_err!(e, "Failed to calculate seated out pots"))?;
                self.showdown()
                    .map_err(|e| trace_err!(e, "Failed to show down"))?;
                return Ok((kicked_players, seated_out_kicked_players));
            }
            return Err(trace_err!(TracedError::new(GameError::ActionNotAllowed {
                reason: "Not enough players to start a betting round".to_string(),
            })));
        }

        self.round_ticker += 1;
        #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
        if self.round_ticker % 10 == 0 {
            let rake_total = self.rake_total.unwrap_or(0);
            self.rake_total = Some(0);
            let fee = if self.config.currency_type == CurrencyType::Real(currency::Currency::BTC) {
                10
            } else {
                ic_ledger_types::DEFAULT_FEE.e8s()
            };
            if rake_total > fee {
                let id = self.id;
                ic_cdk::futures::spawn(async move {
                    match ic_cdk::call(id, "withdraw_rake", (rake_total,)).await {
                        Ok(res) => res,
                        Err(_err) => {}
                    }
                });
            }
        }

        match self.seats[self.dealer_position] {
            SeatStatus::Occupied(_) => {}
            _ => {
                self.rotate_dealer()
                    .map_err(|e| trace_err!(e, "Failed to rotate dealer"))?;
            }
        };

        self.pot = 0;
        self.side_pots.clear();
        self.is_side_pot_active = false;
        self.winners = None;
        self.sorted_users = None;
        self.community_cards.clear();
        self.deck = FlatDeck::new(bytes);

        self.action_logs.clear();
        self.highest_bet = 0;
        self.last_raise = 0;
        let small_blind_user_principal = self
            .get_small_blind_user_principal()
            .map_err(|e| trace_err!(e, "Failed to get small blind principal"))?;
        let big_blind_user_principal = self
            .get_big_blind_user_principal()
            .map_err(|e| trace_err!(e, "Failed to get big blind principal"))?;

        self.handle_blind_sitting_out(small_blind_user_principal, BetType::SmallBlind)
            .map_err(|e| trace_err!(e, "Failed to handle small blind sitting out"))?;
        self.handle_blind_sitting_out(big_blind_user_principal, BetType::BigBlind)
            .map_err(|e| trace_err!(e, "Failed to handle big blind sitting out"))?;
        self.big_blind_user_principal = big_blind_user_principal;
        self.small_blind_user_principal = small_blind_user_principal;
        if let Some(ante_type) = self.config.ante_type.clone() {
            self.handle_ante(ante_type)?;
        }

        self.current_player_index = self
            .get_starting_player_index()
            .map_err(|e| trace_err!(e, "Failed to get starting player index"))?;
        match &self.seats[self.current_player_index] {
            SeatStatus::Occupied(principal) => {
                // Check if the player is sitting out
                if self
                    .get_user_table_data(*principal)
                    .map_err(|e| {
                        trace_err!(
                            e,
                            "Failed to set player action to sitting out in start betting round."
                        )
                    })?
                    .player_action
                    == PlayerAction::SittingOut
                {
                    self.next_player()
                        .map_err(|e| trace_err!(e, "Next player failed in start betting round."))?;
                }
            }
            // All other seat statuses should trigger next player
            _ => self
                .next_player()
                .map_err(|e| trace_err!(e, "Next player failed in start betting round."))?,
        }

        self.deal_stage = DealStage::Opening;

        self.deal_cards(false)
            .map_err(|e| trace_err!(e, "Failed to deal cards in start betting round."))?;

        Ok((kicked_players, seated_out_kicked_players))
    }

    fn handle_ante(&mut self, ante_type: AnteType) -> Result<(), TracedError<GameError>> {
        match ante_type {
            AnteType::PercentageOfBigBlind(_) => {
                let ante_amount = self.get_ante_amount();
                let users = self
                    .seats
                    .iter()
                    .filter_map(|status| match status {
                        SeatStatus::Occupied(principal) => Some(*principal),
                        _ => None,
                    })
                    .collect::<Vec<Principal>>();
                for user_principal in users {
                    self.bet(user_principal, BetType::Ante(ante_amount))?;
                }
            }
            AnteType::BigBlindAnte => {
                let ante_amount = self.get_ante_amount();
                let dealer = self.get_player_at_seat(self.dealer_position)?;
                self.bet(dealer, BetType::Ante(ante_amount))?;
            }
            AnteType::Fixed(amount) => {
                let users = self
                    .seats
                    .iter()
                    .filter_map(|status| match status {
                        SeatStatus::Occupied(principal) => Some(*principal),
                        _ => None,
                    })
                    .collect::<Vec<Principal>>();
                for user_principal in users {
                    self.bet(user_principal, BetType::Ante(amount))?;
                }
            }
            AnteType::None => {}
        }
        Ok(())
    }

    /// Handle the user's blind bet. If the player is sitting out, the blind should still be placed.
    fn handle_blind_sitting_out(
        &mut self,
        blind_uid: Principal,
        bet_type: BetType,
    ) -> Result<(), TracedError<GameError>> {
        if self
            .get_user_table_data(blind_uid)
            .map_err(|e| {
                trace_err!(
                    e,
                    "Failed to get user table data to check if user is sitting out."
                )
            })?
            .player_action
            == PlayerAction::SittingOut
        {
            self.bet(blind_uid, bet_type)
                .map_err(|e| trace_err!(e, "Failed to bet in handle blind sitting out."))?;
            self.get_user_table_data_mut(blind_uid)
                .map_err(|e| {
                    trace_err!(
                        e,
                        "Failed to reset player action to sitting out in handle blind."
                    )
                })?
                .player_action = PlayerAction::SittingOut;
        } else {
            self.bet(blind_uid, bet_type)
                .map_err(|e| trace_err!(e, "Failed to bet in handle blind sitting out."))?;
        }
        Ok(())
    }

    fn check_if_seated_out_for_too_long(
        &mut self,
        user_principal: Principal,
    ) -> Result<Option<u64>, TracedError<GameError>> {
        if let Ok(user_table_data) = self.get_user_table_data(user_principal) {
            if user_table_data.player_action == PlayerAction::SittingOut {
                if let Some(max_seated_out_turns) = self.config.max_seated_out_turns {
                    if let Some(TableType::Cash) = self.config.table_type {
                        // In cash games, we kick the player after max_seated_out_turns
                        if user_table_data.seated_out_turns >= max_seated_out_turns {
                            let balance = self.kick_user(
                                user_principal,
                                "Seated out for too long.".to_string(),
                            )?;
                            return Ok(Some(balance));
                        } else if let Ok(user_table_data) =
                            self.get_user_table_data_mut(user_principal)
                        {
                            user_table_data.seated_out_turns += 1;
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    fn handle_kicked_player(
        &mut self,
        kicked_players: &mut Vec<(Principal, u64)>,
        user_principal: Principal,
        balance: u64,
    ) {
        kicked_players.push((user_principal, balance));

        #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
        {
            let id = self.id;
            if let Some(TableType::Tournament { tournament_id, .. }) =
                self.config.table_type.clone()
            {
                ic_cdk::futures::spawn(async move {
                    ic_cdk::println!("Removing from tournament: {:?}", tournament_id.to_text());
                    let res: Result<(Result<(), TournamentError>,), _> =
                        ic_cdk::call(tournament_id, "handle_user_losing", (user_principal, id))
                            .await;
                    match res {
                        Ok((Err(err),)) => {
                            ic_cdk::println!(
                                "Failed to handle user losing: {:?}\nRetrying...",
                                err
                            );
                            let res: Result<(Result<(), TournamentError>,), _> = ic_cdk::call(
                                tournament_id,
                                "handle_user_losing",
                                (user_principal, id),
                            )
                            .await;
                            match res {
                                Ok((Err(err),)) => {
                                    ic_cdk::println!("Failed to handle user losing: {:?}", err);
                                }
                                Err(err) => {
                                    ic_cdk::println!(
                                        "Failed to handle user losing (Inter canister call): {:?}",
                                        err
                                    );
                                }
                                _ => {}
                            }
                        }
                        Err(err) => {
                            ic_cdk::println!(
                                "Failed to handle user losing (Inter canister call): {:?}\nRetrying...",
                                err
                            );
                            let res: Result<(Result<(), TournamentError>,), _> = ic_cdk::call(
                                tournament_id,
                                "handle_user_losing",
                                (user_principal, id),
                            )
                            .await;
                            match res {
                                Ok((Err(err),)) => {
                                    ic_cdk::println!("Failed to handle user losing: {:?}", err);
                                }
                                Err(err) => {
                                    ic_cdk::println!(
                                        "Failed to handle user losing (Inter canister call): {:?}",
                                        err
                                    );
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                });
            }
        }
    }
}
