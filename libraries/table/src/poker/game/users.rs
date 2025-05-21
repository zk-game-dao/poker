use std::collections::HashMap;

use candid::{CandidType, Principal};
use errors::{
    game_error::GameError, table_error::TableError, trace_err, traced_error::TracedError,
    user_error::UserError,
};
use serde::{Deserialize, Serialize};
use user::user::User;

/// A collection of users
/// that maps a user principal to a user object.
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Users {
    /// Map of user principal to user object.
    pub users: HashMap<Principal, User>,
}

impl Default for Users {
    fn default() -> Self {
        Self::new()
    }
}

impl Users {
    pub fn new() -> Users {
        Users {
            users: HashMap::new(),
        }
    }

    /// Returns the user by `user_principal`.
    ///
    /// # Parameters
    ///
    /// - 'internet_identity_principal_id' - The principal of the user.
    ///
    /// # Errors
    ///
    /// - [`GameError::PlayerNotFound`] if the user cannot be found.
    pub fn get_user_by_principal(
        &self,
        internet_identity_principal_id: &Principal,
    ) -> Result<&User, TracedError<GameError>> {
        self.users
            .values()
            .find(|user| user.principal_id == *internet_identity_principal_id)
            .ok_or_else(|| trace_err!(TracedError::new(GameError::PlayerNotFound)))
    }

    /// Add a user to the users map.
    ///
    /// # Parameters
    ///
    /// - `user` - The user to add.
    ///
    /// # Errors
    ///
    /// - [`GameError::UserAlreadyExists`] if the user already exists in the map.
    pub fn add_user(&mut self, user: User) -> Result<(), TracedError<GameError>> {
        if let std::collections::hash_map::Entry::Vacant(e) = self.users.entry(user.principal_id) {
            e.insert(user);
            Ok(())
        } else {
            Err(trace_err!(TracedError::new(GameError::UserAlreadyExists)))
        }
    }

    /// Maps a new `user` to the given `user_principal`.
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user.
    /// - `user` - The user object.
    ///
    /// # Errors
    ///
    /// - [`GameError::PlayerNotFound`] if the user cannot be found.
    pub fn update_user(
        &mut self,
        user_principal: Principal,
        user: User,
    ) -> Result<(), TracedError<GameError>> {
        if let Some(u) = self.users.get_mut(&user_principal) {
            *u = user;
            Ok(())
        } else {
            Err(trace_err!(TracedError::new(GameError::PlayerNotFound)))
        }
    }

    /// Returns the user by `user_principal` or
    /// `None` if the user does not exist.
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user.
    pub fn get(&self, user_principal: &Principal) -> Option<&User> {
        self.users.get(user_principal)
    }

    /// Returns the user by `user_principal` as mutable or
    /// `None` if the user does not exist.
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user.
    pub fn get_mut(&mut self, user_principal: &Principal) -> Option<&mut User> {
        self.users.get_mut(user_principal)
    }

    /// Removes a user by `user_principal`.
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user.
    pub fn remove_user(&mut self, user_principal: Principal) {
        self.users.remove(&user_principal);
    }

    /// Returns the number of users.
    pub fn user_count(&self) -> usize {
        self.users.len()
    }

    /// Deposits the given `amount` to the user by `user_principal`
    /// and returns the updated user object.
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user.
    /// - `amount` - The amount to deposit.
    ///
    /// # Errors
    ///
    /// - [`UserError::CanisterCallFailed`] if the canister call fails.
    /// - [`TableError::PlayerNotFound`] if the user cannot be found.
    pub async fn deposit_to_user(
        &mut self,
        user_principal: Principal,
        amount: u64,
        currency_type: String,
    ) -> Result<User, TableError> {
        match self.get_mut(&user_principal) {
            Some(_) => {
                let (res,): (Result<User, UserError>,) =
                    ic_cdk::call(user_principal, "deposit", (amount, Some(currency_type)))
                        .await
                        .map_err(|e| UserError::CanisterCallFailed(format!("{:?} {}", e.0, e.1)))?;
                let user_object = res?;

                Ok(user_object)
            }
            None => Err(TableError::PlayerNotFound),
        }
    }

    /// Withdraws the given `amount` from user by `user_principal`.
    /// and returns the updated user object.
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user.
    /// - `amount` - The amount to withdraw.
    ///
    /// # Errors
    ///
    /// - [`UserError::CanisterCallFailed`] if the canister call fails.
    /// - [`TableError::PlayerNotFound`] if the user cannot be found.
    pub async fn withdraw_from_user(
        &mut self,
        user_principal: Principal,
        amount: u64,
        currency_type: String,
    ) -> Result<User, TableError> {
        match self.get_mut(&user_principal) {
            Some(_) => {
                let (res,): (Result<User, UserError>,) =
                    ic_cdk::call(user_principal, "withdraw", (amount, Some(currency_type)))
                        .await
                        .map_err(|e| UserError::CanisterCallFailed(format!("{:?} {}", e.0, e.1)))?;

                let user_object = res?;

                Ok(user_object)
            }
            None => Err(TableError::PlayerNotFound),
        }
    }

    /// Transfer `amount` from `from_user_principal` to `to_user_principal`.
    ///
    /// # Parameters
    ///
    /// - `from_user_principal` - The principal of the user to transfer from.
    /// - `to_user_principal` - The principal of the user to transfer to.
    /// - `amount` - The amount to transfer.
    ///
    /// # Errors
    ///
    /// - [`GameError::ActionNotAllowed`] if the user is trying to transfer to themselves.
    /// - [`GameError::InsufficientFunds`] if the user has insufficient funds.
    /// - [`GameError::PlayerNotFound`] if either user cannot be found.
    /// - [`UserError::CanisterCallFailed`] if the canister call fails.
    pub async fn transfer(
        &mut self,
        from_user_principal: Principal,
        to_user_principal: Principal,
        amount: u64,
        currency_type: String,
    ) -> Result<(), TableError> {
        // Check if the user is trying to transfer to themselves
        if from_user_principal == to_user_principal {
            return Err(GameError::ActionNotAllowed {
                reason: "Cannot transfer to self".to_string(),
            }
            .into());
        }

        match self.get_mut(&from_user_principal) {
            Some(from_user) => {
                if from_user.balance < amount {
                    return Err(GameError::InsufficientFunds.into());
                }
                let mut user = self
                    .withdraw_from_user(from_user_principal, amount, currency_type.clone())
                    .await?;
                user.withdraw(amount);
                self.update_user(from_user_principal, user)
                    .map_err(|e| e.into_inner())?;
            }
            None => return Err(GameError::PlayerNotFound.into()),
        }

        if let Some(_to_user) = self.get_mut(&to_user_principal) {
            let mut user = self
                .deposit_to_user(to_user_principal, amount, currency_type)
                .await?;
            user.deposit(amount);
            self.update_user(to_user_principal, user)
                .map_err(|e| e.into_inner())?;
            Ok(())
        } else {
            if let Some(_from_user) = self.get_mut(&from_user_principal) {
                let mut user = self
                    .deposit_to_user(from_user_principal, amount, currency_type)
                    .await?;
                user.deposit(amount);
                self.update_user(from_user_principal, user)
                    .map_err(|e| e.into_inner())?;
            }
            Err(GameError::PlayerNotFound.into())
        }
    }

    /// Gets the length of the users map.
    pub fn len(&self) -> usize {
        self.users.len()
    }

    /// Returns `true` if the users map is empty.
    pub fn is_empty(&self) -> bool {
        self.users.is_empty()
    }
}
