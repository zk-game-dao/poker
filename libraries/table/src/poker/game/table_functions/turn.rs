use errors::game_error::GameError;
use errors::{trace_err, traced_error::TracedError};
use user::user::WalletPrincipalId;

use super::{
    table::Table,
    types::{DealStage, PlayerAction, SeatStatus},
};

impl Table {
    /// Check if the table should go to the next round or showdown
    pub fn check_next_turn_or_showdown(&mut self) -> Result<(), TracedError<GameError>> {
        if self.go_to_next_round().map_err(|e| trace_err!(e, ""))?
            && self.deal_stage != DealStage::Showdown
        {
            self.calculate_pots().map_err(|e| trace_err!(e, ""))?;
            if self.deal_stage == DealStage::Flop
                || self.deal_stage == DealStage::Turn
                || self.deal_stage == DealStage::River
            {
                self.set_current_player_to_left_of_dealer()
                    .map_err(|e| trace_err!(e, ""))?;
            }
            self.deal_cards(false).map_err(|e| trace_err!(e, ""))?;
        } else if self.go_to_next_round().map_err(|e| trace_err!(e, ""))?
            && self.deal_stage == DealStage::Showdown
        {
            self.calculate_pots().map_err(|e| trace_err!(e, ""))?;
            self.showdown().map_err(|e| trace_err!(e, ""))?;
        } else if self
            .all_in_cycle_to_showdown()
            .map_err(|e| trace_err!(e, ""))?
        {
            self.calculate_pots().map_err(|e| trace_err!(e, ""))?;
            self.cycle_to_showdown().map_err(|e| trace_err!(e, ""))?;
        }
        Ok(())
    }

    pub fn set_current_player_to_left_of_dealer(&mut self) -> Result<(), TracedError<GameError>> {
        let dealer_position = self.dealer_position;
        let mut current_position = dealer_position;
        for _ in 0..self.seats.len() {
            current_position = (current_position + 1) % self.seats.len();

            if let SeatStatus::Occupied(user_principal) = self.seats[current_position] {
                if let Some(user_data) = self.user_table_data.get_mut(&user_principal) {
                    if user_data.player_action != PlayerAction::Folded
                        && user_data.player_action != PlayerAction::SittingOut
                        && user_data.player_action != PlayerAction::Joining
                        && user_data.player_action != PlayerAction::AllIn
                    {
                        self.current_player_index = current_position;
                        #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
                        {
                            ic_cdk::println!(
                                "Clearing turn timer in set_current_player_to_left_of_dealer."
                            );
                            self.clear_turn_timer();
                            let current_player =
                                self.get_player_at_seat(self.current_player_index).map_err(
                                    |e| trace_err!(e, "Failed to get current player after loop."),
                                )?;

                            self.set_last_timer_started_timestamp(ic_cdk::api::time());
                            self.start_turn_timer(
                                current_player,
                                self.config.timer_duration.into(),
                            );
                            self.notifications
                                .clear_notifications_older_than(ic_cdk::api::time() - 6e11 as u64); // 10 minutes

                            self.notifications.add_notification(
                                current_player,
                                crate::poker::game::table_functions::types::NotificationMessage::UserTurnStarted,
                            );
                        }
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    }

    /// Is every player ready for the showdown?
    pub fn all_in_cycle_to_showdown(&self) -> Result<bool, TracedError<GameError>> {
        // Step 1: Get active players (excluding Folded, SittingOut, or Joining)
        let active_players: Vec<WalletPrincipalId> = self
            .seats
            .iter()
            .filter_map(|seat_status| {
                match seat_status {
                    SeatStatus::Occupied(principal) => {
                        // Only include principal if they're actively playing
                        if let Some(user_data) = self.user_table_data.get(principal) {
                            if !matches!(
                                user_data.player_action,
                                PlayerAction::Folded
                                    | PlayerAction::SittingOut
                                    | PlayerAction::Joining
                            ) {
                                Some(*principal)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    _ => None, // Filter out all other seat statuses
                }
            })
            .collect();

        // If there's only one active player, the game can proceed to showdown
        if active_players.len() <= 1 {
            return Ok(true);
        }

        // Check if any players need to act
        let highest_bet = active_players
            .iter()
            .map(|user| self.user_table_data.get(user).unwrap().current_total_bet)
            .max()
            .unwrap_or(0);

        let any_player_needs_to_act = active_players.iter().any(|user| {
            let user_data = self.user_table_data.get(user).unwrap();
            // A player needs to act if:
            // - They are not All-In
            // - Their current total bet is less than the highest bet
            // - They have not folded or are not sitting out
            // - They have not yet acted (action is None)
            user_data.player_action != PlayerAction::AllIn
                && user_data.current_total_bet < highest_bet
                && !matches!(
                    user_data.player_action,
                    PlayerAction::Folded | PlayerAction::SittingOut
                )
        });

        if any_player_needs_to_act {
            return Ok(false);
        }

        // Check if further betting is possible
        let players_with_chips = active_players
            .iter()
            .filter(|user| {
                let user_data = self.user_table_data.get(user).unwrap();
                let user = self.users.get(user).unwrap();
                user_data.player_action != PlayerAction::AllIn && user.balance.0 > 0
            })
            .count();

        if players_with_chips <= 1 {
            // No further betting is possible
            return Ok(true);
        }

        // In all other cases, we should not proceed to showdown yet
        Ok(false)
    }

    /// Can the table go to the next round?
    ///
    /// The table can go to the next round if all
    /// players have either folded, checked, called, or gone all in.
    /// Determines if we should proceed to the next betting round or cycle to showdown.
    pub fn go_to_next_round(&mut self) -> Result<bool, TracedError<GameError>> {
        // First, check if we need to cycle to showdown
        if self
            .all_in_cycle_to_showdown()
            .map_err(|e| trace_err!(e, ""))?
        {
            // We need to cycle to showdown, so we should not go to the next round
            return Ok(false);
        }

        // Now, check if all players have completed their actions for the current betting round
        for user_principal in self.seats.iter() {
            if let SeatStatus::Occupied(user_principal) = user_principal {
                let highest_bet = self
                    .get_highest_current_total_bet()
                    .map_err(|e| trace_err!(e, ""))?;
                if let Some(user_table_data) = self.user_table_data.get_mut(user_principal) {
                    // Skip players who are folded, sitting out, joining, or all-in
                    if matches!(
                        user_table_data.player_action,
                        PlayerAction::Folded
                            | PlayerAction::SittingOut
                            | PlayerAction::Joining
                            | PlayerAction::AllIn
                    ) {
                        continue;
                    }

                    // If the player's action is None (they haven't acted yet), they need to act
                    if user_table_data.player_action == PlayerAction::None {
                        return Ok(false);
                    }

                    if let PlayerAction::Raised(amount) = user_table_data.player_action {
                        if amount == highest_bet
                            && self.big_blind_user_principal == *user_principal
                            && self.deal_stage == DealStage::Flop
                        {
                            return Ok(false);
                        }
                    }

                    // If the player's total bet is less than the highest bet, they need to act
                    if user_table_data.current_total_bet < highest_bet {
                        return Ok(false);
                    }
                }
            }
        }

        // All players have completed their actions; we can proceed to the next betting round
        Ok(true)
    }
}
