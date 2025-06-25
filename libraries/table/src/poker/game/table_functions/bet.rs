use errors::{game_error::GameError, trace_err, traced_error::TracedError};
use user::user::WalletPrincipalId;

use crate::poker::game::types::GameType;

use super::{
    table::Table,
    types::{BetType, DealStage},
};

impl Table {
    /// Placing a bet by either calling, raising, or betting the big or small blind
    ///
    /// # Parameters
    ///
    /// - `user_principal`: The principal of the user placing the bet
    /// - `bet_type`: The type of bet being placed
    ///
    /// # Errors
    ///
    /// - [`GameError::ActionNotAllowed`]: If the amount raised is less than the big blind
    pub fn bet(
        &mut self,
        user_principal: WalletPrincipalId,
        bet_type: BetType,
    ) -> Result<(), TracedError<GameError>> {
        #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
        {
            ic_cdk::println!(
                "Clearing turn timer for user {:?} in bet function with bet type: {:?}",
                user_principal.0.to_text(),
                bet_type
            );
            self.clear_turn_timer();
        }

        if bet_type != BetType::SmallBlind && bet_type != BetType::BigBlind {
            self.get_user_table_data_mut(user_principal)
                .map_err(|e| {
                    trace_err!(
                        e,
                        "Failed to get user table data to set inactive turns to 0 in bet function."
                    )
                })?
                .inactive_turns = 0;
        }

        match bet_type {
            BetType::BigBlind | BetType::SmallBlind => self.raise(user_principal, bet_type, 0, 0),
            BetType::Ante(amount) => {
                self.check_user_balance(amount, user_principal)?;
                self.users
                    .get_mut(&user_principal)
                    .ok_or_else(|| trace_err!(TracedError::new(GameError::PlayerNotFound)))?
                    .balance
                    .0 -= amount;
                self.add_to_pot(amount);
                Ok(())
            }
            BetType::Raised(amount) => {
                if amount < self.big_blind.0 {
                    return Err(trace_err!(TracedError::new(GameError::ActionNotAllowed {
                        reason: format!("Bet must be at least {}", self.big_blind.0 as f64 / 1e8),
                    })));
                }
                if let GameType::PotLimit(_) = self.config.game_type {
                    if amount > self.get_pot()
                        && bet_type != BetType::BigBlind
                        && bet_type != BetType::SmallBlind
                    {
                        return Err(trace_err!(TracedError::new(GameError::ActionNotAllowed {
                            reason: "Bet must be less than the pot".to_string(),
                        })));
                    }
                }

                let user = self.get_user_table_data_mut(user_principal).map_err(|e| {
                    trace_err!(
                        e,
                        "Failed to get user table data to calculate normal bet amount."
                    )
                })?;
                let normal_amount = amount - user.current_total_bet;

                self.check_user_balance(normal_amount, user_principal)
                    .map_err(|e| trace_err!(e, "Failed to check user balance."))?;

                match self.config.game_type {
                    GameType::FixedLimit(small, big) => self.handle_fixed_limit(
                        user_principal,
                        normal_amount,
                        small,
                        big,
                        amount,
                        bet_type,
                    ),
                    GameType::SpreadLimit(min, max) => self.handle_spread_limit(
                        user_principal,
                        normal_amount,
                        min,
                        max,
                        amount,
                        bet_type,
                    ),
                    _ => self.raise(user_principal, bet_type, normal_amount, amount),
                }
            }
            BetType::Called => self.call(user_principal),
        }?;

        Ok(())
    }

    /// Calculates the current pot amount. This is not necessarily accurate as it does not take into account side pots.
    pub fn get_pot(&self) -> u64 {
        let mut pot = 0;
        for user in &self.user_table_data {
            pot += user.1.current_total_bet;
        }
        pot
    }

    /// Raise based on the fixed limit game type
    ///
    /// # Parameters
    ///
    /// - `user_principal`: The principal of the user placing the bet
    /// - `normal_amount`: The amount of the bet
    /// - `small`: a small bet
    /// - `big`: a big bet
    /// - `amount`: The total amount of the bet
    /// - `bet_type`: The type of bet being placed
    ///
    /// # Errors
    ///
    /// - [`GameError::ActionNotAllowed`]: If the bet is not a valid increment
    fn handle_fixed_limit(
        &mut self,
        user_principal: WalletPrincipalId,
        normal_amount: u64,
        small: u64,
        big: u64,
        amount: u64,
        bet_type: BetType,
    ) -> Result<(), TracedError<GameError>> {
        // Enforce betting increments based on the deal stage
        let is_valid_bet = match self.deal_stage {
            DealStage::Opening | DealStage::Flop | DealStage::Turn => {
                (normal_amount == small)
                    || (normal_amount.saturating_sub(self.last_raise) == small)
                    || (self.deal_stage == DealStage::Flop
                        && normal_amount == (small + (small / 2)))
                    || (amount.saturating_sub(self.highest_bet) == small)
            }
            DealStage::River | DealStage::Showdown => {
                (normal_amount == big) || (normal_amount.saturating_sub(self.last_raise) == big)
            }
            _ => false,
        };

        if !is_valid_bet {
            return Err(trace_err!(TracedError::new(GameError::ActionNotAllowed {
                reason: format!(
                    "Bet must be exactly {} or {} depending on the round",
                    small as f64 / 1e8,
                    big as f64 / 1e8
                ),
            })));
        }
        self.raise(user_principal, bet_type, normal_amount, amount)
    }

    /// Raise based on the spread limit game type
    ///
    /// # Parameters
    ///
    /// - `user_principal`: The principal of the user placing the bet
    /// - `normal_amount`: The amount of the bet
    /// - `min`: The minimum bet
    /// - `max`: The maximum bet
    /// - `amount`: The total amount of the bet
    /// - `bet_type`: The type of bet being placed
    ///
    /// # Errors
    ///
    /// - [`GameError::ActionNotAllowed`]: If the bet is not within `min` and `max`
    fn handle_spread_limit(
        &mut self,
        user_principal: WalletPrincipalId,
        normal_amount: u64,
        min: u64,
        max: u64,
        amount: u64,
        bet_type: BetType,
    ) -> Result<(), TracedError<GameError>> {
        // TODO: Refactor: Return error in if !condition
        if (normal_amount >= min && normal_amount <= max)
            || (normal_amount - self.last_raise >= min && normal_amount - self.last_raise <= max)
        {
            self.raise(user_principal, bet_type, normal_amount, amount)
        } else {
            Err(trace_err!(TracedError::new(GameError::ActionNotAllowed {
                reason: format!(
                    "Bet must be between {} and {}",
                    min as f64 / 1e8,
                    max as f64 / 1e8
                ),
            })))
        }
    }
}
