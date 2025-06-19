use errors::{game_error::GameError, trace_err, traced_error::TracedError};
use user::user::WalletPrincipalId;

use super::{
    action_log::ActionType,
    table::Table,
    types::{PlayerAction, SeatStatus},
};

impl Table {
    /// Abandoning the current hand and forfeit any tokens put into the pot
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user who is folding
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the user table data cannot be retrieved
    /// - [`GameError::PlayerNotFound`] if retrieving a player fails
    pub fn user_fold(
        &mut self,
        user_principal: WalletPrincipalId,
        is_inactive_user: bool,
    ) -> Result<(), TracedError<GameError>> {
        if !is_inactive_user {
            #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
            {
                ic_cdk::println!("Clearing turn timer in user fold function for user {:?} with inactive user : {}", user_principal.0.to_text(), is_inactive_user);
                self.clear_turn_timer();
            }
        }

        let user_table_data = self.get_user_table_data_mut(user_principal).map_err(|e| {
            trace_err!(
                e,
                "Failed to get user table data to check if user is folded in user_fold."
            )
        })?;
        if user_table_data.player_action == PlayerAction::Folded {
            return Ok(());
        }
        user_table_data.inactive_turns = 0;
        user_table_data.player_action = PlayerAction::Folded;
        let current_total_bet = user_table_data.current_total_bet;
        user_table_data.current_total_bet = 0;
        if !self.is_side_pot_active {
            self.add_to_pot(current_total_bet);
        } else {
            self.side_pots
                .last_mut()
                .ok_or_else(|| {
                    trace_err!(TracedError::new(GameError::Other(
                        "Could not get side pot".to_string()
                    )))
                })?
                .add_to_side_pot(current_total_bet, user_principal);
        }
        self.log_action(Some(user_principal), ActionType::Fold);

        self.next_player()
            .map_err(|e| trace_err!(e, "Failed to get next player in user fold."))?;
        // If all players have folded, move to the showdown stage
        self.check_next_turn_or_showdown()
            .map_err(|e| trace_err!(e, "Failed to check_next_turn_or_showdown in user fold."))?;
        Ok(())
    }

    /// Abandoning the current hand and forfeit any tokens put into the pot before its the users turn
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user who is folding
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the user table data cannot be retrieved
    /// - [`GameError::PlayerNotFound`] if retrieving a player fails
    pub fn user_pre_fold(
        &mut self,
        user_principal: WalletPrincipalId,
    ) -> Result<(), TracedError<GameError>> {
        let user_table_data = self.get_user_table_data_mut(user_principal).map_err(|e| {
            trace_err!(
                e,
                "Failed to get user table data to set player action in user pre fold."
            )
        })?;
        user_table_data.player_action = PlayerAction::Folded;
        let current_total_bet = user_table_data.current_total_bet;
        user_table_data.inactive_turns = 0;
        user_table_data.current_total_bet = 0;
        if !self.is_side_pot_active {
            self.add_to_pot(current_total_bet);
        } else {
            self.side_pots
                .last_mut()
                .ok_or_else(|| {
                    trace_err!(TracedError::new(GameError::Other(
                        "Could not get side pot".to_string()
                    )))
                })?
                .add_to_side_pot(current_total_bet, user_principal);
        }

        self.log_action(Some(user_principal), ActionType::Fold);
        self.check_next_turn_or_showdown().map_err(|e| {
            trace_err!(e, "Failed to check_next_turn_or_showdown in user pre fold.")
        })?;
        Ok(())
    }

    /// Folds the user by `user_principal`s hand and checks if
    /// all players have folded to move to the showdown stage
    ///
    /// Gets called when the user's turn timer runs out
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user who is folding
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the user table data cannot be retrieved
    /// - [`GameError::PlayerNotFound`] if retrieving a player fails
    pub fn force_fold(&mut self, user_principal: WalletPrincipalId) -> Result<(), TracedError<GameError>> {
        if self.sorted_users.is_none() {
            if self
                .handle_inactive_user(user_principal)
                .map_err(|e| trace_err!(e, "Failed to handle inactive user in force fold."))?
            {
                return Ok(());
            } else {
                let user_table_data = self
                    .get_user_table_data_mut(user_principal)
                    .map_err(|e| trace_err!(e, "Failed get user table data in force fold."))?;
                user_table_data.player_action = PlayerAction::Folded;
                let current_total_bet = user_table_data.current_total_bet;
                user_table_data.current_total_bet = 0;
                if !self.is_side_pot_active {
                    self.add_to_pot(current_total_bet);
                } else {
                    self.side_pots
                        .last_mut()
                        .ok_or_else(|| {
                            trace_err!(TracedError::new(GameError::Other(
                                "Could not get side pot".to_string()
                            )))
                        })?
                        .add_to_side_pot(current_total_bet, user_principal);
                }

                self.log_action(Some(user_principal), ActionType::Fold);

                self.next_player()
                    .map_err(|e| trace_err!(e, "Failed to get next player in force fold."))?;
                // If all players have folded, move to the showdown stage
                self.check_next_turn_or_showdown().map_err(|e| {
                    trace_err!(e, "Failed to check_next_turn_or_showdown in force fold.")
                })?;
            }
        }
        Ok(())
    }

    /// Have all players folded?
    pub fn all_players_folded(&mut self) -> bool {
        // TODO: Refactor: this could be more idiomatic, like using filter
        let mut n = self.number_of_players();
        for user_principal in self.seats.iter() {
            if let SeatStatus::Occupied(user_principal) = user_principal {
                if let Some(user_table_data) = self.user_table_data.get_mut(user_principal) {
                    if user_table_data.player_action == PlayerAction::Folded
                        || user_table_data.player_action == PlayerAction::SittingOut
                        || user_table_data.player_action == PlayerAction::Joining
                    {
                        n -= 1;
                    }
                }
            }
        }
        n == 1
    }
}
