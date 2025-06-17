use candid::{Decode, Encode, Principal};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Bound,
    Cell, Storable, Vec as StableVec,
};
use std::borrow::Cow;
use std::cell::RefCell;

use crate::{TournamentIndex, STATE};

// Define memory type
type Memory = VirtualMemory<ic_stable_structures::DefaultMemoryImpl>;

// Define maximum size for TournamentIndex serialization
const MAX_TOURNAMENT_INDEX_SIZE: u32 = 100_000_000; // 100MB, adjust as needed

// Implement Storable for TournamentIndex
impl Storable for TournamentIndex {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap_or_else(|e| {
            ic_cdk::println!("TournamentIndex serialization error: {:?}", e);
            vec![]
        }))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_else(|e| {
            ic_cdk::println!("TournamentIndex deserialization error: {:?}", e);
            TournamentIndex::new()
        })
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_TOURNAMENT_INDEX_SIZE,
        is_fixed_size: false,
    };
}

// Stable storage definitions
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<ic_stable_structures::DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(ic_stable_structures::DefaultMemoryImpl::default()));

    static STATE_CELL: RefCell<Cell<TournamentIndex, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
            TournamentIndex::new()
        ).unwrap()
    );

    pub static TABLE_CANISTER_POOL: RefCell<StableVec<Principal, Memory>> = RefCell::new(
        StableVec::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        ).unwrap()
    );
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let res = std::panic::catch_unwind(|| {
        // Save STATE
        if let Ok(state) = STATE.lock() {
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
        if let Ok(mut state) = STATE.lock() {
            STATE_CELL.with(|cell| {
                let cell = cell.borrow();
                *state = cell.get().clone();
            });
        } else {
            ic_cdk::println!("Failed to acquire STATE lock during post_upgrade");
        }
    });

    if res.is_err() {
        ic_cdk::println!("Error during post_upgrade for tournament index canister");
    }
}
