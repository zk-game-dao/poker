use candid::{CandidType, Decode, Encode, Principal};
use macros::{impl_principal_traits, impl_u64_comparisons};
use serde::{Deserialize, Serialize};

use ic_stable_structures::{storable::Bound, Storable};
use std::{borrow::Cow, collections::HashMap};

use crate::admin::{AdminRole, BanType};

const MAX_VALUE_SIZE: u32 = 200_000_000;
pub const REFERRAL_PERIOD: u64 = 30 * 24 * 60 * 60 * 1_000_000_000;

pub fn time() -> u64 {
    #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
    {
        ic_cdk::api::time()
    }
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    {
        use std::time::SystemTime;

        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum TransferType {
    CardShowRequest,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct EmojiUserAvatar {
    pub emoji: u64,
    pub style: u64,
}

/// The UserAvatar enum is used to store the user's avatar.
///
/// Right now, the only supported avatar type is an emoji.
#[derive(Debug, Clone, Hash, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum UserAvatar {
    Emoji(EmojiUserAvatar),
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq, Hash, Copy)]
pub struct WalletPrincipalId(pub Principal);

#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq, Hash, Copy)]
pub struct UsersCanisterId(pub Principal);

impl_principal_traits!(WalletPrincipalId);
impl_principal_traits!(UsersCanisterId);

#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq, Hash, Copy)]
#[derive(Default)]
pub struct UserBalance(pub u64);

impl_u64_comparisons!(UserBalance);


/// The User struct is stored in memory on the user canister.
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct User {
    pub principal_id: WalletPrincipalId,
    pub users_canister_id: UsersCanisterId,
    pub user_name: String,
    pub balance: UserBalance,
    pub address: Option<String>,
    pub avatar: Option<UserAvatar>,
    pub created_at: Option<u64>,
    /// The tables that the user is currently active in.
    pub active_tables: Vec<Principal>,
    pub enlarge_text: Option<bool>,
    pub volume_level: Option<u16>,
    pub eth_wallet_address: Option<String>,
    experience_points: Option<u64>,
    pub is_verified: Option<bool>,
    experience_points_pure_poker: Option<u64>,

    /// Referral system fields
    pub referrer: Option<WalletPrincipalId>,
    pub referred_users: Option<HashMap<WalletPrincipalId, u64>>,
    pub referral_start_date: Option<u64>, // Timestamp when user was referred

    /// Admin system
    pub admin_role: Option<AdminRole>,
    pub ban_status: Option<BanType>,
    pub ban_history: Option<Vec<BanType>>,
}

impl User {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        principal_id: WalletPrincipalId,
        users_canister_id: UsersCanisterId,
        user_name: String,
        balance: UserBalance,
        address: Option<String>,
        avatar: Option<UserAvatar>,
        eth_wallet_address: Option<String>,
        referrer: Option<WalletPrincipalId>,
        referral_start_date: Option<u64>,
    ) -> User {
        User {
            user_name,
            users_canister_id,
            balance,
            address,
            principal_id,
            avatar,
            created_at: Some(time()),
            active_tables: Vec::new(),
            enlarge_text: None,
            volume_level: None,
            experience_points: Some(0),
            eth_wallet_address,
            is_verified: None,
            experience_points_pure_poker: Some(0),
            referrer,
            referred_users: Some(HashMap::new()),
            referral_start_date,
            admin_role: None,
            ban_status: None,
            ban_history: None,
        }
    }

    /// Set the user name of the user.
    ///
    /// # Parameters
    ///
    /// - `user_name` - The user name to set.
    pub fn set_user_name(&mut self, user_name: String) {
        self.user_name = user_name;
    }

    /// Set the avatar of the user.
    ///
    /// # Parameters
    ///
    /// - `avatar` - The avatar to set.
    pub fn set_avatar(&mut self, avatar: Option<UserAvatar>) {
        self.avatar = avatar;
    }

    /// Set the balance of the user.
    ///
    /// # Parameters
    ///
    /// - `balance` - The balance to set.
    pub fn set_balance(&mut self, balance: u64) {
        self.balance = UserBalance(balance);
    }

    /// Set the address of the user.
    ///
    /// # Parameters
    ///
    /// - `address` - The address to set.
    pub fn set_address(&mut self, address: Option<String>) {
        self.address = address;
    }

    /// Set the principal id of the user's internet identity.
    ///
    /// # Parameters
    ///
    /// - `principal_id` - The principal id to set.
    pub fn set_principal_id(&mut self, principal_id: Principal) {
        self.principal_id = WalletPrincipalId(principal_id);
    }

    /// Deposit `amount` into the user's balance.
    ///
    /// # Parameters
    ///
    /// - `amount` - The amount to deposit.
    pub fn deposit(&mut self, amount: u64) {
        self.balance.0 += amount;
    }

    /// Withdraw `amount` from the user's balance.
    ///
    /// # Parameters
    ///
    /// - `amount` - The amount to withdraw.
    pub fn withdraw(&mut self, amount: u64) {
        self.balance.0 -= amount;
    }

    /// Gets the user's level
    ///
    /// # Returns
    ///
    /// The user's level.
    pub fn get_level(&self) -> f64 {
        let experience_points = self.experience_points.unwrap_or(0);
        if experience_points == 0 {
            0.0
        } else {
            experience_points as f64 / 100.0
        }
    }

    /// Gets the users experience points.
    ///
    /// # Returns
    ///
    /// The user's experience points.
    pub fn get_experience_points(&self) -> u64 {
        self.experience_points.unwrap_or(0)
    }

    /// Gets the users pure poker experience points.
    ///
    /// # Returns
    ///
    /// The user's pure poker experience points.
    pub fn get_pure_poker_experience_points(&self) -> u64 {
        self.experience_points_pure_poker.unwrap_or(0)
    }

    /// Clear the user's experience points.
    pub fn clear_experience_points(&mut self) {
        self.experience_points = Some(0);
    }

    /// Clear the user's pure poker experience points.
    pub fn clear_pure_poker_experience_points(&mut self) {
        self.experience_points_pure_poker = Some(0);
    }

    /// Add experience points to the user.
    ///
    /// # Parameters
    ///
    /// - `experience_points` - The experience points to add.
    pub fn add_experience_points(&mut self, experience_points: u64) {
        if self.can_gain_xp() {
            self.experience_points = Some(self.experience_points.unwrap_or(0) + experience_points);
        }
    }

    /// Add btc experience points to the user.
    ///
    /// # Parameters
    ///
    /// - `experience_points` - The experience points to add.
    pub fn add_pure_poker_experience_points(&mut self, experience_points: u64) {
        if self.can_gain_xp() {
            self.experience_points_pure_poker =
                Some(self.experience_points_pure_poker.unwrap_or(0) + experience_points);
        }
    }

    pub fn get_referral_tier(&self) -> u8 {
        let referral_count = self
            .referred_users
            .as_ref()
            .unwrap_or(&HashMap::new())
            .len();
        match referral_count {
            0..=3 => 1,
            4..=9 => 2,
            10..=19 => 3,
            20..=49 => 4,
            50..=99 => 5,
            _ => 6,
        }
    }

    pub fn get_referral_rake_percentage(&self) -> u8 {
        match self.get_referral_tier() {
            1 => 10,
            2 => 12,
            3 => 15,
            4 => 18,
            5 => 20,
            6 => 25,
            _ => 10, // Default to 10% for safety
        }
    }

    pub fn add_referred_user(&mut self, user_id: WalletPrincipalId) {
        let referred_users = self.referred_users.get_or_insert_with(HashMap::new);
        let timestamp = time();
        referred_users.entry(user_id).or_insert(timestamp);

        // Check for all referred users and remove those who are no longer within the referral period
        referred_users.retain(|_, &mut time| timestamp + REFERRAL_PERIOD > time);
    }

    pub fn is_within_referral_period(&self) -> bool {
        if let Some(start_date) = self.referral_start_date {
            let now = time();
            let one_month_nanos = REFERRAL_PERIOD;
            now - start_date <= one_month_nanos
        } else {
            false
        }
    }
}

// For a type to be used in a `StableBTreeMap`, it needs to implement the `Storable`
// trait, which specifies how the type can be serialized/deserialized.
//
// In this example, we're using candid to serialize/deserialize the struct, but you
// can use anything as long as you're maintaining backward-compatibility. The
// backward-compatibility allows you to change your struct over time (e.g. adding
// new fields).
//
// The `Storable` trait is already implemented for several common types (e.g. u64),
// so you can use those directly without implementing the `Storable` trait for them.
impl Storable for User {
    /// Serializes the struct into a byte array.
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap_or_else(|e| {
            ic_cdk::println!("Serialization error: {:?}", e);
            vec![]
        }))
    }

    /// Deserializes the struct from a byte array.
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_else(|e| {
            ic_cdk::println!("Deserialization error: {:?}", e);
            User {
                user_name: String::new(),
                balance: UserBalance(0),
                address: None,
                principal_id: WalletPrincipalId(Principal::anonymous()),
                users_canister_id: UsersCanisterId(Principal::anonymous()),
                avatar: None,
                created_at: Some(time()),
                active_tables: Vec::new(),
                enlarge_text: None,
                volume_level: None,
                experience_points: Some(0),
                eth_wallet_address: None,
                is_verified: None,
                experience_points_pure_poker: Some(0),
                referrer: None,
                referred_users: Some(HashMap::new()),
                referral_start_date: None,
                admin_role: None,
                ban_status: None,
                ban_history: None,
            }
        })
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}
