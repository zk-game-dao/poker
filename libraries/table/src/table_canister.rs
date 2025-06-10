use crate::{
    poker::game::{table_functions::table::TableConfig, types::PublicTable},
    types::ReturnResult,
};
use candid::Principal;
use errors::{table_error::TableError, tournament_error::TournamentError, user_error::UserError};
use user::user::User;

pub async fn create_table_wrapper(
    table_id: Principal,
    config: TableConfig,
    raw_bytes: Vec<u8>,
) -> Result<PublicTable, TableError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(table_id, "create_table")
        .with_args(&(config, raw_bytes))
        .await;

    match call_result {
        Ok(table_result) => match table_result.candid() {
            Ok(table) => table,
            Err(err) => {
                ic_cdk::println!("Error creating table: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode create_table response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in create_table call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn get_table_wrapper(table_principal: Principal) -> Result<PublicTable, TableError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(table_principal, "get_table").await;

    match call_result {
        Ok(table_result) => match table_result.candid() {
            Ok(table) => table,
            Err(err) => {
                ic_cdk::println!("Error getting table: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode table: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_table call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn get_players_on_table(
    table_principal: Principal,
) -> Result<Vec<Principal>, TableError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(table_principal, "get_players_on_table").await;

    match call_result {
        Ok(table_result) => match table_result.candid() {
            Ok(table) => table,
            Err(err) => {
                ic_cdk::println!("Error getting tables player count: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode players on table: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_players_on_table call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn get_free_seat_index(table_id: Principal) -> Result<Option<u8>, TableError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(table_id, "get_free_seat_index").await;

    match call_result {
        Ok(seat_index_result) => match seat_index_result.candid() {
            Ok(seat_index) => seat_index,
            Err(err) => {
                ic_cdk::println!("Error getting free seat index: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode free seat index: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_free_seat_index call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn get_seat_index(player: Principal, table: Principal) -> Result<Option<u8>, TableError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(table, "get_seat_index")
        .with_arg(player)
        .await;

    match call_result {
        Ok(seat_index_result) => match seat_index_result.candid() {
            Ok(seat_index) => seat_index,
            Err(err) => {
                ic_cdk::println!("Error getting seat index: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode seat index: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_seat_index call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn leave_table_wrapper(
    table: Principal,
    users_canister_id: Principal,
    user_id: Principal,
) -> Result<PublicTable, TableError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(table, "leave_table")
        .with_args(&(users_canister_id, user_id))
        .await;

    match call_result {
        Ok(leave_result) => match leave_result.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error leaving table: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode leave_table response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in leave_table call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn leave_table_for_table_balancing(
    users_canister_id: Principal,
    user_id: Principal,
    table: Principal,
    to_table: Principal,
) -> Result<PublicTable, TableError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(table, "leave_table_for_table_balancing")
        .with_args(&(users_canister_id, user_id, to_table))
        .await;

    match call_result {
        Ok(leave_result) => match leave_result.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error leaving table for table balancing: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode leave_table_for_table_balancing response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in leave_table_for_table_balancing call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
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
    let call_result = ic_cdk::call::Call::unbounded_wait(table_id, "join_table")
        .with_args(&(
            users_canister_principal,
            user_id,
            seat_index,
            deposit_amount,
            player_sitting_out,
        ))
        .await;

    match call_result {
        Ok(join_result) => match join_result.candid() {
            Ok(table) => table,
            Err(err) => {
                ic_cdk::println!("Error joining table: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode join_table response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in join_table call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn clear_table(table_id: Principal) -> Result<(), TableError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(table_id, "clear_table").await;

    match call_result {
        Ok(clear_result) => match clear_result.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error clearing table: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode clear_table response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in clear_table call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn player_sitting_in(
    table_id: Principal,
    user_principal: Principal,
    auto_start: bool,
) -> Result<(), TableError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(table_id, "player_sitting_in")
        .with_args(&(user_principal, auto_start))
        .await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error sitting in player: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode player_sitting_in response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in player_sitting_in call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn start_new_betting_round_wrapper(table_id: Principal) -> Result<(), TableError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(table_id, "start_new_betting_round").await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error starting new betting round: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode start_new_betting_round response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in start_new_betting_round call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn pause_table_for_addon_wrapper(
    table_id: Principal,
    duration: u64,
) -> Result<(), TableError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(table_id, "pause_table_for_addon")
        .with_arg(duration)
        .await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error pausing table for addon: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode pause_table_for_addon response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in pause_table_for_addon call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn pause_table(table_id: Principal) -> Result<(), TableError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(table_id, "pause_table").await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error pausing table: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode pause_table response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in pause_table call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn resume_table_wrapper(table_id: Principal) -> Result<(), TableError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(table_id, "resume_table").await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error resuming table: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode resume_table response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in resume_table call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
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
    let call_result = ic_cdk::call::Call::unbounded_wait(table_id, "deposit_to_table")
        .with_args(&(users_canister_id, user_id, amount, is_queued))
        .await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(result) => result,
            Err(err) => {
                ic_cdk::println!("Error depositing to table: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode deposit_to_table response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in deposit_to_table call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn return_all_cycles_to_index(table_id: Principal) -> Result<(), TableError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(table_id, "return_all_cycles_to_index").await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error returning all cycles to index: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode return_all_cycles_to_index response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in return_all_cycles_to_index call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn set_as_final_table_wrapper(table_id: Principal) -> Result<(), TableError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(table_id, "set_as_final_table").await;

    match call_result {
        Ok(set_result) => match set_result.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error setting table as final table: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode set_as_final_table response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in set_as_final_table call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn kick_player_wrapper(
    table_id: Principal,
    users_canister_id: Principal,
    user_id: Principal,
    balance: u64,
) -> Result<PublicTable, TableError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(table_id, "kick_player")
        .with_args(&(users_canister_id, user_id, balance))
        .await;

    match call_result {
        Ok(kick_result) => match kick_result.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error kicking player: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode kick_player response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in kick_player call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn is_game_ongoing_wrapper(table_principal: Principal) -> Result<bool, TableError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(table_principal, "is_game_ongoing").await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error checking if game is ongoing: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode is_game_ongoing response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in is_game_ongoing call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn add_experience_points_wrapper(
    users_canister_id: Principal,
    user_principal: Principal,
    experience_points: u64,
    currency: String,
) -> Result<User, UserError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(users_canister_id, "add_experience_points")
            .with_args(&(experience_points, currency, user_principal))
            .await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error adding experience points: {:?}", err);
                Err(UserError::CanisterCallFailed(format!(
                    "Failed to decode add_experience_points response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in add_experience_points call: {:?}", err);
            Err(UserError::CanisterCallFailed(format!("{:?}", err)))
        }
    }
}

pub async fn handle_timer_expiration_wrapper(
    table_principal: Principal,
    user_id: Principal,
) -> Result<(), TableError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(table_principal, "handle_timer_expiration")
            .with_arg(user_id)
            .await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error handling timer expiration: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode handle_timer_expiration response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in handle_timer_expiration call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn handle_user_losing_wrapper(
    tournament_id: Principal,
    user_principal: Principal,
    id: Principal,
) -> Result<(), TournamentError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(tournament_id, "handle_user_losing")
        .with_args(&(user_principal, id))
        .await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error handling user losing: {:?}", err);
                Err(TournamentError::CanisterCallError(format!(
                    "Failed to decode handle_user_losing response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in handle_user_losing call: {:?}", err);
            Err(TournamentError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn withdraw_rake_wrapper(table_id: Principal, rake_total: u64) -> Result<(), TableError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(table_id, "withdraw_rake")
        .with_arg(rake_total)
        .await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error withdrawing rake: {:?}", err);
                Err(TableError::CanisterCallError(format!(
                    "Failed to decode withdraw_rake response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in withdraw_rake call: {:?}", err);
            Err(TableError::CanisterCallError(format!("{:?}", err)))
        }
    }
}
