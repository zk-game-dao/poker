//! Contains functions related to users in the table.

use candid::Principal;
use errors::game_error::GameError;
use errors::trace_err;
use errors::traced_error::TracedError;
use user::user::{User, UserBalance, UsersCanisterId, WalletPrincipalId};

use crate::poker::game::table_functions::table::TableId;
use crate::poker::game::types::QueueItem;
use crate::table_canister::{get_table_wrapper, join_table};

use super::action_log::ActionType;
use super::table::Table;
use super::types::{CardRequestData, PlayerAction, SeatStatus, UserTableData};

impl Table {
    /// Returns the [UserTableData] by `user_principal`.
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the user is not found.
    pub fn get_user_table_data(
        &self,
        user_principal: WalletPrincipalId,
    ) -> Result<&UserTableData, TracedError<GameError>> {
        // TODO: Return a PlayerNotFound error instead?
        self.user_table_data.get(&user_principal).ok_or_else(|| {
            trace_err!(
                TracedError::new(GameError::Other(
                    "Could not get users table data".to_string()
                )),
                ""
            )
        })
    }

    /// Returns the [UserTableData] by `user_principal` as mutable.
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the user is not found.
    pub fn get_user_table_data_mut(
        &mut self,
        user_principal: WalletPrincipalId,
    ) -> Result<&mut UserTableData, TracedError<GameError>> {
        // TODO: Return a PlayerNotFound error instead?
        self.user_table_data
            .get_mut(&user_principal)
            .ok_or_else(|| {
                trace_err!(
                    TracedError::new(GameError::Other(
                        "Could not get mutable users table data".to_string()
                    )),
                    ""
                )
            })
    }

    /// Resets the [UserTableData] by `user_principal`
    /// to the default values.
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user to reset the data for.
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the user is not found.
    pub fn clear_user_table_data(
        &mut self,
        user_principal: WalletPrincipalId,
    ) -> Result<(), TracedError<GameError>> {
        if let Ok(table_data) = self.get_user_table_data_mut(user_principal) {
            if table_data.player_action != PlayerAction::SittingOut {
                table_data.reset();
            } else {
                table_data.reset();
                table_data.player_action = PlayerAction::SittingOut;
            }
        }
        Ok(())
    }

    /// Requests the user by `user_principal` to show their cards
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user to request the cards from.
    /// - `amount` - The amount the user is willing to pay to view the cards.
    /// - `user_principal_of_requester` - The principal of the user requesting the cards.
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the user is not found.
    pub fn request_user_cards(
        &mut self,
        user_principal: WalletPrincipalId,
        amount: u64,
        user_principal_of_requester: Principal,
    ) -> Result<(), TracedError<GameError>> {
        let user_data: &mut UserTableData = self.get_user_table_data_mut(user_principal)?;
        user_data.show_card_requests.push(CardRequestData {
            user_principal: user_principal_of_requester,
            amount,
            show_cards: false,
        });
        Ok(())
    }

    /// Cancels the request for the user to show their cards.
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user the request was made to.
    /// - `user_principal_of_requester` - The principal of the user requesting the cards.
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the user is not found.
    pub fn cancel_user_cards_request(
        &mut self,
        user_principal: WalletPrincipalId,
        user_principal_of_requester: Principal,
    ) -> Result<(), TracedError<GameError>> {
        let user_data = self.get_user_table_data_mut(user_principal)?;
        user_data
            .show_card_requests
            .retain(|x| x.user_principal != user_principal_of_requester);
        Ok(())
    }

    /// Accepts the request to view the user's cards
    /// and return the [UserTableData]
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user the request was made to.
    /// - `user_principal_of_requester` - The principal of the user requesting the cards.
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the user is not found.
    pub fn confirm_user_cards_request(
        &mut self,
        user_principal: WalletPrincipalId,
        user_principal_of_requester: Principal,
    ) -> Result<UserTableData, TracedError<GameError>> {
        let user_data = self.get_user_table_data_mut(user_principal)?;
        for request in &mut user_data.show_card_requests {
            if request.user_principal == user_principal_of_requester {
                request.show_cards = true;
                return Ok(user_data.clone());
            }
        }
        Ok(user_data.clone())
    }

    /// Adds a `user` to the table.
    ///
    /// # Parameters
    ///
    /// - `user` - The user to add to the table.
    ///
    /// # Errors
    ///
    /// - [`GameError::UserAlreadyInGame`] if the user is already in the game.
    /// - [`GameError::GameFull`] if the table is full.
    pub fn add_user(
        &mut self,
        user: User,
        seat_index: u8,
        player_sitting_out: bool,
    ) -> Result<(), TracedError<GameError>> {
        ic_cdk::println!("Adding user to table: {:?}", user.principal_id);
        // Check table capacity
        if self.number_of_players() as u8 == self.config.seats {
            return Err(trace_err!(TracedError::new(GameError::GameFull)));
        }
        if seat_index >= self.config.seats {
            return Err(trace_err!(TracedError::new(GameError::Other(
                "Seat index out of bounds".to_string()
            ))));
        }

        // Check if user is already at the table
        if self.is_user_in_table(user.principal_id) {
            return Err(trace_err!(TracedError::new(GameError::UserAlreadyInGame)));
        }

        // Verify the seat is available
        match self.seats[seat_index as usize] {
            SeatStatus::Empty => {
                // Set appropriate seat status based on game state
                if self.is_game_ongoing() {
                    self.seats[seat_index as usize] = SeatStatus::QueuedForNextRound(
                        user.principal_id,
                        Box::new(user.clone()),
                        player_sitting_out,
                    );
                } else {
                    self.seats[seat_index as usize] = SeatStatus::Occupied(user.principal_id);

                    // Initialize user data
                    self.user_table_data
                        .insert(user.principal_id, UserTableData::new());
                    if player_sitting_out {
                        self.get_user_table_data_mut(user.principal_id)?
                            .player_action = PlayerAction::SittingOut;
                    } else {
                        self.get_user_table_data_mut(user.principal_id)?
                            .player_action = PlayerAction::Joining;
                    }

                    // Add user and log action
                    self.users.add_user(user.clone())?;
                }

                self.log_action(Some(user.principal_id), ActionType::Join);

                // Set dealer position if this is the first hand
                if self.round_ticker == 0 && !self.is_game_ongoing() {
                    self.set_dealer_position()?;
                }

                Ok(())
            }
            _ => Err(trace_err!(TracedError::new(GameError::Other(
                "Seat is not available".to_string()
            )))),
        }
    }

    /// Helper to check if user is in table with new SeatStatus
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user to check.
    ///
    /// # Returns
    ///
    /// - `true` if the user is in the table, `false` otherwise.
    pub fn is_user_in_table(&self, user_principal: WalletPrincipalId) -> bool {
        self.seats.iter().any(|status| {
            matches!(status,
                SeatStatus::Occupied(p) |
                SeatStatus::QueuedForNextRound(p, _, _) |
                SeatStatus::Reserved { principal: p, .. }
                if *p == user_principal
            )
        })
    }

    /// Removes a user from the table.
    /// If the user is the dealer, the dealer position is rotated.
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user to remove.
    /// - `action_type` - The action type to log.
    pub fn remove_user(
        &mut self,
        user_principal: WalletPrincipalId,
        action_type: ActionType,
    ) -> Result<(), TracedError<GameError>> {
        if !self.is_user_in_table(user_principal) {
            return Err(trace_err!(TracedError::new(GameError::PlayerNotFound)));
        }

        if self.is_game_ongoing() {
            self.queue
                .push(QueueItem::RemoveUser(user_principal, action_type));
        } else {
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

        Ok(())
    }

    /// Removes a user from the table.
    /// If the user is the dealer, the dealer position is rotated.
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user to remove.
    /// - `action_type` - The action type to log.
    pub fn remove_user_for_table_balancing(
        &mut self,
        users_canister_id: UsersCanisterId,
        user_id: WalletPrincipalId,
        table_to_move_to: TableId,
    ) -> Result<(), TracedError<GameError>> {
        if !self.is_user_in_table(user_id) {
            return Err(trace_err!(TracedError::new(GameError::PlayerNotFound)));
        }

        if self.is_game_ongoing() {
            ic_cdk::println!(
                "Queueing user {} to leave table for table balancing",
                user_id.0.to_text()
            );
            self.queue.push(QueueItem::LeaveTableToMove(
                users_canister_id,
                user_id,
                table_to_move_to,
            ));
        } else {
            ic_cdk::println!(
                "Removing user {} from table for table balancing",
                user_id.0.to_text()
            );
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
                let table = get_table_wrapper(table_to_move_to).await;
                match table {
                    Ok(_) => {
                        // TODO: Very inefficient loop. We need a way to better ensure user gets placed at the table.
                        //       This is a temporary fix as when over 3 people get placed at the same table the call
                        //       will fail as it seems to get the same free seat index in the join_table function in
                        //       the table canister.
                        for i in 0..5 {
                            let res = join_table(
                                table_to_move_to,
                                users_canister_id,
                                user_id,
                                None,
                                balance.0,
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

        Ok(())
    }

    pub fn has_user_left(&self, user_principal: WalletPrincipalId) -> bool {
        self.queue
            .iter()
            .any(|item| matches!(item, QueueItem::RemoveUser(p, _) if *p == user_principal))
    }

    /// Kicks a user by `user_principal` from the table.
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user to kick.
    /// - `reason` - The reason for kicking the user.
    pub fn kick_user(
        &mut self,
        user_principal: WalletPrincipalId,
        reason: String,
    ) -> Result<UserBalance, TracedError<GameError>> {
        // Clean up all user state immediately
        if let Some(idx) = self
            .seats
            .iter()
            .position(|s| matches!(s, SeatStatus::Occupied(p) if *p == user_principal))
        {
            self.seats[idx] = SeatStatus::Empty;
        }
        self.user_table_data.remove(&user_principal);
        let balance = self
            .users
            .get(&user_principal)
            .map(|u| u.balance)
            .unwrap_or(UserBalance(0));
        self.users.remove_user(user_principal);
        self.log_action(Some(user_principal), ActionType::Kicked { reason });
        Ok(balance)
    }

    /// Checks if a user by `user_principal` has enough balance
    /// for the given `amount`
    ///
    /// # Parameters
    ///
    /// - `amount` - The amount to check if the user has enough balance for.
    /// - `user_principal` - The principal of the user to check.
    ///
    /// # Errors
    ///
    /// - [`GameError::PlayerNotFound`] if the user is not found.
    /// - [`GameError::InsufficientFunds`] if the user has insufficient funds.
    pub fn check_user_balance(
        &self,
        amount: u64,
        user_principal: WalletPrincipalId,
    ) -> Result<(), TracedError<GameError>> {
        let user = self
            .users
            .get(&user_principal)
            .ok_or_else(|| trace_err!(TracedError::new(GameError::PlayerNotFound)))?;

        if amount > user.balance.0 {
            return Err(trace_err!(TracedError::new(GameError::InsufficientFunds)));
        }
        Ok(())
    }

    /// Kicks a user by `user_principal` from the table if they don't
    /// have enough balance for the given `amount`
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user to check and kick.
    /// - `amount` - The amount to check if the user has enough balance for.
    pub fn check_and_kick_user_for_insufficient_funds(
        &mut self,
        user_principal: WalletPrincipalId,
        amount: u64,
    ) -> Result<Option<UserBalance>, TracedError<GameError>> {
        if self.check_user_balance(amount, user_principal).is_err() {
            let balance = self.kick_user(user_principal, "Insufficient funds".to_string())?;
            return Ok(Some(balance));
        }
        Ok(None)
    }

    /// Returns the balance of the user by `user_principal`
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user to get the balance for.
    ///
    /// # Errors
    ///
    /// - [`GameError::PlayerNotFound`] if the user is not found.
    pub fn get_user_balance(
        &self,
        user_principal: WalletPrincipalId,
    ) -> Result<UserBalance, TracedError<GameError>> {
        let user = self
            .users
            .get(&user_principal)
            .ok_or_else(|| trace_err!(TracedError::new(GameError::PlayerNotFound)))?;
        Ok(user.balance)
    }

    /// Returns the current total bet of the user by `user_principal`
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user to get the current total bet for.
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the user is not found.
    pub fn get_users_current_total_bet(
        &self,
        user_principal: WalletPrincipalId,
    ) -> Result<u64, TracedError<GameError>> {
        let user_table_data = self.get_user_table_data(user_principal)?;
        Ok(user_table_data.current_total_bet)
    }

    /// Subtracts the given `amount` from the user
    /// by `user_principal`'s balance
    ///
    /// # Parameters
    ///
    /// - `amount` - The amount to subtract from the user's
    ///   balance and add to their current total bet.
    /// - `user_principal` - The principal of the user to update.
    /// - `player_action` - The action to set the user's current action to.
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the user's [UserTableData] is not found.
    /// - [`GameError::PlayerNotFound`] if the user is not found.
    pub fn update_user_balances(
        &mut self,
        amount: u64,
        user_principal: WalletPrincipalId,
        player_action: PlayerAction,
    ) -> Result<(), TracedError<GameError>> {
        // TODO: Refactor: get the user before mutating the user's table data
        let user_table_data = self.get_user_table_data_mut(user_principal)?;
        user_table_data.player_action = player_action;
        user_table_data.current_total_bet += amount;

        let user = self
            .users
            .users
            .get_mut(&user_principal)
            .ok_or_else(|| trace_err!(TracedError::new(GameError::PlayerNotFound)))?;
        user.balance.0 -= amount;
        Ok(())
    }

    /// Sets the user by `user_principal`'s action to `player_action`
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user to update.
    /// - `player_action` - The action to set the user's current action to.
    ///
    /// # Errors
    ///
    /// - [`GameError::Other`] if the user is not found.
    pub fn set_player_action(
        &mut self,
        user_principal: WalletPrincipalId,
        player_action: PlayerAction,
    ) -> Result<(), TracedError<GameError>> {
        let user_table_data = self.get_user_table_data_mut(user_principal)?;
        user_table_data.player_action = player_action;
        Ok(())
    }
}
