use candid::{Decode, Encode, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{storable::Bound, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
use user::user::{UsersCanisterId, WalletPrincipalId};

use crate::user_index::UserIndex;
use crate::USER_INDEX_STATE;

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 2_000_000_000;

// Implement Storable for the updated UserIndex structure
impl Storable for UserIndex {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap_or_else(|e| {
            ic_cdk::println!("Serialization error: {:?}", e);
            vec![]
        }))
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_else(|e| {
            ic_cdk::println!("Deserialization error: {:?}", e);
            UserIndex::default()
        })
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}

thread_local! {
    // Memory manager remains the same
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Map user principals to their canister IDs
    static USER_CANISTER_MAP: RefCell<StableBTreeMap<Principal, Principal, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );

    // Map canister IDs to user counts
    static CANISTER_COUNT_MAP: RefCell<StableBTreeMap<Principal, u64, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );

    // Store processed transactions in a stable map
    static TRANSACTION_MAP: RefCell<StableBTreeMap<String, bool, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
        )
    );
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let res = std::panic::catch_unwind(|| {
        if let Ok(user_index_state) = USER_INDEX_STATE.lock() {
            // Save user -> canister mapping
            USER_CANISTER_MAP.with(|p| {
                let mut map = p.borrow_mut();
                for (user_id, canister_id) in user_index_state.user_to_canister.iter() {
                    map.insert(user_id.0, canister_id.0);
                }
            });

            // Save canister -> count mapping
            CANISTER_COUNT_MAP.with(|p| {
                let mut map = p.borrow_mut();
                for (canister_id, count) in user_index_state.canister_user_count.iter() {
                    map.insert(canister_id.0, *count as u64);
                }
            });

            // Save processed transactions
            TRANSACTION_MAP.with(|p| {
                let mut map = p.borrow_mut();
                for tx_id in user_index_state.processed_transactions.iter() {
                    map.insert(tx_id.clone(), true);
                }
            });
        } else {
            ic_cdk::println!("Failed to acquire USER_INDEX_STATE lock");
        }
    });

    if res.is_err() {
        ic_cdk::println!("Failed to save state during pre_upgrade");
    }
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let res = std::panic::catch_unwind(|| {
        if let Ok(mut user_index_state) = USER_INDEX_STATE.lock() {
            // Restore user -> canister mapping
            USER_CANISTER_MAP.with(|p| {
                let map = p.borrow();
                for (user_id, canister_id) in map.iter() {
                    user_index_state
                        .user_to_canister
                        .insert(WalletPrincipalId(user_id), UsersCanisterId(canister_id));
                }
            });

            // Restore canister -> count mapping
            CANISTER_COUNT_MAP.with(|p| {
                let map = p.borrow();
                for (canister_id, count) in map.iter() {
                    user_index_state
                        .canister_user_count
                        .insert(UsersCanisterId(canister_id), count as usize);
                }
            });

            // Restore processed transactions
            TRANSACTION_MAP.with(|p| {
                let map = p.borrow();
                for (tx_id, _) in map.iter() {
                    user_index_state
                        .processed_transactions
                        .insert(tx_id.clone());
                }
            });
        } else {
            ic_cdk::println!("Failed to acquire USER_INDEX_STATE lock");
        }
    });

    if res.is_err() {
        ic_cdk::println!("Failed to restore state during post_upgrade");
    }
}

// Helper function to debug stable storage state
#[ic_cdk::query]
fn get_stable_storage_stats() -> String {
    let user_count = USER_CANISTER_MAP.with(|map| map.borrow().len());
    let canister_count = CANISTER_COUNT_MAP.with(|map| map.borrow().len());
    let transaction_count = TRANSACTION_MAP.with(|map| map.borrow().len());

    format!(
        "Stable storage contains mappings for {} users across {} canisters, with {} processed transactions",
        user_count,
        canister_count,
        transaction_count
    )
}
