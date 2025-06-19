use std::{sync::atomic::Ordering, time::Duration};

use errors::tournament_error::TournamentError;
use intercanister_call_wrappers::tournament_canister::{
    get_and_remove_from_pool_wrapper, handle_cancelled_tournament_wrapper, update_blinds,
};
use table::table_canister::{join_table, pause_table_for_addon_wrapper, resume_table_wrapper};
use tournaments::tournaments::{
    table_balancing::calculate_players_per_table,
    tournament_type::{TournamentSizeType, TournamentType},
    types::{TableInfo, TournamentData, TournamentId, TournamentState},
};
use user::user::{UsersCanisterId, WalletPrincipalId};

use crate::{
    table_balancing::check_and_balance_tables,
    utils::{
        create_table, handle_cycle_check_async, update_live_leaderboard, update_tournament_state,
        LEADERBOARD_UPDATE_INTERVAL,
    },
    LAST_HEARTBEAT, LAST_LEADERBOARD_UPDATE, TOURNAMENT, TOURNAMENT_INDEX, TOURNAMENT_START_TIME,
};

const MIN_HEARTBEAT_INTERVAL: u64 = 60_000_000_000; // 1 minute in nanoseconds

#[ic_cdk::heartbeat]
async fn heartbeat() {
    let start_time = TOURNAMENT_START_TIME.load(Ordering::Relaxed);

    let current_time = ic_cdk::api::time();
    let last_beat = LAST_HEARTBEAT.load(Ordering::Relaxed);

    if current_time - last_beat < MIN_HEARTBEAT_INTERVAL {
        return;
    }

    LAST_HEARTBEAT.store(current_time, Ordering::Relaxed);

    {
        let tournament = TOURNAMENT.lock().map_err(|_| TournamentError::LockError);
        let tournament = match tournament {
            Ok(tournament) => tournament,
            Err(_) => return,
        };
        let tournament = tournament
            .as_ref()
            .ok_or(TournamentError::TournamentNotFound);
        let tournament = match tournament {
            Ok(tournament) => tournament,
            Err(_) => return,
        };
        if tournament.state == TournamentState::Completed
            || tournament.state == TournamentState::Cancelled
        {
            return;
        }
    }

    // Only proceed if we've reached the start time
    if current_time < start_time {
        return;
    }

    if let Err(e) = check_and_start_tournament().await {
        ic_cdk::println!("Error in tournament heartbeat: {:?}", e);
    }

    if let Err(e) = check_late_registration_end().await {
        ic_cdk::println!("Error in late registration duration end check: {:?}", e);
    }

    if let Err(e) = check_for_addon_period().await {
        ic_cdk::println!("Error in addon period check: {:?}", e);
    }

    if let Err(e) = check_and_update_blinds().await {
        ic_cdk::println!("Error updating blind levels: {:?}", e);
    }

    if let Err(e) = check_and_balance_tables(false).await {
        ic_cdk::println!("Error balancing tables: {:?}", e);
    }

    let last_update = LAST_LEADERBOARD_UPDATE.load(Ordering::Relaxed);

    if current_time > last_update + LEADERBOARD_UPDATE_INTERVAL {
        if let Err(e) = update_live_leaderboard().await {
            ic_cdk::println!("Error updating live leaderboard in heartbeat: {:?}", e);
        }
    }
}

async fn check_late_registration_end() -> Result<(), TournamentError> {
    let tournament = {
        let tournament = TOURNAMENT.lock().map_err(|_| TournamentError::LockError)?;
        tournament
            .as_ref()
            .ok_or(TournamentError::TournamentNotFound)?
            .clone()
    };

    if tournament.state != TournamentState::LateRegistration {
        return Ok(());
    }

    if tournament.late_registration_duration_ns == 0 {
        return Ok(());
    }

    if tournament.start_time + tournament.late_registration_duration_ns < ic_cdk::api::time() {
        update_tournament_state(TournamentState::Running).await?;
    }

    Ok(())
}

async fn check_for_addon_period() -> Result<(), TournamentError> {
    let tournament = {
        let tournament = TOURNAMENT.lock().map_err(|_| TournamentError::LockError)?;
        tournament
            .as_ref()
            .ok_or(TournamentError::TournamentNotFound)?
            .clone()
    };

    match &tournament.tournament_type {
        TournamentType::BuyIn(tournament_size) => {
            handle_addon_period(tournament_size, &tournament).await?;
        }
        TournamentType::Freeroll(tournament_size) => {
            handle_addon_period(tournament_size, &tournament).await?;
        }
        TournamentType::SitAndGo(tournament_size) => {
            handle_addon_period(tournament_size, &tournament).await?;
        }
        _ => Err(TournamentError::Other(
            "Unsupported tournament type".to_string(),
        ))?,
    }
    Ok(())
}

async fn handle_addon_period(
    tournament_size: &TournamentSizeType,
    tournament: &TournamentData,
) -> Result<(), TournamentError> {
    match tournament_size {
        TournamentSizeType::SingleTable(options) | TournamentSizeType::MultiTable(options, _) => {
            // Checks if addon period has ended by at least 3 minutes so we avoid continuous unnecessary inter-canister calls.
            if !options.addon.enabled
                || options.addon.addon_end_time + 180_000_000_000 < ic_cdk::api::time()
            {
                return Ok(());
            }

            ic_cdk::println!("Addon period check passed");

            let time = ic_cdk::api::time();
            if options.addon.enabled
                && options.addon.addon_start_time <= time
                && options.addon.addon_end_time >= time
            {
                for table_id in tournament.tables.keys() {
                    ic_cdk::println!(
                        "Pausing table for duration {} for table: {:?}",
                        Duration::from_nanos(
                            options.addon.addon_end_time - options.addon.addon_start_time
                        )
                        .as_secs(),
                        table_id
                    );
                    pause_table_for_addon_wrapper(
                        *table_id,
                        options.addon.addon_end_time - options.addon.addon_start_time,
                    )
                    .await?;
                }
            } else if options.addon.enabled
                && options.addon.addon_end_time + 300_000_000_000 < ic_cdk::api::time()
            {
                for table_id in tournament.tables.keys() {
                    resume_table_wrapper(*table_id).await?;
                }
            }
        }
    }
    Ok(())
}

async fn check_and_update_blinds() -> Result<(), TournamentError> {
    let (should_update, new_level) = {
        let mut tournament = TOURNAMENT.lock().map_err(|_| TournamentError::LockError)?;
        let tournament = tournament
            .as_mut()
            .ok_or(TournamentError::TournamentNotFound)?;

        if tournament.state != TournamentState::Running
            && tournament.state != TournamentState::LateRegistration
            && tournament.state != TournamentState::FinalTable
        {
            return Ok(());
        }

        if !tournament.should_increase_blinds() {
            return Ok(());
        }

        // Get next blind level
        let new_level = if let Some(new_level) = tournament.increase_blinds() {
            new_level
        } else {
            return Ok(());
        };

        (true, new_level)
    };

    if should_update {
        let tournament = {
            let tournament = TOURNAMENT.lock().map_err(|_| TournamentError::LockError)?;
            tournament
                .as_ref()
                .ok_or(TournamentError::TournamentNotFound)?
                .clone()
        };

        // Update all tournament tables with new blinds
        for table_id in tournament.tables.keys() {
            if let Err(e) = update_blinds(table_id.0, &new_level).await {
                ic_cdk::println!("Error updating blinds on table {:?}: {:?}", table_id, e);
                continue;
            }
        }
    }

    Ok(())
}

async fn check_and_start_tournament() -> Result<(), TournamentError> {
    let (mut tournament, tournament_state) = {
        let mut tournament = TOURNAMENT.lock().map_err(|_| TournamentError::LockError)?;
        let mut tournament_state = TournamentState::Running;
        let tournament = tournament
            .as_mut()
            .ok_or(TournamentError::TournamentNotFound)?;

        if tournament.state == TournamentState::Running
            || tournament.state == TournamentState::LateRegistration
            || tournament.state == TournamentState::FinalTable
        {
            return Ok(());
        }

        // We already know we've passed start time, just check state
        if tournament.state != TournamentState::Registration {
            return Ok(());
        }

        // Check if we have minimum number of players
        if tournament.current_players.len() < tournament.min_players as usize
            && !matches!(
                tournament.tournament_type,
                TournamentType::SitAndGo(_) | TournamentType::SpinAndGo(_, _)
            )
        {
            tournament.state = TournamentState::Cancelled;

            let id = TournamentId(ic_cdk::api::canister_self());
            ic_cdk::futures::spawn(async move {
                match handle_cancelled_tournament_wrapper(id).await {
                    Ok(_) => {}
                    Err(e) => ic_cdk::println!("Error handling cancelled tournament: {:?}", e),
                }
            });
            return Err(TournamentError::Other(
                "Not enough players to start tournament".to_string(),
            ));
        } else if tournament.current_players.len() < tournament.min_players as usize
            && matches!(
                tournament.tournament_type,
                TournamentType::SitAndGo(_) | TournamentType::SpinAndGo(_, _)
            )
        {
            return Ok(());
        }

        // Update tournament state
        if tournament.late_registration_duration_ns != 0 {
            tournament_state = TournamentState::LateRegistration;
        }

        // Set the initial blind level timing
        if let Some(first_level) = tournament.speed_type.get_params().blind_levels.first() {
            tournament.speed_type.get_params_mut().next_level_time =
                Some(ic_cdk::api::time() + first_level.duration_ns);
        }
        (tournament.clone(), tournament_state)
    };

    update_tournament_state(tournament_state).await?;

    if let Err(e) = deploy_and_distribute_players_to_tables(&mut tournament).await {
        ic_cdk::println!(
            "Error deploying and distributing players to tables: {:?}",
            e
        );
        return Err(e);
    }

    let mut tournament_state = TOURNAMENT.lock().map_err(|_| TournamentError::LockError)?;
    let tournament_state = tournament_state
        .as_mut()
        .ok_or(TournamentError::TournamentNotFound)?;
    tournament_state.tables = tournament.tables.clone();

    Ok(())
}

async fn deploy_and_distribute_players_to_tables(
    tournament: &mut TournamentData,
) -> Result<(), TournamentError> {
    let players_per_table = calculate_players_per_table(tournament);
    let tournament_index = {
        let tournament_index = TOURNAMENT_INDEX
            .lock()
            .map_err(|_| TournamentError::LockError)?;
        *tournament_index.as_ref().ok_or(TournamentError::Other(
            "Tourament index not found.".to_string(),
        ))?
    };

    let table_count = players_per_table.len();
    // Get all players
    let players: Vec<(WalletPrincipalId, UsersCanisterId)> = tournament
        .current_players
        .iter()
        .map(|(uid, data)| (*uid, data.users_canister_principal))
        .collect();
    let mut player_index = 0;

    let mut table_config = tournament.table_config.clone();
    match &tournament.tournament_type {
        TournamentType::BuyIn(TournamentSizeType::MultiTable(_, table_balancer)) => {
            table_config.seats = table_balancer.max_players_per_table;
        }
        TournamentType::Freeroll(TournamentSizeType::MultiTable(_, table_balancer)) => {
            table_config.seats = table_balancer.max_players_per_table;
        }
        _ => {}
    }

    // Create tables and distribute players
    #[allow(clippy::needless_range_loop)]
    for table_index in 0..table_count {
        handle_cycle_check_async().await;
        let table =
            if let Ok(table_principal) = get_and_remove_from_pool_wrapper(tournament_index).await {
                match create_table(tournament, table_config.clone(), table_principal).await {
                    Ok(table) => table,
                    Err(e) => {
                        ic_cdk::println!("Error creating table: {:?}", e);
                        match create_table(tournament, table_config.clone(), None).await {
                            Ok(table) => table,
                            Err(e) => {
                                ic_cdk::println!("Error creating table: {:?}", e);
                                return Err(e);
                            }
                        }
                    }
                }
            } else {
                match create_table(tournament, table_config.clone(), None).await {
                    Ok(table) => table,
                    Err(e) => {
                        ic_cdk::println!("Error creating table: {:?}", e);
                        return Err(e);
                    }
                }
            };

        let mut table_info = TableInfo::new();

        // Assign players to this table
        let players_at_this_table = players_per_table[table_index];
        for _ in 0..players_at_this_table {
            let user = players[player_index];
            table_info.players.insert(user.0);
            player_index += 1;

            // Last player at the table is the dealer
            let res = join_table(
                table.id,
                user.1,
                user.0,
                None,
                tournament.starting_chips,
                false,
            )
            .await;

            if let Err(e) = res {
                ic_cdk::println!("Error joining in player: {:?}", e);
            }
        }
        tournament.tables.insert(table.id, table_info);
        resume_table_wrapper(table.id).await?;
    }
    Ok(())
}
