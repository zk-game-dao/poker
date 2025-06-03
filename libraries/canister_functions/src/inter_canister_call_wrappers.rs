use candid::Principal;
use errors::{
    table_error::TableError, tournament_error::TournamentError,
    tournament_index_error::TournamentIndexError, user_error::UserError,
};
use ic_cdk::api::management_canister::main::{
    canister_status, update_settings, CanisterIdRecord, CanisterSettings, UpdateSettingsArgument,
};
use table::{poker::game::types::PublicTable, types::ReturnResult};
use user::user::User;

pub async fn get_user(user_principal: Principal, user_id: Principal) -> Result<User, UserError> {
    let call_result: Result<(Result<User, UserError>,), _> =
        ic_cdk::call(user_principal, "get_user", (user_id,)).await;

    match call_result {
        Ok((user_result,)) => match user_result {
            Ok(user) => Ok(user),
            Err(err) => {
                ic_cdk::println!("Error getting user: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_user call: {:?}", err);
            Err(UserError::CanisterCallFailed(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn get_table(table_principal: Principal) -> Result<PublicTable, TableError> {
    let call_result: Result<(Result<PublicTable, TableError>,), _> =
        ic_cdk::call(table_principal, "get_table", ()).await;

    match call_result {
        Ok((table_result,)) => match table_result {
            Ok(table) => Ok(table),
            Err(err) => {
                ic_cdk::println!("Error getting table: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_table call: {:?}", err);
            Err(TableError::CanisterCallError(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn get_players_on_table(
    table_principal: Principal,
) -> Result<Vec<Principal>, TableError> {
    let call_result: Result<(Result<Vec<Principal>, TableError>,), _> =
        ic_cdk::call(table_principal, "get_players_on_table", ()).await;

    match call_result {
        Ok((table_result,)) => match table_result {
            Ok(table) => Ok(table),
            Err(err) => {
                ic_cdk::println!("Error getting tables player count: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_players_on_table call: {:?}", err);
            Err(TableError::CanisterCallError(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn get_free_seat_index(table_id: Principal) -> Result<Option<u8>, TableError> {
    let call_result: Result<(Result<Option<u8>, TableError>,), _> =
        ic_cdk::call(table_id, "get_free_seat_index", ()).await;

    match call_result {
        Ok((seat_index_result,)) => match seat_index_result {
            Ok(seat_index) => Ok(seat_index),
            Err(err) => {
                ic_cdk::println!("Error getting free seat index: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_free_seat_index call: {:?}", err);
            Err(TableError::CanisterCallError(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn get_seat_index(player: Principal, table: Principal) -> Result<Option<u8>, TableError> {
    let call_result: Result<(Result<Option<u8>, TableError>,), _> =
        ic_cdk::call(table, "get_seat_index", (player,)).await;

    match call_result {
        Ok((seat_index_result,)) => match seat_index_result {
            Ok(seat_index) => Ok(seat_index),
            Err(err) => {
                ic_cdk::println!("Error getting seat index: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_seat_index call: {:?}", err);
            Err(TableError::CanisterCallError(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn leave_table(
    table: Principal,
    users_canister_id: Principal,
    user_id: Principal,
) -> Result<PublicTable, TableError> {
    let call_result: Result<(Result<PublicTable, TableError>,), _> =
        ic_cdk::call(table, "leave_table", (users_canister_id, user_id)).await;

    match call_result {
        Ok((leave_result,)) => match leave_result {
            Ok(res) => Ok(res),
            Err(err) => {
                ic_cdk::println!("Error leaving table: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in leave_table call: {:?}", err);
            Err(TableError::CanisterCallError(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn leave_table_for_table_balancing(
    users_canister_id: Principal,
    user_id: Principal,
    table: Principal,
    to_table: Principal,
) -> Result<PublicTable, TableError> {
    let call_result: Result<(Result<PublicTable, TableError>,), _> = ic_cdk::call(
        table,
        "leave_table_for_table_balancing",
        (users_canister_id, user_id, to_table),
    )
    .await;

    match call_result {
        Ok((leave_result,)) => match leave_result {
            Ok(res) => Ok(res),
            Err(err) => {
                ic_cdk::println!("Error leaving table for table balancing: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in leave_table_for_table_balancing call: {:?}", err);
            Err(TableError::CanisterCallError(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn join_table(
    table_id: Principal,
    users_canister_principal: Principal,
    user_id: Principal,
    seat_index: Option<u64>, // javascript can't send u8
    deposit_amount: u64,
    player_sitting_out: bool,
) -> Result<PublicTable, TableError> {
    let call_result: Result<(Result<PublicTable, TableError>,), _> = ic_cdk::call(
        table_id,
        "join_table",
        (
            users_canister_principal,
            user_id,
            seat_index,
            deposit_amount,
            player_sitting_out,
        ),
    )
    .await;

    match call_result {
        Ok((join_result,)) => match join_result {
            Ok(table) => Ok(table),
            Err(err) => {
                ic_cdk::println!("Error joining table: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in join_table call: {:?}", err);
            Err(TableError::CanisterCallError(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn clear_table(table_id: Principal) -> Result<(), TableError> {
    let call_result: Result<(Result<(), TableError>,), _> =
        ic_cdk::call(table_id, "clear_table", ()).await;

    match call_result {
        Ok((clear_result,)) => match clear_result {
            Ok(_) => Ok(()),
            Err(err) => {
                ic_cdk::println!("Error clearing table: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in clear_table call: {:?}", err);
            Err(TableError::CanisterCallError(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn player_sitting_in(
    table_id: Principal,
    user_principal: Principal,
    auto_start: bool,
) -> Result<(), TableError> {
    let call_result: Result<(Result<(), TableError>,), _> =
        ic_cdk::call(table_id, "player_sitting_in", (user_principal, auto_start)).await;

    match call_result {
        Ok((res,)) => match res {
            Ok(_) => Ok(()),
            Err(err) => {
                ic_cdk::println!("Error sitting in player: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in player_sitting_in call: {:?}", err);
            Err(TableError::CanisterCallError(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn start_new_betting_round(table_id: Principal) -> Result<(), TableError> {
    let call_result: Result<(Result<(), TableError>,), _> =
        ic_cdk::call(table_id, "start_new_betting_round", ()).await;

    match call_result {
        Ok((res,)) => match res {
            Ok(_) => Ok(()),
            Err(err) => {
                ic_cdk::println!("Error starting new betting round: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in start_new_betting_round call: {:?}", err);
            Err(TableError::CanisterCallError(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn pause_table_for_addon_wrapper(
    table_id: Principal,
    duration: u64,
) -> Result<(), TableError> {
    let call_result: Result<(Result<(), TableError>,), _> =
        ic_cdk::call(table_id, "pause_table_for_addon", (duration,)).await;

    match call_result {
        Ok((res,)) => match res {
            Ok(_) => Ok(()),
            Err(err) => {
                ic_cdk::println!("Error pausing table for addon: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in pause_table_for_addon call: {:?}", err);
            Err(TableError::CanisterCallError(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn pause_table(table_id: Principal) -> Result<(), TableError> {
    let call_result: Result<(Result<(), TableError>,), _> =
        ic_cdk::call(table_id, "pause_table", ()).await;

    match call_result {
        Ok((res,)) => match res {
            Ok(_) => Ok(()),
            Err(err) => {
                ic_cdk::println!("Error pausing table: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in pause_table call: {:?}", err);
            Err(TableError::CanisterCallError(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn resume_table_wrapper(table_id: Principal) -> Result<(), TableError> {
    let call_result: Result<(Result<(), TableError>,), _> =
        ic_cdk::call(table_id, "resume_table", ()).await;

    match call_result {
        Ok((res,)) => match res {
            Ok(_) => Ok(()),
            Err(err) => {
                ic_cdk::println!("Error resuming table: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in resume_table call: {:?}", err);
            Err(TableError::CanisterCallError(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn user_leave_tournament_wrapper(
    tournament_id: Principal,
    user_principal: Principal,
    wallet_principal_id: Principal,
    table_id: Principal,
) -> Result<(), TournamentError> {
    let call_result: Result<(Result<(), TournamentError>,), _> = ic_cdk::call(
        tournament_id,
        "user_leave_tournament",
        (user_principal, wallet_principal_id, table_id),
    )
    .await;

    match call_result {
        Ok((res,)) => match res {
            Ok(_) => Ok(()),
            Err(err) => {
                ic_cdk::println!("Error leaving tournament: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in user_leave_tournament call: {:?}", err);
            Err(TournamentError::CanisterCallError(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn handle_cancelled_tournament_wrapper(
    tournament_id: Principal,
) -> Result<(), TournamentError> {
    let call_result: Result<(Result<(), TournamentError>,), _> =
        ic_cdk::call(tournament_id, "handle_cancelled_tournament", ()).await;

    match call_result {
        Ok((res,)) => match res {
            Ok(_) => Ok(()),
            Err(err) => {
                ic_cdk::println!("Error handling cancelled tournament: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in handle_cancelled_tournament call: {:?}", err);
            Err(TournamentError::CanisterCallError(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn return_all_cycles_to_index(table_id: Principal) -> Result<(), TableError> {
    let call_result: Result<(Result<(), TableError>,), _> =
        ic_cdk::call(table_id, "return_all_cycles_to_index", ()).await;

    match call_result {
        Ok((res,)) => match res {
            Ok(_) => Ok(()),
            Err(err) => {
                ic_cdk::println!("Error returning all cycles to index: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in return_all_cycles_to_index call: {:?}", err);
            Err(TableError::CanisterCallError(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn return_all_cycles_to_tournament_index(
    tournament_id: Principal,
) -> Result<(), TournamentError> {
    let call_result: Result<(Result<(), TournamentError>,), _> =
        ic_cdk::call(tournament_id, "return_all_cycles_to_tournament_index", ()).await;

    match call_result {
        Ok((res,)) => match res {
            Ok(_) => Ok(()),
            Err(err) => {
                ic_cdk::println!("Error returning all cycles to tournament index: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!(
                "Error in return_all_cycles_to_tournament_index call: {:?}",
                err
            );
            Err(TournamentError::CanisterCallError(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn add_to_table_pool_wrapper(
    tournament_index_id: Principal,
    table_principal: Principal,
) -> Result<(), TournamentIndexError> {
    let call_result: Result<(Result<(), TournamentIndexError>,), _> =
        ic_cdk::call(tournament_index_id, "add_to_pool", (table_principal,)).await;

    match call_result {
        Ok((add_result,)) => match add_result {
            Ok(_) => Ok(()),
            Err(err) => {
                ic_cdk::println!("Error adding to table pool: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in add_to_pool call: {:?}", err);
            Err(TournamentIndexError::CanisterCallFailed(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn get_and_remove_from_pool_wrapper(
    tournament_index_id: Principal,
) -> Result<Option<Principal>, TournamentIndexError> {
    let call_result: Result<(Option<Principal>,), _> =
        ic_cdk::call(tournament_index_id, "get_and_remove_from_pool", ()).await;

    match call_result {
        Ok((principal_opt,)) => Ok(principal_opt),
        Err(err) => {
            ic_cdk::println!("Error in get_and_remove_from_pool call: {:?}", err);
            Err(TournamentIndexError::CanisterCallFailed(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn set_as_final_table_wrapper(table_id: Principal) -> Result<(), TableError> {
    let call_result: Result<(Result<(), TableError>,), _> =
        ic_cdk::call(table_id, "set_as_final_table", ()).await;

    match call_result {
        Ok((set_result,)) => match set_result {
            Ok(_) => Ok(()),
            Err(err) => {
                ic_cdk::println!("Error setting table as final table: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in set_as_final_table call: {:?}", err);
            Err(TableError::CanisterCallError(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn ensure_principal_is_controller(
    canister_id: Principal,
    principal: Principal,
) -> Result<(), String> {
    // Get the current canister status to check controllers
    let canister_status = match canister_status(CanisterIdRecord { canister_id }).await {
        Ok((status,)) => status,
        Err((code, msg)) => {
            return Err(format!(
                "Failed to get canister status: {:?} - {}",
                code, msg
            ))
        }
    };

    // Check if the principal is already a controller
    let is_already_controller = canister_status
        .settings
        .controllers.contains(&principal);

    // If principal is already a controller, we're done
    if is_already_controller {
        ic_cdk::println!(
            "Principal {:?} is already a controller of canister {:?}",
            principal,
            canister_id
        );
        return Ok(());
    }

    // Principal is not a controller, so we need to add it
    ic_cdk::println!(
        "Adding principal {:?} as controller for canister {:?}",
        principal,
        canister_id
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
    };

    // Update the canister settings
    match update_settings(UpdateSettingsArgument {
        canister_id,
        settings,
    })
    .await
    {
        Ok(()) => {
            ic_cdk::println!("Successfully added principal as controller");
            Ok(())
        }
        Err((code, msg)) => Err(format!(
            "Failed to update canister settings: {:?} - {}",
            code, msg
        )),
    }
}

pub async fn user_join_tournament(
    tournament_id: Principal,
    user_principal: Principal,
    wallet_principal_id: Principal,
) -> Result<(), TournamentError> {
    let call_result: Result<(Result<(), TournamentError>,), _> = ic_cdk::call(
        tournament_id,
        "user_join_tournament",
        (user_principal, wallet_principal_id),
    )
    .await;

    match call_result {
        Ok((res,)) => match res {
            Ok(_) => Ok(()),
            Err(err) => {
                ic_cdk::println!("Error joining tournament: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in user_join_tournament call: {:?}", err);
            Err(TournamentError::CanisterCallError(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn remove_users_active_table(
    users_canister_id: Principal,
    user_id: Principal,
) -> Result<User, UserError> {
    let call_result: Result<(Result<User, UserError>,), _> = ic_cdk::call(
        users_canister_id,
        "remove_active_table",
        (ic_cdk::api::id(), user_id),
    )
    .await;

    match call_result {
        Ok((user_result,)) => match user_result {
            Ok(user) => Ok(user),
            Err(err) => {
                ic_cdk::println!("Error removing active table: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in remove_active_table call: {:?}", err);
            Err(UserError::CanisterCallFailed(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

pub async fn deposit_to_table(
    table_id: Principal,
    users_canister_id: Principal,
    user_id: Principal,
    amount: u64,
    is_queued: bool,
) -> Result<ReturnResult, TableError> {
    let call_result: Result<(Result<ReturnResult, TableError>,), _> = ic_cdk::call(
        table_id,
        "deposit_to_table",
        (users_canister_id, user_id, amount, is_queued),
    )
    .await;

    match call_result {
        Ok((res,)) => match res {
            Ok(result) => Ok(result),
            Err(err) => {
                ic_cdk::println!("Error depositing to table: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in deposit_to_table call: {:?}", err);
            Err(TableError::CanisterCallError(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}

#[ic_cdk::update]
pub async fn get_users_canister_principal_by_id_wrapper(index_principal: Principal, user_id: Principal) -> Result<Principal, UserError> {
    let call_result: Result<(Result<Principal, UserError>,), _> =
        ic_cdk::call(index_principal, "get_users_canister_principal_by_id", (user_id,)).await;

    match call_result {
        Ok((users_canister_result,)) => match users_canister_result {
            Ok(users_canister) => Ok(users_canister),
            Err(err) => {
                ic_cdk::println!("Error getting users canister principal: {:?}", err);
                Err(err)
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_users_canister_principal_by_id call: {:?}", err);
            Err(UserError::CanisterCallFailed(format!(
                "{:?}: {}",
                err.0, err.1
            )))
        }
    }
}