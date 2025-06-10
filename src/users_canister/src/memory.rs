use candid::Principal;
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
     static STABLE_USERS: RefCell<BTreeMap<Principal, User, Memory>> = RefCell::new(
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
                
                for (uid, user) in users.iter() {
                    stable_users.insert(*uid, user.clone());
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
                for (uid, user) in stable_users.iter() {
                    users.insert(uid, user);
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
