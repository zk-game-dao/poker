use std::time::Duration;

use candid::Principal;
use currency::Currency;
use table::poker::game::{
    table_functions::{
        table::{TableConfig, TableId, TableType},
        types::{BetType, CurrencyType, DealStage},
    },
    utils::convert_to_e8s,
};
use tournaments::tournaments::{
    blind_level::SpeedType,
    table_balancing::TableBalancer,
    tournament_type::{BuyInOptions, TournamentSizeType, TournamentType},
    types::{NewTournament, NewTournamentSpeedType, TournamentId, TournamentState},
};
use user::user::WalletPrincipalId;

use crate::TestEnv;

impl TestEnv {
    pub fn setup_multi_table_tournament(
        &self,
        players_per_table: u8,
        min_players: u8,
        max_players: u8,
    ) -> (TournamentId, NewTournament) {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let now = std::time::SystemTime::now();
        self.pocket_ic.set_time(now.into());

        let tournament_config = NewTournament {
            name: "Multi-Table Test Tournament".to_string(),
            description: "Test Tournament Description".to_string(),
            hero_picture: "".to_string(),
            currency: CurrencyType::Real(Currency::ICP),
            buy_in: convert_to_e8s(10.0),
            starting_chips: 1000,
            speed_type: NewTournamentSpeedType::Regular(110),
            min_players,
            max_players: max_players as u32,
            late_registration_duration_ns: 0,
            require_proof_of_humanity: false,
            guaranteed_prize_pool: None,
            tournament_type: TournamentType::BuyIn(TournamentSizeType::MultiTable(
                BuyInOptions::new_freezout(),
                TableBalancer::new(
                    min_players,
                    players_per_table,
                    &SpeedType::new_regular(1000, 110),
                ),
            )),
            start_time: current_time + 1_000_000_000, // 1 second in future
        };

        let table_config = TableConfig {
            name: "Test Table".to_string(),
            game_type: table::poker::game::types::GameType::NoLimit(10),
            seats: players_per_table,
            timer_duration: 30,
            card_color: 0,
            color: 0,
            environment_color: 0,
            auto_start_timer: 10,
            max_inactive_turns: 3,
            currency_type: table::poker::game::table_functions::types::CurrencyType::Real(
                Currency::ICP,
            ),
            enable_rake: Some(false),
            max_seated_out_turns: None,
            is_private: Some(false),
            ante_type: None,
            table_type: Some(TableType::Tournament {
                tournament_id: Principal::anonymous(),
                is_final_table: false,
            }),
            is_shared_rake: None,
            require_proof_of_humanity: None,
            is_paused: None,
        };

        let id = self
            .create_tournament(&tournament_config, &table_config)
            .unwrap();
        (id, tournament_config)
    }

    pub fn eliminate_players(
        &self,
        tournament_id: TournamentId,
        num_players: u8,
    ) -> Vec<WalletPrincipalId> {
        println!("Eliminating players...");
        let tournament = self.get_tournament(tournament_id).unwrap();

        println!(
            "Tournament current players: {:?}",
            tournament
                .current_players
                .iter()
                .map(|p| p.0 .0.to_text())
                .collect::<Vec<_>>()
        );

        let mut eliminated_players = Vec::new();

        'outer: for _ in 0..num_players {
            for table_id in tournament.tables.keys() {
                let table = self.get_table(*table_id).unwrap();
                if !table.users.is_empty() {
                    let user = table.users.users.into_values().next().unwrap();
                    if let Err(e) =
                        self.handle_user_losing(tournament_id, user.principal_id, *table_id)
                    {
                        println!(
                            "Error eliminating player {}: {:?}",
                            user.principal_id.0.to_text(),
                            e
                        );
                        continue;
                    }
                    println!("Eliminated player: {}", user.principal_id.0.to_text());
                    eliminated_players.push(user.principal_id);
                    if eliminated_players.len() == num_players as usize {
                        break 'outer;
                    }
                }
                if eliminated_players.len() == num_players as usize {
                    break 'outer;
                }
            }
            if eliminated_players.len() == num_players as usize {
                break 'outer;
            }
        }

        println!(
            "#################################\nEliminated {} players: {:?}",
            eliminated_players.len(),
            eliminated_players
                .iter()
                .map(|p| p.0.to_text())
                .collect::<Vec<_>>()
        );

        for _ in 0..30 {
            self.pocket_ic.advance_time(Duration::from_secs(3));
            self.pocket_ic.tick();
        }

        eliminated_players
    }

    pub fn complete_table_round(&self, table_id: TableId) {
        let mut table = self.get_table(table_id).unwrap();
        let mut first_action_made = false;

        // Keep processing until we reach showdown
        while table.deal_stage != DealStage::Showdown {
            // Get the current player whose turn it is
            let current_player = table
                .get_player_at_seat(table.current_player_index)
                .expect("Failed to get current player");

            // Get their balance
            let user_balance = table
                .users
                .get(&current_player)
                .map(|user| user.balance)
                .unwrap_or_default();

            if user_balance > 0 {
                if !first_action_made {
                    // First player goes all-in
                    self.player_bet(table_id, current_player, BetType::Raised(user_balance.0))
                        .unwrap();
                    first_action_made = true;
                } else {
                    // Subsequent players call
                    self.player_bet(table_id, current_player, BetType::Called)
                        .unwrap();
                }
            }

            // Update table state
            table = self.get_table(table_id).unwrap();
        }
        for _ in 0..6 {
            self.pocket_ic.tick();
        }
        let _ = self.start_betting_round_test_table(table_id);
    }
}

#[test]
fn test_basic_multi_table() {
    let test_env = TestEnv::new(None);
    let (tournament_id, _) = test_env.setup_multi_table_tournament(6, 4, 12);

    // Register enough players for two tables
    for i in 0..9 {
        let user = test_env
            .create_user(
                format!("User {}", i),
                WalletPrincipalId(Principal::self_authenticating(format!("user{}balance", i))),
            )
            .unwrap();

        test_env.transfer_approve_tokens_for_testing(
            tournament_id.0,
            user.principal_id,
            1000.0,
            true,
        );

        test_env
            .join_tournament(tournament_id, user.users_canister_id, user.principal_id)
            .unwrap();
    }

    // Advance time past start_time
    test_env.pocket_ic.advance_time(Duration::from_secs(120)); // 2 minutes

    // Now tournament should start
    for _ in 0..30 {
        test_env.pocket_ic.tick();
    }

    // Verify tables are balanced
    let tournament = test_env.get_tournament(tournament_id).unwrap();
    assert_eq!(tournament.tables.len(), 2);
    for table in tournament.tables.keys() {
        let table_info = test_env.get_table(*table).unwrap();
        assert_eq!(table_info.deal_stage, DealStage::Flop);
    }
    assert_eq!(tournament.current_players.len(), 9);
    assert_eq!(tournament.state, TournamentState::Running);
}

#[test]
fn test_table_breaking() {
    let test_env = TestEnv::new(None);
    let (tournament_id, _) = test_env.setup_multi_table_tournament(6, 4, 12);

    // First register enough players for two tables
    let mut players = Vec::new();
    for i in 0..10 {
        let user = test_env
            .create_user(
                format!("User {}", i),
                WalletPrincipalId(Principal::self_authenticating(format!("user{}break", i))),
            )
            .unwrap();

        test_env.transfer_approve_tokens_for_testing(
            tournament_id.0,
            user.principal_id,
            1000.0,
            true,
        );

        test_env
            .join_tournament(tournament_id, user.users_canister_id, user.principal_id)
            .unwrap();
        players.push(user.principal_id);
    }

    // Advance time past start_time
    test_env.pocket_ic.advance_time(Duration::from_secs(120));

    // Now tournament should start
    for _ in 0..30 {
        test_env.pocket_ic.tick();
    }

    // Simulate eliminating players
    let eliminated_players = test_env.eliminate_players(tournament_id, 6);
    assert_eq!(eliminated_players.len(), 6);

    for _ in 0..30 {
        test_env.pocket_ic.advance_time(Duration::from_secs(120));

        // Now tournament should start
        for _ in 0..30 {
            test_env.pocket_ic.tick();
        }
    }

    // Verify table was broken and players moved
    let tournament = test_env.get_tournament(tournament_id).unwrap();
    println!(
        "Tournament tables: {:?}",
        tournament.tables.keys().next().unwrap().0.to_text()
    );
    assert_eq!(tournament.tables.len(), 1, "Table should have been broken");
    let table_id = *tournament.tables.keys().next().unwrap();
    // let _ = test_env.start_betting_round_test_table(table_id);
    let table = test_env.get_table(table_id).unwrap();
    println!("Table queue: {:?}", table.queue);
    assert_eq!(table.users.len(), 4, "Table should have 5 players");
}

#[test]
fn test_table_balancing_with_player_overflow() {
    let test_env = TestEnv::new(None);
    // Setup tournament with 6 max players per table, min 2 players per table, and max total 14 players
    let (tournament_id, _) = test_env.setup_multi_table_tournament(6, 2, 14);

    // Register exactly 14 players
    let mut players = Vec::new();
    for i in 0..14 {
        let user = test_env
            .create_user(
                format!("User {}", i),
                WalletPrincipalId(Principal::self_authenticating(format!("user{}balance", i))),
            )
            .unwrap();

        test_env.transfer_approve_tokens_for_testing(
            tournament_id.0,
            user.principal_id,
            1000.0,
            true,
        );
        test_env
            .join_tournament(tournament_id, user.users_canister_id, user.principal_id)
            .unwrap();
        players.push(user.principal_id);
    }

    // Advance time to start tournament
    test_env.pocket_ic.advance_time(Duration::from_secs(120));

    // Tick multiple times to ensure tournament processing
    for _ in 0..50 {
        test_env.pocket_ic.advance_time(Duration::from_secs(12));

        test_env.pocket_ic.tick();
    }

    // Verify initial table distribution
    let tournament = test_env.get_tournament(tournament_id).unwrap();
    assert_eq!(tournament.state, TournamentState::Running);
    println!("### Initial Tournament State ###");
    println!("Total players: {}", tournament.current_players.len());
    println!("Number of tables: {}", tournament.tables.len());

    // With 14 players and max 5 per table, should have 3 tables
    assert_eq!(tournament.tables.len(), 3, "Should have 3 tables initially");

    // Check each table for proper player count
    let mut total_table_players = 0;
    for (table_id, table_info) in &tournament.tables {
        let table = test_env.get_table(*table_id).unwrap();
        println!(
            "Table {}: {} players in table_info, {} in actual table",
            table_id.0.to_text().chars().take(5).collect::<String>(),
            table_info.players.len(),
            table.users.len()
        );
        assert!(
            table_info.players.len() <= 5,
            "Table should not exceed 5 players"
        );
        assert_eq!(
            table_info.players.len(),
            table.users.len(),
            "Table info and actual table should have matching player counts"
        );
        total_table_players += table_info.players.len();
    }

    // Total players across tables should match tournament players
    assert_eq!(
        total_table_players, 14,
        "Total table players should match tournament players"
    );

    // Print which player is on which table
    for (table_id, table_info) in &tournament.tables {
        println!(
            "Table {}: Players: [{}]",
            table_id.0.to_text().chars().take(5).collect::<String>(),
            table_info
                .players
                .iter()
                .map(|p| p.0.to_text().chars().take(4).collect::<String>())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    // Now eliminate players to force table balancing
    println!("\n### Eliminating Players to Force Table Balancing ###");
    let eliminated_players = test_env.eliminate_players(tournament_id, 2);
    assert_eq!(
        eliminated_players.len(),
        2,
        "Should have eliminated 2 players"
    );

    // Advance time and tick to ensure tournament processing
    for _ in 0..10 {
        test_env.pocket_ic.advance_time(Duration::from_secs(60));
        for _ in 0..30 {
            test_env.pocket_ic.tick();
        }
    }

    let tournament = test_env.get_tournament(tournament_id).unwrap();
    for table in tournament.tables.keys() {
        test_env.complete_table_round(*table);
    }

    // Advance time and tick to ensure tournament processing
    for _ in 0..5 {
        test_env.pocket_ic.advance_time(Duration::from_secs(60));
        for _ in 0..30 {
            test_env.pocket_ic.tick();
        }
    }

    let tournament = test_env.get_tournament(tournament_id).unwrap();
    for table in tournament.tables.keys() {
        test_env.complete_table_round(*table);
    }

    for _ in 0..5 {
        test_env.pocket_ic.advance_time(Duration::from_secs(60));
        for _ in 0..30 {
            test_env.pocket_ic.tick();
        }
    }

    // Verify tables were balanced properly
    let tournament = test_env.get_tournament(tournament_id).unwrap();
    let mut table_infos = Vec::new();
    for table in tournament.tables.keys() {
        let table_info = test_env.get_table(*table).unwrap();
        table_infos.push(table_info.users.users.into_keys().collect::<Vec<_>>());
    }
    let total_players_on_tables = table_infos.iter().map(|t| t.len()).sum::<usize>();
    println!("\n### After Eliminations ###");
    println!("Total players: {}", tournament.current_players.len());
    println!("Number of tables: {}", tournament.tables.len());
    for table in table_infos {
        println!(
            "Table players: {:?}",
            table.iter().map(|p| p.0.to_text()).collect::<Vec<_>>()
        );
    }

    // With 7 players eliminated, should now have 7 players across max 2 tables
    assert_eq!(
        tournament.current_players.len(),
        11,
        "Should have 11 players remaining"
    );
    assert_eq!(
        tournament.current_players.len(),
        total_players_on_tables,
        "Total players on tables should match tournament players"
    );
    assert!(
        tournament.tables.len() <= 2,
        "Should have at most 2 tables after balancing"
    );
}

#[test]
fn test_table_balancing_with_player_overflow_two() {
    let test_env = TestEnv::new(None);
    // Setup tournament with 6 max players per table, min 2 players per table, and max total 14 players
    let (tournament_id, _) = test_env.setup_multi_table_tournament(8, 2, 14);

    // Register exactly 10 players
    let mut players = Vec::new();
    for i in 0..10 {
        let user = test_env
            .create_user(
                format!("User {}", i),
                WalletPrincipalId(Principal::self_authenticating(format!("user{}balance", i))),
            )
            .unwrap();

        test_env.transfer_approve_tokens_for_testing(
            tournament_id.0,
            user.principal_id,
            1000.0,
            true,
        );
        test_env
            .join_tournament(tournament_id, user.users_canister_id, user.principal_id)
            .unwrap();
        players.push(user.principal_id);
    }

    // Advance time to start tournament
    test_env.pocket_ic.advance_time(Duration::from_secs(120));

    // Tick multiple times to ensure tournament processing
    for _ in 0..50 {
        test_env.pocket_ic.advance_time(Duration::from_secs(12));

        test_env.pocket_ic.tick();
    }

    // Verify initial table distribution
    let tournament = test_env.get_tournament(tournament_id).unwrap();
    assert_eq!(tournament.state, TournamentState::Running);
    println!("### Initial Tournament State ###");
    println!("Total players: {}", tournament.current_players.len());
    println!("Number of tables: {}", tournament.tables.len());

    // With 10 players and max 5 per table, should have 3 tables
    assert_eq!(tournament.tables.len(), 2, "Should have 3 tables initially");

    // Check each table for proper player count
    let mut total_table_players = 0;
    for (table_id, table_info) in &tournament.tables {
        let table = test_env.get_table(*table_id).unwrap();
        println!(
            "Table {}: {} players in table_info, {} in actual table",
            table_id.0.to_text().chars().take(5).collect::<String>(),
            table_info.players.len(),
            table.users.len()
        );
        assert!(
            table_info.players.len() <= 5,
            "Table should not exceed 5 players"
        );
        assert_eq!(
            table_info.players.len(),
            table.users.len(),
            "Table info and actual table should have matching player counts"
        );
        total_table_players += table_info.players.len();
    }

    // Total players across tables should match tournament players
    assert_eq!(
        total_table_players, 10,
        "Total table players should match tournament players"
    );

    // Print which player is on which table
    for (table_id, table_info) in &tournament.tables {
        println!(
            "Table {}: Players: [{}]",
            table_id.0.to_text().chars().take(5).collect::<String>(),
            table_info
                .players
                .iter()
                .map(|p| p.0.to_text().chars().take(4).collect::<String>())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    // Now eliminate players to force table balancing
    println!("\n### Eliminating Players to Force Table Balancing ###");
    let eliminated_players = test_env.eliminate_players(tournament_id, 2);
    assert_eq!(
        eliminated_players.len(),
        2,
        "Should have eliminated 2 players"
    );
    // Verify tables were balanced properly
    let tournament = test_env.get_tournament(tournament_id).unwrap();
    assert_eq!(
        tournament.current_players.len(),
        8,
        "Should have 8 players remaining"
    );

    // Advance time and tick to ensure tournament processing
    for _ in 0..10 {
        test_env.pocket_ic.advance_time(Duration::from_secs(60));
        for _ in 0..30 {
            test_env.pocket_ic.tick();
        }
    }

    let tournament = test_env.get_tournament(tournament_id).unwrap();
    for table in tournament.tables.keys() {
        test_env.complete_table_round(*table);
    }

    for _ in 0..5 {
        test_env.pocket_ic.advance_time(Duration::from_secs(60));
        for _ in 0..30 {
            test_env.pocket_ic.tick();
        }
    }

    // Verify tables were balanced properly
    let tournament = test_env.get_tournament(tournament_id).unwrap();
    let mut table_infos = Vec::new();
    for table in tournament.tables.keys() {
        let table_info = test_env.get_table(*table).unwrap();
        table_infos.push(table_info.users.users.into_keys().collect::<Vec<_>>());
    }
    let total_players_on_tables = table_infos.iter().map(|t| t.len()).sum::<usize>();
    println!("\n### After Eliminations ###");
    println!("Total players: {}", tournament.current_players.len());
    println!("Number of tables: {}", tournament.tables.len());
    for table in table_infos {
        println!(
            "Table players: {:?}",
            table.iter().map(|p| p.0.to_text()).collect::<Vec<_>>()
        );
    }

    // With 7 players eliminated, should now have 7 players across max 2 tables
    assert_eq!(
        tournament.current_players.len(),
        7,
        "Should have 7 players remaining"
    );
    assert_eq!(
        tournament.current_players.len(),
        total_players_on_tables,
        "Total players on tables should match tournament players"
    );
    assert!(
        tournament.tables.len() == 1,
        "Should have at most 2 tables after balancing"
    );
}

#[test]
fn test_table_balancing_with_player_overflow_three() {
    let test_env = TestEnv::new(None);
    // Setup tournament with 6 max players per table, min 2 players per table, and max total 14 players
    let (tournament_id, _) = test_env.setup_multi_table_tournament(8, 2, 48);

    // Register exactly 10 players
    let mut players = Vec::new();
    for i in 0..34 {
        let user = test_env
            .create_user(
                format!("User {}", i),
                WalletPrincipalId(Principal::self_authenticating(format!("user{}balance", i))),
            )
            .unwrap();

        test_env.transfer_approve_tokens_for_testing(
            tournament_id.0,
            user.principal_id,
            1000.0,
            true,
        );
        test_env
            .join_tournament(tournament_id, user.users_canister_id, user.principal_id)
            .unwrap();
        players.push(user.principal_id);
    }

    // Advance time to start tournament
    test_env.pocket_ic.advance_time(Duration::from_secs(120));

    // Tick multiple times to ensure tournament processing
    for _ in 0..50 {
        test_env.pocket_ic.advance_time(Duration::from_secs(12));

        test_env.pocket_ic.tick();
    }

    // Verify initial table distribution
    let tournament = test_env.get_tournament(tournament_id).unwrap();
    assert_eq!(tournament.state, TournamentState::Running);
    println!("### Initial Tournament State ###");
    println!("Total players: {}", tournament.current_players.len());
    println!("Number of tables: {}", tournament.tables.len());

    assert_eq!(tournament.tables.len(), 5, "Should have 5 tables initially");

    // Check each table for proper player count
    let mut total_table_players = 0;
    for (table_id, table_info) in &tournament.tables {
        let table = test_env.get_table(*table_id).unwrap();
        println!(
            "Table {}: {} players in table_info, {} in actual table",
            table_id.0.to_text().chars().take(5).collect::<String>(),
            table_info.players.len(),
            table.users.len()
        );
        assert_eq!(
            table_info.players.len(),
            table.users.len(),
            "Table info and actual table should have matching player counts"
        );
        total_table_players += table_info.players.len();
    }

    let tournament = test_env.get_tournament(tournament_id).unwrap();

    // Total players across tables should match tournament players
    assert_eq!(
        total_table_players,
        tournament.current_players.len(),
        "Total table players should match tournament players"
    );

    // // Now eliminate players to force table balancing
    println!("\n### Eliminating Players to Force Table Balancing ###");
    let eliminated_players = test_env.eliminate_players(tournament_id, 7);
    assert_eq!(
        eliminated_players.len(),
        7,
        "Should have eliminated 7 players"
    );

    for _ in 0..10 {
        test_env.pocket_ic.advance_time(Duration::from_secs(60));
        for _ in 0..30 {
            test_env.pocket_ic.tick();
        }
    }

    let mut total_table_players = 0;
    for (table_id, table_info) in &tournament.tables {
        let table = test_env.get_table(*table_id).unwrap();
        println!(
            "Table {}: {} players in table_info, {} in actual table",
            table_id.0.to_text().chars().take(5).collect::<String>(),
            table_info.players.len(),
            table.users.len()
        );

        total_table_players += table.users.len();
    }

    // Verify tables were balanced properly
    let tournament = test_env.get_tournament(tournament_id).unwrap();

    assert_eq!(
        tournament.current_players.len(),
        total_table_players,
        "Total players on tables should match tournament players"
    );

    println!("\n### Eliminating Players to Force Table Balancing ###");
    let eliminated_players = test_env.eliminate_players(tournament_id, 5);
    assert_eq!(
        eliminated_players.len(),
        5,
        "Should have eliminated 5 players"
    );

    for _ in 0..5 {
        test_env.pocket_ic.advance_time(Duration::from_secs(60));
        for _ in 0..30 {
            test_env.pocket_ic.tick();
        }
    }

    let mut total_table_players = 0;
    for (table_id, table_info) in &tournament.tables {
        let table = test_env.get_table(*table_id).unwrap();
        println!(
            "Table {}: {} players in table_info, {} in actual table",
            table_id.0.to_text().chars().take(5).collect::<String>(),
            table_info.players.len(),
            table.users.len()
        );

        total_table_players += table.users.len();
    }

    // Verify tables were balanced properly
    let tournament = test_env.get_tournament(tournament_id).unwrap();

    assert_eq!(
        tournament.current_players.len(),
        total_table_players,
        "Total players on tables should match tournament players"
    );
}
