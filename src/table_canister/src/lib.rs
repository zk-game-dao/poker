use std::{collections::HashMap, sync::Mutex};

use authentication::validate_caller;
use candid::Principal;
use canister_functions::{
    inter_canister_call_wrappers::{get_user, get_users_canister_principal_by_id_wrapper, remove_users_active_table},
    rake_constants,
    rake_stats::RakeStats,
};
use chat::{ChatHistory, ChatMessage, ChatMessageType};
use currency::{state::TransactionState, types::currency_manager::CurrencyManager};
use errors::{
    chat_error::ChatError, game_error::GameError, log_store_error::LogStoreError,
    table_error::TableError, user_error::UserError,
};
use ic_cdk::api::management_canister::main::CanisterIdRecord;
use lazy_static::lazy_static;
use table::{
    poker::{
        core::{Card, Rank},
        game::{
            table_functions::{
                action_log::ActionType,
                ante::AnteType,
                table::{Table, TableConfig, TableType},
                types::{BetType, CurrencyType, DealStage, Notification, PlayerAction, SeatStatus},
            },
            types::{PublicTable, QueueItem, TableStatus},
            utils::rank_hand,
        },
    },
    types::ReturnResult,
    utils::is_table_game_ongoing,
};
use tournaments::tournaments::types::UserTournamentAction;
use user::user::{TransactionType, User, REFERRAL_PERIOD};
use utils::{
    get_user_index_principal, handle_cycle_check, handle_table_validity_check, log_user_transaction, update_player_count_tournament, update_table_player_count
};

pub mod canister_geek;
mod memory;
pub mod utils;

// Define a global instance of GameState wrapped in a Mutex for safe concurrent access.
lazy_static! {
    static ref TABLE: Mutex<Option<Table>> = Mutex::new(None);
    static ref BACKEND_PRINCIPAL: Mutex<Option<Principal>> = Mutex::new(None);
    static ref TRANSACTION_STATE: Mutex<TransactionState> = Mutex::new(TransactionState::new());
    static ref CURRENCY_MANAGER: Mutex<Option<CurrencyManager>> = Mutex::new(None);
    static ref RAKE_WALLET_ACCOUNT_ID: String = rake_constants::RAKE_WALLET_ACCOUNT_ID.to_string();
    static ref RAKE_WALLET_ADDRESS_PRINCIPAL: Principal =
        Principal::from_text(rake_constants::RAKE_WALLET_ADDRESS_PRINCIPAL).unwrap();
    static ref RAKE_STATS: Mutex<RakeStats> = Mutex::new(RakeStats::new());
    static ref CHAT_HISTORY: Mutex<ChatHistory> = Mutex::new(ChatHistory::new(1000));

    static ref CONTROLLER_PRINCIPALS: Vec<Principal> = vec![
        Principal::from_text("km7qz-4bai4-e5ptx-hgrck-z3web-ameqg-ksxcf-u7wbr-t5fna-i7bqp-hqe").unwrap(),
        Principal::from_text("uyxh5-bi3za-gxbfs-op3gj-ere73-a6jhv-5jky3-zawef-b5r2s-k26un-sae").unwrap(),
    ];
}

#[ic_cdk::init]
fn init() {
    let principal = ic_cdk::api::id();
    ic_cdk::print(format!("Table canister {} initialized", principal));
}

#[ic_cdk::update]
async fn create_table(config: TableConfig, bytes: Vec<u8>) -> Result<PublicTable, TableError> {
    let table = {
        let mut backend_principal = BACKEND_PRINCIPAL
            .lock()
            .map_err(|_| TableError::LockError)?;
        
        if let Some(backend_principal) = *backend_principal {
            validate_caller(vec![backend_principal]);
        }
        *backend_principal = Some(ic_cdk::api::caller());

        let mut table_state = TABLE.lock().map_err(|_| TableError::LockError)?;

        let table = Table::new(ic_cdk::api::id(), config.clone(), bytes);

        *table_state = Some(table.clone());
        table.clone()
    };

    let currency_manager = match &config.currency_type {
        CurrencyType::Real(currency) => {
            let mut currency_manager = CurrencyManager::new();
            currency_manager.add_currency(*currency).await?;
            Some(currency_manager)
        }
        CurrencyType::Fake => None,
    };

    *CURRENCY_MANAGER.lock().map_err(|_| TableError::LockError)? = currency_manager;
    Ok(table.into())
}

#[ic_cdk::query]
fn ping() -> String {
    "Ok".to_string()
}

#[ic_cdk::query]
fn get_table() -> Result<PublicTable, TableError> {
    let mut table = TABLE.lock().map_err(|_| TableError::LockError)?;
    let table = table.as_mut().ok_or(TableError::TableNotFound)?;
    let caller = ic_cdk::api::caller();

    if table.deal_stage != DealStage::Showdown {
        table.hide_cards(caller).map_err(|e| e.into_inner())?;
    }
    Ok(table.into())
}

#[ic_cdk::query]
fn is_table_full() -> Result<bool, TableError> {
    let table = TABLE.lock().map_err(|_| TableError::LockError)?;
    let table = table.as_ref().ok_or(TableError::TableNotFound)?;
    Ok(table.is_full())
}

#[ic_cdk::query]
fn is_game_ongoing() -> Result<bool, TableError> {
    let table = TABLE.lock().map_err(|_| TableError::LockError)?;
    let table = table.as_ref().ok_or(TableError::TableNotFound)?;
    Ok(is_table_game_ongoing(table))
}

#[allow(dependency_on_unit_never_type_fallback)]
#[ic_cdk::update]
async fn join_table(
    users_canister_principal: Principal,
    user_id: Principal,
    seat_index: Option<u64>, // javascript can't send u8
    deposit_amount: u64,
    player_sitting_out: bool,
) -> Result<PublicTable, TableError> {
    handle_cycle_check();
    if deposit_amount == 0 {
        return Err(TableError::InvalidRequest(
            "Deposit amount cannot be 0".to_string(),
        ));
    }

    let table = {
        let table = TABLE.lock().map_err(|_| TableError::LockError)?;
        table.as_ref().ok_or(TableError::TableNotFound)?.clone()
    };

    if table.big_blind > deposit_amount {
        return Err(TableError::InsufficientFunds);
    }

    let seat_index = match seat_index {
        Some(seat_index) => seat_index as u8,
        None => table
            .get_free_seat_index()
            .ok_or(TableError::InvalidRequest("Table is full.".to_string()))?,
    };

    {
        let user = get_user(users_canister_principal, user_id).await?;
        if let Some(require_proof_of_humanity) = table.config.require_proof_of_humanity {
            if require_proof_of_humanity && !user.is_verified.unwrap_or(false) {
                return Err(TableError::UserNotVerified);
            }
        }
    }

    match table.config.currency_type {
        CurrencyType::Real(currency) => {
            let currency_manager = {
                let currency_manager =
                    CURRENCY_MANAGER.lock().map_err(|_| TableError::LockError)?;
                currency_manager
                    .as_ref()
                    .ok_or(TableError::StateNotInitialized)?
                    .clone()
            };

            currency_manager
                .validate_allowance(&currency, user_id, deposit_amount)
                .await?;
        }
        CurrencyType::Fake => {}
    }

    let currency_type = match table.config.currency_type {
        CurrencyType::Real(currency) => currency.to_string(),
        CurrencyType::Fake => "Fake".to_string(),
    };

    let (user,): (Result<User, UserError>,) = ic_cdk::call(
        users_canister_principal,
        "add_active_table",
        (ic_cdk::api::id(), user_id),
    )
    .await
    .map_err(|e| TableError::CanisterCallError(format!("{:?} {}", e.0, e.1)))?;
    let mut user = user?;

    user.balance = deposit_amount;

    if table.users.users.contains_key(&user_id) || table.is_user_in_table(user_id) {
        return Err(TableError::UserAlreadyInGame);
    }

    let (ret, mut table_state) = {
        let mut table_state = TABLE.lock().map_err(|_| TableError::LockError)?;
        let table_state = table_state.as_mut().ok_or(TableError::TableNotFound)?;
        let ret = table_state.add_user(user.clone(), seat_index, player_sitting_out);
        (ret.map_err(|e| e.into_inner()), table_state.clone())
    };

    if let Err(e) = ret {
        ic_cdk::println!("Error adding user to table: {:?}", e);
        remove_users_active_table(users_canister_principal, user_id).await?;
        return Err(e.into());
    }

    match table.config.currency_type {
        CurrencyType::Real(currency) => {
            let mut transaction_state = TRANSACTION_STATE
                .lock()
                .map_err(|_| TableError::LockError)?
                .clone();
            let currency_manager = {
                let currency_manager =
                    CURRENCY_MANAGER.lock().map_err(|_| TableError::LockError)?;
                currency_manager
                    .as_ref()
                    .ok_or(TableError::StateNotInitialized)?
                    .clone()
            };
            match currency_manager
                .deposit(&mut transaction_state, &currency, user_id, deposit_amount)
                .await
            {
                Ok(_) => {
                    *TRANSACTION_STATE
                        .lock()
                        .map_err(|_| TableError::LockError)? = transaction_state;
                }
                Err(e) => {
                    ic_cdk::println!("Error depositing: {:?}", e);
                    remove_users_active_table(users_canister_principal, user_id).await?;

                    let mut table = TABLE.lock().map_err(|_| TableError::LockError)?;
                    let table = table.as_mut().ok_or(TableError::TableNotFound)?;
                    table
                        .remove_user(users_canister_principal, ActionType::Leave)
                        .map_err(|e| e.into_inner())?;
                    return Err(e.into());
                }
            }
            match log_user_transaction(
                users_canister_principal,
                user_id,
                deposit_amount,
                TransactionType::TableDeposit { table_id: table.id },
                None,
                Some(currency_type.clone()),
            )
            .await
            {
                Ok(_) => {}
                Err(e) => {
                    ic_cdk::println!("Error logging user transaction: {:?}", e);

                    remove_users_active_table(users_canister_principal, user_id).await?;
                    return Err(e);
                }
            }
        }
        CurrencyType::Fake => {}
    }

    let is_paused = table_state.config.is_paused.unwrap_or(false);

    if table_state.number_of_players() >= 2 && !table_state.is_game_ongoing() && !is_paused {
        let (_res,): (Result<(), TableError>,) =
            ic_cdk::call(ic_cdk::api::id(), "start_new_betting_round", ())
                .await
                .map_err(|e| TableError::CanisterCallError(format!("{:?} {}", e.0, e.1)))?;
    }
    let caller = ic_cdk::api::caller();
    table_state.hide_cards(caller).map_err(|e| e.into_inner())?;
    let res = match table_state.config.table_type {
        Some(TableType::Cash) | None => update_table_player_count(table_state.users.len()),
        _ => update_player_count_tournament(UserTournamentAction::Join(user_id)),
    };
    if let Err(e) = res {
        ic_cdk::println!("Error updating table player count: {:?}", e);
    }

    Ok(table_state.into())
}

#[ic_cdk::update]
async fn kick_player(
    users_canister_principal: Principal,
    user_id: Principal,
    balance: u64,
) -> Result<PublicTable, TableError> {
    handle_cycle_check();

    let mut table = {
        let mut table = TABLE.lock().map_err(|_| TableError::LockError)?;
        let table = table.as_mut().ok_or(TableError::TableNotFound)?;

        if table.has_user_left(user_id) {
            return Err(TableError::InvalidRequest(
                "User has already left the table".to_string(),
            ));
        }

        let backend_principal = BACKEND_PRINCIPAL
            .lock()
            .map_err(|_| TableError::LockError)?
            .ok_or(TableError::CanisterCallError(
                "Backend principal not found.".to_string(),
            ))?;

        validate_caller(vec![backend_principal, table.id]);

        if is_table_game_ongoing(table) {
            if table.is_players_turn(user_id) {
                if let Err(e) = table.user_fold(user_id, false) {
                    ic_cdk::println!("Error folding user: {:?}", e);
                }
            } else if let Err(e) = table.user_pre_fold(user_id) {
                ic_cdk::println!("Error pre-folding user: {:?}", e);
            }
        }

        table.clone()
    };

    let currency_type = match &table.config.currency_type.clone() {
        CurrencyType::Real(currency) => currency.to_string(),
        CurrencyType::Fake => "Fake".to_string(),
    };

    match &table.config.currency_type {
        CurrencyType::Real(currency) => {
            ic_cdk::println!("Balance: {}", balance);

            let fee = if table.config.currency_type == CurrencyType::Real(currency::Currency::BTC) {
                10
            } else {
                ic_ledger_types::DEFAULT_FEE.e8s()
            };
            if balance > fee {
                ic_cdk::println!("Withdrawing: {}", balance);
                let currency_manager = {
                    let currency_manager =
                        CURRENCY_MANAGER.lock().map_err(|_| TableError::LockError)?;
                    currency_manager
                        .as_ref()
                        .ok_or(TableError::StateNotInitialized)?
                        .clone()
                };

                currency_manager
                    .withdraw(currency, user_id, balance)
                    .await?;
                let amount = balance - fee;
                log_user_transaction(
                    users_canister_principal,
                    user_id,
                    amount,
                    TransactionType::TableWithdraw { table_id: table.id },
                    None,
                    Some(currency_type),
                )
                .await?;
            }
        }
        CurrencyType::Fake => {}
    }

    ic_cdk::spawn(async move {
        if let Err(e) = remove_users_active_table(users_canister_principal, user_id).await {
            ic_cdk::println!("Error removing active table: {}", e);
        }
    });

    {
        let mut table_state = TABLE.lock().map_err(|_| TableError::LockError)?;
        let table_state = table_state.as_mut().ok_or(TableError::TableNotFound)?;
        table_state
            .remove_user(user_id, ActionType::Leave)
            .map_err(|e| e.into_inner())?;
    }

    let caller = ic_cdk::api::caller();
    table.hide_cards(caller).map_err(|e| e.into_inner())?;
    let _ = update_table_player_count(table.users.len());

    Ok(table.into())
}

#[ic_cdk::update]
async fn leave_table(
    users_canister_id: Principal,
    user_id: Principal,
) -> Result<PublicTable, TableError> {
    handle_cycle_check();

    let (mut table, balance) = {
        let mut table = TABLE.lock().map_err(|_| TableError::LockError)?;
        let table = table.as_mut().ok_or(TableError::TableNotFound)?;

        if table.has_user_left(user_id) {
            return Err(TableError::InvalidRequest(
                "User has already left the table".to_string(),
            ));
        }

        let backend_principal = BACKEND_PRINCIPAL
            .lock()
            .map_err(|_| TableError::LockError)?
            .ok_or(TableError::CanisterCallError(
                "Backend principal not found.".to_string(),
            ))?;

        let user = table.users.get(&user_id).ok_or(TableError::UserNotFound)?;
        validate_caller(vec![
            user_id,
            user.principal_id,
            backend_principal,
            table.id,
        ]);

        if is_table_game_ongoing(table) {
            if table.is_players_turn(user_id) {
                table
                    .user_fold(user_id, false)
                    .map_err(|e| e.into_inner())?;
            } else {
                table.user_pre_fold(user_id).map_err(|e| e.into_inner())?;
            }
        }

        let balance = table
            .users
            .get(&user_id)
            .map(|user| user.balance)
            .unwrap_or(0);

        (table.clone(), balance)
    };

    let currency_type = match &table.config.currency_type.clone() {
        CurrencyType::Real(currency) => currency.to_string(),
        CurrencyType::Fake => "Fake".to_string(),
    };

    match &table.config.currency_type {
        CurrencyType::Real(currency) => {
            ic_cdk::println!("Balance: {}", balance);

            let fee = if table.config.currency_type == CurrencyType::Real(currency::Currency::BTC) {
                10
            } else {
                ic_ledger_types::DEFAULT_FEE.e8s()
            };
            if balance > fee {
                ic_cdk::println!("Withdrawing: {}", balance);
                let currency_manager = {
                    let currency_manager =
                        CURRENCY_MANAGER.lock().map_err(|_| TableError::LockError)?;
                    currency_manager
                        .as_ref()
                        .ok_or(TableError::StateNotInitialized)?
                        .clone()
                };

                currency_manager
                    .withdraw(currency, user_id, balance)
                    .await?;
                let amount = balance - fee;
                log_user_transaction(
                    users_canister_id,
                    user_id,
                    amount,
                    TransactionType::TableWithdraw { table_id: table.id },
                    None,
                    Some(currency_type),
                )
                .await?;
            }
        }
        CurrencyType::Fake => {}
    }

    ic_cdk::spawn(async move {
        if let Err(e) = remove_users_active_table(users_canister_id, user_id).await {
            ic_cdk::println!("Error removing active table: {}", e);
        }
    });

    {
        let mut table_state = TABLE.lock().map_err(|_| TableError::LockError)?;
        let table_state = table_state.as_mut().ok_or(TableError::TableNotFound)?;
        table_state
            .remove_user(user_id, ActionType::Leave)
            .map_err(|e| e.into_inner())?;
    }

    let caller = ic_cdk::api::caller();
    table.hide_cards(caller).map_err(|e| e.into_inner())?;
    let res = match table.config.table_type {
        Some(TableType::Cash) | None => update_table_player_count(table.users.len()),
        _ => update_player_count_tournament(UserTournamentAction::Leave(user_id)),
    };
    if let Err(e) = res {
        ic_cdk::println!("Error updating table player count: {:?}", e);
    }

    Ok(table.into())
}

#[ic_cdk::update]
async fn leave_table_for_table_balancing(
    users_canister_id: Principal,
    user_id: Principal,
    table_to_move_to_id: Principal,
) -> Result<PublicTable, TableError> {
    handle_cycle_check();

    let mut table = {
        let mut table = TABLE.lock().map_err(|_| TableError::LockError)?;
        let table = table.as_mut().ok_or(TableError::TableNotFound)?;

        if table.has_user_left(user_id) {
            return Err(TableError::InvalidRequest(
                "User has already left the table".to_string(),
            ));
        }

        let backend_principal = BACKEND_PRINCIPAL
            .lock()
            .map_err(|_| TableError::LockError)?
            .ok_or(TableError::CanisterCallError(
                "Backend principal not found.".to_string(),
            ))?;

        let user = table.users.get(&user_id).ok_or(TableError::UserNotFound)?;
        validate_caller(vec![
            user_id,
            user.principal_id,
            backend_principal,
            table.id,
        ]);

        table.clone()
    };

    ic_cdk::spawn(async move {
        if let Err(e) = remove_users_active_table(users_canister_id, user_id).await {
            ic_cdk::println!("Error removing active table: {}", e);
        }
    });

    {
        let mut table_state = TABLE.lock().map_err(|_| TableError::LockError)?;
        let table_state = table_state.as_mut().ok_or(TableError::TableNotFound)?;
        table_state
            .remove_user_for_table_balancing(users_canister_id, user_id, table_to_move_to_id)
            .map_err(|e| e.into_inner())?;
    }

    let caller = ic_cdk::api::caller();
    table.hide_cards(caller).map_err(|e| e.into_inner())?;
    let res = match table.config.table_type {
        Some(TableType::Cash) | None => update_table_player_count(table.users.len()),
        _ => update_player_count_tournament(UserTournamentAction::Leave(user_id)),
    };
    if let Err(e) = res {
        ic_cdk::println!("Error updating table player count: {:?}", e);
    }

    Ok(table.into())
}

#[ic_cdk::update]
async fn withdraw_from_table(
    users_canister_id: Principal,
    user_id: Principal,
    amount: u64,
) -> Result<(), TableError> {
    handle_cycle_check();
    handle_table_validity_check()?;
    if amount == 0 {
        return Err(TableError::InvalidRequest(
            "Withdraw amount cannot be 0".to_string(),
        ));
    }

    let table = {
        let mut table = TABLE.lock().map_err(|_| TableError::LockError)?;
        let table = table.as_mut().ok_or(TableError::TableNotFound)?;
        let user = table.users.get(&user_id).ok_or(TableError::UserNotFound)?;

        let backend_principal = BACKEND_PRINCIPAL
            .lock()
            .map_err(|_| TableError::LockError)?
            .ok_or(TableError::CanisterCallError(
                "Backend principal not found.".to_string(),
            ))?;
        validate_caller(vec![user_id, user.principal_id, backend_principal]);

        let user_table_data = table
            .get_user_table_data(user_id)
            .map_err(|e| e.into_inner())?;
        if is_table_game_ongoing(table) && user_table_data.player_action != PlayerAction::Folded {
            ic_cdk::println!("Error withdrawing from table: Game is ongoing");
            return Err(GameError::ActionNotAllowed {
                reason: "Game is ongoing. You need to fold to withdraw from an ongoing game."
                    .to_string(),
            }
            .into());
        }

        let user = table
            .users
            .get_mut(&user_id)
            .ok_or(TableError::UserNotFound)?;
        if user.balance < amount {
            return Err(GameError::InsufficientFunds.into());
        }
        user.withdraw(amount);
        table.clone()
    };

    let currency_type = match table.config.currency_type {
        CurrencyType::Real(currency) => currency.to_string(),
        CurrencyType::Fake => "Fake".to_string(),
    };

    match table.config.currency_type {
        CurrencyType::Real(currency) => {
            let currency_manager = {
                let currency_manager =
                    CURRENCY_MANAGER.lock().map_err(|_| TableError::LockError)?;
                currency_manager
                    .as_ref()
                    .ok_or(TableError::StateNotInitialized)?
                    .clone()
            };

            currency_manager
                .withdraw(&currency, user_id, amount)
                .await?;
            let amount = amount - ic_ledger_types::DEFAULT_FEE.e8s();
            log_user_transaction(
                users_canister_id,
                user_id,
                amount,
                TransactionType::TableWithdraw { table_id: table.id },
                None,
                Some(currency_type),
            )
            .await?;
        }
        CurrencyType::Fake => {}
    }
    Ok(())
}

#[ic_cdk::update]
async fn deposit_to_table(
    users_canister_id: Principal,
    user_id: Principal,
    amount: u64,
    is_queued: bool,
) -> Result<ReturnResult, TableError> {
    handle_cycle_check();
    if amount == 0 {
        return Err(TableError::InvalidRequest(
            "Deposit amount cannot be 0".to_string(),
        ));
    }

    {
        let table = {
            let table = TABLE.lock().map_err(|_| TableError::LockError)?;
            table.as_ref().ok_or(TableError::TableNotFound)?.clone()
        };
        match table.config.currency_type {
            CurrencyType::Real(currency) => {
                let currency_manager = {
                    let currency_manager =
                        CURRENCY_MANAGER.lock().map_err(|_| TableError::LockError)?;
                    currency_manager
                        .as_ref()
                        .ok_or(TableError::StateNotInitialized)?
                        .clone()
                };

                if let Err(e) = currency_manager
                    .validate_allowance(&currency, user_id, amount)
                    .await
                {
                    return Err(e.into());
                }
            }
            CurrencyType::Fake => {}
        }
    }

    let table = {
        let mut table = TABLE.lock().map_err(|_| TableError::LockError)?;
        let table = table.as_mut().ok_or(TableError::TableNotFound)?;

        if table.is_game_ongoing()
            && !is_queued
            && table
                .get_user_table_data(user_id)
                .map_err(|e| e.into_inner())?
                .player_action
                != PlayerAction::Folded
        {
            table.append_to_queue(QueueItem::Deposit(user_id, users_canister_id, amount));
            return Ok(ReturnResult::DepositQueued);
        }

        let user = table
            .users
            .get_mut(&user_id)
            .ok_or(TableError::UserNotFound)?;
        user.deposit(amount);
        table.clone()
    };

    let currency_type = match table.config.currency_type {
        CurrencyType::Real(currency) => currency.to_string(),
        CurrencyType::Fake => "Fake".to_string(),
    };

    match table.config.currency_type {
        CurrencyType::Real(currency) => {
            let currency_manager = {
                let currency_manager =
                    CURRENCY_MANAGER.lock().map_err(|_| TableError::LockError)?;
                currency_manager
                    .as_ref()
                    .ok_or(TableError::StateNotInitialized)?
                    .clone()
            };

            let mut transaction_state = {
                let transaction_state = TRANSACTION_STATE
                    .lock()
                    .map_err(|_| TableError::LockError)?;
                transaction_state.clone()
            };

            match currency_manager
                .deposit(&mut transaction_state, &currency, user_id, amount)
                .await
            {
                Ok(_) => {
                    *TRANSACTION_STATE
                        .lock()
                        .map_err(|_| TableError::LockError)? = transaction_state;
                }
                Err(e) => {
                    ic_cdk::println!("Error depositing: {:?}", e);
                    let mut table = TABLE.lock().map_err(|_| TableError::LockError)?;
                    let table = table.as_mut().ok_or(TableError::TableNotFound)?;
                    let user = table
                        .users
                        .get_mut(&user_id)
                        .ok_or(TableError::UserNotFound)?;
                    user.withdraw(amount);
                    return Err(e.into());
                }
            }
            log_user_transaction(
                users_canister_id,
                user_id,
                amount,
                TransactionType::TableDeposit { table_id: table.id },
                None,
                Some(currency_type),
            )
            .await?;
        }
        CurrencyType::Fake => {}
    }
    Ok(ReturnResult::DepositSuccessful)
}

#[ic_cdk::update]
fn set_player_action(
    user_principal: Principal,
    player_action: PlayerAction,
) -> Result<(), TableError> {
    handle_cycle_check();
    let mut table_state = TABLE.lock().map_err(|_| TableError::LockError)?;
    let table_state = table_state.as_mut().ok_or(TableError::TableNotFound)?;
    let user = table_state
        .users
        .get(&user_principal)
        .ok_or(TableError::UserNotFound)?;
    let backend_principal = BACKEND_PRINCIPAL
        .lock()
        .map_err(|_| TableError::LockError)?
        .ok_or(TableError::CanisterCallError(
            "Backend principal not found.".to_string(),
        ))?;
    validate_caller(vec![user_principal, user.principal_id, backend_principal]);

    table_state
        .set_player_action(user_principal, player_action)
        .map_err(|e| e.into_inner())?;
    Ok(())
}

// #[ic_cdk::update]
// fn set_auto_check_fold(user_principal: Principal, enabled: bool) -> Result<(), TableError> {
//     handle_cycle_check();
//     let mut table_state = TABLE.lock().map_err(|_| TableError::LockError)?;
//     let table_state = table_state.as_mut().ok_or(TableError::TableNotFound)?;
//     let user = table_state
//         .users
//         .get(&user_principal)
//         .ok_or(TableError::UserNotFound)?;
//     let backend_principal = BACKEND_PRINCIPAL
//         .lock()
//         .map_err(|_| TableError::LockError)?
//         .ok_or(TableError::CanisterCallError(
//             "Backend principal not found.".to_string(),
//         ))?;
//     validate_caller(vec![user_principal, user.principal_id, backend_principal]);

//     if table_state.is_players_turn(user_principal) {
//         if table_state.is_users_current_total_bet_equal_to_highest_bet(user_principal) {
//             table_state
//                 .user_check(user_principal, false)
//                 .map_err(|e| e.into_inner())?;
//             if table_state.is_everyone_auto_check_fold(user_principal) {
//                 table_state.calculate_pots().map_err(|e| e.into_inner())?;
//                 table_state
//                     .cycle_to_showdown()
//                     .map_err(|e| e.into_inner())?;
//             } else {
//                 table_state
//                     .set_user_auto_check_fold(user_principal, enabled)
//                     .map_err(|e| e.into_inner())?;
//             }
//         } else {
//             table_state
//                 .user_fold(user_principal, false)
//                 .map_err(|e| e.into_inner())?;
//         }
//     } else {
//         table_state
//             .set_user_auto_check_fold(user_principal, enabled)
//             .map_err(|e| e.into_inner())?;
//     }

//     Ok(())
// }

#[ic_cdk::update]
fn player_sitting_out(user_principal: Principal) -> Result<(), TableError> {
    handle_cycle_check();
    let mut table_state = TABLE.lock().map_err(|_| TableError::LockError)?;
    let table_state = table_state.as_mut().ok_or(TableError::TableNotFound)?;
    let user = table_state
        .users
        .get(&user_principal)
        .ok_or(TableError::UserNotFound)?;
    let backend_principal = BACKEND_PRINCIPAL
        .lock()
        .map_err(|_| TableError::LockError)?
        .ok_or(TableError::CanisterCallError(
            "Backend principal not found.".to_string(),
        ))?;
    validate_caller(vec![user_principal, user.principal_id, backend_principal]);

    if table_state.users.get(&user_principal).is_none() {
        return Err(GameError::ActionNotAllowed {
            reason: "User not in table".to_string(),
        }
        .into());
    }

    table_state
        .user_sitting_out(user_principal, false)
        .map_err(|e| e.into_inner())?;

    Ok(())
}

#[ic_cdk::update]
async fn player_sitting_in(
    users_canister_id: Principal,
    user_id: Principal,
    auto_start: bool,
) -> Result<(), TableError> {
    handle_cycle_check();
    let mut table_state = {
        let mut table_state = TABLE.lock().map_err(|_| TableError::LockError)?;
        let table_state = table_state.as_mut().ok_or(TableError::TableNotFound)?;
        let user_table_data = table_state
            .get_user_table_data_mut(user_id)
            .map_err(|e| e.into_inner())?;
        user_table_data.seated_out_turns = 0;
        let user = table_state
            .users
            .get(&user_id)
            .ok_or(TableError::UserNotFound)?;
        let backend_principal = BACKEND_PRINCIPAL
            .lock()
            .map_err(|_| TableError::LockError)?
            .ok_or(TableError::CanisterCallError(
                "Backend principal not found.".to_string(),
            ))?;
        validate_caller(vec![user_id, user.principal_id, backend_principal]);
        table_state.clone()
    };

    match table_state.users.get(&user_id) {
        Some(user) => {
            if user.balance < table_state.big_blind {
                table_state
                    .kick_user(user_id, "Insufficient Funds".to_string())
                    .map_err(|e| e.into_inner())?;
                remove_users_active_table(users_canister_id, user_id).await?;

                *TABLE.lock().map_err(|_| TableError::LockError)? = Some(table_state);
                return Err(GameError::InsufficientFunds.into());
            }
        }
        None => {
            ic_cdk::println!("User not in table");
            return Err(GameError::ActionNotAllowed {
                reason: "User not in table".to_string(),
            }
            .into());
        }
    }

    let user_table_data = table_state
        .get_user_table_data(user_id)
        .map_err(|e| e.into_inner())?;
    if user_table_data.player_action != PlayerAction::SittingOut {
        return Err(GameError::ActionNotAllowed {
            reason: "User not sitting out".to_string(),
        }
        .into());
    }

    if table_state.is_game_ongoing() {
        let is_game_paused = table_state
            .get_playing_users()
            .map_err(|e| e.into_inner())?
            < 2;
        table_state.append_to_queue(QueueItem::SittingIn(user_id, is_game_paused));
        *TABLE.lock().map_err(|_| TableError::LockError)? = Some(table_state);
        return Ok(());
    } else {
        table_state
            .set_player_action(user_id, PlayerAction::None)
            .map_err(|e| e.into_inner())?;
        *TABLE.lock().map_err(|_| TableError::LockError)? = Some(table_state.clone());
    }

    if table_state.number_of_players() >= 2
        && table_state.number_of_active_players() >= 2
        && !table_state.is_game_ongoing()
        && auto_start
    {
        let (_res,): (Result<(), TableError>,) =
            ic_cdk::call(ic_cdk::api::id(), "start_new_betting_round", ())
                .await
                .map_err(|e| TableError::CanisterCallError(format!("{:?} {}", e.0, e.1)))?;
    }

    Ok(())
}

#[ic_cdk::update]
fn place_bet(user_principal: Principal, bet_type: BetType) -> Result<(), TableError> {
    handle_cycle_check();

    let mut table_state = TABLE.lock().map_err(|_| TableError::LockError)?;
    let table_state = table_state.as_mut().ok_or(TableError::TableNotFound)?;
    let user = table_state
        .users
        .get(&user_principal)
        .ok_or(TableError::UserNotFound)?;
    let backend_principal = BACKEND_PRINCIPAL
        .lock()
        .map_err(|_| TableError::LockError)?
        .ok_or(TableError::CanisterCallError(
            "Backend principal not found.".to_string(),
        ))?;
    validate_caller(vec![user_principal, user.principal_id, backend_principal]);

    if table_state
        .get_player_at_seat(table_state.current_player_index)
        .map_err(|e| e.into_inner())?
        != user_principal
    {
        return Err(GameError::ActionNotAllowed {
            reason: "Not your turn".to_string(),
        }
        .into());
    }

    table_state
        .bet(user_principal, bet_type)
        .map_err(|e| e.into_inner())?;
    Ok(())
}

#[ic_cdk::update]
fn fold(user_principal: Principal, is_pre_fold: bool) -> Result<(), TableError> {
    handle_cycle_check();

    let mut table_state = TABLE.lock().map_err(|_| TableError::LockError)?;
    let table_state = table_state.as_mut().ok_or(TableError::TableNotFound)?;
    let user = table_state
        .users
        .get(&user_principal)
        .ok_or(TableError::UserNotFound)?;
    let backend_principal = BACKEND_PRINCIPAL
        .lock()
        .map_err(|_| TableError::LockError)?
        .ok_or(TableError::CanisterCallError(
            "Backend principal not found.".to_string(),
        ))?;
    validate_caller(vec![user_principal, user.principal_id, backend_principal]);

    if is_pre_fold {
        table_state
            .user_pre_fold(user_principal)
            .map_err(|e| e.into_inner())?;
    } else {
        table_state
            .user_fold(user_principal, false)
            .map_err(|e| e.into_inner())?;
    }

    Ok(())
}

#[ic_cdk::update]
fn check(user_principal: Principal) -> Result<(), TableError> {
    handle_cycle_check();

    let mut table_state = TABLE.lock().map_err(|_| TableError::LockError)?;
    let table_state = table_state.as_mut().ok_or(TableError::TableNotFound)?;
    let user = table_state
        .users
        .get(&user_principal)
        .ok_or(TableError::UserNotFound)?;
    let backend_principal = BACKEND_PRINCIPAL
        .lock()
        .map_err(|_| TableError::LockError)?
        .ok_or(TableError::CanisterCallError(
            "Backend principal not found.".to_string(),
        ))?;
    validate_caller(vec![user_principal, user.principal_id, backend_principal]);

    if let SeatStatus::Occupied(current_player) =
        table_state.seats[table_state.current_player_index]
    {
        if current_player != user_principal {
            return Ok(());
        }
    } else {
        return Err(TableError::UserNotFound);
    }

    if table_state
        .get_player_at_seat(table_state.current_player_index)
        .map_err(|e| e.into_inner())?
        != user_principal
    {
        return Err(GameError::ActionNotAllowed {
            reason: "Not your turn".to_string(),
        }
        .into());
    }

    table_state
        .user_check(user_principal, false)
        .map_err(|e| e.into_inner())?;
    Ok(())
}

#[ic_cdk::update]
fn handle_timer_expiration(user_id: Principal) -> Result<(), TableError> {
    handle_cycle_check();
    let mut table_state = TABLE.lock().map_err(|_| TableError::LockError)?;
    let table_state = table_state.as_mut().ok_or(TableError::TableNotFound)?;
    let backend_principal = BACKEND_PRINCIPAL
        .lock()
        .map_err(|_| TableError::LockError)?
        .ok_or(TableError::CanisterCallError(
            "Backend principal not found.".to_string(),
        ))?;
    let mut valid_callers = (*CONTROLLER_PRINCIPALS).clone();
    valid_callers.push(backend_principal);
    valid_callers.push(table_state.id);
    validate_caller(valid_callers);

    let res = if table_state.is_users_current_total_bet_equal_to_highest_bet(user_id) {
        table_state
            .user_check(user_id, true)
            .map_err(|e| e.into_inner())
    } else {
        table_state.force_fold(user_id).map_err(|e| e.into_inner())
    };

    res?;

    Ok(())
}

#[ic_cdk::update]
async fn start_new_betting_round() -> Result<(), TableError> {
    handle_cycle_check();

    let raw_bytes = ic_cdk::api::management_canister::main::raw_rand().await;
    let raw_bytes = raw_bytes
        .map_err(|e| {
            TableError::CanisterCallError(format!("Failed to generate random bytes: {:?}", e))
        })?
        .0;

    let (kicked_players, action_logs, table_id, total_users, seated_out_kicked_players, users) = {
        let mut table_state = TABLE.lock().map_err(|_| TableError::LockError)?;
        let table_state = table_state.as_mut().ok_or(TableError::TableNotFound)?;
        let backend_principal = BACKEND_PRINCIPAL
            .lock()
            .map_err(|_| TableError::LockError)?
            .ok_or(TableError::CanisterCallError(
                "Backend principal not found.".to_string(),
            ))?;

        validate_caller(vec![backend_principal, table_state.id]);

        if table_state.config.is_paused.unwrap_or(false) {
            ic_cdk::println!("Game is paused.");
            return Err(GameError::ActionNotAllowed {
                reason: "Game is paused".to_string(),
            }
            .into());
        }

        if table_state.is_game_ongoing() {
            return Err(GameError::ActionNotAllowed {
                reason: "Game is ongoing".to_string(),
            }
            .into());
        } else if table_state.config.is_paused.unwrap_or(false) {
            table_state.status = TableStatus::Paused;
            return Err(GameError::ActionNotAllowed {
                reason: "Game is paused".to_string(),
            }
            .into());
        }

        let action_logs = table_state.action_logs.clone();

        let (kicked_players, seated_out_kicked_players) =
            match table_state.start_betting_round(raw_bytes) {
                Ok(kicked_players) => kicked_players,
                Err(e) => {
                    ic_cdk::println!("Error starting betting round: {:?}", e);
                    return Err(e.into_inner().into());
                }
            };

        (
            kicked_players,
            action_logs,
            table_state.id,
            table_state.users.len(),
            seated_out_kicked_players,
            table_state.users.clone(),
        )
    };

    if !kicked_players.is_empty() || !seated_out_kicked_players.is_empty() {
        if let Err(e) = update_table_player_count(total_users) {
            ic_cdk::println!("Error updating table player count: {}", e);
        }

        // TODO: This should be reduced to one call.
        if !kicked_players.is_empty() {
            for player in &kicked_players {
                if let Err(e) =
                    update_player_count_tournament(UserTournamentAction::Leave(player.0))
                {
                    ic_cdk::println!("Error updating tournament table player count: {}", e);
                }
            }
        }
        if !seated_out_kicked_players.is_empty() {
            for player in &seated_out_kicked_players {
                if let Err(e) =
                    update_player_count_tournament(UserTournamentAction::Leave(player.0))
                {
                    ic_cdk::println!("Error updating tournament table player count: {}", e);
                }
            }
        }
    }

    let mut total_kicked_players = kicked_players.clone();
    total_kicked_players.extend(seated_out_kicked_players.clone());
    let mut total_kicked_users = Vec::new();
    for (user_principal, balance) in total_kicked_players {
        let user = match users.get(&user_principal) {
            Some(user) => user,
            None => {
                ic_cdk::println!("User not found in table");
                continue;
            }
        };
        total_kicked_users.push((user.clone(), balance));
    }

    ic_cdk::spawn(async move {
        for (user, balance) in total_kicked_users {
            let leave_result: Result<(Result<PublicTable, TableError>,), _> = ic_cdk::call(
                ic_cdk::api::id(),
                "kick_player",
                (user.users_canister_id, user.principal_id, balance),
            )
            .await;

            match leave_result {
                Ok((result,)) => match result {
                    Ok(_) => {}
                    Err(err) => {
                        ic_cdk::println!(
                            "Error kicking inactive player: Error leaving table: {:?}",
                            err
                        );
                    }
                },
                Err(err) => {
                    ic_cdk::println!(
                        "Error kicking inactive player: Error leaving table: {:?}",
                        err
                    );
                }
            }
        }
    });

    ic_cdk::spawn(async move {
        let backend_principal = match BACKEND_PRINCIPAL.lock() {
            Ok(guard) => match *guard {
                Some(principal) => principal,
                None => {
                    ic_cdk::println!("Error storing logs: Backend principal not found");
                    return;
                }
            },
            Err(_) => {
                ic_cdk::println!("Error storing logs: Failed to acquire lock");
                return;
            }
        };

        if backend_principal == Principal::from_text("zbspl-ziaaa-aaaam-qbe2q-cai").unwrap() {
            // Check if the backend principal is the prod deployment or dev deployment
            let (_res,): (Result<(), LogStoreError>,) = match ic_cdk::call(
                Principal::from_text("ztuys-vyaaa-aaaam-qbezq-cai").unwrap(),
                "log_actions",
                (table_id, action_logs),
            )
            .await
            {
                Ok(res) => res,
                Err(e) => {
                    ic_cdk::println!("Error storing logs: {:?}", e);
                    return;
                }
            };
        } else if backend_principal == Principal::from_text("e4yx7-lqaaa-aaaah-qdslq-cai").unwrap()
        {
            let (_res,): (Result<(), LogStoreError>,) = match ic_cdk::call(
                Principal::from_text("es22x-qaaaa-aaaah-qdskq-cai").unwrap(),
                "log_actions",
                (table_id, action_logs),
            )
            .await
            {
                Ok(res) => res,
                Err(e) => {
                    ic_cdk::println!("Error storing logs: {:?}", e);
                    return;
                }
            };
        } else if backend_principal == Principal::from_text("by6od-j4aaa-aaaaa-qaadq-cai").unwrap()
        {
            let (_res,): (Result<(), LogStoreError>,) = match ic_cdk::call(
                Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").unwrap(),
                "log_actions",
                (table_id, action_logs),
            )
            .await
            {
                Ok(res) => res,
                Err(e) => {
                    ic_cdk::println!("Error storing logs: {:?}", e);
                    return;
                }
            };
        }
    });

    Ok(())
}

#[ic_cdk::update]
async fn withdraw_rake(mut rake_amount: u64) -> Result<(), TableError> {
    handle_cycle_check();
    handle_table_validity_check()?;

    let table = {
        let mut table = TABLE.lock().map_err(|_| TableError::LockError)?;
        let table = table.as_mut().ok_or(TableError::TableNotFound)?;

        table.clone()
    };

    let user_index = {
        let backend_principal = BACKEND_PRINCIPAL
            .lock()
            .map_err(|_| TableError::LockError)?
            .ok_or(TableError::CanisterCallError(
                "Backend principal not found.".to_string(),
            ))?;
        get_user_index_principal(backend_principal)
    };

    match table.config.currency_type {
        CurrencyType::Real(currency) => {
            let currency_manager = {
                let currency_manager =
                    CURRENCY_MANAGER.lock().map_err(|_| TableError::LockError)?;
                currency_manager
                    .as_ref()
                    .ok_or(TableError::StateNotInitialized)?
                    .clone()
            };

            {
                let is_shared = table.config.is_shared_rake.is_some();
                // Update rake stats
                let mut rake_stats = RAKE_STATS.lock().map_err(|_| TableError::LockError)?;
                rake_stats.add_rake(rake_amount, is_shared);
            }

            let mut house_rake = rake_amount / 2;
            rake_amount -= house_rake;
            let mut referrers: HashMap<Principal, User> = HashMap::new();
            
            // For each player at the table, check if they were referred
            for user in table.users.users.values() {                
                if let Some(referrer_principal) = user.referrer {
                    // Get referrer's canister and check if referral is still active
                    let referrer_canister_id = match get_users_canister_principal_by_id_wrapper(user_index, referrer_principal).await {
                        Ok(canister_id) => canister_id,
                        Err(_) => continue,
                    };
                    let referrer = match referrers.get(&referrer_principal) {
                        Some(referrer) => referrer.clone(),
                        None => {
                            let referrer = get_user(referrer_canister_id, referrer_principal).await?;
                            referrers.insert(referrer_principal, referrer.clone());
                            referrer
                        }
                    };

                    // Check if referral is active
                    let is_active = user.referral_start_date.unwrap_or(0) + REFERRAL_PERIOD > ic_cdk::api::time();

                    if is_active {
                        // Get referrer's rake percentage based on tier
                        let rake_percentage = referrer.get_referral_rake_percentage();

                        // Calculate referrer's share
                        let player_share = house_rake / table.users.users.len() as u64;
                        let referrer_amount = player_share * rake_percentage as u64 / 100;

                        if referrer_amount > 0 && referrer_amount < house_rake {
                            // Transfer rake share to referrer
                            if let Err(e) = currency_manager
                                .withdraw(&currency, referrer_principal, referrer_amount)
                                .await
                            {
                                ic_cdk::println!("Error distributing referral rake: {:?}", e);
                            } else {
                                house_rake -= referrer_amount;
                            }
                        }
                    }
                }
            }

            // TODO: At some point we need to figure if we want to withdraw rake to external blockchains.
            if let Some((rake_share_principal, _rake_share_account_id)) =
                table.config.is_shared_rake
            {
                if let Err(e) = currency_manager
                    .withdraw_rake(&currency, *RAKE_WALLET_ADDRESS_PRINCIPAL, house_rake - ic_ledger_types::DEFAULT_FEE.e8s())
                    .await
                {
                    ic_cdk::println!("Error withdrawing rake: {:?}", e);
                    let mut table = TABLE.lock().map_err(|_| TableError::LockError)?;
                    let table = table.as_mut().ok_or(TableError::TableNotFound)?;
                    let mut rake_total = table.rake_total.unwrap_or(0);
                    rake_total += house_rake;
                    table.rake_total = Some(rake_total);
                    return Err(e.into());
                }
                if let Err(e) = currency_manager
                    .withdraw(&currency, rake_share_principal, rake_amount - ic_ledger_types::DEFAULT_FEE.e8s())
                    .await
                {
                    ic_cdk::println!("Error withdrawing rake: {:?}", e);
                    let mut table = TABLE.lock().map_err(|_| TableError::LockError)?;
                    let table = table.as_mut().ok_or(TableError::TableNotFound)?;
                    let mut rake_total = table.rake_total.unwrap_or(0);
                    rake_total += rake_amount;
                    table.rake_total = Some(rake_total);
                    return Err(e.into());
                }
            } else if let Err(e) = currency_manager
                .withdraw_rake(&currency, *RAKE_WALLET_ADDRESS_PRINCIPAL, rake_amount + house_rake - ic_ledger_types::DEFAULT_FEE.e8s())
                .await
            {
                ic_cdk::println!("Error withdrawing rake: {:?}", e);
                let mut table = TABLE.lock().map_err(|_| TableError::LockError)?;
                let table = table.as_mut().ok_or(TableError::TableNotFound)?;
                let mut rake_total = table.rake_total.unwrap_or(0);
                rake_total += rake_amount;
                table.rake_total = Some(rake_total);
                return Err(e.into());
            }
        }
        CurrencyType::Fake => {}
    }
    Ok(())
}

#[ic_cdk::query]
fn get_rake_stats() -> Result<RakeStats, TableError> {
    let rake_stats = RAKE_STATS.lock().map_err(|_| TableError::LockError)?;
    Ok(rake_stats.clone())
}

#[ic_cdk::query]
fn rank_cards(hand: Vec<Card>) -> Result<Rank, TableError> {
    Ok(rank_hand(hand))
}

#[ic_cdk::query]
fn get_notifications() -> Result<Vec<Notification>, TableError> {
    let table_state = TABLE.lock().map_err(|_| TableError::LockError)?;
    let table_state = table_state.as_ref().ok_or(TableError::TableNotFound)?;

    Ok(table_state.notifications.notifications.clone())
}

#[ic_cdk::update]
async fn return_all_cycles_to_index() -> Result<(), TableError> {
    // TODO: We are losing cycles on this we need to efficiently find a way to return all cycles to the index canister
    // Get the current balance of cycles in the canister
    {
        let mut table_state = TABLE.lock().map_err(|_| TableError::LockError)?;
        let table_state = table_state.as_mut().ok_or(TableError::TableNotFound)?;
        let backend_principal = BACKEND_PRINCIPAL
            .lock()
            .map_err(|_| TableError::LockError)?
            .ok_or(TableError::CanisterCallError(
                "Backend principal not found.".to_string(),
            ))?;

        validate_caller(vec![backend_principal, table_state.id]);
    }
    let all_cycles = ic_cdk::api::canister_balance().saturating_sub(35_000_000_000) as u128;
    if all_cycles == 0 {
        return Err(TableError::CanisterCallError(
            "No cycles available to send".to_string(),
        ));
    }

    let _ = transfer_cycles_to_table_index(all_cycles).await;
    Ok(())
}

#[ic_cdk::update]
async fn return_cycles_to_index(cycles_amount: u128) -> Result<(), TableError> {
    {
        let mut table_state = TABLE.lock().map_err(|_| TableError::LockError)?;
        let table_state = table_state.as_mut().ok_or(TableError::TableNotFound)?;
        let backend_principal = BACKEND_PRINCIPAL
            .lock()
            .map_err(|_| TableError::LockError)?
            .ok_or(TableError::CanisterCallError(
                "Backend principal not found.".to_string(),
            ))?;

        validate_caller(vec![backend_principal, table_state.id]);
    }
    transfer_cycles_to_table_index(cycles_amount).await
}

async fn transfer_cycles_to_table_index(cycles_amount: u128) -> Result<(), TableError> {
    let backend_principal = BACKEND_PRINCIPAL
        .lock()
        .map_err(|_| TableError::LockError)?
        .ok_or(TableError::CanisterCallError(
            "Backend principal not found.".to_string(),
        ))?;

    // Transfer all cycles to the index canister
    let res = ic_cdk::api::management_canister::main::deposit_cycles(
        CanisterIdRecord {
            canister_id: backend_principal,
        },
        cycles_amount,
    )
    .await;

    match res {
        Ok(_) => Ok(()),
        Err((code, msg)) => Err(TableError::CanisterCallError(format!(
            "Failed to send cycles: {}: {}",
            code as u32, msg
        ))),
    }
}

#[ic_cdk::query]
fn get_cycles() -> u64 {
    ic_cdk::api::canister_balance()
}

#[ic_cdk::update]
async fn update_blinds(small_blind: u64, big_blind: u64, ante: AnteType) -> Result<(), TableError> {
    handle_cycle_check();

    let mut table = TABLE.lock().map_err(|_| TableError::LockError)?;
    let table = table.as_mut().ok_or(TableError::TableNotFound)?;
    let backend_principal = BACKEND_PRINCIPAL
        .lock()
        .map_err(|_| TableError::LockError)?
        .ok_or(TableError::CanisterCallError(
            "Backend principal not found.".to_string(),
        ))?;

    validate_caller(vec![backend_principal, table.id]);

    let ante = if ante != AnteType::None {
        Some(ante)
    } else {
        None
    };

    // Only allow blind updates between hands
    if table.is_game_ongoing() {
        table.append_to_queue(QueueItem::UpdateBlinds(small_blind, big_blind, ante));
        return Ok(());
    }

    // Update the blinds
    table.small_blind = small_blind;
    table.big_blind = big_blind;
    table.config.ante_type = ante;

    Ok(())
}

#[ic_cdk::query]
async fn get_free_seat_index() -> Result<Option<u8>, TableError> {
    handle_cycle_check();

    let table = TABLE.lock().map_err(|_| TableError::LockError)?;
    let table = table.as_ref().ok_or(TableError::TableNotFound)?;

    Ok(table.get_free_seat_index())
}

#[ic_cdk::update]
async fn clear_table() -> Result<(), TableError> {
    handle_cycle_check();
    {
        let mut table_state = TABLE.lock().map_err(|_| TableError::LockError)?;
        let table_state = table_state.as_mut().ok_or(TableError::TableNotFound)?;
        let backend_principal = BACKEND_PRINCIPAL
            .lock()
            .map_err(|_| TableError::LockError)?
            .ok_or(TableError::CanisterCallError(
                "Backend principal not found.".to_string(),
            ))?;

        validate_caller(vec![backend_principal, table_state.id]);
    }

    let users_to_remove = {
        let table = TABLE.lock().map_err(|_| TableError::LockError)?;
        let table = table.as_ref().ok_or(TableError::TableNotFound)?;

        // Collect all users and their wallet principals into a Vec
        table
            .users
            .users
            .iter()
            .map(|(principal, user)| (*principal, user.principal_id))
            .collect::<Vec<(Principal, Principal)>>()
    };

    // Create futures for all leave_table operations
    let leave_futures =
        users_to_remove
            .into_iter()
            .map(|(user_principal, wallet_principal)| async move {
                // TODO: Handle this properly
                match canister_functions::inter_canister_call_wrappers::leave_table(
                    user_principal,
                    ic_cdk::api::id(),
                    wallet_principal,
                )
                .await
                {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        ic_cdk::println!("Error removing user {}: {:?}", user_principal, e);
                        Err(e)
                    }
                }
            });

    // Execute all leave_table operations in parallel
    let results = futures::future::join_all(leave_futures).await;

    // Check for any errors
    for result in results {
        if let Err(e) = result {
            ic_cdk::println!("Error during clear_table: {:?}", e);
        }
    }

    Ok(())
}

#[ic_cdk::query]
fn get_seat_index(user: Principal) -> Result<Option<u8>, TableError> {
    let table = TABLE.lock().map_err(|_| TableError::LockError)?;
    let table = table.as_ref().ok_or(TableError::TableNotFound)?;

    Ok(table.get_seat_index(user))
}

#[ic_cdk::update]
fn pause_table() -> Result<(), TableError> {
    handle_cycle_check();

    let mut table = TABLE.lock().map_err(|_| TableError::LockError)?;
    let table = table.as_mut().ok_or(TableError::TableNotFound)?;
    let backend_principal = BACKEND_PRINCIPAL
        .lock()
        .map_err(|_| TableError::LockError)?
        .ok_or(TableError::CanisterCallError(
            "Backend principal not found.".to_string(),
        ))?;

    validate_caller(vec![backend_principal, table.id]);

    table.config.is_paused = Some(true);
    Ok(())
}

#[ic_cdk::update]
fn pause_table_for_addon(duration: u64) -> Result<(), TableError> {
    handle_cycle_check();

    let mut table = TABLE.lock().map_err(|_| TableError::LockError)?;
    let table = table.as_mut().ok_or(TableError::TableNotFound)?;
    if table.config.is_paused.unwrap_or(false) {
        return Ok(());
    }
    let backend_principal = BACKEND_PRINCIPAL
        .lock()
        .map_err(|_| TableError::LockError)?
        .ok_or(TableError::CanisterCallError(
            "Backend principal not found.".to_string(),
        ))?;

    validate_caller(vec![backend_principal, table.id]);

    ic_cdk::println!("Pausing table");

    if table.config.table_type.is_none()
        || table.config.table_type.clone().unwrap() == TableType::Cash
    {
        return Err(TableError::InvalidRequest(
            "Table is not a tournament table".to_string(),
        ));
    }

    table.append_to_queue(QueueItem::PauseTableForAddon(duration));
    Ok(())
}

#[ic_cdk::update]
async fn resume_table() -> Result<(), TableError> {
    handle_cycle_check();
    ic_cdk::println!("Resuming table");
    let is_paused = {
        let mut table = TABLE.lock().map_err(|_| TableError::LockError)?;
        let table = table.as_mut().ok_or(TableError::TableNotFound)?;
        if !table.config.is_paused.unwrap_or(false) {
            return Ok(());
        }
        let backend_principal = BACKEND_PRINCIPAL
            .lock()
            .map_err(|_| TableError::LockError)?
            .ok_or(TableError::CanisterCallError(
                "Backend principal not found.".to_string(),
            ))?;

        validate_caller(vec![backend_principal, table.id]);

        table.config.is_paused = Some(false);
        false
    };

    if !is_paused {
        let (res,): (Result<(), TableError>,) =
            ic_cdk::call(ic_cdk::api::id(), "start_new_betting_round", ())
                .await
                .map_err(|e| TableError::CanisterCallError(format!("{:?} {}", e.0, e.1)))?;
        if let Err(e) = res {
            ic_cdk::println!("Error resuming table: {:?}", e);
            return Ok(());
        }
    }
    Ok(())
}

#[ic_cdk::update]
async fn set_as_final_table() -> Result<(), TableError> {
    handle_cycle_check();
    let mut table = TABLE.lock().map_err(|_| TableError::LockError)?;
    let table = table.as_mut().ok_or(TableError::TableNotFound)?;
    let backend_principal = BACKEND_PRINCIPAL
        .lock()
        .map_err(|_| TableError::LockError)?
        .ok_or(TableError::CanisterCallError(
            "Backend principal not found.".to_string(),
        ))?;

    validate_caller(vec![backend_principal, table.id]);

    if let Some(TableType::Tournament {
        tournament_id: _,
        is_final_table: _,
    }) = table.config.table_type
    {
        table.config.table_type = Some(TableType::Tournament {
            tournament_id: ic_cdk::api::caller(),
            is_final_table: true,
        });
    } else {
        return Err(TableError::InvalidRequest(
            "Table is not a tournament table".to_string(),
        ));
    }
    Ok(())
}

#[ic_cdk::query]
async fn get_rake_wallet_account_id() -> String {
    RAKE_WALLET_ACCOUNT_ID.clone()
}

#[ic_cdk::query]
async fn get_rake_wallet_principal() -> Principal {
    *RAKE_WALLET_ADDRESS_PRINCIPAL
}

#[ic_cdk::update]
async fn get_players_on_table() -> Result<Vec<Principal>, TableError> {
    let table = TABLE.lock().map_err(|_| TableError::LockError)?;
    let table = table.as_ref().ok_or(TableError::TableNotFound)?;

    let mut players = Vec::new();
    table.seats.iter().for_each(|seat| match seat {
        SeatStatus::Occupied(user_id) => {
            if !players.contains(user_id) {
                players.push(*user_id)
            }
        }
        SeatStatus::QueuedForNextRound(user_id, _, _) => {
            if !players.contains(user_id) {
                players.push(*user_id)
            }
        }
        SeatStatus::Reserved {
            principal: user_id,
            timestamp: _,
        } => {
            if !players.contains(user_id) {
                players.push(*user_id)
            }
        }
        _ => {}
    });
    Ok(players)
}

// Chat functions

#[ic_cdk::update]
fn send_chat_message(
    user_principal: Principal,
    content: String,
    message_type: ChatMessageType,
    recipient: Option<Principal>,
) -> Result<u64, ChatError> {
    handle_cycle_check();

    // Validate the user is in the table
    let table_state = TABLE
        .lock()
        .map_err(|_| ChatError::LockError("Failed to acquire table lock".to_string()))?;
    let table_state = table_state
        .as_ref()
        .ok_or(ChatError::InternalError("Table not found".to_string()))?;

    let user = table_state
        .users
        .get(&user_principal)
        .ok_or(ChatError::UserNotInTable(user_principal))?;

    // Validate caller
    let backend_principal = BACKEND_PRINCIPAL
        .lock()
        .map_err(|_| ChatError::LockError("Failed to acquire backend principal lock".to_string()))?
        .ok_or(ChatError::InternalError(
            "Backend principal not found".to_string(),
        ))?;

    validate_caller(vec![user_principal, user.principal_id, backend_principal]);

    // Check message length
    if content.len() > 2000 {
        return Err(ChatError::MessageTooLong {
            current_size: content.len(),
            max_size: 2000,
        });
    }

    // Get user display name
    let sender_name = user.user_name.clone();

    // Add the message to chat history
    let mut chat_history = CHAT_HISTORY
        .lock()
        .map_err(|_| ChatError::LockError("Failed to acquire chat history lock".to_string()))?;
    let msg_id = chat_history.add_message(
        user_principal,
        sender_name,
        content,
        message_type,
        recipient,
    );

    Ok(msg_id)
}

#[ic_cdk::update]
fn edit_chat_message(
    user_principal: Principal,
    message_id: u64,
    new_content: String,
) -> Result<(), ChatError> {
    handle_cycle_check();

    // Validate the user is in the table
    let table_state = TABLE
        .lock()
        .map_err(|_| ChatError::LockError("Failed to acquire table lock".to_string()))?;
    let table_state = table_state
        .as_ref()
        .ok_or(ChatError::InternalError("Table not found".to_string()))?;

    let user = table_state
        .users
        .get(&user_principal)
        .ok_or(ChatError::UserNotInTable(user_principal))?;

    // Validate caller
    let backend_principal = BACKEND_PRINCIPAL
        .lock()
        .map_err(|_| ChatError::LockError("Failed to acquire backend principal lock".to_string()))?
        .ok_or(ChatError::InternalError(
            "Backend principal not found".to_string(),
        ))?;

    validate_caller(vec![user_principal, user.principal_id, backend_principal]);

    // Check message length
    if new_content.len() > 2000 {
        return Err(ChatError::MessageTooLong {
            current_size: new_content.len(),
            max_size: 2000,
        });
    }

    // Edit the message
    let mut chat_history = CHAT_HISTORY
        .lock()
        .map_err(|_| ChatError::LockError("Failed to acquire chat history lock".to_string()))?;
    chat_history.edit_message(message_id, new_content, user_principal)?;

    Ok(())
}

#[ic_cdk::query]
fn get_recent_chat_messages(
    from_message_id: Option<u64>,
    page_size: usize,
) -> Result<Vec<ChatMessage>, ChatError> {
    let chat_history = CHAT_HISTORY
        .lock()
        .map_err(|_| ChatError::LockError("Failed to acquire chat history lock".to_string()))?;
    Ok(chat_history.get_messages(from_message_id, page_size))
}

#[ic_cdk::query]
fn get_chat_messages_for_user(user_principal: Principal) -> Result<Vec<ChatMessage>, ChatError> {
    let chat_history = CHAT_HISTORY
        .lock()
        .map_err(|_| ChatError::LockError("Failed to acquire chat history lock".to_string()))?;
    Ok(chat_history.get_messages_for_user(user_principal))
}

#[ic_cdk::update]
fn clear_chat_history() -> Result<(), ChatError> {
    handle_cycle_check();

    // Only allow the backend to clear chat history
    let backend_principal = BACKEND_PRINCIPAL
        .lock()
        .map_err(|_| ChatError::LockError("Failed to acquire backend principal lock".to_string()))?
        .ok_or(ChatError::InternalError(
            "Backend principal not found".to_string(),
        ))?;

    validate_caller(vec![backend_principal]);

    let mut chat_history = CHAT_HISTORY
        .lock()
        .map_err(|_| ChatError::LockError("Failed to acquire chat history lock".to_string()))?;
    chat_history.clear();

    Ok(())
}

ic_cdk::export_candid!();
