use std::time::Duration;

use candid::Principal;
use errors::game_error::GameError;
use errors::trace_err;
use errors::traced_error::TracedError;

use crate::poker::game::types::{GameType, QueueItem};
use crate::table_canister::{deposit_to_table, get_table_wrapper, join_table, resume_table_wrapper, start_new_betting_round_wrapper};

use super::action_log::ActionType;
use super::types::{CurrencyType, PlayerAction, SeatStatus, UserTableData};
use super::{table::Table, types::DealStage};

impl Table {
    /// Is the table full?
    pub fn is_full(&self) -> bool {
        self.number_of_players() == self.config.seats as usize
    }

    /// Switches the dealer position to the next player.
    ///
    /// If the current dealer is the last player,
    /// the dealer position is switched to the first player.
    pub fn rotate_dealer(&mut self) -> Result<(), TracedError<GameError>> {
        let mut dealer_position = self.dealer_position;
        let mut counter = 0;
        while counter < self.seats.len() {
            // Changed from addition to subtraction for clockwise rotation
            dealer_position = (dealer_position + 1) % self.seats.len();
            if let SeatStatus::Occupied(_user_principal) = self.seats[dealer_position] {
                self.dealer_position = dealer_position;
                return Ok(());
            }
            counter += 1;
        }
        Err(trace_err!(TracedError::new(GameError::Other(
            "Could not calculate current player index".to_string(),
        ))))
    }

    /// Returns the principal of the player who is the small blind.
    pub fn get_small_blind_user_principal(&self) -> Result<Principal, TracedError<GameError>> {
        let mut index = (self.dealer_position + 1) % (self.config.seats as usize);

        for _ in 0..self.config.seats {
            if let SeatStatus::Occupied(user_principal) = self.seats[index] {
                if self.user_table_data.contains_key(&user_principal) {
                    return Ok(user_principal);
                }
            }
            index = (index + 1) % self.seats.len();
        }
        Err(trace_err!(TracedError::new(GameError::Other(
            "Small blind not found".to_string()
        ))))
    }

    /// Returns the principal of the player who is the big blind.
    pub fn get_big_blind_user_principal(&self) -> Result<Principal, TracedError<GameError>> {
        let mut index = (self.dealer_position + 1) % self.seats.len();

        let mut found_small_blind = false;
        for _ in 0..self.config.seats * 2 {
            if let SeatStatus::Occupied(user_principal) = self.seats[index] {
                if found_small_blind && self.user_table_data.contains_key(&user_principal) {
                    return Ok(user_principal);
                }
                found_small_blind = true;
            }
            index = (index + 1) % self.seats.len();
        }
        Err(trace_err!(TracedError::new(GameError::Other(
            "Big blind not found".to_string()
        ))))
    }

    /// Sets the deal stage.
    ///
    /// # Parameters
    ///
    /// - `deal_stage` - The deal stage to set
    pub fn set_deal_stage(&mut self, deal_stage: DealStage) {
        self.deal_stage = deal_stage;
    }

    /// Switches the turn to the next active player
    /// and resets the turn timer.
    ///
    /// If the current player is the last player,
    /// the turn is switched to the first active player.
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the user table data cannot be retrieved
    pub fn next_player(&mut self) -> Result<(), TracedError<GameError>> {
        let active_players = self.number_of_active_players();
        let mut iterations = 0;

        #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
        let previous_player = if self.deal_stage != DealStage::Fresh {
            if let Ok(previous_player) = self.get_player_at_seat(self.current_player_index) {
                previous_player
            } else {
                Principal::anonymous()
            }
        } else {
            Principal::anonymous()
        };

        // Set the next player to the next active player
        if active_players > 0 {
            loop {
                self.current_player_index = self.calculate_current_player_index().map_err(|e| {
                    trace_err!(e, "Failed to calculate current player index in loop.")
                })?;

                if let SeatStatus::Occupied(user) = self.seats[self.current_player_index] {
                    match self.user_table_data.get_mut(&user) {
                        Some(table_data) => {
                            if table_data.player_action != PlayerAction::Folded
                                && table_data.player_action != PlayerAction::AllIn
                                && table_data.player_action != PlayerAction::SittingOut
                                && !table_data.auto_check_fold
                            {
                                break;
                            } else if table_data.auto_check_fold {
                                self.check_fold(user)?;
                            }
                        }
                        None => {
                            return Err(trace_err!(TracedError::new(GameError::Other(
                                "Could not get users table data".to_string(),
                            ))));
                        }
                    }
                } else {
                    return Err(trace_err!(TracedError::new(GameError::Other(
                        "Could not get user: calculate_current_player_index failed".to_string(),
                    ))));
                }

                iterations += 1;
                if iterations > self.seats.len() {
                    break;
                }
            }
        }

        #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
        {
            let current_player = self
                .get_player_at_seat(self.current_player_index)
                .map_err(|e| trace_err!(e, "Failed to get current player after loop."))?;

            if previous_player == current_player {
                return Ok(());
            }
            self.set_last_timer_started_timestamp(ic_cdk::api::time());
            self.start_turn_timer(current_player, self.config.timer_duration.into());
            self.notifications
                .clear_notifications_older_than(ic_cdk::api::time() - 6e11 as u64); // 10 minutes

            self.notifications.add_notification(
                current_player,
                crate::poker::game::table_functions::types::NotificationMessage::UserTurnStarted,
            );
        }

        Ok(())
    }

    /// Calculate the current player index
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the current player index cannot be calculated
    pub fn calculate_current_player_index(&self) -> Result<usize, TracedError<GameError>> {
        let mut current_player_index = self.current_player_index;
        let mut counter = 0;
        while counter < self.seats.len() * 3 {
            current_player_index = (current_player_index + 1) % self.seats.len();

            if let SeatStatus::Occupied(_user_principal) = self.seats[current_player_index] {
                return Ok(current_player_index);
            }
            counter += 1;
        }
        Err(trace_err!(TracedError::new(GameError::Other(
            "Could not calculate current player index".to_string(),
        ))))
    }

    /// Gets the starting player index.
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the starting player index cannot be calculated
    pub fn get_starting_player_index(&mut self) -> Result<usize, TracedError<GameError>> {
        let starting_player_index = self
            .calculate_starting_player_index()
            .map_err(|e| trace_err!(e, "Failed to calculate starting player index."))?;

        #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
        self.start_turn_timer(
            self.get_player_at_seat(starting_player_index)
                .map_err(|e| trace_err!(e, "Failed to get player at seat to start turn timer."))?,
            self.config.timer_duration.into(),
        );
        Ok(starting_player_index)
    }

    /// Calculate starting player index
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the starting player index cannot be calculated
    pub fn calculate_starting_player_index(&mut self) -> Result<usize, TracedError<GameError>> {
        let num_players = self
            .seats
            .iter()
            .filter(|&seat| matches!(seat, SeatStatus::Occupied(_)))
            .count();

        if num_players < 2 {
            return Err(trace_err!(TracedError::new(GameError::Other(
                "Not enough players to start the game".to_string(),
            ))));
        }

        if num_players == 2 {
            // Heads-up play: find the small blind
            return self.calculate_small_blind_user_index();
        } else if num_players == 3 {
            return Ok(self.dealer_position);
        } else {
            let mut counter = 2;
            let mut starting_player_index = self.dealer_position;

            for _ in 0..self.seats.len() {
                // Changed from addition to subtraction for clockwise rotation
                starting_player_index = (starting_player_index + 1) % self.seats.len();

                // Check if seat is occupied
                if matches!(self.seats[starting_player_index], SeatStatus::Occupied(_)) {
                    if counter > 0 {
                        counter -= 1;
                    } else {
                        let user = match self.seats[starting_player_index] {
                            SeatStatus::Occupied(user) => user,
                            _ => {
                                return Err(trace_err!(TracedError::new(GameError::Other(
                                    "Could not get user: calculate_starting_player_index failed"
                                        .to_string(),
                                ))));
                            }
                        };
                        let user_table_data = self.get_user_table_data(user)?;
                        if user_table_data.auto_check_fold {
                            self.check_fold(user)?;
                            counter += 1;
                        } else {
                            return Ok(starting_player_index);
                        }
                    }
                }
            }
        }

        Err(trace_err!(TracedError::new(GameError::Other(
            "Could not calculate starting player index".to_string(),
        ))))
    }

    /// Calculate small blind user index
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the small blind user index cannot be calculated
    pub fn calculate_small_blind_user_index(&self) -> Result<usize, TracedError<GameError>> {
        let small_blind_uid = self.get_small_blind_user_principal().map_err(|e| {
            trace_err!(
                e,
                "Failed to get small blind user principal to calculate the small blind index."
            )
        })?;
        for (i, user_principal) in self.seats.iter().enumerate() {
            if let SeatStatus::Occupied(user_principal) = user_principal {
                if user_principal == &small_blind_uid {
                    return Ok(i);
                }
            }
        }
        Err(trace_err!(TracedError::new(GameError::Other(
            "Small blind not found".to_string()
        ))))
    }

    /// Transfers every side pot to the user who contributed first.
    ///
    /// # Errors
    ///
    /// - [`GameError::PlayerNotFound`] if the user is not found.
    pub fn clean_up_side_pots(&mut self) -> Result<(), TracedError<GameError>> {
        // TODO: Refactor: instead of using a separate vector, use retain
        let mut side_pots_to_delete = Vec::new();
        for (i, side_pot) in self.side_pots.iter().enumerate() {
            if side_pot.user_principals.len() == 1 {
                let user = self
                    .users
                    .get_mut(&side_pot.user_principals[0])
                    .ok_or_else(|| trace_err!(TracedError::new(GameError::PlayerNotFound), ""))?;
                user.balance += side_pot.pot;
                side_pots_to_delete.push(i);
            }
        }

        for i in side_pots_to_delete.iter().rev() {
            self.side_pots.remove(*i);
        }

        Ok(())
    }

    pub fn set_user_auto_check_fold(
        &mut self,
        user: Principal,
        enabled: bool,
    ) -> Result<(), TracedError<GameError>> {
        self.get_user_table_data_mut(user)
            .map_err(|e| trace_err!(e, "Failed to get user table data to set auto check fold."))?
            .auto_check_fold = enabled;

        Ok(())
    }

    pub fn is_everyone_auto_check_fold(&self, current_user_principal: Principal) -> bool {
        for user_principal in self.seats.iter() {
            if let SeatStatus::Occupied(user_principal) = user_principal {
                if current_user_principal == *user_principal {
                    continue;
                }
                if let Some(table_data) = self.user_table_data.get(user_principal) {
                    if table_data.player_action == PlayerAction::Folded
                        || table_data.player_action == PlayerAction::SittingOut
                    {
                        continue;
                    }
                    if !table_data.auto_check_fold {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Checks if the user can check or fold.
    pub fn check_fold(&mut self, user: Principal) -> Result<(), TracedError<GameError>> {
        let can_check = self.get_users_current_total_bet(user)? >= self.highest_bet;

        if can_check {
            self.user_check(user, false)
        } else {
            self.user_fold(user, false)
        }
    }

    /// Prepares the users for the next [DealStage]
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the user table data cannot be retrieved
    pub fn prepare_user_actions(
        &mut self,
        is_cycling_to_showdown: bool,
    ) -> Result<(), TracedError<GameError>> {
        let exp_points = self.calculate_experience_points();
        for user_principal in self.seats.iter() {
            if let SeatStatus::Occupied(user_principal) = user_principal {
                match self.user_table_data.get_mut(user_principal) {
                    Some(table_data) => {
                        table_data.total_bet += table_data.current_total_bet;
                        table_data.current_total_bet = 0;
                        if table_data.player_action != PlayerAction::Folded
                            && table_data.player_action != PlayerAction::AllIn
                            && table_data.player_action != PlayerAction::SittingOut
                        {
                            table_data.player_action = PlayerAction::None;
                        }

                        if table_data.player_action == PlayerAction::SittingOut {
                            table_data.seated_out_turns += 1;
                        } else {
                            table_data.seated_out_turns = 0;
                        }
                        if !is_cycling_to_showdown
                            && !self.config.is_private.unwrap_or(true)
                            && table_data.player_action != PlayerAction::SittingOut
                            && self.config.currency_type != CurrencyType::Fake
                        {
                            table_data.experience_points += exp_points;
                        }
                    }
                    None => {
                        return Err(trace_err!(TracedError::new(GameError::Other(
                            "Could not get users table data".to_string(),
                        ))));
                    }
                }
            }
        }
        Ok(())
    }

    /// Calculate the amount of experience points to give to the user.
    /// If its 2 players its 50%, 3-5 players its 75% and everything above its 100%.
    fn calculate_experience_points(&self) -> u64 {
        let num_players = self.number_of_players();
        let base_xp = match num_players {
            2 => 12,
            3..=5 => 17,
            _ => 25,
        };

        let pot_multiplier = match self.pot {
            0..=100_000 => 0.5,              // 0.001 ICP
            100_001..=1_000_000 => 1.0,      // 0.01 ICP
            1_000_001..=10_000_000 => 1.5,   // 0.1 ICP
            10_000_001..=100_000_000 => 2.0, // 1 ICP
            _ => 3.0,                        // >1 ICP
        };

        (base_xp as f64 * pot_multiplier) as u64
    }

    /// Checks if the users current total bet is equal to the highest bet.
    pub fn is_users_current_total_bet_equal_to_highest_bet(
        &self,
        user_principal: Principal,
    ) -> bool {
        let user_table_data = self.user_table_data.get(&user_principal);
        if let Some(user_table_data) = user_table_data {
            user_table_data.current_total_bet == self.highest_bet
        } else {
            false
        }
    }

    /// Are the current total bets of all users equal?
    pub fn are_users_current_total_bets_equal(&self) -> bool {
        let mut current_total_bets = Vec::new();
        for user_principal in self.seats.iter() {
            if let SeatStatus::Occupied(user_principal) = user_principal {
                if let Some(user_table_data) = self.user_table_data.get(user_principal) {
                    if (user_table_data.player_action == PlayerAction::Folded
                        || user_table_data.player_action == PlayerAction::SittingOut)
                        || (user_table_data.player_action == PlayerAction::AllIn
                            && user_table_data.current_total_bet <= self.highest_bet)
                    {
                        continue;
                    }
                    current_total_bets.push(user_table_data.current_total_bet);
                }
            }
        }
        current_total_bets
            .iter()
            .all(|&x| x == current_total_bets[0])
    }

    /// Updates the highest bet in the pot
    /// if the user by `user_principal`s has a higher bet than the current highest bet.
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user who may have the highest bet
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the user is not found.
    pub fn update_highest_bet(
        &mut self,
        user_principal: Principal,
    ) -> Result<(), TracedError<GameError>> {
        let user_table_data = self
            .get_user_table_data(user_principal)
            .map_err(|e| trace_err!(e, "Could not get user table data to update highest bet."))?;
        if user_table_data.current_total_bet > self.highest_bet {
            self.highest_bet = user_table_data.current_total_bet;
        }
        Ok(())
    }

    /// Does the user by `user_principal`
    /// not have enough balance to call?
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user who may not have enough balance
    ///
    /// # Errors
    ///
    /// - [`GameError::PlayerNotFound`] if the user is not found.
    pub fn is_user_all_in_spread_or_fixed_limit(
        &self,
        user_principal: Principal,
    ) -> Result<bool, TracedError<GameError>> {
        match self.config.game_type {
            GameType::SpreadLimit(min, _) => {
                let user = self
                    .users
                    .get(&user_principal)
                    .ok_or_else(|| trace_err!(TracedError::new(GameError::PlayerNotFound), ""))?;
                Ok(user.balance < min)
            }
            GameType::FixedLimit(min, _) => {
                let user = self
                    .users
                    .get(&user_principal)
                    .ok_or_else(|| trace_err!(TracedError::new(GameError::PlayerNotFound), ""))?;
                Ok(user.balance < min)
            }
            _ => Ok(false),
        }
    }

    /// Sets the last timer started timestamp.
    ///
    /// # Parameters
    ///
    /// - `timestamp` - The timestamp to set
    pub fn set_last_timer_started_timestamp(&mut self, timestamp: u64) {
        self.last_timer_started_timestamp = timestamp;
    }

    /// Is the game ongoing? (i.e. not in the opening or fresh stage)
    pub fn is_game_ongoing(&self) -> bool {
        self.deal_stage != DealStage::Opening
            && self.deal_stage != DealStage::Fresh
            && self.sorted_users.is_none()
    }

    /// Returns the number of players on the table.
    pub fn number_of_players(&self) -> usize {
        self.seats
            .iter()
            .filter(|&seat_status| {
                matches!(
                    seat_status,
                    SeatStatus::Occupied(_) | SeatStatus::QueuedForNextRound(_, _, _)
                )
            })
            .count()
    }

    /// Returns the number of active players (i.e. not folded or all-in)
    pub fn number_of_active_players(&self) -> usize {
        self.seats
            .iter()
            .filter(|&user_principal| {
                if let SeatStatus::Occupied(user) = user_principal {
                    if let Some(table_data) = self.user_table_data.get(user) {
                        return table_data.player_action != PlayerAction::Folded
                            && table_data.player_action != PlayerAction::AllIn;
                    }
                }
                false
            })
            .count()
    }

    /// Set dealer position.
    pub fn set_dealer_position(&mut self) -> Result<(), TracedError<GameError>> {
        self.dealer_position = 0;
        let mut counter = 0;
        while counter < self.seats.len() {
            if let SeatStatus::Occupied(_user_principal) = self.seats[self.dealer_position] {
                return Ok(());
            }
            self.dealer_position = (self.dealer_position + 1) % self.seats.len();
            counter += 1;
        }
        Err(trace_err!(TracedError::new(GameError::Other(
            "Could not calculate dealer position".to_string(),
        ))))
    }

    /// Gets users in the game.
    pub fn get_playing_users(&self) -> Result<u8, TracedError<GameError>> {
        let mut total_users = 0;
        for user in self.seats.iter() {
            if let SeatStatus::Occupied(user) = user {
                let user_table_data = self
                    .user_table_data
                    .get(user)
                    .ok_or_else(|| trace_err!(TracedError::new(GameError::PlayerNotFound), ""))?;
                if user_table_data.player_action != PlayerAction::SittingOut
                    && user_table_data.player_action != PlayerAction::Joining
                {
                    total_users += 1;
                }
            }
        }
        Ok(total_users)
    }

    /// Activates any players who were queued for the next round
    pub fn activate_queued_players(&mut self) -> Result<(), TracedError<GameError>> {
        // First collect all the changes we need to make
        let mut changes = Vec::new();

        let seats = self.seats.clone();
        for (i, seat) in seats.iter().enumerate() {
            if let SeatStatus::QueuedForNextRound(principal, user, sitting_out) = seat {
                changes.push((i, principal, user.clone(), sitting_out));
            }
        }

        // Now apply all the changes
        for (i, principal, user, sitting_out) in changes {
            // Update seat status
            self.seats[i] = SeatStatus::Occupied(*principal);

            // Initialize user data
            self.user_table_data
                .insert(user.principal_id, UserTableData::new());
            if *sitting_out {
                self.get_user_table_data_mut(user.principal_id)
                    .map_err(|e| trace_err!(e, "Error getting user table data to initialise it."))?
                    .player_action = PlayerAction::SittingOut;
            } else {
                self.get_user_table_data_mut(user.principal_id)
                    .map_err(|e| trace_err!(e, "Error getting user table data to initialise it."))?
                    .player_action = PlayerAction::Joining;
            }

            // Add user and log action
            self.users.add_user(*user).map_err(|e| {
                trace_err!(e, "Error adding user to users in activate queued players.")
            })?;

            // Update player action from Joining to None
            if let Some(table_data) = self.user_table_data.get_mut(principal) {
                if table_data.player_action == PlayerAction::Joining {
                    table_data.player_action = PlayerAction::None;
                }
            }
        }

        Ok(())
    }

    /// Gets the highest current total bet.
    pub fn get_highest_current_total_bet(&self) -> Result<u64, TracedError<GameError>> {
        let highest_bet = self
            .seats
            .iter()
            .filter_map(|seat_status| match seat_status {
                SeatStatus::Occupied(principal) => {
                    // Only check user data for occupied seats
                    if let Some(user_data) = self.user_table_data.get(principal) {
                        // Only consider active players
                        if !matches!(
                            user_data.player_action,
                            PlayerAction::Folded | PlayerAction::SittingOut | PlayerAction::Joining
                        ) {
                            Some(user_data.current_total_bet)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .max()
            .unwrap_or(0);

        Ok(highest_bet)
    }

    /// Cycles through the deal stages to the showdown
    pub fn cycle_to_showdown(&mut self) -> Result<(), TracedError<GameError>> {
        while self.deal_stage != DealStage::Showdown {
            self.deal_cards(true)
                .map_err(|e| trace_err!(e, "Error dealing cards in cycle to showdown."))?;
        }
        self.showdown().map_err(|e| trace_err!(e, ""))?;
        Ok(())
    }

    /// Adds an item to the action queue.
    pub fn append_to_queue(&mut self, item: QueueItem) {
        self.queue.push(item);
    }

    /// Handles the items in the queue.
    #[allow(dependency_on_unit_never_type_fallback)]
    pub fn handle_queue_items(&mut self) -> Result<(), TracedError<GameError>> {
        while let Some(item) = self.queue.pop() {
            match item {
                QueueItem::SittingIn(user_principal, is_game_paused) => {
                    if !self.user_table_data.contains_key(&user_principal) {
                        ic_cdk::println!("Warning: User data not found for {}", user_principal);
                        continue;
                    }
                    self.set_player_action(user_principal, PlayerAction::None)
                        .map_err(|e| trace_err!(e, ""))?;
                    if is_game_paused {
                        let table_principal = self.id;
                        ic_cdk::futures::spawn(async move {
                            match start_new_betting_round_wrapper(table_principal).await
                            {
                                Ok(res) => res,
                                Err(_err) => {}
                            }
                        })
                    }
                }
                QueueItem::Deposit(user_id, users_canister_id, amount) => {
                    if !self.user_table_data.contains_key(&user_id) {
                        ic_cdk::println!("Warning: User data not found for {}", user_id);
                        continue;
                    }
                    let table_principal = self.id;
                    ic_cdk::futures::spawn(async move {
                        match deposit_to_table(table_principal, users_canister_id, user_id, amount, true).await {
                            Ok(res) => {
                                ic_cdk::println!("Deposit successful: {:?}", res);
                            },
                            Err(err) => {
                                ic_cdk::println!("Error depositing to table: {:?}", err);
                            }
                        }
                    })
                }
                QueueItem::RemoveUser(user_principal, action_type) => {
                    // Find and clear the user's seat
                    if let Some(player_seat_index) = self.seats.iter().position(|status| {
                        matches!(status,
                            SeatStatus::Occupied(p) |
                            SeatStatus::QueuedForNextRound(p, _, _) |
                            SeatStatus::Reserved { principal: p, .. }
                            if *p == user_principal
                        )
                    }) {
                        // User found - clear their seat
                        self.seats[player_seat_index] = SeatStatus::Empty;
                    }
                    // Clean up user data
                    self.log_action(Some(user_principal), action_type);
                    self.user_table_data.remove(&user_principal);
                    self.users.remove_user(user_principal);
                }
                QueueItem::LeaveTableToMove(users_canister_id, user_id, to_table) => {
                    // Find and clear the user's seat
                    if let Some(player_seat_index) = self.seats.iter().position(|status| {
                        matches!(status,
                            SeatStatus::Occupied(p) |
                            SeatStatus::QueuedForNextRound(p, _, _) |
                            SeatStatus::Reserved { principal: p, .. }
                            if *p == user_id
                        )
                    }) {
                        // User found - clear their seat
                        self.seats[player_seat_index] = SeatStatus::Empty;
                    }
                    // Clean up user data
                    self.log_action(Some(user_id), ActionType::Leave);

                    let balance = self
                        .users
                        .get(&user_id)
                        .ok_or(trace_err!(TracedError::new(GameError::PlayerNotFound), ""))?
                        .balance;
                    self.user_table_data.remove(&user_id);
                    self.users.remove_user(user_id);
                    ic_cdk::futures::spawn(async move {
                        let table = get_table_wrapper(to_table).await;
                        match table {
                            Ok(_) => {
                                // TODO: Very inefficient loop. We need a way to better ensure user gets placed at the table.
                                //       This is a temporary fix as when over 3 people get placed at the same table the call
                                //       will fail as it seems to get the same free seat index in the join_table function in
                                //       the table canister.
                                for i in 0..5 {
                                    let res = join_table(
                                        to_table,
                                        users_canister_id,
                                        user_id,
                                        None,
                                        balance,
                                        false,
                                    )
                                    .await;
                                    if let Err(err) = res {
                                        ic_cdk::println!("Error joining table: {:?}", err);
                                        ic_cdk::println!("Retrying to join table ({})", i);
                                    } else {
                                        ic_cdk::println!("Successfully joined table");
                                        break;
                                    }
                                }
                            }
                            Err(err) => {
                                ic_cdk::println!("Error getting table: {:?}", err);
                            }
                        }
                    });
                }
                QueueItem::SittingOut(user_principal) => {
                    if !self.user_table_data.contains_key(&user_principal) {
                        ic_cdk::println!("Warning: User data not found for {}", user_principal);
                        continue;
                    }
                    self.set_player_action(user_principal, PlayerAction::SittingOut)
                        .map_err(|e| trace_err!(e, ""))?;
                }
                QueueItem::UpdateBlinds(small_blind, big_blind, ante) => {
                    self.small_blind = small_blind;
                    self.big_blind = big_blind;
                    self.config.ante_type = ante;
                }
                QueueItem::PauseTable => self.config.is_paused = Some(true),
                QueueItem::PauseTableForAddon(duration) => {
                    self.config.is_paused = Some(true);
                    let table_id = self.id;
                    let duration = Duration::from_nanos(duration);
                    ic_cdk::println!(
                        "Pausing table for addon period of {} seconds",
                        duration.as_secs()
                    );
                    let _ = ic_cdk_timers::set_timer(duration, move || {
                        ic_cdk::futures::spawn(async move {
                            ic_cdk::println!(
                                "Resuming table after addon period of {} seconds",
                                duration.as_secs()
                            );
                            match resume_table_wrapper(table_id).await {
                                Ok(_) => {
                                    ic_cdk::println!("Table resumed");
                                }
                                Err(err) => {
                                    ic_cdk::println!(
                                        "Error resuming table: {:?}\nRetrying...",
                                        err
                                    );
                                    match resume_table_wrapper(table_id).await {
                                        Ok(_) => {
                                            ic_cdk::println!("Table resumed");
                                        }
                                        Err(err) => {
                                            ic_cdk::println!("Error resuming table: {:?}", err);
                                        }
                                    }
                                }
                            }
                        });
                    });
                }
            }
        }
        Ok(())
    }

    // Removes all the cards from the user table data apart from the user specified by the principal.
    pub fn hide_cards(&mut self, user_id: Principal) -> Result<(), TracedError<GameError>> {
        for (user_principal, table_data) in self.user_table_data.iter_mut() {
            let user = self
                .users
                .get(user_principal)
                .ok_or_else(|| trace_err!(TracedError::new(GameError::PlayerNotFound), ""))?;
            if user.principal_id != user_id {
                table_data.cards.clear();
            }
        }
        Ok(())
    }

    /// Handles when a user is inactive for a turn.
    pub fn handle_inactive_user(
        &mut self,
        user_principal: Principal,
    ) -> Result<bool, TracedError<GameError>> {
        let user_table_data = self
            .get_user_table_data_mut(user_principal)
            .map_err(|e| trace_err!(e, ""))?;
        user_table_data.inactive_turns += 1;

        if user_table_data.inactive_turns >= self.config.max_inactive_turns {
            self.user_sitting_out(user_principal, true)
                .map_err(|e| trace_err!(e, ""))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn get_player_at_seat(&self, index: usize) -> Result<Principal, TracedError<GameError>> {
        match self.seats.get(index) {
            Some(SeatStatus::Occupied(principal)) => Ok(*principal),
            _ => Err(trace_err!(TracedError::new(GameError::PlayerNotFound))),
        }
    }

    pub fn get_seat_index(&self, user_principal: Principal) -> Option<u8> {
        self.seats.iter().position(|seat| matches!(seat, SeatStatus::Occupied(principal) if principal == &user_principal)).map(|i| i as u8)
    }

    pub fn is_players_turn(&self, user_principal: Principal) -> bool {
        if let SeatStatus::Occupied(principal) = self.seats[self.current_player_index] {
            principal == user_principal
        } else {
            false
        }
    }

    pub fn get_free_seat_index(&self) -> Option<u8> {
        let index = self
            .seats
            .iter()
            .position(|seat| matches!(seat, SeatStatus::Empty));
        index.map(|i| i as u8)
    }
}
