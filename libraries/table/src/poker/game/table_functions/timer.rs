use std::time::Duration;

use errors::{game_error::GameError, table_error::TableError};
use ic_cdk::futures::spawn;
use user::user::WalletPrincipalId;

use crate::table_canister::{handle_timer_expiration_wrapper, start_new_betting_round_wrapper};

use super::table::Table;

impl Table {
    /// Sets a turn timer for a given user on a table.
    ///
    /// # Parameters
    ///
    /// - `user_id`: The principal of the user whose turn timer is being set
    /// - `delay_seconds`: The number of seconds to wait before the timer expires
    pub fn start_turn_timer(&mut self, user_id: WalletPrincipalId, delay_seconds: u64) {
        let delay = Duration::from_secs(delay_seconds);

        ic_cdk::println!(
            "Starting turn timer for user: {:?} with delay: {} seconds",
            user_id.0.to_text(),
            delay_seconds
        );
        // If a timer already exists, clear it before setting a new one
        self.clear_turn_timer();

        let table_principal = self.id;
        let timer_id: ic_cdk_timers::TimerId = ic_cdk_timers::set_timer(delay, move || {
            spawn(async move {
                let mut retries = 0;
                while retries < 5 {
                    match handle_timer_expiration_wrapper(table_principal, user_id).await {
                        Ok(_) => return,
                        Err(err) => {
                            ic_cdk::println!(
                                    "Error handling inactive user: {:?}\nAttempting retry after delay...",
                                    err
                                );
                        }
                    }

                    retries += 1;
                }
            })
        });

        self.timer = Some(timer_id);
    }

    /// Sets a timer to start the next turn.
    ///
    /// # Parameters
    ///
    /// - `delay_seconds`: The number of seconds to wait before the timer expires
    pub fn start_next_turn_timer(&mut self, delay_seconds: u64) {
        let delay = Duration::from_secs(delay_seconds);

        let table_principal = self.id;
        let _: ic_cdk_timers::TimerId = ic_cdk_timers::set_timer(delay, move || {
            spawn(async move {
                let mut retries = 0;
                while retries < 3 {
                    match start_new_betting_round_wrapper(table_principal).await {
                        Ok(_) => return,
                        Err(err) => {
                            if err
                                == TableError::Game(GameError::ActionNotAllowed {
                                    reason: "Game is ongoing".to_string(),
                                })
                                || err
                                    == TableError::Game(GameError::ActionNotAllowed {
                                        reason: "Not enough players to start a betting round"
                                            .to_string(),
                                    })
                                || err
                                    == TableError::Game(GameError::ActionNotAllowed {
                                        reason: "Game is paused".to_string(),
                                    })
                            {
                                return;
                            }
                            ic_cdk::println!(
                                "Error starting new betting round: {:?}\nAttempting retry after delay...",
                                err
                            );
                        }
                    }

                    retries += 1;
                }
            })
        });
    }

    /// Clears the timer for a user.
    pub fn clear_turn_timer(&mut self) {
        if let Some(timer_id) = self.timer {
            ic_cdk_timers::clear_timer(timer_id);
            self.timer = None;
        }
    }
}
