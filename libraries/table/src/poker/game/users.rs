use std::collections::HashMap;

use candid::{CandidType, Principal};
use errors::{game_error::GameError, trace_err, traced_error::TracedError};
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

    /// Gets the length of the users map.
    pub fn len(&self) -> usize {
        self.users.len()
    }

    /// Returns `true` if the users map is empty.
    pub fn is_empty(&self) -> bool {
        self.users.is_empty()
    }
}
