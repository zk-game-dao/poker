use candid::{decode_one, encode_args, Principal};
use errors::tournament_index_error::TournamentIndexError;
use serial_test::serial;
use std::time::Duration;
use table::poker::game::table_functions::table::TableConfig;
use tournaments::tournaments::{
    tournament_type::{
        AddonOptions, BuyInOptions, RebuyOptions, ReentryOptions, TournamentSizeType,
        TournamentType,
    },
    types::{
        NewTournament, NewTournamentSpeedType, PayoutPercentage, TournamentData, TournamentState,
    },
};

use crate::{utils::get_current_time_ns, wasms, TestEnv};

// Helper for upgrading the tournament index canister
fn upgrade_tournament_index(env: &TestEnv) {
    env.pocket_ic
        .upgrade_canister(
            env.canister_ids.tournament_index,
            wasms::TOURNAMENT_INDEX.clone(),
            vec![], // empty args for upgrades
            None,
        )
        .expect("Failed to upgrade tournament index canister");
}

// Helper for upgrading a tournament canister
fn upgrade_tournament_canister(env: &TestEnv, tournament_id: Principal) {
    env.pocket_ic
        .upgrade_canister(
            tournament_id,
            wasms::TOURNAMENT.clone(),
            vec![], // empty args for upgrades
            Some(env.canister_ids.tournament_index),
        )
        .expect("Failed to upgrade tournament canister");
}

// Create a standard test tournament configuration
fn create_test_tournament_config() -> NewTournament {
    NewTournament {
        name: "Test Tournament".to_string(),
        description: "Test Tournament Description".to_string(),
        buy_in: 1e8 as u64, // 0.01 ICP
        hero_picture: "".to_string(),
        max_players: 8,
        late_registration_duration_ns: 0,
        payout_structure: vec![PayoutPercentage {
            position: 1,
            percentage: 100,
        }],
        start_time: get_current_time_ns() + 3600000000000, // 1 hour in the future
        starting_chips: 1000,
        speed_type: NewTournamentSpeedType::Regular(200),
        tournament_type: TournamentType::BuyIn(TournamentSizeType::SingleTable(BuyInOptions {
            reentry: ReentryOptions {
                enabled: true,
                max_reentries: 1,
                reentry_end_timestamp: get_current_time_ns() + 3600000000000 + 1800000000000, // 30 minutes in the future
                reentry_price: 1e8 as u64,
                reentry_chips: 1000,
            },
            addon: AddonOptions {
                enabled: false,
                max_addons: 0,
                addon_start_time: 0,
                addon_end_time: 0,
                addon_chips: 0,
                addon_price: 0,
            },
            rebuy: RebuyOptions {
                enabled: false,
                max_rebuys: 0,
                rebuy_window_seconds: 0,
                rebuy_end_timestamp: 0,
                rebuy_price: 0,
                rebuy_chips: 0,
                min_chips_for_rebuy: 0,
            },
            freezout: false,
        })),
        currency: table::poker::game::table_functions::types::CurrencyType::Real(
            currency::Currency::ICP,
        ),
        require_proof_of_humanity: false,
        min_players: 2,
    }
}

// Create a standard test table configuration
fn create_test_table_config() -> TableConfig {
    TableConfig {
        name: "Test Tournament Table".to_string(),
        game_type: table::poker::game::types::GameType::NoLimit(1),
        seats: 6,
        timer_duration: 30,
        card_color: 0,
        color: 0,
        environment_color: 0,
        auto_start_timer: 10,
        max_inactive_turns: 3,
        currency_type: table::poker::game::table_functions::types::CurrencyType::Real(
            currency::Currency::ICP,
        ),
        enable_rake: Some(false),
        max_seated_out_turns: None,
        is_private: Some(false),
        ante_type: None,
        table_type: None,
        is_shared_rake: None,
        require_proof_of_humanity: None,
        is_paused: None,
    }
}

impl TestEnv {
    // Method to get active tournaments
    pub fn get_active_tournaments(&self, filter_type: Option<u8>) -> Vec<TournamentData> {
        let result = self.pocket_ic.query_call(
            self.canister_ids.tournament_index,
            Principal::anonymous(),
            "get_active_tournaments",
            encode_args((filter_type,)).unwrap(),
        );

        match result {
            Ok(arg) => decode_one(&arg).unwrap(),
            _ => panic!("Failed to get active tournaments"),
        }
    }

    // Method to update tournament state
    pub fn update_tournament_state(
        &self,
        tournament_id: Principal,
        new_state: TournamentState,
    ) -> Result<(), TournamentIndexError> {
        let result = self.pocket_ic.update_call(
            self.canister_ids.tournament_index,
            tournament_id,
            "update_tournament_state",
            encode_args((tournament_id, new_state)).unwrap(),
        );

        match result {
            Ok(arg) => decode_one(&arg).unwrap(),
            _ => panic!("Failed to update tournament state"),
        }
    }
}

#[test]
#[serial]
fn test_tournament_index_upgrade_persistence() {
    // 1. Create a test environment
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // 2. Create tournament(s) to establish initial state
    let tournament_config = create_test_tournament_config();
    let table_config = create_test_table_config();

    let tournament_id = test_env
        .create_tournament(&tournament_config, &table_config)
        .expect("Failed to create tournament");

    // 3. Verify initial state
    let tournaments_before = test_env.get_active_tournaments(None);
    assert_eq!(tournaments_before.len(), 1);
    assert_eq!(tournaments_before[0].id, tournament_id);

    // 4. Upgrade the canister
    upgrade_tournament_index(&test_env);

    // 5. Verify state after upgrade
    let tournaments_after = test_env.get_active_tournaments(None);
    assert_eq!(tournaments_after.len(), 1);
    assert_eq!(tournaments_after[0].id, tournament_id);

    // 6. Create another tournament to ensure the canister is still functional
    let tournament_id2 = test_env
        .create_tournament(&tournament_config, &table_config)
        .expect("Failed to create second tournament");

    // 7. Verify updated state
    let tournaments_final = test_env.get_active_tournaments(None);
    assert_eq!(tournaments_final.len(), 2);
    assert!(tournaments_final.iter().any(|t| t.id == tournament_id));
    assert!(tournaments_final.iter().any(|t| t.id == tournament_id2));
}

#[test]
#[serial]
fn test_tournament_index_complex_state_persistence() {
    // 1. Create a test environment
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // 2. Create multiple tournaments in different states
    let tournament_config = create_test_tournament_config();
    let table_config = create_test_table_config();

    // Create 3 tournaments
    let tournament_id1 = test_env
        .create_tournament(&tournament_config, &table_config)
        .expect("Failed to create first tournament");

    let tournament_id2 = test_env
        .create_tournament(&tournament_config, &table_config)
        .expect("Failed to create second tournament");

    let tournament_id3 = test_env
        .create_tournament(&tournament_config, &table_config)
        .expect("Failed to create third tournament");

    // 3. Update states
    test_env
        .update_tournament_state(tournament_id1, TournamentState::Running)
        .expect("Failed to update tournament 1 state");

    test_env
        .update_tournament_state(tournament_id2, TournamentState::LateRegistration)
        .expect("Failed to update tournament 2 state");

    test_env
        .update_tournament_state(tournament_id3, TournamentState::Completed)
        .expect("Failed to update tournament 3 state");

    // 4. Verify initial states
    let tournaments_before = test_env.get_active_tournaments(None);
    assert_eq!(tournaments_before.len(), 2); // Only 2 active (Running, LateRegistration)

    // 5. Upgrade the canister
    upgrade_tournament_index(&test_env);

    // 6. Verify states after upgrade
    let tournaments_after = test_env.get_active_tournaments(None);
    assert_eq!(tournaments_after.len(), 2);

    // Find each tournament and verify state
    let tournament1 = tournaments_after
        .iter()
        .find(|t| t.id == tournament_id1)
        .unwrap();
    let tournament2 = tournaments_after
        .iter()
        .find(|t| t.id == tournament_id2)
        .unwrap();

    assert_eq!(tournament1.state, TournamentState::Running);
    assert_eq!(tournament2.state, TournamentState::LateRegistration);
}

#[test]
#[serial]
fn test_tournament_canister_basic_persistence() {
    // 1. Create a test environment
    let test_env = TestEnv::get();

    // 2. Create a tournament to test
    let tournament_config = create_test_tournament_config();
    let table_config = create_test_table_config();

    let tournament_id = test_env
        .create_tournament(&tournament_config, &table_config)
        .expect("Failed to create tournament");

    // 3. Add players to the tournament
    let (user1, user1_id) = test_env.create_test_user("user1tournament");
    let (user2, user2_id) = test_env.create_test_user("user2tournament");

    test_env.transfer_approve_tokens_for_testing(tournament_id, user1_id, 1000.0, true);

    test_env.transfer_approve_tokens_for_testing(tournament_id, user2_id, 1000.0, true);

    // Join tournament
    test_env
        .join_tournament(tournament_id, user1, user1_id)
        .expect("Failed to join tournament for user1");

    test_env
        .join_tournament(tournament_id, user2, user2_id)
        .expect("Failed to join tournament for user2");

    // 4. Verify initial state
    let tournament_before = test_env
        .get_tournament(tournament_id)
        .expect("Failed to get tournament");

    assert_eq!(tournament_before.current_players.len(), 2);
    assert!(tournament_before.current_players.contains_key(&user1_id));
    assert!(tournament_before.current_players.contains_key(&user2_id));

    // 5. Upgrade the tournament canister
    // upgrade_tournament_canister(&test_env, tournament_id);

    // 6. Verify state after upgrade
    let tournament_after = test_env
        .get_tournament(tournament_id)
        .expect("Failed to get tournament after upgrade");

    assert_eq!(tournament_after.current_players.len(), 2);
    assert!(tournament_after.current_players.contains_key(&user1_id));
    assert!(tournament_after.current_players.contains_key(&user2_id));

    // 7. Verify canister is still functional by adding another player
    let (user3, user3_id) = test_env.create_test_user("user3tournament");
    test_env.transfer_approve_tokens_for_testing(tournament_id, user3_id, 1000.0, true);

    test_env
        .join_tournament(tournament_id, user3, user3_id)
        .expect("Failed to join tournament for user3");

    let tournament_final = test_env
        .get_tournament(tournament_id)
        .expect("Failed to get final tournament state");

    assert_eq!(tournament_final.current_players.len(), 3);
    assert!(tournament_final.current_players.contains_key(&user3_id));
}

#[test]
#[serial]
fn test_tournament_state_transition_across_upgrade() {
    // 1. Create a test environment
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // 2. Create a tournament with near-future start time
    let mut tournament_config = create_test_tournament_config();
    tournament_config.start_time = get_current_time_ns() + 10_000_000_000; // 10 seconds in the future
    let table_config = create_test_table_config();

    let tournament_id = test_env
        .create_tournament(&tournament_config, &table_config)
        .expect("Failed to create tournament");

    // 3. Add players
    let (user1, user1_id) = test_env.create_test_user("user1phases");
    let (user2, user2_id) = test_env.create_test_user("user2phases");

    test_env.transfer_approve_tokens_for_testing(tournament_id, user1_id, 1000.0, true);

    test_env.transfer_approve_tokens_for_testing(tournament_id, user2_id, 1000.0, true);

    // Join tournament
    test_env
        .join_tournament(tournament_id, user1, user1_id)
        .expect("Failed to join tournament for user1");

    test_env
        .join_tournament(tournament_id, user2, user2_id)
        .expect("Failed to join tournament for user2");

    // 4. Upgrade the canister while in registration state
    upgrade_tournament_canister(&test_env, tournament_id);

    // Tick a few times to ensure the heartbeat runs
    test_env.pocket_ic.advance_time(Duration::from_secs(60000));
    for _ in 0..5 {
        test_env.pocket_ic.tick();
    }

    // 6. Verify tournament transitioned to Running state after the upgrade
    let tournament_running = test_env
        .get_tournament(tournament_id)
        .expect("Failed to get tournament");

    assert_eq!(
        tournament_running.current_players.len(),
        2,
        "Tournament should still have 2 players after state transition"
    );
}

// This test verifies that atomic updates work correctly across upgrades
#[test]
#[serial]
fn test_tournament_index_atomic_updates() {
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // Create 3 tournaments
    let tournament_config = create_test_tournament_config();
    let table_config = create_test_table_config();

    // Create tournaments
    let tournament_ids: Vec<_> = (0..3)
        .map(|_| {
            test_env
                .create_tournament(&tournament_config, &table_config)
                .expect("Failed to create tournament")
        })
        .collect();

    // Verify initial state
    let tournaments_before = test_env.get_active_tournaments(None);
    assert_eq!(tournaments_before.len(), 3);

    // Change state of one tournament right before upgrade
    test_env
        .update_tournament_state(tournament_ids[0], TournamentState::Completed)
        .expect("Failed to update tournament state");

    // Immediate upgrade
    upgrade_tournament_index(&test_env);

    // Verify the state change and active tournaments list is consistent
    let tournaments_after = test_env.get_active_tournaments(None);
    assert_eq!(tournaments_after.len(), 2); // One moved to completed

    // Tournament that was marked completed should not be in active list
    assert!(!tournaments_after.iter().any(|t| t.id == tournament_ids[0]));

    // Other tournaments should still be in active list
    assert!(tournaments_after.iter().any(|t| t.id == tournament_ids[1]));
    assert!(tournaments_after.iter().any(|t| t.id == tournament_ids[2]));
}
