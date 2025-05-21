use candid::Principal;
use errors::{game_error::GameError, trace_err, traced_error::TracedError};

use super::{
    table::Table,
    types::{PlayerAction, SeatStatus},
};

impl Table {
    /// Is at least one player all in?
    pub fn is_player_all_in(&self) -> bool {
        for user_principal in self.seats.iter() {
            if let SeatStatus::Occupied(user_principal) = user_principal {
                if let Some(user_table_data) = self.user_table_data.get(user_principal) {
                    if user_table_data.player_action == PlayerAction::AllIn {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Gets the principal of the player who is all in
    ///
    /// # Returns
    ///
    /// Some(Principal) if a player is all in, None otherwise
    fn get_all_in_player(&self) -> Option<Principal> {
        for user_principal in self.seats.iter() {
            if let SeatStatus::Occupied(user_principal) = user_principal {
                if let Some(user_table_data) = self.user_table_data.get(user_principal) {
                    if user_table_data.player_action == PlayerAction::AllIn {
                        return Some(*user_principal);
                    }
                }
            }
        }
        None
    }

    /// Is the side pot filled with all the 'all in' players' bets?
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the user table data cannot be retrieved
    pub fn is_all_in_pot_full(&self) -> Result<bool, TracedError<GameError>> {
        if let Some(uid) = self.get_all_in_player() {
            let total_bet = {
                let user_table_data = self.get_user_table_data(uid).map_err(|e| {
                    trace_err!(e, "Failed to get user table data for is_all_in_pot_full")
                })?;
                user_table_data.total_bet
            };

            if self.pot / self.number_of_players() as u64 == total_bet {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
