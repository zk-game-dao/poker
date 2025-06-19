use std::time::Duration;

use candid::Principal;
use currency::Currency;
use errors::table_error::TableError;
use table::poker::game::{
    table_functions::{
        table::{TableConfig, TableId, TableType},
        types::{CurrencyType, DealStage},
    },
    utils::convert_to_e8s,
};
use tournaments::tournaments::{
    tournament_type::{BuyInOptions, TournamentSizeType, TournamentType},
    types::{NewTournament, NewTournamentSpeedType, PayoutPercentage, TournamentState},
};
use user::user::WalletPrincipalId;

use crate::TestEnv;

impl TestEnv {
    pub fn complete_betting_round(&self, table_id: TableId) -> Result<(), TableError> {
        // Get current table state
        let mut table = self.get_table(table_id)?;

        // Continue processing until we reach showdown or the round is complete
        while table.deal_stage != DealStage::Showdown && table.sorted_users.is_some() {
            // Get the current player whose turn it is
            let current_player = table
                .get_player_at_seat(table.current_player_index)
                .expect("Failed to get current player");
            self.pocket_ic.tick();

            if self.player_check(table_id, current_player).is_err() {
                self.player_bet(
                    table_id,
                    current_player,
                    table::poker::game::table_functions::types::BetType::Called,
                )?;
            }
            self.pocket_ic.tick();

            // Update table state
            table = self.get_table(table_id).unwrap();
        }

        Ok(())
    }
}

#[test]
fn create_tournament() {
    let test_env = TestEnv::new(None);

    let tournament_config = NewTournament {
        name: "Test Tournament".to_string(),
        description: "Test Tournament Description".to_string(),
        hero_picture: "".to_string(),
        currency: CurrencyType::Real(Currency::ICP),
        buy_in: convert_to_e8s(10.0),
        starting_chips: 1000,
        speed_type: NewTournamentSpeedType::Regular(20),
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
        start_time: u64::MAX,
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

    let id = test_env
        .create_tournament(&tournament_config, &table_config)
        .unwrap();
    let new_tournament = test_env.get_tournament(id).unwrap();
    assert_eq!(new_tournament.name, tournament_config.name);
}

#[test]
fn join_tournament() {
    let test_env = TestEnv::new(None);

    let tournament_config = NewTournament {
        name: "Test Tournament".to_string(),
        description: "Test Tournament Description".to_string(),
        hero_picture: "".to_string(),
        currency: CurrencyType::Real(Currency::ICP),
        buy_in: convert_to_e8s(10.0),
        starting_chips: 1000,
        speed_type: NewTournamentSpeedType::Regular(20),
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
        start_time: u64::MAX,
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

    let id = test_env
        .create_tournament(&tournament_config, &table_config)
        .unwrap();

    let user_1 = test_env
        .create_user(
            "User 1".to_string(),
            WalletPrincipalId(Principal::self_authenticating("user1jointournament")),
        )
        .expect("Failed to create user");
    test_env.transfer_approve_tokens_for_testing(id.0, user_1.principal_id, 1000.0, true);

    test_env
        .join_tournament(id, user_1.users_canister_id, user_1.principal_id)
        .unwrap();
    let tournament_info = test_env.get_tournament(id).unwrap();
    assert_eq!(tournament_info.current_players.len(), 1);
}

#[test]
fn duplicate_join_tournament() {
    let test_env = TestEnv::new(None);

    let tournament_config = NewTournament {
        name: "Test Tournament".to_string(),
        description: "Test Tournament Description".to_string(),
        hero_picture: "".to_string(),
        currency: CurrencyType::Real(Currency::ICP),
        buy_in: convert_to_e8s(10.0),
        starting_chips: 1000,
        speed_type: NewTournamentSpeedType::Regular(20),
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
        start_time: u64::MAX,
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

    let id = test_env
        .create_tournament(&tournament_config, &table_config)
        .unwrap();

    let user_1 = test_env
        .create_user(
            "User 1".to_string(),
            WalletPrincipalId(Principal::self_authenticating("user1duplicatejointournament")),
        )
        .expect("Failed to create user");
    test_env.transfer_approve_tokens_for_testing(id.0, user_1.principal_id, 1000.0, true);

    test_env
        .join_tournament(id, user_1.users_canister_id, user_1.principal_id)
        .unwrap();
    let tournament_info = test_env.get_tournament(id).unwrap();
    assert_eq!(tournament_info.current_players.len(), 1);

    let res = test_env.join_tournament(id, user_1.users_canister_id, user_1.principal_id);
    assert!(res.is_err());
    let tournament_info = test_env.get_tournament(id).unwrap();
    assert_eq!(tournament_info.current_players.len(), 1);
}

#[test]
fn invalid_join_tournament() {
    let test_env = TestEnv::new(None);

    let tournament_config = NewTournament {
        name: "Test Tournament".to_string(),
        description: "Test Tournament Description".to_string(),
        hero_picture: "".to_string(),
        currency: CurrencyType::Real(Currency::ICP),
        buy_in: convert_to_e8s(10.0),
        starting_chips: 1000,
        speed_type: NewTournamentSpeedType::Regular(20),
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
        start_time: u64::MAX,
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

    let id = test_env
        .create_tournament(&tournament_config, &table_config)
        .unwrap();

    let user_1 = test_env
        .create_user(
            "User 1".to_string(),
            WalletPrincipalId(Principal::self_authenticating("user1invalidjointournament")),
        )
        .expect("Failed to create user");
    test_env.transfer_approve_tokens_for_testing(id.0, user_1.principal_id, 1.0, true);

    let res = test_env.join_tournament(id, user_1.users_canister_id, user_1.principal_id);
    assert!(res.is_err());
    let tournament_info = test_env.get_tournament(id).unwrap();
    assert_eq!(tournament_info.current_players.len(), 0);
}

#[test]
fn leave_tournament() {
    let test_env = TestEnv::new(None);

    let tournament_config = NewTournament {
        name: "Test Tournament".to_string(),
        description: "Test Tournament Description".to_string(),
        hero_picture: "".to_string(),
        currency: CurrencyType::Real(Currency::ICP),
        buy_in: convert_to_e8s(10.0),
        starting_chips: 1000,
        speed_type: NewTournamentSpeedType::Regular(20),
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
        start_time: u64::MAX,
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

    let id = test_env
        .create_tournament(&tournament_config, &table_config)
        .unwrap();

    let user_1 = test_env
        .create_user(
            "User 1".to_string(),
            WalletPrincipalId(Principal::self_authenticating("user1leavetournament")),
        )
        .expect("Failed to create user");
    test_env.transfer_approve_tokens_for_testing(id.0, user_1.principal_id, 1000.0, true);

    test_env
        .join_tournament(id, user_1.users_canister_id, user_1.principal_id)
        .unwrap();
    let tournament_info = test_env.get_tournament(id).unwrap();
    assert_eq!(tournament_info.current_players.len(), 1);

    test_env
        .leave_tournament(id, user_1.users_canister_id, user_1.principal_id)
        .unwrap();
    let tournament_info = test_env.get_tournament(id).unwrap();
    assert_eq!(tournament_info.current_players.len(), 0);
}

#[test]
fn start_tournament() {
    let test_env = TestEnv::new(None);

    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    let now = std::time::SystemTime::now();
    test_env.pocket_ic.set_time(now.into());

    // Create tournament config with start time 1 second in the future
    let tournament_config = NewTournament {
        name: "Test Tournament".to_string(),
        description: "Test Tournament Description".to_string(),
        hero_picture: "".to_string(),
        currency: CurrencyType::Real(Currency::ICP),
        buy_in: convert_to_e8s(10.0),
        starting_chips: 1000,
        speed_type: NewTournamentSpeedType::Regular(20),
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
        start_time: current_time + 1_000_000_000, // 1 second in future
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

    let id = test_env
        .create_tournament(&tournament_config, &table_config)
        .unwrap();

    println!("Joining tournament");
    // Add 5 players (minimum required)
    for i in 0..5 {
        let user = test_env
            .create_user(
                format!("User {}", i),
                WalletPrincipalId(Principal::self_authenticating(format!("user{}starttournament", i))),
            )
            .expect("Failed to create user");

        test_env.transfer_approve_tokens_for_testing(id.0, user.principal_id, 1000.0, true);

        test_env
            .join_tournament(id, user.users_canister_id, user.principal_id)
            .unwrap();
    }

    // Verify initial state
    let tournament = test_env.get_tournament(id).unwrap();
    assert_eq!(tournament.state, TournamentState::Registration);
    assert_eq!(tournament.current_players.len(), 5);

    // Tournament shouldn't start before start time
    test_env.pocket_ic.tick();
    let tournament = test_env.get_tournament(id).unwrap();
    assert_eq!(tournament.state, TournamentState::Registration);

    println!("Starting tournament");
    // Advance time past start_time
    test_env
        .pocket_ic
        .advance_time(Duration::from_nanos(3_000_000_000_000)); // 2 seconds

    // Now tournament should start
    for _ in 0..20 {
        test_env.pocket_ic.tick();
    }
    let tournament = test_env.get_tournament(id).unwrap();
    let table = *tournament.tables.keys().next().unwrap();
    let table_info = test_env.get_table(table).unwrap();
    assert_eq!(table_info.deal_stage, DealStage::Flop);
    assert_eq!(tournament.state, TournamentState::Running);
    assert!(tournament.speed_type.get_params().next_level_time.is_some());
}
