use std::collections::{hash_map::Entry, HashMap, HashSet};

use candid::Principal;
use canister_functions::inter_canister_call_wrappers::{
    add_to_table_pool_wrapper, ensure_principal_is_controller, get_players_on_table,
    leave_table_for_table_balancing, set_as_final_table_wrapper,
};
use errors::{table_error::TableError, tournament_error::TournamentError};
use tournaments::tournaments::{
    tournament_type::{TournamentSizeType, TournamentType},
    types::{TournamentData, TournamentState},
};

use crate::{
    utils::{move_player_from_current_players_to_all_players, update_tournament_state}, LAST_BALANCE_TIMESTAMP, LEADERBOARD, TOURNAMENT, TOURNAMENT_INDEX
};

pub async fn check_and_balance_tables(skip_check: bool) -> Result<(), TournamentError> {
    let mut tournament = {
        let tournament = TOURNAMENT.lock().map_err(|_| TournamentError::LockError)?;
        tournament
            .as_ref()
            .ok_or(TournamentError::TournamentNotFound)?
            .clone()
    };

    if tournament.state == TournamentState::Completed
        || tournament.state == TournamentState::FinalTable
        || tournament.state == TournamentState::Cancelled
    {
        return Ok(());
    }

    if !matches!(
        tournament.tournament_type,
        TournamentType::Freeroll(TournamentSizeType::MultiTable(_, _))
            | TournamentType::BuyIn(TournamentSizeType::MultiTable(_, _))
    ) || tournament.tables.len() == 1
    {
        return Ok(());
    }

    let table_balancer = match &tournament.tournament_type {
        TournamentType::BuyIn(TournamentSizeType::MultiTable(_, balancer)) => balancer,
        TournamentType::Freeroll(TournamentSizeType::MultiTable(_, balancer)) => balancer,
        _ => return Ok(()),
    };

    if !skip_check
        && (ic_cdk::api::time() < tournament.start_time
            || ic_cdk::api::time()
                < LAST_BALANCE_TIMESTAMP.load(std::sync::atomic::Ordering::Relaxed)
                    + table_balancer.balance_interval_ns)
        && tournament.state != TournamentState::Registration
        && tournament.state != TournamentState::Completed
    {
        return Ok(());
    }

    let tournament_index = {
        let tournament = TOURNAMENT_INDEX
            .lock()
            .map_err(|_| TournamentError::LockError)?;
        *tournament
            .as_ref()
            .ok_or(TournamentError::TournamentNotFound)?
    };

    // TODO: We need to decide if we continue with synchronzing tables. If so we should remove the
    //       ICCs `update_player_count_tournament` in the table canister
    let cached_tables = synchronize_tables(&mut tournament).await?;

    // Only balance if tournament is running and is multi-table
    if let TournamentType::BuyIn(TournamentSizeType::MultiTable(_, balancer))
    | TournamentType::Freeroll(TournamentSizeType::MultiTable(_, balancer)) =
        &tournament.tournament_type
    {
        // Get balance moves needed
        let moves = balancer.get_balance_moves(&mut tournament.tables);

        ic_cdk::println!(
            "Moves: {:?}",
            moves
                .iter()
                .map(|(a, b)| (a.to_text(), b.to_text()))
                .collect::<Vec<_>>()
        );

        execute_moves(moves, &mut tournament, cached_tables).await?;

        // Clean up empty tables
        for (table_id, table_info) in tournament.tables.clone().iter() {
            if table_info.players.is_empty() {
                if let Err(e) = ensure_principal_is_controller(*table_id, tournament_index).await {
                    ic_cdk::println!("Error ensuring principal is controller: {:?}", e);
                } else if let Err(e) = add_to_table_pool_wrapper(tournament_index, *table_id).await
                {
                    ic_cdk::println!("Error adding table to table pool: {:?}", e);
                }
                tournament.tables.remove(table_id);
            }
        }

        if tournament.tables.len() == 1 {
            tournament.state = TournamentState::FinalTable;
            let table_id = tournament
                .tables
                .keys()
                .next()
                .ok_or(TournamentError::TableError(TableError::TableNotFound))?;
            if let Err(e) = set_as_final_table_wrapper(*table_id).await {
                ic_cdk::println!("Error setting table as final table: {:?}", e);
            };
            update_tournament_state(TournamentState::FinalTable).await?;
        }
    } else {
        return Ok(());
    }

    *TOURNAMENT.lock().map_err(|_| TournamentError::LockError)? = Some(tournament);
    LAST_BALANCE_TIMESTAMP.store(ic_cdk::api::time(), std::sync::atomic::Ordering::Relaxed);

    Ok(())
}

async fn execute_moves(
    moves: Vec<(Principal, Principal)>,
    tournament: &mut TournamentData,
    mut cached_tables: HashMap<Principal, Vec<Principal>>,
) -> Result<(), TournamentError> {
    // Execute moves
    let mut processed_players = HashMap::new();

    for (from_table, to_table) in moves {
        let table = match cached_tables.entry(from_table) {
            Entry::Occupied(entry) => entry.get().clone(),
            Entry::Vacant(entry) => {
                let table = get_players_on_table(from_table).await?;
                entry.insert(table.clone());
                table
            }
        };

        // Keep track of which players we've already moved from each table
        let moved_players = processed_players
            .entry(from_table)
            .or_insert_with(HashSet::new);

        // Try to find a player to move who hasn't been moved yet
        let player_to_move = table
            .iter()
            .find(|&player| !moved_players.contains(player))
            .copied();

        if let Some(player) = player_to_move {
            // Move the player
            move_player_to_table(player, from_table, to_table, tournament).await?;

            // Record that this player has been moved from this table
            moved_players.insert(player);

            // Record the move time
            tournament.record_table_move(to_table)?;
        } else {
            // This shouldn't happen if the balancer is working correctly, but log if it does
            ic_cdk::println!(
                "Warning: No available players to move from table {}",
                from_table.to_text()
            );
        }
    }
    Ok(())
}

async fn synchronize_tables(
    tournament: &mut TournamentData,
) -> Result<HashMap<Principal, Vec<Principal>>, TournamentError> {
    let mut cached_tables = HashMap::new();
    let mut players_on_tables = HashSet::new();
    
    for table in &mut tournament.tables {
        let public_table_players = get_players_on_table(*table.0).await?;
        cached_tables.insert(*table.0, public_table_players.clone());
        
        players_on_tables.extend(public_table_players.iter().copied());

        if public_table_players.len() != table.1.players.len() {
            table.1.players = public_table_players.iter().copied().collect::<HashSet<_>>();
        }
    }

    if players_on_tables.is_empty() {
        return Ok(cached_tables);
    }

    let unassigned_players: Vec<Principal> = tournament
        .current_players
        .keys()
        .filter(|player| !players_on_tables.contains(player))
        .copied()
        .collect();

    if !unassigned_players.is_empty() {
        ic_cdk::println!(
            "Found {} unassigned players that will be removed from the tournament: {:?}",
            unassigned_players.len(),
            unassigned_players
                .iter()
                .map(|p| p.to_text())
                .collect::<Vec<_>>()
        );
    }
    if unassigned_players.len() == tournament.current_players.len() {
        return Ok(cached_tables);
    }
    
    for player in unassigned_players {
        match move_player_from_current_players_to_all_players(tournament, &vec![player]) {
            Ok(_) => {
                match LEADERBOARD.lock() {
                    Ok(mut leaderboard) => {
                        if !leaderboard.contains(&player) {
                            leaderboard.push(player)
                        }
                    }
                    Err(e) => {
                        ic_cdk::println!("Error getting leaderboard: {:?}", e);
                        return Err(TournamentError::LockError);
                    }
                }
            }
            Err(e) => {
                ic_cdk::println!("Error moving player {}: {:?}", player.to_text(), e);
            }
        }
    }

    Ok(cached_tables)
}

pub async fn move_player_to_table(
    player: Principal,
    from_table: Principal,
    to_table: Principal,
    tournament: &mut TournamentData,
) -> Result<(), TournamentError> {
    let user_tournament_data =
        tournament
            .current_players
            .get(&player)
            .ok_or(TournamentError::Other(
                "Could not get users tournament data in move_player_to_table".to_string(),
            ))?;
    ic_cdk::println!(
        "Moving player {} from table {} to table {}",
        player.to_text(),
        from_table.to_text(),
        to_table.to_text()
    );
    let res = leave_table_for_table_balancing(
        user_tournament_data.users_canister_principal,
        player,
        from_table,
        to_table,
    )
    .await?;
    ic_cdk::println!(
        "*********** Leave table result: {:?}",
        res.is_user_queued_to_leave(player)
    );

    tournament
        .tables
        .get_mut(&from_table)
        .ok_or(TournamentError::TableError(TableError::TableNotFound))?
        .players
        .remove(&player);

    Ok(())
}
