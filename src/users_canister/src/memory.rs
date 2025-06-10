use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{Cell, DefaultMemoryImpl, Storable};
use std::borrow::Cow;
use std::cell::RefCell;

use crate::{Users, USERS};

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_USERS_SIZE: u32 = 1_000_000_000; // 100MB, adjust as needed


// Implement Storable for TournamentIndex
impl Storable for Users {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap_or_else(|e| {
            ic_cdk::println!("TournamentIndex serialization error: {:?}", e);
            vec![]
        }))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_else(|e| {
            ic_cdk::println!("TournamentIndex deserialization error: {:?}", e);
            Users::new()
        })
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_USERS_SIZE,
        is_fixed_size: false,
    };
}

thread_local! {
    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static STATE_CELL: RefCell<Cell<Users, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
            Users::new()
        ).unwrap()
    );
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let res = std::panic::catch_unwind(|| {
        // Save STATE
        if let Ok(state) = USERS.lock() {
            STATE_CELL.with(|cell| {
                let mut cell = cell.borrow_mut();
                let _ = cell.set(state.clone());
            });
        } else {
            ic_cdk::println!("Failed to acquire STATE lock during pre_upgrade");
        }
    });

    if res.is_err() {
        ic_cdk::println!("Error during pre_upgrade for tournament index canister");
    }
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let res = std::panic::catch_unwind(|| {
        // Restore STATE
        if let Ok(mut state) = USERS.lock() {
            STATE_CELL.with(|cell| {
                let cell = cell.borrow();
                *state = cell.get().clone();
            });
        } else {
            ic_cdk::println!("Failed to acquire STATE lock during post_upgrade");
        }
    });

    if res.is_err() {
        ic_cdk::println!("Error during post_upgrade for users canister");
    }
}
