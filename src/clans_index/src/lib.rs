use authentication::validate_caller;
use candid::{Nat, Principal};
use canister_functions::{
    create_canister_wrapper,
    cycle::{check_and_top_up_canister, monitor_and_top_up_canisters, top_up_canister},
    install_wasm_code, stop_and_delete_canister,
};
use clan::{subscriptions::SubscriptionTierId, tags::ClanTag, Clan, ClanId, CreateClanRequest};
use currency::{types::currency_manager::CurrencyManager, Currency};
use errors::{
    canister_management_error::CanisterManagementError, clan_index_error::ClanIndexError,
};
use ic_cdk::management_canister::{canister_status, CanisterStatusArgs};
use intercanister_call_wrappers::clan_canister::{create_clan_wrapper, get_clan_wrapper, join_clan_wrapper, leave_clan_wrapper, upgrade_subscription_wrapper};
use lazy_static::lazy_static;
use user::user::{UsersCanisterId, WalletPrincipalId};
use std::sync::Mutex;

pub mod clans_index;
pub mod memory;
pub mod tags;

use clans_index::{ClanIndex, ClanSearchFilters};

const MINIMUM_CYCLE_THRESHOLD: u128 = 2_000_000_000_000;
const SINGLE_CLAN_CYCLE_START_AMOUNT: u128 = 3_000_000_000_000;

async fn handle_cycle_check() -> Result<(), ClanIndexError> {
    let id = ic_cdk::api::canister_self();
    let cycle_dispenser_canister_id =
        if id == Principal::from_text("zclan-index-prod-canister-id").unwrap() {
            *CYCLE_DISPENSER_CANISTER_PROD
        } else if id == Principal::from_text("zclan-index-test-canister-id").unwrap() {
            *CYCLE_DISPENSER_CANISTER_TEST
        } else if id == Principal::from_text("zclan-index-dev-canister-id").unwrap() {
            *CYCLE_DISPENSER_CANISTER_DEV
        } else {
            return Ok(());
        };
    check_and_top_up_canister(id, cycle_dispenser_canister_id, MINIMUM_CYCLE_THRESHOLD).await?;
    Ok(())
}

lazy_static! {
    static ref STATE: Mutex<ClanIndex> = Mutex::new(ClanIndex::new());
    static ref CURRENCY_MANAGER: Mutex<CurrencyManager> = Mutex::new(CurrencyManager::new());
    static ref CYCLE_DISPENSER_CANISTER_PROD: Principal =
        Principal::from_text("zuv6g-yaaaa-aaaam-qbeza-cai").unwrap();
    static ref CYCLE_DISPENSER_CANISTER_TEST: Principal =
        Principal::from_text("ev34d-5yaaa-aaaah-qdska-cai").unwrap();
    static ref CYCLE_DISPENSER_CANISTER_DEV: Principal =
        Principal::from_text("tz2ag-zx777-77776-aaabq-cai").unwrap();
    static ref CONTROLLER_PRINCIPALS: Vec<Principal> = vec![
        Principal::from_text("km7qz-4bai4-e5ptx-hgrck-z3web-ameqg-ksxcf-u7wbr-t5fna-i7bqp-hqe")
            .unwrap(),
        Principal::from_text("uyxh5-bi3za-gxbfs-op3gj-ere73-a6jhv-5jky3-zawef-b5r2s-k26un-sae")
            .unwrap(),
    ];
    static ref CLAN_CANISTER_WASM: &'static [u8] =
        include_bytes!("../../../target/wasm32-unknown-unknown/release/clans_canister.wasm");
}

#[ic_cdk::init]
fn init() {
    let id = ic_cdk::api::canister_self();
    ic_cdk::println!("Clan index canister {id} initialized");
}

#[ic_cdk::query]
fn ping() -> String {
    "Ok".to_string()
}

#[ic_cdk::update]
async fn create_clan(
    request: CreateClanRequest,
    creator: WalletPrincipalId,
    creator_canister: UsersCanisterId,
) -> Result<Principal, ClanIndexError> {
    handle_cycle_check().await?;

    // Validate the request
    if request.name.is_empty() || request.name.len() > 50 {
        return Err(ClanIndexError::InvalidRequest(
            "Clan name must be 1-50 characters".to_string(),
        ));
    }

    if request.tag.is_empty() || request.tag.len() > 10 {
        return Err(ClanIndexError::InvalidRequest(
            "Clan tag must be 1-10 characters".to_string(),
        ));
    }

    // Create clan canister
    let clan_canister = create_clan_canister().await?;

    // Create the clan
    let clan = create_clan_wrapper(
        ClanId(clan_canister),
        request.clone(),
        creator,
        creator_canister
    ).await?;

    // Store in index
    {
        let mut state = STATE.lock().map_err(|_| ClanIndexError::LockError)?;
        state.add_clan(clan.clone())?;
    }

    Ok(clan_canister)
}

#[ic_cdk::update]
async fn get_clan(clan_id: ClanId) -> Result<Clan, ClanIndexError> {
    handle_cycle_check().await?;
    let clan = get_clan_wrapper(clan_id).await?;
    Ok(clan)
}

#[ic_cdk::query]
fn get_all_clans() -> Result<Vec<Clan>, ClanIndexError> {
    let state = STATE.lock().map_err(|_| ClanIndexError::LockError)?;
    Ok(state.get_all_clans())
}

#[ic_cdk::query]
fn search_clans(
    filters: Option<ClanSearchFilters>,
    page: u64,
    page_size: u64,
) -> Result<Vec<Clan>, ClanIndexError> {
    let state = STATE.lock().map_err(|_| ClanIndexError::LockError)?;
    Ok(state.search_clans(filters, page, page_size))
}

#[ic_cdk::query]
fn get_clans_by_tag(tag: ClanTag) -> Result<Vec<Clan>, ClanIndexError> {
    let state = STATE.lock().map_err(|_| ClanIndexError::LockError)?;
    Ok(state.get_clans_by_tag(&tag))
}

#[ic_cdk::query]
fn get_clans_by_currency(currency: Currency) -> Result<Vec<Clan>, ClanIndexError> {
    let state = STATE.lock().map_err(|_| ClanIndexError::LockError)?;
    Ok(state.get_clans_by_currency(&currency))
}

#[ic_cdk::query]
fn get_user_clans(user_principal: WalletPrincipalId) -> Result<Vec<Clan>, ClanIndexError> {
    let state = STATE.lock().map_err(|_| ClanIndexError::LockError)?;
    Ok(state.get_user_clans(&user_principal))
}

#[ic_cdk::update]
async fn join_clan(
    clan_id: ClanId,
    user_principal: WalletPrincipalId,
    users_canister: UsersCanisterId,
    joining_fee_paid: u64,
) -> Result<(), ClanIndexError> {
    handle_cycle_check().await?;
    
    join_clan_wrapper(clan_id, users_canister, user_principal, joining_fee_paid).await?;
    
    // Update local index
    {
        let mut state = STATE.lock().map_err(|_| ClanIndexError::LockError)?;
        state.add_member_to_clan(clan_id, user_principal)?;
    }
    
    Ok(())
}

#[ic_cdk::update]
async fn leave_clan(
    clan_id: ClanId,
    user_principal: WalletPrincipalId,
) -> Result<(), ClanIndexError> {
    handle_cycle_check().await?;
    
    leave_clan_wrapper(clan_id, user_principal).await?;
    
    // Update local index
    {
        let mut state = STATE.lock().map_err(|_| ClanIndexError::LockError)?;
        state.remove_member_from_clan(clan_id, user_principal)?;
    }
    
    Ok(())
}

#[ic_cdk::update]
async fn upgrade_subscription(
    clan_id: ClanId,
    user_principal: WalletPrincipalId,
    new_tier: SubscriptionTierId,
    paid_amount: u64,
    months: u32,
) -> Result<(), ClanIndexError> {
    handle_cycle_check().await?;
    
    upgrade_subscription_wrapper(
        clan_id,
        user_principal,
        new_tier,
        paid_amount,
        months,
    ).await?;
    
    Ok(())
}

#[ic_cdk::query]
fn get_clan_count() -> Result<usize, ClanIndexError> {
    let state = STATE.lock().map_err(|_| ClanIndexError::LockError)?;
    Ok(state.get_clan_count())
}

#[ic_cdk::query]
fn get_total_members() -> Result<usize, ClanIndexError> {
    let state = STATE.lock().map_err(|_| ClanIndexError::LockError)?;
    Ok(state.get_total_members())
}

#[ic_cdk::query]
fn get_clans_by_member_count(min_members: usize, max_members: Option<usize>) -> Result<Vec<Clan>, ClanIndexError> {
    let state = STATE.lock().map_err(|_| ClanIndexError::LockError)?;
    Ok(state.get_clans_by_member_count(min_members, max_members))
}

#[ic_cdk::query]
fn get_popular_clans(limit: usize) -> Result<Vec<Clan>, ClanIndexError> {
    let state = STATE.lock().map_err(|_| ClanIndexError::LockError)?;
    Ok(state.get_popular_clans(limit))
}

#[ic_cdk::update]
async fn delete_clan(clan_id: ClanId) -> Result<(), ClanIndexError> {
    handle_cycle_check().await?;
    let controllers = CONTROLLER_PRINCIPALS.clone();
    validate_caller(controllers);

    // Remove from index first
    {
        let mut state = STATE.lock().map_err(|_| ClanIndexError::LockError)?;
        state.remove_clan(clan_id)?;
    }

    // Delete the canister
    stop_and_delete_canister(clan_id.0).await?;
    
    Ok(())
}

#[ic_cdk::update]
async fn top_up_clan_canister(
    canister_id: Principal,
    amount: u128,
) -> Result<(), ClanIndexError> {
    handle_cycle_check().await?;
    top_up_canister(canister_id, amount).await?;
    Ok(())
}

#[ic_cdk::update]
async fn monitor_and_top_up_clan_canisters() -> Result<(), ClanIndexError> {
    handle_cycle_check().await?;
    
    let clan_canisters = {
        let state = STATE.lock().map_err(|_| ClanIndexError::LockError)?;
        state.get_all_clan_ids().iter().map(|id| id.0.clone()).collect::<Vec<Principal>>()
    };
    
    monitor_and_top_up_canisters(clan_canisters).await?;
    Ok(())
}

async fn create_clan_canister() -> Result<Principal, ClanIndexError> {
    handle_cycle_check().await?;
    let controllers = CONTROLLER_PRINCIPALS.clone();
    let wasm_module = CLAN_CANISTER_WASM.to_vec();
    let cycle_amount = SINGLE_CLAN_CYCLE_START_AMOUNT;
    let clan_canister_principal = create_canister_wrapper(controllers, Some(cycle_amount)).await?;
    install_wasm_code(clan_canister_principal, wasm_module).await?;
    Ok(clan_canister_principal)
}

const CYCLES_TOP_UP_AMOUNT: u128 = 750_000_000_000;

#[ic_cdk::update]
async fn request_cycles() -> Result<(), ClanIndexError> {
    let cycles = ic_cdk::api::canister_cycle_balance();
    let caller = ic_cdk::api::msg_caller();
    if cycles < CYCLES_TOP_UP_AMOUNT {
        return Err(ClanIndexError::ManagementCanisterError(
            CanisterManagementError::InsufficientCycles,
        ));
    }

    transfer_cycles(CYCLES_TOP_UP_AMOUNT, caller).await
}

async fn transfer_cycles(cycles_amount: u128, caller: Principal) -> Result<(), ClanIndexError> {
    {
        let state = STATE.lock().map_err(|_| ClanIndexError::LockError)?;
        if !state.is_valid_clan_canister(&ClanId(caller)) {
            return Err(ClanIndexError::ManagementCanisterError(
                CanisterManagementError::Transfer(format!(
                    "Caller is not a valid clan canister: {}",
                    caller
                )),
            ));
        }
    }

    top_up_canister(caller, cycles_amount).await?;
    Ok(())
}

#[ic_cdk::update]
async fn upgrade_all_clan_canisters() -> Result<Vec<(Principal, CanisterManagementError)>, ClanIndexError> {
    // Validate caller permissions
    let caller = ic_cdk::api::msg_caller();
    if !CONTROLLER_PRINCIPALS.contains(&caller) {
        return Err(ClanIndexError::NotAuthorized);
    }

    handle_cycle_check().await?;

    const BATCH_SIZE: usize = 30; // Process 30 clans at a time

    let clans: Vec<Principal> = {
        let state = STATE.lock().map_err(|_| ClanIndexError::LockError)?;
        state.get_all_clan_ids().iter()
            .map(|id| id.0.clone())
            .collect()
    };

    let wasm_module = CLAN_CANISTER_WASM.to_vec();
    let mut failed_upgrades = Vec::new();

    // Process clans in batches
    for chunk in clans.chunks(BATCH_SIZE) {
        let futures: Vec<_> = chunk
            .iter()
            .map(|&clan_canister| {
                let wasm_clone = wasm_module.clone();
                async move {
                    match canister_functions::upgrade_wasm_code(clan_canister, wasm_clone).await {
                        Ok(_) => {
                            ic_cdk::println!("Successfully upgraded clan canister {}", clan_canister);
                            Ok(clan_canister)
                        }
                        Err(e) => {
                            ic_cdk::println!(
                                "Failed to upgrade clan canister {}: {:?}",
                                clan_canister,
                                e
                            );
                            Err((clan_canister, e))
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
async fn upgrade_clan_canister(clan_canister: Principal) -> Result<(), ClanIndexError> {
    // Validate caller permissions
    let caller = ic_cdk::api::msg_caller();
    if !CONTROLLER_PRINCIPALS.contains(&caller) {
        return Err(ClanIndexError::NotAuthorized);
    }

    handle_cycle_check().await?;

    let wasm_module = CLAN_CANISTER_WASM.to_vec();
    canister_functions::upgrade_wasm_code(clan_canister, wasm_module).await?;
    Ok(())
}

#[ic_cdk::update]
async fn get_canister_status_formatted() -> Result<String, ClanIndexError> {
    // Validate caller is a controller
    let controllers = (*CONTROLLER_PRINCIPALS).clone();
    validate_caller(controllers);

    handle_cycle_check().await?;

    // Call the management canister to get status
    let canister_status_arg = CanisterStatusArgs {
        canister_id: ic_cdk::api::canister_self(),
    };

    let status_response = canister_status(&canister_status_arg).await.map_err(|e| {
        ClanIndexError::CanisterCallError(format!("Failed to get canister status: {:?}", e))
    })?;

    // Format the status into a readable string
    let formatted_status = format!(
        "📊 Canister Status Report
════════════════════════════════════════════════════════════════
🆔 Canister ID: {}
🔄 Status: {:?}
💾 Memory Size: {} bytes ({:.2} MB)
⚡ Cycles: {} ({:.2} T cycles)
🎛️  Controllers: {}
📈 Compute Allocation: {}
🧠 Memory Allocation: {} bytes
🧊 Freezing Threshold: {}
📊 Reserved Cycles Limit: {}
════════════════════════════════════════════════════════════════",
        ic_cdk::api::canister_self().to_text(),
        status_response.status,
        status_response.memory_size,
        status_response.memory_size.clone() / Nat::from(1_048_576_u64), // Convert to MB
        status_response.cycles,
        status_response.cycles.clone() / Nat::from(1_000_000_000_000_u64), // Convert to T cycles
        status_response
            .settings
            .controllers
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", "),
        status_response.settings.compute_allocation,
        status_response.settings.memory_allocation,
        status_response.settings.freezing_threshold,
        status_response.settings.reserved_cycles_limit
    );

    ic_cdk::println!("{}", formatted_status);
    Ok(formatted_status)
}

ic_cdk::export_candid!();
