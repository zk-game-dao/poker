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
    types::{NewTournament, NewTournamentSpeedType, PayoutPercentage, TournamentState},
};

use crate::TestEnv;

#[test]
fn test_late_registration() {
    let test_env = TestEnv::new(None);

    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    let now = std::time::SystemTime::now();
    test_env.pocket_ic.set_time(now.into());

    // Create tournament config with late registration period of 10 seconds
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
        late_registration_duration_ns: 600_000_000_000, // 10 minutes late registration
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

    // Register minimum required players (5)
    for i in 0..5 {
        let user = test_env
            .create_user(
                format!("User {}", i),
                Principal::self_authenticating(format!("user{}latereg", i)),
            )
            .expect("Failed to create user");

        test_env.transfer_approve_tokens_for_testing(id, user.principal_id, 1000.0, true);

        test_env
            .join_tournament(id, user.users_canister_id, user.principal_id)
            .unwrap();
    }

    // Verify initial state
    let tournament = test_env.get_tournament(id).unwrap();
    assert_eq!(tournament.state, TournamentState::Registration);
    assert_eq!(tournament.current_players.len(), 5);

    // Advance time past start_time
    test_env.pocket_ic.advance_time(Duration::from_secs(60)); // 2 seconds
    for _ in 0..6 {
        test_env.pocket_ic.tick();
    }

    // Verify tournament has started and is in late registration
    let tournament = test_env.get_tournament(id).unwrap();
    assert_eq!(tournament.state, TournamentState::LateRegistration);

    // Try to register during late registration period
    let late_user = test_env
        .create_user(
            "Late User".to_string(),
            Principal::self_authenticating("lateusertest"),
        )
        .expect("Failed to create user");

    test_env.transfer_approve_tokens_for_testing(id, late_user.principal_id, 1000.0, true);

    for _ in 0..6 {
        test_env.pocket_ic.advance_time(Duration::from_secs(60)); // 2 seconds
        for _ in 0..6 {
            test_env.pocket_ic.tick();
        }
        test_env.pocket_ic.tick();
    }

    // Should succeed during late registration
    test_env
        .join_tournament(id, late_user.users_canister_id, late_user.principal_id)
        .unwrap();

    let tournament = test_env.get_tournament(id).unwrap();
    assert_eq!(tournament.current_players.len(), 6);

    // Advance time past late registration period
    test_env.pocket_ic.advance_time(Duration::from_secs(660)); // 11 minutes total
    for _ in 0..6 {
        test_env.pocket_ic.tick();
    }

    // Try to register after late registration period
    let too_late_user = test_env
        .create_user(
            "Too Late User".to_string(),
            Principal::self_authenticating("toolateusertest"),
        )
        .expect("Failed to create user");

    test_env.transfer_approve_tokens_for_testing(id, too_late_user.principal_id, 1000.0, true);

    // Should fail after late registration period
    let result = test_env.join_tournament(
        id,
        too_late_user.users_canister_id,
        too_late_user.principal_id,
    );
    assert!(result.is_err());

    let tournament = test_env.get_tournament(id).unwrap();
    assert_eq!(tournament.current_players.len(), 6); // Still 6 players
    assert_eq!(tournament.state, TournamentState::Running);
}
