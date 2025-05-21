use candid::Principal;

use crate::tournaments::{
    table_balancing::calculate_players_per_table,
    types::{TournamentData, UserTournamentData},
};

#[test]
fn test_calculate_players_per_table() {
    let mut tournament = TournamentData::default();
    tournament.table_config.seats = 8;
    for i in 0..8 {
        tournament.current_players.insert(
            Principal::self_authenticating(format!("user{}", i)),
            UserTournamentData::default(),
        );
    }

    let players_per_table = calculate_players_per_table(&tournament);
    assert_eq!(players_per_table, vec![8]);
}

#[test]
fn test_calculate_players_per_table_multi_table_tournament_even_split() {
    let mut tournament = TournamentData::default();
    tournament.table_config.seats = 8;
    for i in 0..16 {
        tournament.current_players.insert(
            Principal::self_authenticating(format!("user{}", i)),
            UserTournamentData::default(),
        );
    }

    let players_per_table = calculate_players_per_table(&tournament);
    assert_eq!(players_per_table, vec![8, 8]);
}

#[test]
fn test_calculate_players_per_table_multi_table_tournament_even_split_2() {
    let mut tournament = TournamentData::default();
    tournament.table_config.seats = 8;
    for i in 0..18 {
        tournament.current_players.insert(
            Principal::self_authenticating(format!("user{}", i)),
            UserTournamentData::default(),
        );
    }

    let players_per_table = calculate_players_per_table(&tournament);
    assert_eq!(players_per_table, vec![6, 6, 6]);
}

#[test]
fn test_calculate_players_per_table_multi_table_tournament_uneven_split() {
    let mut tournament = TournamentData::default();
    tournament.table_config.seats = 8;
    for i in 0..15 {
        tournament.current_players.insert(
            Principal::self_authenticating(format!("user{}", i)),
            UserTournamentData::default(),
        );
    }

    let players_per_table = calculate_players_per_table(&tournament);
    assert_eq!(players_per_table, vec![8, 7]);
}

#[test]
fn test_calculate_players_per_table_multi_table_tournament_uneven_split_2() {
    let mut tournament = TournamentData::default();
    tournament.table_config.seats = 8;
    for i in 0..23 {
        tournament.current_players.insert(
            Principal::self_authenticating(format!("user{}", i)),
            UserTournamentData::default(),
        );
    }

    let players_per_table = calculate_players_per_table(&tournament);
    assert_eq!(players_per_table, vec![8, 8, 7]);
}

#[test]
fn test_calculate_players_per_table_multi_table_tournament_descending() {
    let mut tournament = TournamentData::default();
    tournament.table_config.seats = 8;
    for i in 0..26 {
        tournament.current_players.insert(
            Principal::self_authenticating(format!("user{}", i)),
            UserTournamentData::default(),
        );
    }

    let players_per_table = calculate_players_per_table(&tournament);
    assert_eq!(players_per_table, vec![7, 7, 6, 6]);
}
