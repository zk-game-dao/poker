use candid::Principal;
use canister_functions::rake_stats::RakeStats;
use chat::ChatHistory;
use currency::state::TransactionState;
use currency::types::currency_manager::CurrencyManager;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::Cell;
use ic_stable_structures::DefaultMemoryImpl;
use std::cell::RefCell;
use table::poker::game::table_functions::table::Table;
use table::poker::game::types::StorableTable;

use crate::{
    BACKEND_PRINCIPAL, CHAT_HISTORY, CURRENCY_MANAGER, RAKE_STATS, TABLE, TRANSACTION_STATE,
};

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static BACKEND_PRINCIPAL_CELL: RefCell<Cell<Option<Principal>, Memory>> = RefCell::new(Cell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), None).unwrap());

    static TRANSACTION_STATE_CELL: RefCell<Cell<TransactionState, Memory>> = RefCell::new(Cell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))), TransactionState::default()).unwrap());

    static CELL: RefCell<Cell<StorableTable, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
            StorableTable::default()
        ).unwrap()
    );

    static CURRENCY_MANAGER_CELL: RefCell<Cell<Option<CurrencyManager>, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))),
            None
        ).unwrap()
    );

    static RAKE_STATS_CELL: RefCell<Cell<RakeStats, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))),
            RakeStats::new()
        ).unwrap()
    );

    static CHAT_HISTORY_CELL: RefCell<Cell<ChatHistory, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))),
            ChatHistory::new(1000)
        ).unwrap()
    );
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let res = std::panic::catch_unwind(|| {
        if let Ok(table) = TABLE.lock() {
            CELL.with(|p| {
                let mut cell = p.borrow_mut();
                let table = match table.as_ref() {
                    Some(table) => table.clone(),
                    None => Table::default(),
                };
                let storable_table = StorableTable::from(table);
                let _ = cell.set(storable_table);
            });
        } else {
            println!("Failed to acquire TABLE lock");
        }

        let backend_principal = BACKEND_PRINCIPAL.lock();
        match backend_principal {
            Ok(backend_principal) => {
                BACKEND_PRINCIPAL_CELL.with(|p| {
                    let mut cell = p.borrow_mut();
                    let _ = cell.set(*backend_principal);
                });
            }
            Err(_) => {
                ic_cdk::println!("Failed to acquire BACKEND_PRINCIPAL lock");
            }
        }

        let transaction_state = TRANSACTION_STATE.lock();
        match transaction_state {
            Ok(transaction_state) => {
                TRANSACTION_STATE_CELL.with(|p| {
                    let mut cell = p.borrow_mut();
                    let _ = cell.set(transaction_state.clone());
                });
            }
            Err(_) => {
                ic_cdk::println!("Failed to acquire TRANSACTION_STATE lock");
            }
        }

        let currency_manager = CURRENCY_MANAGER.lock();
        match currency_manager {
            Ok(currency_manager) => {
                CURRENCY_MANAGER_CELL.with(|p| {
                    let mut cell = p.borrow_mut();
                    let _ = cell.set(currency_manager.clone());
                });
            }
            Err(_) => {
                ic_cdk::println!("Failed to acquire CURRENCY_MANAGER lock");
            }
        }

        let rake_stats = RAKE_STATS.lock();
        match rake_stats {
            Ok(rake_stats) => {
                RAKE_STATS_CELL.with(|p| {
                    let mut cell = p.borrow_mut();
                    let _ = cell.set(rake_stats.clone());
                });
            }
            Err(_) => {
                ic_cdk::println!("Failed to acquire RAKE_STATS lock");
            }
        }

        let chat_history = CHAT_HISTORY.lock();
        match chat_history {
            Ok(chat_history) => {
                CHAT_HISTORY_CELL.with(|p| {
                    let mut cell = p.borrow_mut();
                    let _ = cell.set(chat_history.clone());
                });
            }
            Err(_) => {
                ic_cdk::println!("Failed to acquire CHAT_HISTORY lock");
            }
        }
    });

    if res.is_err() {
        ic_cdk::println!("Error during pre_upgrade for table_canister");
    }
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let res = std::panic::catch_unwind(|| {
        if let Ok(mut table) = TABLE.lock() {
            CELL.with(|p| {
                let cell = p.borrow();
                table.clone_from(&Some(cell.get().clone().into()));
            });
        } else {
            println!("Failed to acquire TABLE lock");
        }

        if let Ok(mut backend_principal) = BACKEND_PRINCIPAL.lock() {
            BACKEND_PRINCIPAL_CELL.with(|p| {
                let cell = p.borrow();
                backend_principal.clone_from(&cell.get().clone());
            });
        } else {
            println!("Failed to acquire BACKEND_PRINCIPAL_CELL");
        }

        if let Ok(mut transaction_state) = TRANSACTION_STATE.lock() {
            TRANSACTION_STATE_CELL.with(|p| {
                let cell = p.borrow();
                transaction_state.clone_from(&cell.get().clone());
            });
        } else {
            println!("Failed to acquire TRANSACTION_STATE_CELL");
        }

        if let Ok(mut currency_manager) = CURRENCY_MANAGER.lock() {
            CURRENCY_MANAGER_CELL.with(|p| {
                let cell = p.borrow();
                currency_manager.clone_from(&cell.get().clone());
            });
        } else {
            println!("Failed to acquire CURRENCY_MANAGER lock");
        }

        if let Ok(mut rake_stats) = RAKE_STATS.lock() {
            RAKE_STATS_CELL.with(|p| {
                let cell = p.borrow();
                rake_stats.clone_from(&cell.get().clone());
            });
        } else {
            println!("Failed to acquire RAKE_STATS lock");
        }

        if let Ok(mut chat_history) = CHAT_HISTORY.lock() {
            CHAT_HISTORY_CELL.with(|p| {
                let cell = p.borrow();
                chat_history.clone_from(&cell.get().clone());
            });
        } else {
            println!("Failed to acquire CHAT_HISTORY lock");
        }
    });

    if res.is_err() {
        ic_cdk::println!("Error during post_upgrade for table_canister");
    }
}
