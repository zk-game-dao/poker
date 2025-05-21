use candid::{CandidType, Principal};
use errors::{game_error::GameError, trace_err, traced_error::TracedError};
use serde::{Deserialize, Serialize};

/// a SidePot is created when one or more players
/// go all-in and can no longer contribute to the main pot.
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct SidePot {
    /// The amount in the side pot.
    pub pot: u64,
    pub confirmed_pot: u64,
    /// The players who have contributed to the side pot.
    pub user_principals: Vec<Principal>,
    pub highest_bet: u64,
}

impl Default for SidePot {
    fn default() -> Self {
        Self::new()
    }
}

impl SidePot {
    pub fn new() -> SidePot {
        SidePot {
            pot: 0,
            confirmed_pot: 0,
            user_principals: Vec::new(),
            highest_bet: 0,
        }
    }

    /// Adds the given `amount` to the side pot
    /// and contributes the `user_principal` to the side pot.
    ///
    /// # Parameters
    ///
    /// - `amount` - The amount to add to the side pot.
    /// - `user_principal` - The principal of the user to add to the side pot.
    pub fn add_to_side_pot(&mut self, amount: u64, user_principal: Principal) {
        self.pot += amount;
        if !self.user_principals.contains(&user_principal) {
            self.user_principals.push(user_principal);
        }
        if amount > self.highest_bet {
            self.highest_bet = amount;
        }
    }

    /// Confirms the side pot.
    pub fn confirm_pot(&mut self) {
        self.confirmed_pot = self.pot;
        self.pot = 0;
    }

    /// Removes the given `amount` from the side pot.
    ///
    /// # Parameters
    ///
    /// - `amount` - The amount to remove from the side pot.
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the side pot does not have enough funds.
    pub fn remove_from_side_pot(&mut self, amount: u64) -> Result<(), TracedError<GameError>> {
        if self.pot.checked_sub(amount).is_none() {
            Err(trace_err!(TracedError::new(GameError::Other(format!(
                "Cannot remove {} from side pot as it only has {} left",
                amount, self.pot
            )))))
        } else {
            self.pot -= amount;
            Ok(())
        }
    }
}
