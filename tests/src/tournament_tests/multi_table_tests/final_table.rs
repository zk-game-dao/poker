use std::time::Duration;

use candid::Principal;
use table::poker::game::table_functions::table::TableType;

use crate::TestEnv;

#[test]
fn test_final_table_formation() {
    let test_env = TestEnv::new(None);
    let (tournament_id, _) = test_env.setup_multi_table_tournament(6, 3, 12);

    // Register enough players for 2 tables
    let mut players = Vec::new();
    for i in 0..12 {
        let user = test_env
            .create_user(
                format!("User {}", i),
                Principal::self_authenticating(format!("user{}final", i)),
            )
            .unwrap();

        test_env.transfer_approve_tokens_for_testing(
            tournament_id,
            user.principal_id,
            1000.0,
            true,
        );

        test_env
            .join_tournament(tournament_id, user.users_canister_id, user.principal_id)
            .unwrap();
        players.push(user.principal_id);
    }

    // Start tournament
    test_env.pocket_ic.advance_time(Duration::from_secs(120));
    for _ in 0..30 {
        test_env.pocket_ic.tick();
    }

    let tournament = test_env.get_tournament(tournament_id).unwrap();
    for table in tournament.tables.keys() {
        test_env.complete_betting_round(*table).unwrap();
    }

    // test_env.pocket_ic.advance_time(Duration::from_secs(300));
    for _ in 0..30 {
        test_env.pocket_ic.tick();
    }

    // Reduce to final table size
    let eliminated_players = test_env.eliminate_players(tournament_id, 7);
    assert_eq!(eliminated_players.len(), 7);

    // test_env.pocket_ic.advance_time(Duration::from_secs(300));
    for _ in 0..30 {
        test_env.pocket_ic.tick();
    }

    let tournament = test_env.get_tournament(tournament_id).unwrap();
    for table in tournament.tables.keys() {
        test_env.complete_betting_round(*table).unwrap();
    }

    test_env.pocket_ic.advance_time(Duration::from_secs(300));
    for _ in 0..30 {
        test_env.pocket_ic.tick();
    }

    // Verify final table formation
    let tournament = test_env.get_tournament(tournament_id).unwrap();
    // assert_eq!(tournament.state, TournamentState::FinalTable);
    assert_eq!(tournament.tables.len(), 1);

    // Verify final table configuration
    let final_table_id = tournament.tables.keys().next().unwrap();
    let table = test_env.get_table(*final_table_id).unwrap();

    if let Some(TableType::Tournament { is_final_table, .. }) = table.config.table_type {
        assert!(is_final_table, "Table should be marked as final table");
    }
}
