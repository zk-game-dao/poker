use candid::Principal;
use errors::{game_error::GameError, trace_err, traced_error::TracedError};

use super::{action_log::ActionType, table::Table, types::PlayerAction};

impl Table {
    /// Remains in the hand without betting
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user who is checking
    /// - `is_auto_checked` - Whether the check was automatically performed
    ///
    /// # Errors
    ///
    /// - [`GameError::ActionNotAllowed`] if the user cannot check with an active raise in progress
    /// - [`GameError::InsufficientFunds`] if the user has insufficient funds to check
    pub fn user_check(
        &mut self,
        user_principal: Principal,
        is_auto_checked: bool,
    ) -> Result<(), TracedError<GameError>> {
        if is_auto_checked {
            if self.handle_inactive_user(user_principal).map_err(|e| {
                trace_err!(e, "Failed to handle inactive user in user check function.")
            })? {
                return Ok(());
            }
        } else {
            #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
            {
                ic_cdk::println!("Clearing turn timer in user check function for user {:?} with auto check: {:?}", user_principal.to_text(), is_auto_checked);
                self.clear_turn_timer();
            }
            self.get_user_table_data_mut(user_principal).map_err(|e| trace_err!(e, "Failed to get user table data to set inactive turns to 0 in user check function."))?.inactive_turns = 0;
        }

        if let Some(user) = self.users.get_mut(&user_principal) {
            if user.balance > 0 {
                if self.get_users_current_total_bet(user_principal).map_err(|e| trace_err!(e, "Failed to get users current total bet to check if its lower than highest bet in user check function."))? < self.highest_bet {
                    return Err(trace_err!(TracedError::new(GameError::ActionNotAllowed {
                        reason: "User cannot check with an active raise in progress".to_string(),
                    })));
                } else {
                    let user_table_data = self.get_user_table_data_mut(user_principal).map_err(|e| trace_err!(e, "Failed to get user table data to set user action to Checked in user check function."))?;
                    user_table_data.player_action = PlayerAction::Checked;
                }
                self.log_action(Some(user_principal), ActionType::Check);

                self.next_player()
                    .map_err(|e| trace_err!(e, "Failed to set next player in user check."))?;
                self.check_next_turn_or_showdown().map_err(|e| {
                    trace_err!(e, "Failed to check_next_turn_or_showdown in user check.")
                })?;
                Ok(())
            } else {
                Err(trace_err!(TracedError::new(GameError::InsufficientFunds)))
            }
        } else {
            Err(trace_err!(TracedError::new(GameError::PlayerNotFound)))
        }
    }
}
