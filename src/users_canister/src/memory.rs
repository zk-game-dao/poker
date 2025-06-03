use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BTreeMap, DefaultMemoryImpl};
use std::cell::RefCell;
use user::user::User;

use crate::USERS;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

     // Use StableVec to store the vector of users
     static STABLE_USERS: RefCell<BTreeMap<u64, User, Memory>> = RefCell::new(
        BTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );
}

/// Save all users to stable memory before canister upgrade
#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("Starting pre_upgrade...");

    let res = std::panic::catch_unwind(|| {
        if let Ok(users) = USERS.lock() {
            STABLE_USERS.with(|stable_users_ref| {
                let mut stable_users = stable_users_ref.borrow_mut();
                // Delete old data
                stable_users.clear_new();
                
                // Store users with transaction_history removed
                for (i, user) in users.iter().enumerate() {
                    let mut user_clone = user.clone();
                    user_clone.transaction_history = None; // Affects stored data
                    stable_users.insert(i as u64, user_clone);
                }
            });
            
        } else {
            ic_cdk::println!("Failed to acquire USERS lock during pre_upgrade");
        }
    });

    if res.is_err() {
        ic_cdk::println!("Error during pre_upgrade: {:?}", res);
    }
}

/// Restore all users from stable memory after canister upgrade
#[ic_cdk::post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("Starting post_upgrade...");

    let res = std::panic::catch_unwind(|| {
        if let Ok(mut users) = USERS.lock() {
            // Clear existing users
            users.clear();

            STABLE_USERS.with(|stable_users_ref| {
                let stable_users = stable_users_ref.borrow();

                // Retrieve each user from stable storage
                for user in stable_users.values() {
                    users.push(user);
                }

                ic_cdk::println!(
                    "Successfully restored {} users from stable memory",
                    users.len()
                );
            });
        } else {
            ic_cdk::println!("Failed to acquire USERS lock during post_upgrade");
        }
    });

    if res.is_err() {
        ic_cdk::println!("Error during post_upgrade: {:?}", res);
    }
}

// Helper function to debug stable storage state
#[ic_cdk::query]
fn get_stable_storage_stats() -> String {
    let user_count = STABLE_USERS.with(|stable_users_ref| stable_users_ref.borrow().len());

    let volatile_count = match USERS.lock() {
        Ok(users) => users.len(),
        Err(_) => 0,
    };

    format!(
        "Stable storage contains {} users, volatile memory contains {} users",
        user_count, volatile_count
    )
}
