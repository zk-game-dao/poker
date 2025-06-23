use candid::Principal;
use errors::{
    table_error::TableError, tournament_error::TournamentError,
    tournament_index_error::TournamentIndexError,
};
use ic_cdk::management_canister::{
    CanisterSettings, CanisterStatusArgs, UpdateSettingsArgs, canister_status, update_settings,
};
use table::poker::game::{table_functions::table::{TableConfig, TableId}, types::PublicTable};
use tournaments::tournaments::{
    blind_level::BlindLevel,
    types::{TournamentData, TournamentId, TournamentState, UserTournamentAction},
};
use user::user::{UsersCanisterId, WalletPrincipalId};

pub async fn create_tournament_wrapper(
    tournament_id: TournamentId,
    tournament_data: TournamentData,
    table_config: TableConfig,
    prize_pool: u64,
) -> Result<TournamentData, TournamentError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(tournament_id.0, "create_tournament")
        .with_args(&(tournament_data, table_config, prize_pool))
        .await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error creating tournament: {:?}", err);
                Err(TournamentError::CanisterCallError(format!(
                    "Failed to decode create_tournament response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in create_tournament call: {:?}", err);
            Err(TournamentError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn user_leave_tournament_wrapper(
    tournament_id: TournamentId,
    user_principal: UsersCanisterId,
    wallet_principal_id: WalletPrincipalId,
    table_id: TableId,
) -> Result<(), TournamentError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(tournament_id.0, "user_leave_tournament")
        .with_args(&(user_principal, wallet_principal_id, table_id))
        .await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error leaving tournament: {:?}", err);
                Err(TournamentError::CanisterCallError(format!(
                    "Failed to decode user_leave_tournament response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in user_leave_tournament call: {:?}", err);
            Err(TournamentError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn handle_cancelled_tournament_wrapper(
    tournament_id: TournamentId,
) -> Result<(), TournamentError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(tournament_id.0, "handle_cancelled_tournament").await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error handling cancelled tournament: {:?}", err);
                Err(TournamentError::CanisterCallError(format!(
                    "Failed to decode handle_cancelled_tournament response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in handle_cancelled_tournament call: {:?}", err);
            Err(TournamentError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn return_all_cycles_to_tournament_index_wrapper(
    tournament_id: TournamentId,
) -> Result<(), TournamentError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(tournament_id.0, "return_all_cycles_to_tournament_index")
            .await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error returning all cycles to tournament index: {:?}", err);
                Err(TournamentError::CanisterCallError(format!(
                    "Failed to decode return_all_cycles_to_tournament_index response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!(
                "Error in return_all_cycles_to_tournament_index call: {:?}",
                err
            );
            Err(TournamentError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn add_to_table_pool_wrapper(
    tournament_index_id: Principal,
    table_principal: TableId,
) -> Result<(), TournamentIndexError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(tournament_index_id, "add_to_pool")
        .with_arg(table_principal)
        .await;

    match call_result {
        Ok(add_result) => match add_result.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error adding to table pool: {:?}", err);
                Err(TournamentIndexError::CanisterCallFailed(format!(
                    "Failed to decode add_to_pool response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in add_to_pool call: {:?}", err);
            Err(TournamentIndexError::CanisterCallFailed(format!(
                "{:?}",
                err
            )))
        }
    }
}

pub async fn get_and_remove_from_pool_wrapper(
    tournament_index_id: Principal,
) -> Result<Option<Principal>, TournamentIndexError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(tournament_index_id, "get_and_remove_from_pool").await;

    match call_result {
        Ok(principal_opt) => match principal_opt.candid() {
            Ok(principal) => principal,
            Err(err) => {
                ic_cdk::println!("Error decoding principal from pool: {:?}", err);
                Err(TournamentIndexError::CanisterCallFailed(format!(
                    "Failed to decode principal: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_and_remove_from_pool call: {:?}", err);
            Err(TournamentIndexError::CanisterCallFailed(format!(
                "{:?}",
                err
            )))
        }
    }
}

pub async fn ensure_principal_is_controller(
    canister_id: Principal,
    principal: Principal,
) -> Result<(), String> {
    // Get the current canister status to check controllers
    let canister_status = match canister_status(&CanisterStatusArgs { canister_id }).await {
        Ok(status) => status,
        Err(e) => return Err(format!("Failed to get canister status: {:?}", e)),
    };

    // Check if the principal is already a controller
    let is_already_controller = canister_status.settings.controllers.contains(&principal);

    // If principal is already a controller, we're done
    if is_already_controller {
        ic_cdk::println!(
            "Principal {:?} is already a controller of canister {:?}",
            principal.to_text(),
            canister_id.to_text()
        );
        return Ok(());
    }

    // Principal is not a controller, so we need to add it
    ic_cdk::println!(
        "Adding principal {:?} as controller for canister {:?}",
        principal.to_text(),
        canister_id.to_text()
    );

    // Get the current controllers and append the new principal
    let mut new_controllers = canister_status.settings.controllers;
    new_controllers.push(principal);

    // Create updated settings with the new controllers list
    let settings = CanisterSettings {
        controllers: Some(new_controllers),
        compute_allocation: None,
        memory_allocation: None,
        freezing_threshold: None,
        reserved_cycles_limit: None,
        wasm_memory_limit: None,
        log_visibility: None,
        wasm_memory_threshold: None,
    };

    // Update the canister settings
    match update_settings(&UpdateSettingsArgs {
        canister_id,
        settings,
    })
    .await
    {
        Ok(()) => {
            ic_cdk::println!("Successfully added principal as controller");
            Ok(())
        }
        Err(e) => Err(format!("Failed to update canister settings: {:?}", e)),
    }
}

pub async fn user_join_tournament(
    tournament_id: TournamentId,
    user_principal: UsersCanisterId,
    wallet_principal_id: WalletPrincipalId,
) -> Result<(), TournamentError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(tournament_id.0, "user_join_tournament")
        .with_args(&(user_principal, wallet_principal_id))
        .await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error joining tournament: {:?}", err);
                Err(TournamentError::CanisterCallError(format!(
                    "Failed to decode user_join_tournament response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in user_join_tournament call: {:?}", err);
            Err(TournamentError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn update_player_count_tournament_wrapper(
    tournament_id: TournamentId,
    table_id: Principal,
    user_action: UserTournamentAction,
) -> Result<(), TournamentError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(tournament_id.0, "update_player_count_tournament")
            .with_args(&(table_id, user_action))
            .await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error updating player count in tournament: {:?}", err);
                Err(TournamentError::CanisterCallError(format!(
                    "Failed to decode update_player_count_tournament response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in update_player_count_tournament call: {:?}", err);
            Err(TournamentError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn distribute_winnings_wrapper(
    tournament_id: TournamentId,
    table: PublicTable,
) -> Result<(), TournamentError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(tournament_id.0, "distribute_winnings")
        .with_arg(table)
        .await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error distributing winnings: {:?}", err);
                Err(TournamentError::CanisterCallError(format!(
                    "Failed to decode distribute_winnings response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in distribute_winnings call: {:?}", err);
            Err(TournamentError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn handle_tournament_end_wrapper(
    tournament_id: TournamentId,
) -> Result<(), TournamentError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(tournament_id.0, "handle_tournament_end").await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error handling tournament end: {:?}", err);
                Err(TournamentError::CanisterCallError(format!(
                    "Failed to decode handle_tournament_end response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in handle_tournament_end call: {:?}", err);
            Err(TournamentError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn update_tournament_state_icc_wrapper(
    tournament_index: Principal,
    tournament_id: TournamentId,
    new_state: TournamentState,
) -> Result<(), TournamentIndexError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(tournament_index, "update_tournament_state")
            .with_args(&(tournament_id, new_state))
            .await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error updating tournament state: {:?}", err);
                Err(TournamentIndexError::CanisterCallFailed(format!(
                    "Failed to decode update_tournament_state response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in update_tournament_state call: {:?}", err);
            Err(TournamentIndexError::CanisterCallFailed(format!(
                "{:?}",
                err
            )))
        }
    }
}

pub async fn update_blinds(table_id: Principal, new_level: &BlindLevel) -> Result<(), TableError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(table_id, "update_blinds")
        .with_args(&(
            new_level.small_blind,
            new_level.big_blind,
            new_level.ante_type.clone(),
        ))
        .await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error updating blinds: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode update_blinds response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in update_blinds call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
        }
    }
}
