use authentication::validate_caller;
// use authentication::validate_caller;
use candid::{Nat, Principal};
use canister_functions::{
    create_canister_wrapper,
    cycle::{
        check_and_top_up_canister, get_cycle_balances, monitor_and_top_up_canisters,
        top_up_canister,
    },
    install_wasm_code,
    upgrade_wasm_code,
};
use currency::types::currency_manager::CurrencyManager;
use errors::{canister_management_error::CanisterManagementError, user_error::UserError};
use ic_ledger_types::{
    AccountIdentifier, Subaccount, DEFAULT_SUBACCOUNT,
};
use intercanister_call_wrappers::users_canister::{create_user_wrapper, get_user_wrapper, update_user_wrapper};
use lazy_static::lazy_static;
use user::user::{User, UserAvatar};
use user_index::{get_position_in_leaderboard, UserIndex};

use std::sync::Mutex;

mod memory;
pub mod reset_xp_utils;
pub mod user_index;

const MINIMUM_CYCLE_THRESHOLD: u128 = 2_000_000_000_000;

pub struct CanisterState {
    pub owner: Principal,
    pub default_subaccount: Subaccount,
    pub account_identifier: AccountIdentifier,
}

async fn handle_cycle_check() -> Result<(), UserError> {
    let id = ic_cdk::api::canister_self();
    let cycle_dispenser_canister_id =
        if id == Principal::from_text("lvq5c-nyaaa-aaaam-qdswa-cai").unwrap() {
            *CYCLE_DISPENSER_CANISTER_PROD
        } else if id == Principal::from_text("m3tym-daaaa-aaaah-qqbsq-cai").unwrap() {
            *CYCLE_DISPENSER_CANISTER_TEST
        } else if id == Principal::from_text("txyno-ch777-77776-aaaaq-cai").unwrap() {
            *CYCLE_DISPENSER_CANISTER_DEV
        } else {
            return Ok(());
        };
    check_and_top_up_canister(id, cycle_dispenser_canister_id, MINIMUM_CYCLE_THRESHOLD).await?;
    Ok(())
}

pub type PlayerId = u64;
pub type TableId = u64;

// Define a global instance of GameState wrapped in a Mutex for safe concurrent access.
lazy_static! {
    static ref USER_INDEX_STATE: Mutex<UserIndex> = Mutex::new(UserIndex::new());
    static ref CANISTER_STATE: Mutex<Option<CanisterState>> = Mutex::new(None);
    static ref SUPPORT_US_WALLET: Principal =
        Principal::from_text("amwxf-a2rkd-b42qc-jwbst-oy3co-d5ues-jgfcp-khbg4-zdxoa-n66ja-2ae")
            .unwrap();

    static ref CYCLE_DISPENSER_CANISTER_PROD: Principal = Principal::from_text("zuv6g-yaaaa-aaaam-qbeza-cai").unwrap();
    static ref CYCLE_DISPENSER_CANISTER_TEST: Principal = Principal::from_text("ev34d-5yaaa-aaaah-qdska-cai").unwrap();
    static ref CYCLE_DISPENSER_CANISTER_DEV: Principal = Principal::from_text("tz2ag-zx777-77776-aaabq-cai").unwrap();
    static ref CONTROLLER_PRINCIPALS: Vec<Principal> = vec![
        Principal::from_text("km7qz-4bai4-e5ptx-hgrck-z3web-ameqg-ksxcf-u7wbr-t5fna-i7bqp-hqe").unwrap(),
        Principal::from_text("uyxh5-bi3za-gxbfs-op3gj-ere73-a6jhv-5jky3-zawef-b5r2s-k26un-sae").unwrap(),
    ];
    static ref USER_CANISTER_WASM: &'static [u8] = include_bytes!("../../../target/wasm32-unknown-unknown/release/users_canister.wasm");

    static ref LEADERBOARD_CACHE: Mutex<Option<Vec<(Principal, u64)>>> = Mutex::new(None);
    static ref LEADERBOARD_CACHE_TIMESTAMP: Mutex<Option<u64>> = Mutex::new(None);
    static ref PURE_POKER_LEADERBOARD_CACHE: Mutex<Option<Vec<(Principal, u64)>>> = Mutex::new(None);
    static ref PURE_POKER_LEADERBOARD_CACHE_TIMESTAMP: Mutex<Option<u64>> = Mutex::new(None);

    static ref VERIFIED_LEADERBOARD_CACHE: Mutex<Option<Vec<(Principal, u64)>>> = Mutex::new(None);
    static ref VERIFIED_LEADERBOARD_CACHE_TIMESTAMP: Mutex<Option<u64>> = Mutex::new(None);
    static ref VERIFIED_PURE_POKER_LEADERBOARD_CACHE: Mutex<Option<Vec<(Principal, u64)>>> = Mutex::new(None);
    static ref VERIFIED_PURE_POKER_LEADERBOARD_CACHE_TIMESTAMP: Mutex<Option<u64>> = Mutex::new(None);

    static ref CURRENCY_MANAGER: Mutex<CurrencyManager> = Mutex::new(CurrencyManager::new());
}

fn get_canister_state() -> CanisterState {
    let owner_principal = ic_cdk::api::canister_self();

    let account_identifier = AccountIdentifier::new(&owner_principal, &DEFAULT_SUBACCOUNT);
    CanisterState {
        owner: owner_principal,
        default_subaccount: DEFAULT_SUBACCOUNT,
        account_identifier,
    }
}

#[ic_cdk::init]
fn init() {
    let id = ic_cdk::api::canister_self();
    ic_cdk::println!("Users index canister {id} initialized");
    let canister_state = get_canister_state();
    let mut canister_state_mutex = match CANISTER_STATE.lock() {
        Ok(canister_state) => canister_state,
        Err(_) => {
            ic_cdk::println!("Failed to acquire lock");
            return;
        }
    };
    *canister_state_mutex = Some(canister_state);
}

#[ic_cdk::query]
fn get_account_number() -> Result<Option<String>, UserError> {
    let canister_state = get_canister_state();
    Ok(Some(canister_state.account_identifier.to_string()))
}

#[ic_cdk::query]
fn ping() -> String {
    "Ok".to_string()
}

#[ic_cdk::update]
async fn create_user(
    user_name: String,
    address: Option<String>,
    principal_id: Principal,
    avatar: Option<UserAvatar>,
    referrer: Option<Principal>,
) -> Result<User, UserError> {
    handle_cycle_check().await?;
    if user_name.is_empty() {
        return Err(UserError::InvalidRequest(
            "User name cannot be empty".to_string(),
        ));
    }

    let mut user_index_state = {
        USER_INDEX_STATE
            .lock()
            .map_err(|_| UserError::LockError)?
            .clone()
    };

    // Check if user already exists
    if user_index_state
        .user_to_canister
        .contains_key(&principal_id)
    {
        return Err(UserError::UserAlreadyExists);
    }

    // Find an available canister or create a new one
    let user_canister = match user_index_state.get_available_canister() {
        Some(canister_id) => canister_id,
        None => {
            // No available canister, create a new one
            let controller_principals = CONTROLLER_PRINCIPALS.clone();
            let wasm_module = USER_CANISTER_WASM.to_vec();
            let new_canister = create_canister_wrapper(controller_principals, None).await?;
            install_wasm_code(new_canister, wasm_module).await?;

            // Initialize the count for this new canister
            user_index_state.canister_user_count.insert(new_canister, 0);

            new_canister
        }
    };

    let res = create_user_wrapper(
        user_canister,
        user_name,
        address,
        principal_id,
        avatar,
        referrer,
    )
    .await;

    let mut user_index_state = USER_INDEX_STATE.lock().map_err(|_| UserError::LockError)?;

    // Handle the result and update the index
    match res {
        Ok((user, player_count)) => {
            user_index_state.add_user(principal_id, user_canister)?;

            // Mark the canister as full if needed
            if let Some(count) = user_index_state.canister_user_count.get_mut(&user_canister) {
                *count = player_count;
            } else {
                user_index_state
                    .canister_user_count
                    .insert(user_canister, player_count);
            }

            Ok(user)
        }
        Err(e) => Err(e),
    }
}

#[ic_cdk::update]
async fn get_users_canister_principal_by_id(user_id: Principal) -> Result<Principal, UserError> {
    let user_index_state = USER_INDEX_STATE.lock().map_err(|_| UserError::LockError)?;

    match user_index_state.get_users_canister_principal(user_id) {
        Some(user_id) => Ok(*user_id),
        None => Err(UserError::UserNotFound),
    }
}

#[ic_cdk::update]
async fn update_user(
    user_canister_principal_id: Principal,
    user_name: Option<String>,
    balance: Option<u64>,
    address: Option<String>,
    principal_id: Principal,
    wallet_principal_id: Option<String>,
    avatar: Option<UserAvatar>,
) -> Result<User, UserError> {
    validate_caller(vec![principal_id]);
    handle_cycle_check().await?;

    let res = update_user_wrapper(
        user_canister_principal_id,
        user_name,
        balance,
        address,
        principal_id,
        wallet_principal_id,
        avatar,
    )
    .await;

    res
}

#[ic_cdk::update]
async fn get_user(user_id: Principal) -> Result<User, UserError> {
    handle_cycle_check().await?;
    let user_canister_principal_id = {
        let user_index_state = USER_INDEX_STATE.lock().map_err(|_| UserError::LockError)?;
        *user_index_state
            .get_users_canister_principal(user_id)
            .ok_or(UserError::UserNotFound)?
    };
    get_user_wrapper(user_canister_principal_id, user_id).await
}

#[ic_cdk::update]
async fn top_up_user_canister(canister_id: Principal, amount: u128) -> Result<(), UserError> {
    handle_cycle_check().await?;
    top_up_canister(canister_id, amount).await?;
    Ok(())
}

#[ic_cdk::update]
async fn monitor_and_top_up_user_canisters() -> Result<(), UserError> {
    handle_cycle_check().await?;
    let user_canisters = {
        let user_index_state = USER_INDEX_STATE.lock().map_err(|_| UserError::LockError)?;
        user_index_state.get_user_canisters()
    };

    monitor_and_top_up_canisters(user_canisters).await?;
    Ok(())
}

#[ic_cdk::query]
fn get_user_canisters() -> Result<Vec<Principal>, UserError> {
    let user_index_state = USER_INDEX_STATE.lock().map_err(|_| UserError::LockError)?;
    Ok(user_index_state.get_user_canisters())
}

#[ic_cdk::query]
fn get_number_of_registered_users() -> Result<usize, UserError> {
    let user_index_state = USER_INDEX_STATE.lock().map_err(|_| UserError::LockError)?;
    Ok(user_index_state.user_count())
}

#[ic_cdk::query]
async fn get_user_canisters_cycles() -> Result<Vec<(Principal, Nat)>, UserError> {
    let user_canisters = {
        let user_index_state = USER_INDEX_STATE.lock().map_err(|_| UserError::LockError)?;
        user_index_state.get_user_canisters()
    };

    let balances = get_cycle_balances(user_canisters).await;
    Ok(balances)
}

#[ic_cdk::query]
async fn get_user_canister_cycles(user_canister: Principal) -> Result<Nat, UserError> {
    let balances = get_cycle_balances(vec![user_canister]).await;
    Ok(balances[0].1.clone())
}

const CYCLES_TOP_UP_AMOUNT: u128 = 750_000_000_000;

#[ic_cdk::update]
async fn request_cycles() -> Result<(), UserError> {
    let cycles = ic_cdk::api::canister_cycle_balance();
    let caller = ic_cdk::api::msg_caller();
    if cycles < CYCLES_TOP_UP_AMOUNT {
        return Err(UserError::ManagementCanisterError(
            CanisterManagementError::InsufficientCycles,
        ));
    }

    transfer_cycles(CYCLES_TOP_UP_AMOUNT, caller).await
}

async fn transfer_cycles(cycles_amount: u128, caller: Principal) -> Result<(), UserError> {
    {
        let user_index_state = USER_INDEX_STATE.lock().map_err(|_| UserError::LockError)?;
        if !user_index_state.canister_user_count.contains_key(&caller) {
            return Err(UserError::ManagementCanisterError(
                CanisterManagementError::Transfer(format!(
                    "Caller is not a valid destination: {}",
                    caller
                )),
            ));
        }
    }

    top_up_canister(caller, cycles_amount).await?;
    Ok(())
}

fn safely_get_leaderboard_page(
    leaderboard: &[(Principal, u64)],
    page: u64,
    page_size: u64,
) -> Result<Vec<(Principal, u64)>, UserError> {
    let start = page as usize * page_size as usize;
    let end = start + page_size as usize;
    if start >= leaderboard.len() {
        return Ok(vec![]);
    }
    if end > leaderboard.len() {
        return Ok(leaderboard[start..].to_vec());
    }
    Ok(leaderboard[start..end].to_vec())
}

#[ic_cdk::update]
async fn get_experience_points_leaderboard(
    page: u64,
    page_size: u64,
) -> Result<Vec<(Principal, u64)>, UserError> {
    {
        if let Some(leaderboard_cache_timestamp) = *LEADERBOARD_CACHE_TIMESTAMP
            .lock()
            .map_err(|_| UserError::LockError)?
        {
            if ic_cdk::api::time() - leaderboard_cache_timestamp < 3_600_000_000_000 {
                // 1 hour
                let leaderboard_cache =
                    LEADERBOARD_CACHE.lock().map_err(|_| UserError::LockError)?;
                if let Some(leaderboard) = &*leaderboard_cache {
                    return safely_get_leaderboard_page(leaderboard, page, page_size);
                }
            }
        }
    }

    let user_index_state = USER_INDEX_STATE
        .lock()
        .map_err(|_| UserError::LockError)?
        .clone();
    let leaderboard = user_index_state.get_experience_points_leaderboard().await?;

    let mut leaderboard_cache = LEADERBOARD_CACHE.lock().map_err(|_| UserError::LockError)?;
    let mut leaderboard_cache_timestamp = LEADERBOARD_CACHE_TIMESTAMP
        .lock()
        .map_err(|_| UserError::LockError)?;
    leaderboard_cache.replace(leaderboard.clone());
    leaderboard_cache_timestamp.replace(ic_cdk::api::time());

    safely_get_leaderboard_page(&leaderboard, page, page_size)
}

#[ic_cdk::update]
async fn get_verified_experience_points_leaderboard(
    page: u64,
    page_size: u64,
) -> Result<Vec<(Principal, u64)>, UserError> {
    {
        if let Some(leaderboard_cache_timestamp) = *VERIFIED_LEADERBOARD_CACHE_TIMESTAMP
            .lock()
            .map_err(|_| UserError::LockError)?
        {
            if ic_cdk::api::time() - leaderboard_cache_timestamp < 3_600_000_000_000 {
                // 1 hour
                let leaderboard_cache = VERIFIED_LEADERBOARD_CACHE
                    .lock()
                    .map_err(|_| UserError::LockError)?;
                if let Some(leaderboard) = &*leaderboard_cache {
                    return safely_get_leaderboard_page(leaderboard, page, page_size);
                }
            }
        }
    }

    let user_index_state = USER_INDEX_STATE
        .lock()
        .map_err(|_| UserError::LockError)?
        .clone();
    let leaderboard = user_index_state.get_verified_experience_points_leaderboard().await?;

    let mut leaderboard_cache = VERIFIED_LEADERBOARD_CACHE
        .lock()
        .map_err(|_| UserError::LockError)?;
    let mut leaderboard_cache_timestamp = VERIFIED_LEADERBOARD_CACHE_TIMESTAMP
        .lock()
        .map_err(|_| UserError::LockError)?;
    leaderboard_cache.replace(leaderboard.clone());
    leaderboard_cache_timestamp.replace(ic_cdk::api::time());

    safely_get_leaderboard_page(&leaderboard, page, page_size)
}

#[ic_cdk::update]
async fn get_verified_experience_points_leaderboard_length() -> Result<usize, UserError> {
    {
        if let Some(leaderboard_cache_timestamp) = *VERIFIED_LEADERBOARD_CACHE_TIMESTAMP
            .lock()
            .map_err(|_| UserError::LockError)?
        {
            if ic_cdk::api::time() - leaderboard_cache_timestamp < 3_600_000_000_000 {
                // 1 hour
                let leaderboard_cache = VERIFIED_LEADERBOARD_CACHE
                    .lock()
                    .map_err(|_| UserError::LockError)?;
                if let Some(leaderboard) = &*leaderboard_cache {
                    return Ok(leaderboard.len());
                }
            }
        }
    }

    let user_index_state = USER_INDEX_STATE
        .lock()
        .map_err(|_| UserError::LockError)?
        .clone();
    let leaderboard = user_index_state.get_verified_experience_points_leaderboard().await?;

    let mut leaderboard_cache = VERIFIED_LEADERBOARD_CACHE
        .lock()
        .map_err(|_| UserError::LockError)?;
    let mut leaderboard_cache_timestamp = VERIFIED_LEADERBOARD_CACHE_TIMESTAMP
        .lock()
        .map_err(|_| UserError::LockError)?;
    leaderboard_cache.replace(leaderboard.clone());
    leaderboard_cache_timestamp.replace(ic_cdk::api::time());
    Ok(leaderboard.len())
}

#[ic_cdk::update]
async fn get_pure_poker_experience_points(
    page: u64,
    page_size: u64,
) -> Result<Vec<(Principal, u64)>, UserError> {
    {
        if let Some(leaderboard_cache_timestamp) = *PURE_POKER_LEADERBOARD_CACHE_TIMESTAMP
            .lock()
            .map_err(|_| UserError::LockError)?
        {
            if ic_cdk::api::time() - leaderboard_cache_timestamp < 3_600_000_000_000 {
                // 1 hour
                let leaderboard_cache = PURE_POKER_LEADERBOARD_CACHE
                    .lock()
                    .map_err(|_| UserError::LockError)?;
                if let Some(leaderboard) = &*leaderboard_cache {
                    return safely_get_leaderboard_page(leaderboard, page, page_size);
                }
            }
        }
    }

    let user_index_state = USER_INDEX_STATE
        .lock()
        .map_err(|_| UserError::LockError)?
        .clone();
    let leaderboard = user_index_state
        .get_pure_poker_experience_points_leaderboard()
        .await?;

    let mut leaderboard_cache = PURE_POKER_LEADERBOARD_CACHE
        .lock()
        .map_err(|_| UserError::LockError)?;
    let mut leaderboard_cache_timestamp = PURE_POKER_LEADERBOARD_CACHE_TIMESTAMP
        .lock()
        .map_err(|_| UserError::LockError)?;
    leaderboard_cache.replace(leaderboard.clone());
    leaderboard_cache_timestamp.replace(ic_cdk::api::time());

    safely_get_leaderboard_page(&leaderboard, page, page_size)
}

#[ic_cdk::update]
async fn get_verified_pure_poker_experience_points(
    page: u64,
    page_size: u64,
) -> Result<Vec<(Principal, u64)>, UserError> {
    {
        if let Some(leaderboard_cache_timestamp) = *VERIFIED_PURE_POKER_LEADERBOARD_CACHE_TIMESTAMP
            .lock()
            .map_err(|_| UserError::LockError)?
        {
            if ic_cdk::api::time() - leaderboard_cache_timestamp < 3_600_000_000_000 {
                // 1 hour
                let leaderboard_cache = VERIFIED_PURE_POKER_LEADERBOARD_CACHE
                    .lock()
                    .map_err(|_| UserError::LockError)?;
                if let Some(leaderboard) = &*leaderboard_cache {
                    return safely_get_leaderboard_page(leaderboard, page, page_size);
                }
            }
        }
    }

    let user_index_state = USER_INDEX_STATE
        .lock()
        .map_err(|_| UserError::LockError)?
        .clone();
    let leaderboard = user_index_state
        .get_verified_pure_poker_experience_points_leaderboard()
        .await?;

    let mut leaderboard_cache = VERIFIED_PURE_POKER_LEADERBOARD_CACHE
        .lock()
        .map_err(|_| UserError::LockError)?;
    let mut leaderboard_cache_timestamp = VERIFIED_PURE_POKER_LEADERBOARD_CACHE_TIMESTAMP
        .lock()
        .map_err(|_| UserError::LockError)?;
    leaderboard_cache.replace(leaderboard.clone());
    leaderboard_cache_timestamp.replace(ic_cdk::api::time());

    safely_get_leaderboard_page(&leaderboard, page, page_size)
}

#[ic_cdk::query]
fn get_leaderboard_length() -> Result<usize, UserError> {
    let user_index_state = USER_INDEX_STATE.lock().map_err(|_| UserError::LockError)?;
    Ok(user_index_state.user_to_canister.len())
}

#[ic_cdk::update]
async fn upgrade_all_user_canisters() -> Result<Vec<(Principal, CanisterManagementError)>, UserError>
{
    // Validate caller permissions
    let caller = ic_cdk::api::msg_caller();
    if !CONTROLLER_PRINCIPALS.contains(&caller) {
        return Err(UserError::AuthorizationError);
    }

    handle_cycle_check().await?;

    const BATCH_SIZE: usize = 30; // Process 30 users at a time, same as in get_experience_points_leaderboard

    let users: Vec<Principal> = {
        let user_index_state = USER_INDEX_STATE.lock().map_err(|_| UserError::LockError)?;
        user_index_state.get_users_canisters()
    };

    let wasm_module = USER_CANISTER_WASM.to_vec();
    let mut failed_upgrades = Vec::new();

    // Process users in batches
    for chunk in users.chunks(BATCH_SIZE) {
        let futures: Vec<_> = chunk
            .iter()
            .map(|&user_canister| {
                let wasm_clone = wasm_module.clone();
                async move {
                    match upgrade_wasm_code(user_canister, wasm_clone).await {
                        Ok(_) => {
                            ic_cdk::println!("Successfully upgraded canister {}", user_canister);
                            Ok(user_canister)
                        }
                        Err(e) => {
                            ic_cdk::println!(
                                "Failed to upgrade canister {}: {:?}",
                                user_canister,
                                e
                            );
                            Err((user_canister, e))
                        }
                    }
                }
            })
            .collect();

        // Execute batch of upgrades
        let batch_results = futures::future::join_all(futures).await;
        for result in batch_results {
            if let Err((canister_id, error)) = result {
                failed_upgrades.push((canister_id, error));
            }
        }
    }

    Ok(failed_upgrades)
}

#[ic_cdk::update]
async fn upgrade_user_canister(
    user_canister: Principal,
) -> Result<(), UserError> {
    // Validate caller permissions
    let caller = ic_cdk::api::msg_caller();
    if !CONTROLLER_PRINCIPALS.contains(&caller) {
        return Err(UserError::AuthorizationError);
    }

    handle_cycle_check().await?;

    let wasm_module = USER_CANISTER_WASM.to_vec();
    upgrade_wasm_code(user_canister, wasm_module).await?;
    Ok(())
}

#[ic_cdk::update]
async fn get_experience_points_position(
    user_principal: Principal,
) -> Result<Option<u64>, UserError> {
    handle_cycle_check().await?;
    get_position_in_leaderboard(user_principal, false).await
}

#[ic_cdk::update]
async fn get_pure_poker_position(user_principal: Principal) -> Result<Option<u64>, UserError> {
    handle_cycle_check().await?;
    get_position_in_leaderboard(user_principal, true).await
}

ic_cdk::export_candid!();
