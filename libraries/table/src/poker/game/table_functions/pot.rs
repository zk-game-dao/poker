use candid::Principal;
use errors::{game_error::GameError, trace_err, traced_error::TracedError};

use super::{action_log::ActionType, side_pot::SidePot, table::Table, types::PlayerAction};

impl Table {
    /// Adds the given `amount` to the pot
    pub fn add_to_pot(&mut self, amount: u64) {
        self.pot += amount;
    }

    /// Removes the given `amount` from the pot
    ///
    /// # Parameters
    ///
    /// - `amount` - The amount to remove from the pot
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the amount to remove is greater than the pot
    pub fn remove_from_pot(&mut self, amount: u64) -> Result<(), TracedError<GameError>> {
        if self.pot.checked_sub(amount).is_none() {
            Err(trace_err!(TracedError::new(GameError::Other(format!(
                "Cannot remove {} from pot as it only has {} left",
                amount, self.pot
            )))))
        } else {
            self.pot -= amount;
            Ok(())
        }
    }

    /// Returns the last side pot
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if there is no side pot
    pub fn get_side_pot(&self) -> Result<SidePot, TracedError<GameError>> {
        self.side_pots
            .first()
            .ok_or_else(|| {
                trace_err!(TracedError::new(GameError::Other(
                    "Could not get side pot".to_string()
                )))
            })
            .cloned()
    }

    /// Returns the last side pot as mutable
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if there is no side pot
    pub fn get_side_pot_mut(&mut self) -> Result<&mut SidePot, TracedError<GameError>> {
        self.side_pots.first_mut().ok_or_else(|| {
            trace_err!(TracedError::new(GameError::Other(
                "Could not get side pot".to_string()
            )))
        })
    }

    /// Returns the highest bet in the last side pot
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if there is no side pot
    pub fn get_side_pot_highest_bet(&self) -> Result<u64, TracedError<GameError>> {
        let highest_bet = self
            .get_side_pot()
            .map_err(|e| trace_err!(e, "Faield to get highest bet in get_side_pot_highest_bet."))?
            .highest_bet;
        Ok(highest_bet)
    }

    /// Creates a new side pot
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user who is contributing to the side pot
    /// - `amount` - The starting amount of the side pot
    pub fn create_side_pot(&mut self, user_principal: Principal, amount: u64) {
        let mut side_pot = SidePot::new();
        side_pot.highest_bet = amount;
        side_pot.pot = amount;
        side_pot.user_principals.push(user_principal);
        self.side_pots.push(side_pot);
        self.log_action(Some(user_principal), ActionType::SidePotCreated);
    }

    /// Calculates the main pot and side pots based on the total bets of all players.
    ///
    /// This function should be called after each betting round to adjust the pots accordingly.
    pub fn calculate_pots(&mut self) -> Result<(), TracedError<GameError>> {
        // Collect all active players and their current_total_bet.
        // Active players are those who have not folded.
        let mut players_bets: Vec<(Principal, u64)> = self
            .user_table_data
            .iter()
            .filter(|(_, data)| {
                data.player_action != PlayerAction::Folded
                    && data.player_action != PlayerAction::SittingOut
                    && data.player_action != PlayerAction::Joining
            })
            .map(|(principal, data)| (*principal, data.current_total_bet))
            .collect();

        // If there are no active players, nothing to do.
        if players_bets.is_empty() {
            return Ok(());
        }

        // Sort the players by their current_total_bet in ascending order.
        players_bets.sort_by_key(|&(_, bet)| bet);

        // Collect the unique bet amounts.
        let mut bet_amounts: Vec<u64> = players_bets.iter().map(|&(_, bet)| bet).collect();
        bet_amounts.sort();
        bet_amounts.dedup();

        let mut previous_bet = if !self.is_side_pot_active {
            let lowest_bet = bet_amounts.remove(0);
            let players_in_pot: Vec<Principal> = players_bets
                .iter()
                .filter(|&&(_, player_bet)| player_bet >= lowest_bet)
                .map(|&(principal, _)| principal)
                .collect();
            self.pot += lowest_bet * players_in_pot.len() as u64;
            lowest_bet
        } else {
            0
        };

        for &bet in &bet_amounts {
            let bet_difference = bet - previous_bet;
            // Players who have bet at least the current bet amount.
            let players_in_pot: Vec<Principal> = players_bets
                .iter()
                .filter(|&&(_, player_bet)| player_bet >= bet)
                .map(|&(principal, _)| principal)
                .collect();

            // The pot amount is the bet difference times the number of players in this pot.
            let pot_amount = bet_difference * players_in_pot.len() as u64;

            // Create or update the side pot.
            let side_pot = SidePot {
                pot: pot_amount,
                user_principals: players_in_pot.clone(),
                confirmed_pot: pot_amount,
                highest_bet: bet,
            };

            // Add the side pot to the list.
            self.side_pots.push(side_pot);
            self.log_action(None, ActionType::SidePotCreated);

            // Update previous bet amount.
            previous_bet = bet;
        }

        // Set is_side_pot_active accordingly.
        self.is_side_pot_active = self.side_pots.len() > 1;

        Ok(())
    }

    /// Calculates the main pot and side pots based on the total bets of all players.
    ///
    /// This function should be called after each betting round to adjust the pots accordingly.
    pub fn calculate_seated_out_pots(&mut self) -> Result<(), TracedError<GameError>> {
        // Collect all active players and their current_total_bet.
        // Active players are those who have not folded.
        let players_bets: Vec<(Principal, u64)> = self
            .user_table_data
            .iter()
            .filter(|(_, data)| {
                data.player_action != PlayerAction::Folded
                    && data.player_action != PlayerAction::Joining
            })
            .map(|(principal, data)| (*principal, data.current_total_bet))
            .collect();

        // If there are no active players, nothing to do.
        if players_bets.is_empty() {
            return Ok(());
        }

        for bet in players_bets {
            self.pot += bet.1;
        }

        Ok(())
    }
}
