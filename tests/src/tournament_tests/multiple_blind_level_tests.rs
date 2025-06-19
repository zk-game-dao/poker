use std::time::Duration;

use candid::Principal;
use currency::Currency;
use table::poker::game::{
    table_functions::{
        table::{TableConfig, TableType},
        types::CurrencyType,
    },
    utils::convert_to_e8s,
};
use tournaments::tournaments::{
    tournament_type::{BuyInOptions, TournamentSizeType, TournamentType},
    types::{NewTournament, NewTournamentSpeedType, PayoutPercentage, TournamentId},
};
use user::user::WalletPrincipalId;

use crate::TestEnv;

// Test utilities remain mostly the same, but adapt for SpeedType
impl TestEnv {
    fn setup_tournament_with_speed(
        &self,
        speed_type: NewTournamentSpeedType,
        starting_chips: u64,
    ) -> (TournamentId, NewTournament) {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let now = std::time::SystemTime::now();
        self.pocket_ic.set_time(now.into());

        let tournament_config = NewTournament {
            name: "Test Tournament".to_string(),
            description: "Test Tournament Description".to_string(),
            hero_picture: "".to_string(),
            currency: CurrencyType::Real(Currency::ICP),
            buy_in: convert_to_e8s(10.0),
            starting_chips,
            speed_type,
            min_players: 5,
            max_players: 8,
            late_registration_duration_ns: 10,
            payout_structure: vec![PayoutPercentage {
                position: 1,
                percentage: 100,
            }],
            tournament_type: TournamentType::BuyIn(TournamentSizeType::SingleTable(
                BuyInOptions::new_freezout(),
            )),
            start_time: current_time + 60_000_000_000,
            require_proof_of_humanity: false,
        };

        let table_config = TableConfig {
            name: "Test Table".to_string(),
            game_type: table::poker::game::types::GameType::NoLimit(0),
            seats: 6,
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
                is_final_table: true,
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

    fn register_min_players(&self, tournament_id: TournamentId) -> Vec<WalletPrincipalId> {
        let mut players = Vec::new();

        for i in 0..5 {
            let user = self
                .create_user(
                    format!("User {}", i),
                    WalletPrincipalId(Principal::self_authenticating(format!("user{}blindtest", i))),
                )
                .expect("Failed to create user");

            self.transfer_approve_tokens_for_testing(
                tournament_id.0,
                user.principal_id,
                1000.0,
                true,
            );

            self.join_tournament(tournament_id, user.users_canister_id, user.principal_id)
                .unwrap();
            players.push(user.principal_id);
        }

        players
    }
}

#[test]
fn test_regular_speed_initial_blinds() {
    let test_env = TestEnv::new(None);
    let starting_chips = 1500;
    let speed_type = NewTournamentSpeedType::Regular(20);

    let (tournament_id, _) = test_env.setup_tournament_with_speed(speed_type, starting_chips);
    test_env.register_min_players(tournament_id);
    test_env.start_tournament();

    // Verify initial blind levels (should be 2% of starting stack)
    let tournament = test_env.get_tournament(tournament_id).unwrap();
    assert_eq!(tournament.speed_type.get_params().current_level, 0);
    let (small, big, ante) = tournament.get_current_blinds();
    assert_eq!(small, 8); // 2% of 1500
    assert_eq!(big, 16); // 4% of 1500
    assert_eq!(ante, 0); // No ante in first level

    // Advance time to trigger blind level increase (10 minute levels)
    test_env.pocket_ic.advance_time(Duration::from_secs(300));
    for _ in 0..30 {
        test_env.pocket_ic.tick();
    }

    let tournament = test_env.get_tournament(tournament_id).unwrap();
    let table_id = *tournament.tables.keys().next().unwrap();
    // Verify table blinds match tournament
    let table = test_env.get_table(table_id).unwrap();
    assert_eq!(table.small_blind, 8);
    assert_eq!(table.big_blind, 16);
}

#[test]
fn test_turbo_blind_progression() {
    let test_env = TestEnv::new(None);
    let starting_chips = 1500;
    let speed_type = NewTournamentSpeedType::Turbo(30);

    let (tournament_id, _) = test_env.setup_tournament_with_speed(speed_type, starting_chips);
    test_env.register_min_players(tournament_id);
    test_env.start_tournament();

    // Verify initial state
    let tournament = test_env.get_tournament(tournament_id).unwrap();
    assert_eq!(tournament.speed_type.get_params().current_level, 0);
    assert!(tournament.speed_type.get_params().next_level_time.is_some());

    // Advance time to trigger blind level increase (10 minute levels)
    test_env.pocket_ic.advance_time(Duration::from_secs(900));
    for _ in 0..10 {
        test_env.pocket_ic.tick();
    }

    // Verify blind level increased by 75%
    let tournament = test_env.get_tournament(tournament_id).unwrap();
    assert_eq!(tournament.speed_type.get_params().current_level, 1);
    let (small, big, _) = tournament.get_current_blinds();
    assert_eq!(small, 14); // ~30 * 1.75 rounded
    assert_eq!(big, 28); // small * 2
}

#[test]
fn test_hyper_turbo_ante_progression() {
    let test_env = TestEnv::new(None);
    let starting_chips = 500; // Smaller stack for hyper-turbo
    let speed_type = NewTournamentSpeedType::HyperTurbo(40);

    let (tournament_id, _) = test_env.setup_tournament_with_speed(speed_type, starting_chips);
    test_env.register_min_players(tournament_id);
    test_env.start_tournament();

    // Level 0: No ante initially
    let tournament = test_env.get_tournament(tournament_id).unwrap();
    let (_, _, ante) = tournament.get_current_blinds();
    assert_eq!(ante, 0);

    // Advance to level where antes kick in (level 2 for hyper-turbo)
    test_env.pocket_ic.advance_time(Duration::from_secs(190)); // 2 levels * 180 seconds
    for _ in 0..5 {
        test_env.pocket_ic.tick();
    }

    test_env.pocket_ic.advance_time(Duration::from_secs(190)); // 2 levels * 180 seconds
    for _ in 0..5 {
        test_env.pocket_ic.tick();
    }

    let tournament = test_env.get_tournament(tournament_id).unwrap();
    let (_, big, ante) = tournament.get_current_blinds();
    assert_eq!(ante, big * 15 / 100); // 15% ante for hyper-turbo
}
