use std::{collections::HashMap, sync::atomic::Ordering};

use candid::{Nat, Principal};
use canister_functions::{
    create_canister_wrapper, cycle::check_and_top_up_canister, install_wasm_code,
    stop_and_delete_canister,
};
use currency::{types::currency::CKTokenSymbol, Currency};
use errors::{
    table_error::TableError, tournament_error::TournamentError,
    tournament_index_error::TournamentIndexError,
};
use ic_cdk::management_canister::DepositCyclesArgs;
use ic_ledger_types::{AccountIdentifier, Subaccount};
use intercanister_call_wrappers::tournament_canister::{
    handle_tournament_end_wrapper, update_tournament_state_icc_wrapper,
};
use table::{
    poker::game::{
        table_functions::{
            table::{TableConfig, TableId},
            types::CurrencyType,
        },
        types::PublicTable,
    },
    table_canister::{
        create_table_wrapper, deposit_to_table, get_table_wrapper, is_game_ongoing_wrapper,
        join_table, leave_table_wrapper, return_all_cycles_to_index,
    },
};
use tournaments::tournaments::{
    tournament_type::{TournamentSizeType, TournamentType},
    types::{TournamentData, TournamentId, TournamentState},
    utils::calculate_rake,
};
use user::user::{UsersCanisterId, WalletPrincipalId};

use crate::{
    CONTROLLER_PRINCIPALS, CURRENCY_MANAGER, LAST_LEADERBOARD_UPDATE, LEADERBOARD,
    LIVE_LEADERBOARD, PRIZE_POOL, RAKE_AMOUNT, TABLE_CANISTER_WASM, TOURNAMENT, TOURNAMENT_INDEX,
    TRANSACTION_STATE,
};

const MINIMUM_CYCLE_THRESHOLD: u128 = 1_000_000_000_000;

// Leaderboard update interval in nanoseconds
pub const LEADERBOARD_UPDATE_INTERVAL: u64 = 150_000_000_000;

pub struct CanisterState {
    pub owner: Principal,
    pub default_subaccount: Subaccount,
    pub account_identifier: AccountIdentifier,
}

pub fn create_default_subaccount() -> Subaccount {
    let bytes = [0u8; 32];

    Subaccount(bytes)
}

pub fn get_canister_state() -> CanisterState {
    let owner_principal = ic_cdk::api::canister_self();
    let default_subaccount = create_default_subaccount();

    let account_identifier = AccountIdentifier::new(&owner_principal, &default_subaccount);
    CanisterState {
        owner: owner_principal,
        default_subaccount,
        account_identifier,
    }
}

pub async fn handle_cycle_check_async() {
    let cycles = ic_cdk::api::canister_cycle_balance();
    if cycles >= MINIMUM_CYCLE_THRESHOLD {
        return;
    }
    ic_cdk::println!("%%%%%%%%%%% Cycles balance is low: {}", Nat::from(cycles));

    let tournament_index_result = TOURNAMENT_INDEX.lock();
    let tournament_index = match tournament_index_result {
        Ok(lock) => match *lock {
            Some(index) => index,
            None => {
                ic_cdk::println!("User not found");
                return; // or perform some error handling
            }
        },
        Err(_) => {
            ic_cdk::println!("Lock error occurred");
            return; // or handle the lock error
        }
    };

    ic_cdk::println!(
        "%%%%%%%%%%%%%%%% Requesting cycles from tournament index: {:?}",
        tournament_index.to_text()
    );

    if let Err(e) = check_and_top_up_canister(
        ic_cdk::api::canister_self(),
        tournament_index,
        MINIMUM_CYCLE_THRESHOLD,
    )
    .await
    {
        ic_cdk::println!("Failed to top up canister: {:?}", e);
    }
    ic_cdk::println!(
        "%%%%%%%%%%%%%%%% Finished requesting cycles: {}",
        Nat::from(ic_cdk::api::canister_cycle_balance())
    );
}

pub async fn create_table(
    tournament_config: &TournamentData,
    table_config: TableConfig,
    table_canister: Option<Principal>,
) -> Result<PublicTable, TournamentError> {
    handle_cycle_check_async().await;
    let controllers = CONTROLLER_PRINCIPALS.clone();
    let wasm_module = TABLE_CANISTER_WASM.to_vec();
    let table_canister_principal = if let Some(table_canister) = table_canister {
        table_canister
    } else {
        create_canister_wrapper(controllers, None).await?
    };

    if let Err(e) = install_wasm_code(table_canister_principal, wasm_module).await {
        ic_cdk::println!("Error installing table canister: {:?}", e);
        return Err(TournamentError::CanisterCallError(format!("{:?}", e)));
    }
    let raw_bytes = ic_cdk::management_canister::raw_rand().await;
    let raw_bytes = raw_bytes.map_err(|e| {
        TournamentError::CanisterCallError(format!("Failed to generate random bytes: {:?}", e))
    })?;

    let config = match &tournament_config.tournament_type {
        TournamentType::BuyIn(buy_in_type) => {
            get_table_config(buy_in_type, tournament_config, table_config)
        }
        TournamentType::SitAndGo(buy_in_type) => {
            get_table_config(buy_in_type, tournament_config, table_config)
        }
        TournamentType::SpinAndGo(buy_in_type, _) => {
            get_table_config(buy_in_type, tournament_config, table_config)
        }
        TournamentType::Freeroll(buy_in_type) => {
            get_table_config(buy_in_type, tournament_config, table_config)
        }
    };

    let table = create_table_wrapper(TableId(table_canister_principal), config, raw_bytes).await?;

    Ok(table)
}

fn get_table_config(
    buy_in_type: &TournamentSizeType,
    tournament_config: &TournamentData,
    table_config: TableConfig,
) -> TableConfig {
    match buy_in_type {
        TournamentSizeType::SingleTable(_) | TournamentSizeType::MultiTable(_, _) => TableConfig {
            enable_rake: Some(false),
            name: tournament_config.name.clone()
                + " Table "
                + tournament_config.tables.len().to_string().as_str(),
            game_type: table::poker::game::types::GameType::NoLimit(
                tournament_config.speed_type.get_params().blind_levels[0].small_blind,
            ),
            seats: table_config.seats,
            currency_type: table::poker::game::table_functions::types::CurrencyType::Fake,
            ..table_config
        },
    }
}

pub fn handle_refund(
    wallet_principal_id: WalletPrincipalId,
    amount: u64,
    currency_type: String,
) -> Result<(), TableError> {
    ic_cdk::futures::spawn(async move {
        let currency = match currency_type.as_str() {
            "ICP" => Currency::ICP,
            "USDC" => Currency::CKETHToken(CKTokenSymbol::USDC),
            "USDT" => Currency::CKETHToken(CKTokenSymbol::USDT),
            "ETH" => Currency::CKETHToken(CKTokenSymbol::ETH),
            _ => return,
        };
        let currency_manager = {
            CURRENCY_MANAGER
                .lock()
                .map_err(|_| TournamentError::LockError)
                .unwrap()
                .clone()
        };
        if let Err(e) = currency_manager
            .withdraw(&currency, wallet_principal_id.0, amount)
            .await
        {
            ic_cdk::println!("Failed to refund user: {:?}", e);
        };
    });

    Ok(())
}

pub async fn handle_tournament_deposit(
    currency: CurrencyType,
    amount: u64,
    wallet_principal_id: WalletPrincipalId,
) -> Result<(), TournamentError> {
    match currency {
        CurrencyType::Real(currency) => {
            let currency_manager = {
                CURRENCY_MANAGER
                    .lock()
                    .map_err(|_| TournamentError::LockError)
                    .unwrap()
                    .clone()
            };
            let mut transaction_state = {
                TRANSACTION_STATE
                    .lock()
                    .map_err(|_| TournamentError::LockError)?
                    .clone()
            };
            match currency_manager
                .deposit(
                    &mut transaction_state,
                    &currency,
                    wallet_principal_id.0,
                    amount,
                )
                .await
            {
                Ok(_) => Ok(()),
                Err(e) => {
                    ic_cdk::println!("Error depositing ICP: {:?}", e);
                    Err(TournamentError::TransferFailed(format!("{:?}", e)))
                }
            }
        }
        CurrencyType::Fake => Ok(()),
    }
}

pub fn handle_invalid_join<T>(
    user_id: WalletPrincipalId,
    currency_type: String,
    should_refund: bool,
    error: T,
) -> Result<(), T>
where
    T: std::error::Error,
    T: From<TournamentError>,
{
    let mut tournament_state = TOURNAMENT.lock().map_err(|_| TournamentError::LockError)?;
    let tournament_state = tournament_state.as_mut();
    let tournament_state = match tournament_state {
        Some(tournament_state) => tournament_state,
        None => return Err(error),
    };
    tournament_state.current_players.remove(&user_id);
    if should_refund {
        if let Err(e) = handle_refund(user_id, tournament_state.buy_in, currency_type) {
            ic_cdk::println!("Error refunding user: {:?}", e);
        };
    }
    Err(error)
}

pub async fn check_tournament_end(remaining_players: usize) -> Result<(), TournamentError> {
    if remaining_players == 1 {
        handle_tournament_end_wrapper(TournamentId(ic_cdk::api::canister_self())).await?;
    }

    Ok(())
}

pub async fn delete_table(table_principal: TableId) -> Result<(), TournamentError> {
    handle_cycle_check_async().await;
    let res = is_game_ongoing_wrapper(table_principal).await?;

    if res {
        return Err(TournamentError::CanisterCallError(
            "Cannot delete table with ongoing game".to_string(),
        ));
    } else {
        return_all_cycles_to_index(table_principal).await?;

        stop_and_delete_canister(table_principal.0).await?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn handle_reentry(
    buy_in_type: TournamentSizeType,
    users_canister_id: UsersCanisterId,
    user_id: WalletPrincipalId,
    table_id: TableId,
    tournament_state: &TournamentData,
    currency_type: String,
) -> Result<u64, TournamentError> {
    match buy_in_type {
        TournamentSizeType::SingleTable(buy_in_options)
        | TournamentSizeType::MultiTable(buy_in_options, _) => {
            if !buy_in_options.reentry.enabled {
                return Err(TournamentError::ReentryNotAllowed(
                    "Reentry not enabled".to_string(),
                ));
            }
            if ic_cdk::api::time() >= buy_in_options.reentry.reentry_end_timestamp {
                return Err(TournamentError::ReentryNotAllowed(
                    "Reentry period has ended".to_string(),
                ));
            }
            let user_tournament_data = tournament_state.get_user_tournament_data(&user_id)?;
            if user_tournament_data.reentries >= buy_in_options.reentry.max_reentries {
                return Err(TournamentError::ReentryNotAllowed(
                    "Max rebuys reached".to_string(),
                ));
            }

            handle_tournament_deposit(
                tournament_state.currency,
                buy_in_options.reentry.reentry_price,
                user_id,
            )
            .await?;

            join_table(
                table_id,
                users_canister_id,
                user_id,
                None,
                buy_in_options.reentry.reentry_chips,
                false,
            )
            .await
            .map_err(|e| {
                ic_cdk::println!("Error joining table: {:?}", e);
                if let Err(e) = handle_refund(
                    user_id,
                    buy_in_options.reentry.reentry_price,
                    currency_type.clone(),
                ) {
                    ic_cdk::println!("Error refunding user: {:?}", e);
                };
                TournamentError::CanisterCallError(format!("{:?}", e))
            })?;

            add_to_tournament_prize_pool(buy_in_options.reentry.reentry_price, false)?;

            Ok(buy_in_options.reentry.reentry_price)
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn handle_rebuy(
    buy_in_type: TournamentSizeType,
    users_canister_id: UsersCanisterId,
    user_id: WalletPrincipalId,
    table_id: TableId,
    tournament_state: &TournamentData,
    currency_type: String,
) -> Result<u64, TournamentError> {
    match buy_in_type {
        TournamentSizeType::SingleTable(buy_in_options)
        | TournamentSizeType::MultiTable(buy_in_options, _) => {
            if !buy_in_options.reentry.enabled {
                return Err(TournamentError::ReentryNotAllowed(
                    "Reentry not enabled".to_string(),
                ));
            }
            if ic_cdk::api::time() >= buy_in_options.reentry.reentry_end_timestamp {
                return Err(TournamentError::ReentryNotAllowed(
                    "Reentry period has ended".to_string(),
                ));
            }
            let user_tournament_data = tournament_state.get_user_tournament_data(&user_id)?;
            if user_tournament_data.reentries >= buy_in_options.reentry.max_reentries {
                return Err(TournamentError::ReentryNotAllowed(
                    "Max rebuys reached".to_string(),
                ));
            }

            handle_tournament_deposit(
                tournament_state.currency,
                buy_in_options.reentry.reentry_price,
                user_id,
            )
            .await?;

            join_table(
                table_id,
                users_canister_id,
                user_id,
                None,
                buy_in_options.reentry.reentry_chips,
                false,
            )
            .await
            .map_err(|e| {
                ic_cdk::println!("Error joining table: {:?}", e);
                if let Err(e) = handle_refund(
                    user_id,
                    buy_in_options.reentry.reentry_price,
                    currency_type.clone(),
                ) {
                    ic_cdk::println!("Error refunding user: {:?}", e);
                };
                TournamentError::CanisterCallError(format!("{:?}", e))
            })?;

            add_to_tournament_prize_pool(buy_in_options.reentry.reentry_price, false)?;

            Ok(buy_in_options.reentry.reentry_price)
        }
    }
}

pub async fn handle_addon(
    buy_in_type: &TournamentSizeType,
    users_canister_id: UsersCanisterId,
    user_id: WalletPrincipalId,
    table_id: TableId,
    currency_type: String,
    tournament: &TournamentData,
) -> Result<u64, TournamentError> {
    match buy_in_type {
        TournamentSizeType::SingleTable(buy_in_options)
        | TournamentSizeType::MultiTable(buy_in_options, _) => {
            let user_data = tournament.get_user_tournament_data(&user_id)?;
            if !buy_in_options.can_addon(user_data.addons) {
                return Err(TournamentError::AddonNotAllowed(
                    "Addon not enabled".to_string(),
                ));
            }

            if buy_in_options.addon.addon_start_time > ic_cdk::api::time() {
                return Err(TournamentError::AddonNotAllowed(
                    "Addon period has not started".to_string(),
                ));
            } else if buy_in_options.addon.addon_end_time < ic_cdk::api::time() {
                return Err(TournamentError::AddonNotAllowed(
                    "Addon period has ended".to_string(),
                ));
            }

            handle_tournament_deposit(
                tournament.currency,
                buy_in_options.addon.addon_price,
                user_id,
            )
            .await?;

            let _ = deposit_to_table(
                table_id,
                users_canister_id,
                user_id,
                buy_in_options.addon.addon_chips,
                false,
            )
            .await
            .map_err(|e| {
                if let Err(e) = handle_refund(
                    user_id,
                    buy_in_options.addon.addon_price,
                    currency_type.clone(),
                ) {
                    ic_cdk::println!("Error refunding user: {:?}", e);
                };
                TournamentError::CanisterCallError(format!("{:?}", e))
            })?;

            add_to_tournament_prize_pool(buy_in_options.addon.addon_price, false)?;

            Ok(buy_in_options.addon.addon_price)
        }
    }
}

pub async fn handle_lost_user_rebuy_availability(
    buy_in_type: &TournamentSizeType,
    user_principal: WalletPrincipalId,
    table_id: TableId,
    tournament: &TournamentData,
) -> Result<(), TournamentError> {
    match buy_in_type {
        TournamentSizeType::SingleTable(buy_in_options)
        | TournamentSizeType::MultiTable(buy_in_options, _) => {
            if buy_in_options.rebuy.enabled
                && ic_cdk::api::time() < buy_in_options.rebuy.rebuy_end_timestamp
            {
                let user_data = tournament.get_user_tournament_data(&user_principal)?;
                if user_data.rebuys < buy_in_options.rebuy.max_rebuys {
                    let current_user_rebuys = user_data.rebuys;
                    ic_cdk_timers::set_timer(
                        std::time::Duration::from_secs(buy_in_options.rebuy.rebuy_window_seconds),
                        move || {
                            ic_cdk::futures::spawn(async move {
                                let tournament = {
                                    let tournament = match TOURNAMENT.lock() {
                                        Ok(tournament) => tournament,
                                        Err(e) => {
                                            ic_cdk::println!("Error getting tournament: {:?}", e);
                                            return;
                                        }
                                    };
                                    match tournament
                                        .as_ref()
                                        .ok_or(TournamentError::TournamentNotFound)
                                    {
                                        Ok(tournament) => tournament.clone(),
                                        Err(e) => {
                                            ic_cdk::println!("Error getting tournament: {:?}", e);
                                            return;
                                        }
                                    }
                                };

                                // Check if they rebought by comparing rebuy count
                                let user_data =
                                    match tournament.get_user_tournament_data(&user_principal) {
                                        Ok(user_data) => user_data,
                                        Err(e) => {
                                            ic_cdk::println!("Error getting user data: {:?}", e);
                                            return;
                                        }
                                    };

                                if user_data.rebuys == current_user_rebuys {
                                    // They didn't rebuy in time, eliminate them
                                    if let Err(e) =
                                        handle_user_kick(Some(table_id), user_principal).await
                                    {
                                        ic_cdk::println!("Error kicking user: {:?}", e);
                                    }
                                } else {
                                    let mut tournament = match TOURNAMENT.lock() {
                                        Ok(tournament) => tournament,
                                        Err(e) => {
                                            ic_cdk::println!("Error getting tournament: {:?}", e);
                                            return;
                                        }
                                    };
                                    match tournament
                                        .as_mut()
                                        .ok_or(TournamentError::TournamentNotFound)
                                    {
                                        Ok(tournament) => {
                                            if let Some(user_data) =
                                                tournament.current_players.get_mut(&user_principal)
                                            {
                                                user_data.rebuys += 1;
                                            }
                                        }
                                        Err(e) => {
                                            ic_cdk::println!("Error getting tournament: {:?}", e);
                                        }
                                    }
                                }
                            });
                        },
                    );
                    return Ok(());
                }
            } else {
                handle_user_kick(Some(table_id), user_principal).await?;
            }
        }
    }
    Ok(())
}

pub async fn handle_user_kick(
    table_id: Option<TableId>,
    user_principal: WalletPrincipalId,
) -> Result<(), TournamentError> {
    let tournament = {
        let mut tournament = match TOURNAMENT.lock() {
            Ok(tournament) => tournament,
            Err(e) => {
                ic_cdk::println!("Error getting tournament: {:?}", e);
                return Err(TournamentError::LockError);
            }
        };
        let tournament = match tournament.as_mut() {
            Some(tournament) => tournament,
            None => {
                ic_cdk::println!("Tournament not found");
                return Err(TournamentError::TournamentNotFound);
            }
        };

        move_player_from_current_players_to_all_players(tournament, &vec![user_principal])?;

        for table in tournament.tables.values_mut() {
            table.players.retain(|&x| x != user_principal);
        }
        tournament.clone()
    };

    let user_tournament_data =
        tournament
            .all_players
            .get(&user_principal)
            .ok_or(TournamentError::Other(
                "User tournament data not found".to_string(),
            ))?;

    match LEADERBOARD.lock() {
        Ok(mut leaderboard) => {
            if !leaderboard.contains(&user_principal) {
                leaderboard.push(user_principal)
            }
        }
        Err(e) => {
            ic_cdk::println!("Error getting leaderboard: {:?}", e);
            return Err(TournamentError::LockError);
        }
    }

    let remaining_players = tournament.current_players.len();

    if remaining_players <= 1 {
        match check_tournament_end(remaining_players).await {
            Ok(_) => {}
            Err(e) => {
                ic_cdk::println!("Error checking tournament end: {:?}", e);
                return Err(e);
            }
        };
    }

    if let Some(table_id) = table_id {
        let res = leave_table_wrapper(
            table_id,
            user_tournament_data.users_canister_principal,
            user_principal,
        )
        .await;
        if let Err(e) = res {
            ic_cdk::println!("Error leaving table: {:?}", e);
        }
    }
    Ok(())
}

pub fn move_player_from_current_players_to_all_players(
    tournament: &mut TournamentData,
    user_principals: &Vec<WalletPrincipalId>,
) -> Result<(), TournamentError> {
    for user_principal in user_principals {
        let user_data = tournament
            .current_players
            .get(user_principal)
            .ok_or(TournamentError::Other(
                "User tournament data not found.".to_owned(),
            ))?
            .clone();
        tournament.current_players.remove(user_principal);
        tournament.all_players.insert(*user_principal, user_data);
    }
    Ok(())
}

pub async fn transfer_cycles_to_tournament_index(
    cycles_amount: u128,
) -> Result<(), TournamentError> {
    let backend_principal = TOURNAMENT_INDEX
        .lock()
        .map_err(|_| TournamentError::LockError)?
        .ok_or(TournamentError::CanisterCallError(
            "Backend principal not found.".to_string(),
        ))?;

    // Transfer all cycles to the index canister
    let res = ic_cdk::management_canister::deposit_cycles(
        &DepositCyclesArgs {
            canister_id: backend_principal,
        },
        cycles_amount,
    )
    .await;

    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(TournamentError::CanisterCallError(format!(
            "Failed to send cycles: {:?}",
            e
        ))),
    }
}

pub async fn update_tournament_state(new_state: TournamentState) -> Result<(), TournamentError> {
    let tournament_id = {
        let mut tournament = TOURNAMENT.lock().map_err(|_| TournamentError::LockError)?;
        let tournament = tournament
            .as_mut()
            .ok_or(TournamentError::TournamentNotFound)?;
        tournament.state = new_state.clone();
        tournament.id
    };
    update_tournament_state_wrapper(tournament_id, new_state)
        .await
        .map_err(|e| TournamentError::CanisterCallError(format!("{:?}", e)))
}

pub async fn update_tournament_state_wrapper(
    tournament_id: TournamentId,
    new_state: TournamentState,
) -> Result<(), TournamentIndexError> {
    let tournament_index = {
        let tournament_index = TOURNAMENT_INDEX
            .lock()
            .map_err(|_| TournamentIndexError::LockError)?;
        *tournament_index
            .as_ref()
            .ok_or(TournamentIndexError::CanisterCallFailed(
                "Tournament index not found".to_string(),
            ))?
    };
    update_tournament_state_icc_wrapper(tournament_index, tournament_id, new_state).await?;
    Ok(())
}

pub fn add_to_tournament_prize_pool(amount: u64, is_admin: bool) -> Result<(), TournamentError> {
    if !is_admin {
        let (prize_pool, rake_amount) = calculate_rake(amount)?;
        PRIZE_POOL.fetch_add(prize_pool, Ordering::SeqCst);
        RAKE_AMOUNT.fetch_add(rake_amount, Ordering::SeqCst);
    } else {
        PRIZE_POOL.fetch_add(amount, Ordering::SeqCst);
    }
    Ok(())
}

pub async fn update_live_leaderboard() -> Result<(), TournamentError> {
    let mut tournament = {
        let tournament = TOURNAMENT.lock().map_err(|_| TournamentError::LockError)?;
        tournament
            .as_ref()
            .ok_or(TournamentError::TournamentNotFound)?
            .clone()
    };

    // If tournament is completed, use the final leaderboard
    if tournament.state == TournamentState::Completed {
        if let Some(sorted_users) = &tournament.sorted_users {
            let mut live_leaderboard = LIVE_LEADERBOARD
                .lock()
                .map_err(|_| TournamentError::LockError)?;
            *live_leaderboard = sorted_users
                .iter()
                .map(|p| {
                    (
                        p.0,
                        tournament
                            .all_players
                            .get(&p.0)
                            .map_or(0, |data| data.chips),
                    )
                })
                .collect();
            return Ok(());
        }
    }

    // For running tournaments, get chip counts from tables
    let mut player_chips = HashMap::new();
    let mut updated_tournament = false;

    // Then update with latest data from tables
    for (table_id, table_info) in &tournament.tables {
        // Get table data to get current chip counts
        match get_table_wrapper(*table_id).await {
            Ok(table) => {
                if table.users.is_empty() {
                    continue;
                }

                for (principal, user_data) in &table.users.users {
                    if tournament.current_players.contains_key(principal) {
                        player_chips.insert(*principal, user_data.balance.0);

                        // Update UserTournamentData with latest chip count
                        if let Some(user_tournament_data) =
                            tournament.current_players.get_mut(principal)
                        {
                            if user_tournament_data.chips != user_data.balance {
                                user_tournament_data.chips = user_data.balance.0;
                                updated_tournament = true;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                ic_cdk::println!("Error fetching table data: {:?}", e);
                // Continue with existing chip count data if we can't get table data
                for principal in &table_info.players {
                    if let Some(user_data) = tournament.current_players.get(principal) {
                        player_chips.insert(*principal, user_data.chips);
                    }
                }
            }
        }
    }

    // Update the tournament state if chip counts were updated
    if updated_tournament {
        let mut tournament_lock = TOURNAMENT.lock().map_err(|_| TournamentError::LockError)?;
        if let Some(tournament_data) = tournament_lock.as_mut() {
            // Update current_players with new chip values
            for (principal, chips) in &player_chips {
                if let Some(user_data) = tournament_data.current_players.get_mut(principal) {
                    user_data.chips = *chips;
                }
            }
        }
    }

    // Convert to sorted vec
    let mut players_with_chips: Vec<(WalletPrincipalId, u64)> = player_chips.into_iter().collect();

    // Sort by chip count in descending order
    players_with_chips.sort_by(|a, b| b.1.cmp(&a.1));

    // Update the live leaderboard
    let mut live_leaderboard = LIVE_LEADERBOARD
        .lock()
        .map_err(|_| TournamentError::LockError)?;
    *live_leaderboard = players_with_chips;

    // Update timestamp
    LAST_LEADERBOARD_UPDATE.store(ic_cdk::api::time(), Ordering::Relaxed);

    Ok(())
}
