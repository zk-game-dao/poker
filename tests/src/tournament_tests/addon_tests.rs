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
    tournament_type::{
        AddonOptions, BuyInOptions, RebuyOptions, ReentryOptions, TournamentSizeType,
        TournamentType,
    },
    types::{NewTournament, NewTournamentSpeedType, PayoutPercentage},
};

use crate::TestEnv;

impl TestEnv {
    pub fn create_addon_tournament(
        &self,
        addon_enabled: bool,
        mut addon_start_time: u64,
        addon_end_time: u64,
        addon_chips: u64,
        addon_price: u64,
        max_addons: u32,
    ) -> Principal {
        // Get current time for setting start time
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        // Set the start time to be in the near future
        let start_time = current_time + 1_000_000_000; // 1 second from now
        if addon_start_time == 0 {
            addon_start_time = start_time + 4e11 as u64;
        }

        // Set the mock time to current for the test
        let now = std::time::SystemTime::now();
        self.pocket_ic.set_time(now.into());

        // Create tournament configuration
        let tournament_config = NewTournament {
            name: "Test Tournament".to_string(),
            description: "Test Tournament Description".to_string(),
            hero_picture: "".to_string(),
            currency: CurrencyType::Real(Currency::ICP),
            buy_in: convert_to_e8s(10.0),
            starting_chips: 1000,
            speed_type: NewTournamentSpeedType::Regular(20),
            min_players: 2,
            max_players: 8,
            late_registration_duration_ns: 10,
            payout_structure: vec![PayoutPercentage {
                position: 1,
                percentage: 100,
            }],
            tournament_type: TournamentType::BuyIn(TournamentSizeType::SingleTable(BuyInOptions {
                freezout: false,
                addon: AddonOptions {
                    enabled: addon_enabled,
                    addon_start_time,
                    addon_end_time,
                    addon_chips,
                    addon_price,
                    max_addons,
                },
                reentry: ReentryOptions {
                    enabled: false,
                    max_reentries: 0,
                    reentry_end_timestamp: 0,
                    reentry_price: 0,
                    reentry_chips: 0,
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
            })),
            start_time,
            require_proof_of_humanity: false,
        };

        // Create table configuration
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

        // Create the tournament and return its ID
        self.create_tournament(&tournament_config, &table_config)
            .unwrap()
    }
}

#[test]
fn test_addon_for_tournament() {
    let test_env = TestEnv::new(None);

    let id = test_env.create_addon_tournament(
        true,                 // addon_enabled
        0,                    // addon_start_time (0 means it can start immediately)
        u64::MAX,             // addon_end_time (u64::MAX means it never ends)
        1000,                 // addon_chips
        convert_to_e8s(10.0), // addon_price
        1,                    // max_addons
    );

    let user_1 = test_env
        .create_user(
            "User 1".to_string(),
            Principal::self_authenticating("user1addontest"),
        )
        .expect("Failed to create user");
    test_env.transfer_approve_tokens_for_testing(id, user_1.principal_id, 1000.0, true);

    test_env
        .join_tournament(id, user_1.users_canister_id, user_1.principal_id)
        .unwrap();
    let tournament_info = test_env.get_tournament(id).unwrap();
    assert_eq!(tournament_info.current_players.len(), 1);

    let user_2 = test_env
        .create_user(
            "User 2".to_string(),
            Principal::self_authenticating("user2invalidbuyinaddon"),
        )
        .expect("Failed to create user");
    test_env.transfer_approve_tokens_for_testing(id, user_2.principal_id, 15.0, true);

    test_env
        .join_tournament(id, user_2.users_canister_id, user_2.principal_id)
        .unwrap();
    let tournament_info = test_env.get_tournament(id).unwrap();
    assert_eq!(tournament_info.current_players.len(), 2);

    // Advance time past start_time
    test_env
        .pocket_ic
        .advance_time(Duration::from_nanos(3_000_000_000_000)); // 2 seconds

    // Now tournament should start
    for _ in 0..20 {
        test_env.pocket_ic.tick();
    }
    let new_tournament = test_env.get_tournament(id).unwrap();
    let tournament_table = new_tournament.tables.keys().next().unwrap();

    let table = test_env.get_table(*tournament_table).unwrap();
    assert!(table.users.get(&user_1.principal_id).unwrap().balance == 990 || table.users.get(&user_1.principal_id).unwrap().balance == 995);

    test_env
        .user_refill_chips(
            id,
            user_1.users_canister_id,
            *tournament_table,
            user_1.principal_id,
        )
        .unwrap();
    test_env
        .player_fold(*tournament_table, user_2.principal_id)
        .unwrap();
    test_env
        .pocket_ic
        .advance_time(Duration::from_nanos(3_000_000_000_000)); // 2 seconds

    // Now tournament should start
    for _ in 0..20 {
        test_env.pocket_ic.tick();
    }
    let table = test_env.get_table(*tournament_table).unwrap();
    assert!(table.users.get(&user_1.principal_id).unwrap().balance == 1997 || table.users.get(&user_1.principal_id).unwrap().balance == 1994);
}

#[test]
fn test_invalid_buyin_addon_for_tournament() {
    let test_env = TestEnv::new(None);

    let id = test_env.create_addon_tournament(
        true,                 // addon_enabled
        0,                    // addon_start_time
        u64::MAX,             // addon_end_time
        1000,                 // addon_chips
        convert_to_e8s(11.0), // addon_price (higher than buy-in)
        1,                    // max_addons
    );

    let user_1 = test_env
        .create_user(
            "User 1".to_string(),
            Principal::self_authenticating("user1invalidbuyinaddon"),
        )
        .expect("Failed to create user");
    test_env.transfer_approve_tokens_for_testing(id, user_1.principal_id, 15.0, true);

    test_env
        .join_tournament(id, user_1.users_canister_id, user_1.principal_id)
        .unwrap();
    let tournament_info = test_env.get_tournament(id).unwrap();
    assert_eq!(tournament_info.current_players.len(), 1);

    let user_2 = test_env
        .create_user(
            "User 2".to_string(),
            Principal::self_authenticating("user2invalidbuyinaddon"),
        )
        .expect("Failed to create user");
    test_env.transfer_approve_tokens_for_testing(id, user_2.principal_id, 15.0, true);

    test_env
        .join_tournament(id, user_2.users_canister_id, user_2.principal_id)
        .unwrap();
    let tournament_info = test_env.get_tournament(id).unwrap();
    assert_eq!(tournament_info.current_players.len(), 2);

    test_env
        .pocket_ic
        .advance_time(Duration::from_nanos(3_000_000_000_000)); // 2 seconds

    // Now tournament should start
    for _ in 0..20 {
        test_env.pocket_ic.tick();
    }
    let new_tournament = test_env.get_tournament(id).unwrap();
    let tournament_table = new_tournament.tables.keys().next().unwrap();
    let table = test_env.get_table(*tournament_table).unwrap();
    assert!(table.users.get(&user_1.principal_id).unwrap().balance == 990 || table.users.get(&user_1.principal_id).unwrap().balance == 995);

    let res = test_env.user_refill_chips(
        id,
        user_1.users_canister_id,
        *tournament_table,
        user_1.principal_id,
    );
    assert!(res.is_err());
    let table = test_env.get_table(*tournament_table).unwrap();
    assert!(table.users.get(&user_1.principal_id).unwrap().balance == 990 || table.users.get(&user_1.principal_id).unwrap().balance == 995);
}

#[test]
fn test_addon_time_passed_for_tournament() {
    let test_env = TestEnv::new(None);

    let id = test_env.create_addon_tournament(
        true,                 // addon_enabled
        0,                    // addon_start_time
        1000,                 // addon_end_time (very short time window)
        1000,                 // addon_chips
        convert_to_e8s(11.0), // addon_price
        1,                    // max_addons
    );

    let user_1 = test_env
        .create_user(
            "User 1".to_string(),
            Principal::self_authenticating("user1invalidbuyinaddontimehasntended"),
        )
        .expect("Failed to create user");
    test_env.transfer_approve_tokens_for_testing(id, user_1.principal_id, 1000.0, true);

    test_env
        .join_tournament(id, user_1.users_canister_id, user_1.principal_id)
        .unwrap();
    let tournament_info = test_env.get_tournament(id).unwrap();
    assert_eq!(tournament_info.current_players.len(), 1);

    let user_2 = test_env
        .create_user(
            "User 2".to_string(),
            Principal::self_authenticating("user2invalidbuyinaddon"),
        )
        .expect("Failed to create user");
    test_env.transfer_approve_tokens_for_testing(id, user_2.principal_id, 15.0, true);

    test_env
        .join_tournament(id, user_2.users_canister_id, user_2.principal_id)
        .unwrap();
    let tournament_info = test_env.get_tournament(id).unwrap();
    assert_eq!(tournament_info.current_players.len(), 2);

    test_env
        .pocket_ic
        .advance_time(Duration::from_nanos(3_000_000_000_000)); // 2 seconds

    // Now tournament should start
    for _ in 0..20 {
        test_env.pocket_ic.tick();
    }
    let new_tournament = test_env.get_tournament(id).unwrap();
    let tournament_table = new_tournament.tables.keys().next().unwrap();

    let table = test_env.get_table(*tournament_table).unwrap();
    assert!(table.users.get(&user_1.principal_id).unwrap().balance == 990 || table.users.get(&user_1.principal_id).unwrap().balance == 995);

    let res = test_env.user_refill_chips(
        id,
        user_1.users_canister_id,
        *tournament_table,
        user_1.principal_id,
    );
    assert!(res.is_err());
    let table = test_env.get_table(*tournament_table).unwrap();
    assert!(table.users.get(&user_1.principal_id).unwrap().balance == 990 || table.users.get(&user_1.principal_id).unwrap().balance == 995);
}

#[test]
fn test_addon_time_hasnt_started_for_tournament() {
    let test_env = TestEnv::new(None);

    let id = test_env.create_addon_tournament(
        true,                 // addon_enabled
        u64::MAX - 1000,      // addon_start_time
        u64::MAX,             // addon_end_time (very short time window)
        1000,                 // addon_chips
        convert_to_e8s(11.0), // addon_price
        1,                    // max_addons
    );

    let user_1 = test_env
        .create_user(
            "User 1".to_string(),
            Principal::self_authenticating("user1invalidbuyinaddontimehasntstarted"),
        )
        .expect("Failed to create user");
    test_env.transfer_approve_tokens_for_testing(id, user_1.principal_id, 1000.0, true);

    test_env
        .join_tournament(id, user_1.users_canister_id, user_1.principal_id)
        .unwrap();
    let tournament_info = test_env.get_tournament(id).unwrap();
    assert_eq!(tournament_info.current_players.len(), 1);

    let user_2 = test_env
        .create_user(
            "User 2".to_string(),
            Principal::self_authenticating("user2invalidbuyinaddon"),
        )
        .expect("Failed to create user");
    test_env.transfer_approve_tokens_for_testing(id, user_2.principal_id, 15.0, true);

    test_env
        .join_tournament(id, user_2.users_canister_id, user_2.principal_id)
        .unwrap();
    let tournament_info = test_env.get_tournament(id).unwrap();
    assert_eq!(tournament_info.current_players.len(), 2);

    test_env
        .pocket_ic
        .advance_time(Duration::from_nanos(3_000_000_000_000)); // 2 seconds

    // Now tournament should start
    for _ in 0..20 {
        test_env.pocket_ic.tick();
    }
    let new_tournament = test_env.get_tournament(id).unwrap();
    let tournament_table = new_tournament.tables.keys().next().unwrap();

    let table = test_env.get_table(*tournament_table).unwrap();
    assert!(table.users.get(&user_1.principal_id).unwrap().balance == 990 || table.users.get(&user_1.principal_id).unwrap().balance == 995);

    let res = test_env.user_refill_chips(
        id,
        user_1.users_canister_id,
        *tournament_table,
        user_1.principal_id,
    );
    assert!(res.is_err());
    let table = test_env.get_table(*tournament_table).unwrap();
    assert!(table.users.get(&user_1.principal_id).unwrap().balance == 990 || table.users.get(&user_1.principal_id).unwrap().balance == 995);
}
