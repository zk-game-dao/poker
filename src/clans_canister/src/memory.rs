use candid::{Decode, Encode, Principal};
use clan::{Clan, ClanId};
use currency::state::TransactionState;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{Cell, DefaultMemoryImpl, Storable};
use std::borrow::Cow;
use std::cell::RefCell;
use user::user::WalletPrincipalId;

use crate::{ClanEvents, BACKEND_PRINCIPAL, CLAN, CLAN_EVENTS, TRANSACTION_STATE};

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_CLAN_EVENTS_SIZE: u32 = 10_000_000; // 10MB for events

// Implement Storable for Vec<ClanEvent>
impl Storable for ClanEvents {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap_or_else(|e| {
            ic_cdk::println!("ClanEvents serialization error: {:?}", e);
            vec![]
        }))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_else(|e| {
            ic_cdk::println!("ClanEvents deserialization error: {:?}", e);
            ClanEvents::new()
        })
    }

    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Bounded {
            max_size: MAX_CLAN_EVENTS_SIZE,
            is_fixed_size: false,
        };
}

thread_local! {
    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static CLAN_CELL: RefCell<Cell<Clan, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
            Clan {
                id: ClanId::default(),
                name: "".to_string(),
                description: "".to_string(),
                tags: std::collections::HashSet::new(),
                avatar: None,
                supported_currency: currency::Currency::ICP,
                members: std::collections::HashMap::new(),
                member_limit: 100,
                pending_requests: Vec::new(),
                invited_users: std::collections::HashMap::new(),
                privacy: clan::ClanPrivacy::Public,
                require_proof_of_humanity: false,
                subscription_enabled: true,
                subscription_tiers: std::collections::HashMap::new(),
                joining_fee: 0,
                treasury: clan::treasury::ClanTreasury::default(),
                environment_settings: clan::environment::ClanEnvironmentSettings::default(),
                stats: clan::ClanStats::default(),
                active_tables: Vec::new(),
                hosted_tournaments: Vec::new(),
                created_at: 0,
                created_by: WalletPrincipalId::default(),
                website: None,
                discord: None,
                twitter: None,
            }
        ).unwrap()
    );

    static BACKEND_PRINCIPAL_CELL: RefCell<Cell<Option<Principal>, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
            None
        ).unwrap()
    );

    static TRANSACTION_STATE_CELL: RefCell<Cell<TransactionState, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
            TransactionState::new()
        ).unwrap()
    );

    static CLAN_EVENTS_CELL: RefCell<Cell<ClanEvents, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))),
            ClanEvents::new()
        ).unwrap()
    );
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let res = std::panic::catch_unwind(|| {
        // Save CLAN
        if let Ok(clan) = CLAN.lock() {
            CLAN_CELL.with(|cell| {
                let mut cell = cell.borrow_mut();
                if let Some(clan_data) = clan.as_ref() {
                    let _ = cell.set(clan_data.clone());
                } else {
                    ic_cdk::println!("No clan data to save during pre_upgrade");
                }
            });
        } else {
            ic_cdk::println!("Failed to acquire CLAN lock during pre_upgrade");
        }

        // Save BACKEND_PRINCIPAL
        if let Ok(backend_principal) = BACKEND_PRINCIPAL.lock() {
            BACKEND_PRINCIPAL_CELL.with(|cell| {
                let mut cell = cell.borrow_mut();
                let _ = cell.set(*backend_principal);
            });
        } else {
            ic_cdk::println!("Failed to acquire BACKEND_PRINCIPAL lock during pre_upgrade");
        }

        // Save TRANSACTION_STATE
        if let Ok(transaction_state) = TRANSACTION_STATE.lock() {
            TRANSACTION_STATE_CELL.with(|cell| {
                let mut cell = cell.borrow_mut();
                let _ = cell.set(transaction_state.clone());
            });
        } else {
            ic_cdk::println!("Failed to acquire TRANSACTION_STATE lock during pre_upgrade");
        }

        // Save CLAN_EVENTS
        if let Ok(clan_events) = CLAN_EVENTS.lock() {
            CLAN_EVENTS_CELL.with(|cell| {
                let mut cell = cell.borrow_mut();
                let _ = cell.set(clan_events.clone());
            });
        } else {
            ic_cdk::println!("Failed to acquire CLAN_EVENTS lock during pre_upgrade");
        }
    });

    if res.is_err() {
        ic_cdk::println!("Error during pre_upgrade for clan canister");
    }
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let res = std::panic::catch_unwind(|| {
        // Restore CLAN
        if let Ok(mut clan) = CLAN.lock() {
            CLAN_CELL.with(|cell| {
                let cell = cell.borrow();
                let restored_clan = cell.get().clone();

                // Only restore if it's not the default empty clan
                if restored_clan.id != Principal::anonymous() {
                    *clan = Some(restored_clan);
                } else {
                    ic_cdk::println!("Restored default clan, keeping None state");
                }
            });
        } else {
            ic_cdk::println!("Failed to acquire CLAN lock during post_upgrade");
        }

        // Restore BACKEND_PRINCIPAL
        if let Ok(mut backend_principal) = BACKEND_PRINCIPAL.lock() {
            BACKEND_PRINCIPAL_CELL.with(|cell| {
                let cell = cell.borrow();
                *backend_principal = cell.get().clone();
            });
        } else {
            ic_cdk::println!("Failed to acquire BACKEND_PRINCIPAL lock during post_upgrade");
        }

        // Restore TRANSACTION_STATE
        if let Ok(mut transaction_state) = TRANSACTION_STATE.lock() {
            TRANSACTION_STATE_CELL.with(|cell| {
                let cell = cell.borrow();
                *transaction_state = cell.get().clone();
            });
        } else {
            ic_cdk::println!("Failed to acquire TRANSACTION_STATE lock during post_upgrade");
        }

        // Restore CLAN_EVENTS
        if let Ok(mut clan_events) = CLAN_EVENTS.lock() {
            CLAN_EVENTS_CELL.with(|cell| {
                let cell = cell.borrow();
                *clan_events = cell.get().clone();
            });
        } else {
            ic_cdk::println!("Failed to acquire CLAN_EVENTS lock during post_upgrade");
        }
    });

    if res.is_err() {
        ic_cdk::println!("Error during post_upgrade for clan canister");
    }
}
