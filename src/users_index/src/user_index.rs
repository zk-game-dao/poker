use std::collections::{HashMap, HashSet};

use candid::{CandidType, Principal};
use errors::user_error::UserError;
use futures::future::join_all;
use intercanister_call_wrappers::users_canister::{
    get_pure_poker_user_experience_points_wrapper, get_user_experience_points_wrapper,
    get_verified_pure_poker_user_experience_points_wrapper,
    get_verified_user_experience_points_wrapper,
};
use serde::Deserialize;
use user::user::{UsersCanisterId, WalletPrincipalId};

use crate::{
    LEADERBOARD_CACHE, LEADERBOARD_CACHE_TIMESTAMP, PURE_POKER_LEADERBOARD_CACHE,
    PURE_POKER_LEADERBOARD_CACHE_TIMESTAMP, USER_INDEX_STATE,
};

#[derive(Debug, Clone, PartialEq, CandidType, Deserialize)]
pub struct UserIndex {
    // Maps user principal to their canister
    pub user_to_canister: HashMap<WalletPrincipalId, UsersCanisterId>,

    // Maps canister to count of users it contains
    pub canister_user_count: HashMap<UsersCanisterId, usize>,

    pub processed_transactions: HashSet<String>,
}

impl Default for UserIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl UserIndex {
    pub fn new() -> UserIndex {
        UserIndex {
            user_to_canister: HashMap::new(),
            canister_user_count: HashMap::new(),
            processed_transactions: HashSet::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction_id: String) {
        self.processed_transactions.insert(transaction_id);
    }

    pub fn transaction_exists(&self, transaction_id: &str) -> bool {
        self.processed_transactions.contains(transaction_id)
    }

    pub fn add_user(
        &mut self,
        user_id: WalletPrincipalId,
        user_canister_id: UsersCanisterId,
    ) -> Result<(), UserError> {
        self.user_to_canister.insert(user_id, user_canister_id);
        Ok(())
    }

    pub fn get_users_canister_principal(&self, user_id: WalletPrincipalId) -> Option<&UsersCanisterId> {
        self.user_to_canister.get(&user_id)
    }

    pub fn remove_user(&mut self, user_id: WalletPrincipalId) {
        self.user_to_canister.remove(&user_id);
    }

    pub fn user_count(&self) -> usize {
        self.user_to_canister.len()
    }

    pub fn get_user_canisters(&self) -> Vec<WalletPrincipalId> {
        self.user_to_canister.keys().cloned().collect()
    }

    pub fn get_available_canister(&self) -> Option<UsersCanisterId> {
        // Find a canister with less than 1000 users
        self.canister_user_count
            .iter()
            .find(|(_, &count)| count < 1000)
            .map(|(canister_id, _)| *canister_id)
    }

    pub fn get_users_canisters(&self) -> Vec<UsersCanisterId> {
        self.canister_user_count.keys().copied().collect()
    }

    pub async fn get_experience_points_leaderboard(
        &self,
    ) -> Result<Vec<(WalletPrincipalId, u64)>, UserError> {
        const BATCH_SIZE: usize = 30; // Process 30 users at a time

        let users: Vec<UsersCanisterId> = self.canister_user_count.keys().copied().collect();
        let mut all_results = Vec::new();

        // Process users in batches
        for chunk in users.chunks(BATCH_SIZE) {
            let futures: Vec<_> = chunk
                .iter()
                .map(|&users_canister| async move {
                    match get_user_experience_points_wrapper(users_canister).await {
                        Ok(points) => points,
                        Err(e) => {
                            ic_cdk::println!(
                                "Failed to get experience points for user {}: {:?}",
                                users_canister.0.to_text(),
                                e
                            );
                            vec![]
                        }
                    }
                })
                .collect();

            // Execute batch of queries
            let batch_results: Vec<Vec<(WalletPrincipalId, u64)>> = join_all(futures).await;
            all_results.extend(batch_results.into_iter().flatten());
        }

        // Sort by experience points in descending order
        all_results.sort_by(|a, b| b.1.cmp(&a.1));

        Ok(all_results)
    }

    pub async fn get_pure_poker_experience_points_leaderboard(
        &self,
    ) -> Result<Vec<(WalletPrincipalId, u64)>, UserError> {
        const BATCH_SIZE: usize = 30; // Process 30 users at a time

        let users: Vec<UsersCanisterId> = self.canister_user_count.keys().copied().collect();
        let mut all_results = Vec::new();

        // Process users in batches
        for chunk in users.chunks(BATCH_SIZE) {
            let futures: Vec<_> = chunk
                .iter()
                .map(|&user| async move {
                    match get_pure_poker_user_experience_points_wrapper(user).await {
                        Ok(points) => points,
                        Err(e) => {
                            ic_cdk::println!(
                                "Failed to get experience points for user {}: {:?}",
                                user.0.to_text(),
                                e
                            );
                            vec![]
                        }
                    }
                })
                .collect();

            // Execute batch of queries
            let batch_results: Vec<Vec<(WalletPrincipalId, u64)>> = join_all(futures).await;
            all_results.extend(batch_results.into_iter().flatten());
        }

        // Sort by experience points in descending order
        all_results.sort_by(|a, b| b.1.cmp(&a.1));

        Ok(all_results)
    }

    pub async fn get_verified_experience_points_leaderboard(
        &self,
    ) -> Result<Vec<(WalletPrincipalId, u64)>, UserError> {
        const BATCH_SIZE: usize = 30; // Process 30 users at a time

        let users: Vec<UsersCanisterId> = self.canister_user_count.keys().copied().collect();
        let mut all_results = Vec::new();

        // Process users in batches
        for chunk in users.chunks(BATCH_SIZE) {
            let futures: Vec<_> = chunk
                .iter()
                .map(|&users_canister| async move {
                    match get_verified_user_experience_points_wrapper(users_canister).await {
                        Ok(points) => points,
                        Err(e) => {
                            ic_cdk::println!(
                                "Failed to get verified experience points for user {}: {:?}",
                                users_canister.0.to_text(),
                                e
                            );
                            vec![]
                        }
                    }
                })
                .collect();

            // Execute batch of queries
            let batch_results: Vec<Vec<(WalletPrincipalId, u64)>> = join_all(futures).await;
            all_results.extend(batch_results.into_iter().flatten());
        }

        // Sort by experience points in descending order
        all_results.sort_by(|a, b| b.1.cmp(&a.1));

        Ok(all_results)
    }

    pub async fn get_verified_pure_poker_experience_points_leaderboard(
        &self,
    ) -> Result<Vec<(WalletPrincipalId, u64)>, UserError> {
        const BATCH_SIZE: usize = 30; // Process 30 users at a time

        let users: Vec<UsersCanisterId> = self.canister_user_count.keys().copied().collect();
        let mut all_results = Vec::new();

        // Process users in batches
        for chunk in users.chunks(BATCH_SIZE) {
            let futures: Vec<_> = chunk
                .iter()
                .map(|&user| async move {
                    match get_verified_pure_poker_user_experience_points_wrapper(user).await {
                        Ok(points) => points,
                        Err(e) => {
                            ic_cdk::println!(
                                "Failed to get verified pure poker experience points for user {}: {:?}",
                                user.0.to_text(),
                                e
                            );
                            vec![]
                        },
                    }
                })
                .collect();

            // Execute batch of queries
            let batch_results: Vec<Vec<(WalletPrincipalId, u64)>> = join_all(futures).await;
            all_results.extend(batch_results.into_iter().flatten());
        }

        // Sort by experience points in descending order
        all_results.sort_by(|a, b| b.1.cmp(&a.1));

        Ok(all_results)
    }
}

pub async fn get_position_in_leaderboard(
    user_principal: Principal,
    is_pure_poker: bool,
) -> Result<Option<u64>, UserError> {
    let leaderboard = {
        let cache_timestamp = if is_pure_poker {
            *PURE_POKER_LEADERBOARD_CACHE_TIMESTAMP
                .lock()
                .map_err(|_| UserError::LockError)?
        } else {
            *LEADERBOARD_CACHE_TIMESTAMP
                .lock()
                .map_err(|_| UserError::LockError)?
        };
        if let Some(timestamp) = cache_timestamp {
            if ic_cdk::api::time() - timestamp < 3_600_000_000_000 {
                // Use cache if available and not expired
                let cache = if is_pure_poker {
                    PURE_POKER_LEADERBOARD_CACHE
                        .lock()
                        .map_err(|_| UserError::LockError)?
                } else {
                    LEADERBOARD_CACHE.lock().map_err(|_| UserError::LockError)?
                };
                (*cache).clone()
            } else {
                None
            }
        } else {
            None
        }
    };

    let leaderboard = if let Some(lb) = leaderboard {
        lb
    } else {
        // Cache is empty or expired, fetch fresh leaderboard
        let user_index = USER_INDEX_STATE
            .lock()
            .map_err(|_| UserError::LockError)?
            .clone();
        let leaderboard = if is_pure_poker {
            user_index
                .get_pure_poker_experience_points_leaderboard()
                .await?
        } else {
            user_index.get_experience_points_leaderboard().await?
        };

        // Update cache
        if is_pure_poker {
            *PURE_POKER_LEADERBOARD_CACHE
                .lock()
                .map_err(|_| UserError::LockError)? = Some(leaderboard.clone());
            *PURE_POKER_LEADERBOARD_CACHE_TIMESTAMP
                .lock()
                .map_err(|_| UserError::LockError)? = Some(ic_cdk::api::time());
        } else {
            *LEADERBOARD_CACHE.lock().map_err(|_| UserError::LockError)? =
                Some(leaderboard.clone());
            *LEADERBOARD_CACHE_TIMESTAMP
                .lock()
                .map_err(|_| UserError::LockError)? = Some(ic_cdk::api::time());
        }
        leaderboard
    };

    // Find user's position in the leaderboard (1-indexed)
    let position = leaderboard
        .iter()
        .position(|(principal, _)| *principal == user_principal)
        .map(|pos| (pos) as u64); // Add 1 to make it 1-indexed

    Ok(position)
}
