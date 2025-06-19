use candid::Principal;
use currency::state::TransactionState;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    Cell, Storable,
};
use user::user::WalletPrincipalId;
use std::cell::RefCell;
use std::sync::atomic::Ordering;
use tournaments::tournaments::types::TournamentData;

use crate::{
    LAST_BALANCE_TIMESTAMP, LAST_HEARTBEAT, LEADERBOARD, PRIZE_POOL, TOURNAMENT, TOURNAMENT_INDEX,
    TOURNAMENT_START_TIME, TRANSACTION_STATE,
};

// Define memory type
type Memory = VirtualMemory<ic_stable_structures::DefaultMemoryImpl>;

// Stable storage definitions
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<ic_stable_structures::DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(ic_stable_structures::DefaultMemoryImpl::default()));

    static TOURNAMENT_CELL: RefCell<Cell<Option<TournamentData>, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
            None
        ).unwrap()
    );

    static TOURNAMENT_INDEX_CELL: RefCell<Cell<Option<Principal>, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
            None
        ).unwrap()
    );

    static LEADERBOARD_CELL: RefCell<Cell<Vec<u8>, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
            Vec::new()
        ).unwrap()
    );

    static PRIZE_POOL_CELL: RefCell<Cell<u64, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))),
            0
        ).unwrap()
    );

    static TOURNAMENT_START_TIME_CELL: RefCell<Cell<u64, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))),
            u64::MAX
        ).unwrap()
    );

    static LAST_BALANCE_TIMESTAMP_CELL: RefCell<Cell<u64, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))),
            0
        ).unwrap()
    );

    static LAST_HEARTBEAT_CELL: RefCell<Cell<u64, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6))),
            0
        ).unwrap()
    );

    static TRANSACTION_STATE_CELL: RefCell<Cell<TransactionState, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7))),
            TransactionState::new()
        ).unwrap()
    );
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let res = std::panic::catch_unwind(|| {
        // Save TOURNAMENT
        if let Ok(tournament) = TOURNAMENT.lock() {
            TOURNAMENT_CELL.with(|cell| {
                let mut cell = cell.borrow_mut();
                let _ = cell.set(tournament.clone());
            });
        } else {
            ic_cdk::println!("Failed to acquire TOURNAMENT lock");
        }

        // Save TOURNAMENT_INDEX
        if let Ok(tournament_index) = TOURNAMENT_INDEX.lock() {
            TOURNAMENT_INDEX_CELL.with(|cell| {
                let mut cell = cell.borrow_mut();
                let _ = cell.set(*tournament_index);
            });
        } else {
            ic_cdk::println!("Failed to acquire TOURNAMENT_INDEX lock");
        }

        // Save LEADERBOARD
        if let Ok(leaderboard) = LEADERBOARD.lock() {
            LEADERBOARD_CELL.with(|cell| {
                let mut cell = cell.borrow_mut();
                let mut bytes = Vec::new();
                for p in leaderboard.iter() {
                    let p_bytes = p.0.to_bytes().into_owned();
                    bytes.extend_from_slice(&(p_bytes.len() as u32).to_le_bytes()); // 4-byte length prefix
                    bytes.extend_from_slice(&p_bytes);
                }
                let _ = cell.set(bytes);
            });
        } else {
            ic_cdk::println!("Failed to acquire LEADERBOARD lock");
        }

        // Save PRIZE_POOL
        PRIZE_POOL_CELL.with(|cell| {
            let mut cell = cell.borrow_mut();
            let _ = cell.set(PRIZE_POOL.load(Ordering::Relaxed));
        });

        // Save TOURNAMENT_START_TIME
        TOURNAMENT_START_TIME_CELL.with(|cell| {
            let mut cell = cell.borrow_mut();
            let _ = cell.set(TOURNAMENT_START_TIME.load(Ordering::Relaxed));
        });

        // Save LAST_BALANCE_TIMESTAMP
        LAST_BALANCE_TIMESTAMP_CELL.with(|cell| {
            let mut cell = cell.borrow_mut();
            let _ = cell.set(LAST_BALANCE_TIMESTAMP.load(Ordering::Relaxed));
        });

        // Save LAST_HEARTBEAT
        LAST_HEARTBEAT_CELL.with(|cell| {
            let mut cell = cell.borrow_mut();
            let _ = cell.set(LAST_HEARTBEAT.load(Ordering::Relaxed));
        });

        // Save TRANSACTION_STATE
        if let Ok(transaction_state) = TRANSACTION_STATE.lock() {
            TRANSACTION_STATE_CELL.with(|cell| {
                let mut cell = cell.borrow_mut();
                let _ = cell.set(transaction_state.clone());
            });
        } else {
            ic_cdk::println!("Failed to acquire TRANSACTION_STATE lock");
        }
    });

    if res.is_err() {
        ic_cdk::println!("Error during pre_upgrade for tournament canister");
    }
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let res = std::panic::catch_unwind(|| {
        // Restore TOURNAMENT
        if let Ok(mut tournament) = TOURNAMENT.lock() {
            TOURNAMENT_CELL.with(|cell| {
                let cell = cell.borrow();
                *tournament = cell.get().clone();
            });
        } else {
            ic_cdk::println!("Failed to acquire TOURNAMENT lock");
        }

        // Restore TOURNAMENT_INDEX
        if let Ok(mut tournament_index) = TOURNAMENT_INDEX.lock() {
            TOURNAMENT_INDEX_CELL.with(|cell| {
                let cell = cell.borrow();
                *tournament_index = *cell.get();
            });
        } else {
            ic_cdk::println!("Failed to acquire TOURNAMENT_INDEX lock");
        }

        // Restore LEADERBOARD
        if let Ok(mut leaderboard) = LEADERBOARD.lock() {
            LEADERBOARD_CELL.with(|cell| {
                let cell = cell.borrow();
                let bytes = cell.get();
                let mut leaderboard_vec = Vec::new();
                let mut offset = 0;
                while offset + 4 <= bytes.len() {
                    // Check for length prefix
                    if let Ok(len_bytes) = bytes[offset..offset + 4].try_into() {
                        // Try to get 4 bytes
                        let len = u32::from_le_bytes(len_bytes) as usize;
                        offset += 4;
                        if offset + len <= bytes.len() {
                            let principal = WalletPrincipalId(Principal::from_slice(&bytes[offset..offset + len]));
                            leaderboard_vec.push(principal);

                            offset += len;
                        } else {
                            ic_cdk::println!(
                                "Warning: Truncated leaderboard data at offset {}",
                                offset
                            );
                            break;
                        }
                    } else {
                        ic_cdk::println!(
                            "Warning: Failed to parse length prefix at offset {}",
                            offset
                        );
                        break; // Skip rest if length prefix is malformed
                    }
                }
                *leaderboard = leaderboard_vec;
            });
        } else {
            ic_cdk::println!("Failed to acquire LEADERBOARD lock");
        }

        // Restore PRIZE_POOL
        PRIZE_POOL_CELL.with(|cell| {
            let cell = cell.borrow();
            PRIZE_POOL.store(*cell.get(), Ordering::Relaxed);
        });

        // Restore TOURNAMENT_START_TIME
        TOURNAMENT_START_TIME_CELL.with(|cell| {
            let cell = cell.borrow();
            TOURNAMENT_START_TIME.store(*cell.get(), Ordering::Relaxed);
        });

        // Restore LAST_BALANCE_TIMESTAMP
        LAST_BALANCE_TIMESTAMP_CELL.with(|cell| {
            let cell = cell.borrow();
            LAST_BALANCE_TIMESTAMP.store(*cell.get(), Ordering::Relaxed);
        });

        // Restore LAST_HEARTBEAT
        LAST_HEARTBEAT_CELL.with(|cell| {
            let cell = cell.borrow();
            LAST_HEARTBEAT.store(*cell.get(), Ordering::Relaxed);
        });

        // Restore TRANSACTION_STATE
        if let Ok(mut transaction_state) = TRANSACTION_STATE.lock() {
            TRANSACTION_STATE_CELL.with(|cell| {
                let cell = cell.borrow();
                *transaction_state = cell.get().clone();
            });
        } else {
            ic_cdk::println!("Failed to acquire TRANSACTION_STATE lock");
        }
    });

    if res.is_err() {
        ic_cdk::println!("Error during post_upgrade for tournament canister");
    }
}
