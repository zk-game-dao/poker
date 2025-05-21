use candid::Principal;
use errors::{game_error::GameError, trace_err, traced_error::TracedError};

use crate::poker::game::types::QueueItem;

use super::{table::Table, types::PlayerAction};

impl Table {
    /// Remains in the hand without betting
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user who is checking
    ///
    /// # Errors
    ///
    /// - [`GameError::ActionNotAllowed`] if the user cannot check with an active raise in progress
    /// - [`GameError::InsufficientFunds`] if the user has insufficient funds to check
    pub fn user_sitting_out(
        &mut self,
        user_principal: Principal,
        is_inactive_user: bool,
    ) -> Result<(), TracedError<GameError>> {
        self.users
            .get(&user_principal)
            .ok_or_else(|| trace_err!(TracedError::new(GameError::PlayerNotFound)))?;
        if self.is_game_ongoing() {
            if self.is_players_turn(user_principal) {
                self.user_fold(user_principal, is_inactive_user)
                    .map_err(|e| trace_err!(e, "Failed to user fold in user sitting out."))?;
            } else {
                self.user_pre_fold(user_principal)
                    .map_err(|e| trace_err!(e, "Failed to user pre fold in user sitting out."))?;
            }
            self.queue.push(QueueItem::SittingOut(user_principal));
        } else {
            self.set_player_action(user_principal, PlayerAction::SittingOut)
                .map_err(|e| trace_err!(e, "Failed to set player action in user sitting out."))?;
        }
        Ok(())
    }
}
