use candid::{Decode, Encode};

use currency::state::TransactionState;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::Cell;
use ic_stable_structures::{storable::Bound, DefaultMemoryImpl, Storable};
use std::{borrow::Cow, cell::RefCell};

use crate::table_index::{PrivateTableIndex, PublicTableIndex};
use crate::{PRIVATE_TABLE_INDEX_STATE, PUBLIC_TABLE_INDEX_STATE, TRANSACTION_STATE};

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 2_000_000_000;

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
impl Storable for PublicTableIndex {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap_or_else(|e| {
            ic_cdk::println!("Serialization error: {:?}", e);
            vec![]
        }))
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_else(|e| {
            ic_cdk::println!("Deserialization error: {:?}", e);
            PublicTableIndex::new()
        })
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}

impl Storable for PrivateTableIndex {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap_or_else(|e| {
            ic_cdk::println!("Serialization error: {:?}", e);
            vec![]
        }))
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_else(|e| {
            ic_cdk::println!("Deserialization error: {:?}", e);
            PrivateTableIndex::new()
        })
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}

thread_local! {
    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static PUBLIC_TABLE_INDEX_STATE_MAP: RefCell<Cell<PublicTableIndex, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
            PublicTableIndex::new()
        ).unwrap()
    );

    static PRIVATE_TABLE_INDEX_STATE_MAP: RefCell<Cell<PrivateTableIndex, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
            PrivateTableIndex::new()
        ).unwrap()
    );

    static TRANSACTION_STATE_CELL: RefCell<Cell<TransactionState, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))),
            TransactionState::new()
        ).unwrap()
    );
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let res = std::panic::catch_unwind(|| {
        if let Ok(index_state) = PUBLIC_TABLE_INDEX_STATE.lock() {
            PUBLIC_TABLE_INDEX_STATE_MAP.with(|p| {
                let mut cell = p.borrow_mut();
                let _ = cell.set(index_state.clone());
            });
        } else {
            ic_cdk::println!("Failed to acquire PUBLIC_TABLE_INDEX_STATE lock");
        }

        if let Ok(index_state) = PRIVATE_TABLE_INDEX_STATE.lock() {
            PRIVATE_TABLE_INDEX_STATE_MAP.with(|p| {
                let mut cell = p.borrow_mut();
                let _ = cell.set(index_state.clone());
            });
        } else {
            ic_cdk::println!("Failed to acquire PRIVATE_TABLE_INDEX_STATE lock");
        }

        if let Ok(transaction_state) = TRANSACTION_STATE.lock() {
            TRANSACTION_STATE_CELL.with(|p| {
                let mut cell = p.borrow_mut();
                let _ = cell.set(transaction_state.clone());
            });
        } else {
            ic_cdk::println!("Failed to acquire TRANSACTION_STATE lock");
        }
    });

    if res.is_err() {
        ic_cdk::println!("Failed to upgrade table_index");
    }
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let res = std::panic::catch_unwind(|| {
        if let Ok(mut index_state) = PUBLIC_TABLE_INDEX_STATE.lock() {
            PUBLIC_TABLE_INDEX_STATE_MAP.with(|p| {
                let cell = p.borrow();
                index_state.clone_from(&cell.get().clone());
            });
        } else {
            ic_cdk::println!("Failed to acquire PUBLIC_TABLE_INDEX_STATE lock");
        }

        if let Ok(mut index_state) = PRIVATE_TABLE_INDEX_STATE.lock() {
            PRIVATE_TABLE_INDEX_STATE_MAP.with(|p| {
                let cell = p.borrow();
                index_state.clone_from(&cell.get().clone());
            });
        } else {
            ic_cdk::println!("Failed to acquire PRIVATE_TABLE_INDEX_STATE lock");
        }

        if let Ok(mut transaction_state) = TRANSACTION_STATE.lock() {
            TRANSACTION_STATE_CELL.with(|p| {
                let cell = p.borrow();
                transaction_state.clone_from(&cell.get().clone());
            });
        } else {
            ic_cdk::println!("Failed to acquire TRANSACTION_STATE lock");
        }
    });

    if res.is_err() {
        ic_cdk::println!("Failed to upgrade table_index");
    }
}
