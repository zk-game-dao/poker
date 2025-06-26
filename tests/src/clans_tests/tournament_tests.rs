use candid::Principal;
use clan::subscriptions::ClanRole;
use currency::Currency;
use errors::clan_error::ClanError;
use serial_test::serial;
use table::poker::game::{
    table_functions::{
        table::{TableConfig, TableType},
        types::CurrencyType,
    },
    utils::convert_to_e8s,
};
use tournaments::tournaments::{
    tournament_type::{BuyInOptions, TournamentSizeType, TournamentType},
    types::{NewTournament, NewTournamentSpeedType, TournamentId},
};
use user::user::WalletPrincipalId;

use crate::TestEnv;

#[test]
#[serial]
fn test_create_clan_tournament_success() {
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // Create clan
    let (clan, creator_id, _) = test_env.create_test_clan("Tournament Clan", "tournament_creator");

    // Create tournament configuration using the same structure as actual tournament tests
    let tournament_config = NewTournament {
        name: "Clan Championship".to_string(),
        description: "Monthly clan championship tournament".to_string(),
        hero_picture: "".to_string(),
        currency: CurrencyType::Real(Currency::ICP),
        buy_in: convert_to_e8s(10.0),
        starting_chips: 1000,
        speed_type: NewTournamentSpeedType::Regular(20),
        min_players: 5,
        max_players: 8,
        late_registration_duration_ns: 10,
        guaranteed_prize_pool: None,
        tournament_type: TournamentType::BuyIn(TournamentSizeType::SingleTable(
            BuyInOptions::new_freezout(),
        )),
        start_time: u64::MAX, // Manual start
        require_proof_of_humanity: false,
    };

    // Get table config for tournament
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
        currency_type: CurrencyType::Real(Currency::ICP),
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

    // Create clan tournament
    let result = test_env.create_clan_tournament(
        clan.id.0,
        &tournament_config,
        &table_config,
        creator_id,
    );
    assert!(result.is_ok(), "Failed to create clan tournament");

    let tournament_id = result.unwrap();

    // Verify tournament is added to clan's tournament list
    let clan_tournaments = test_env
        .get_clan_tournaments(clan.id.0)
        .expect("Failed to get clan tournaments");
    assert_eq!(clan_tournaments.len(), 1);
    assert!(clan_tournaments.contains(&tournament_id.id));

    // Verify tournament details
    let tournament = test_env.get_tournament(tournament_id.id).unwrap();
    assert_eq!(tournament.name, "Clan Championship");
    assert_eq!(tournament.buy_in, convert_to_e8s(10.0));
    assert_eq!(tournament.max_players, 8);
}

#[test]
#[serial]
fn test_create_multiple_clan_tournaments() {
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // Create clan
    let (clan, creator_id, _) = test_env.create_test_clan("Multi Tournament Clan", "multi_tournament_creator");

    // Create first tournament (low stakes)
    let low_stakes_tournament = NewTournament {
        name: "Weekly Low Stakes".to_string(),
        description: "Weekly low stakes tournament".to_string(),
        hero_picture: "".to_string(),
        currency: CurrencyType::Real(Currency::ICP),
        buy_in: convert_to_e8s(5.0),
        starting_chips: 1000,
        speed_type: NewTournamentSpeedType::Regular(20),
        min_players: 4,
        max_players: 6,
        late_registration_duration_ns: 10,
        guaranteed_prize_pool: None,
        tournament_type: TournamentType::BuyIn(TournamentSizeType::SingleTable(
            BuyInOptions::new_freezout(),
        )),
        start_time: u64::MAX,
        require_proof_of_humanity: false,
    };

    // Create second tournament (high stakes)
    let high_stakes_tournament = NewTournament {
        name: "Monthly High Stakes".to_string(),
        description: "Monthly high stakes championship".to_string(),
        hero_picture: "".to_string(),
        currency: CurrencyType::Real(Currency::ICP),
        buy_in: convert_to_e8s(50.0),
        starting_chips: 2000,
        speed_type: NewTournamentSpeedType::Turbo(30),
        min_players: 6,
        max_players: 10,
        late_registration_duration_ns: 20,
        guaranteed_prize_pool: Some(convert_to_e8s(1000.0)),
        tournament_type: TournamentType::BuyIn(TournamentSizeType::SingleTable(
            BuyInOptions::new_freezout(),
        )),
        start_time: u64::MAX,
        require_proof_of_humanity: false,
    };

    let table_config = TableConfig {
        name: "Test Table".to_string(),
        game_type: table::poker::game::types::GameType::NoLimit(0),
        seats: 8,
        timer_duration: 30,
        card_color: 0,
        color: 0,
        environment_color: 0,
        auto_start_timer: 10,
        max_inactive_turns: 3,
        currency_type: CurrencyType::Real(Currency::ICP),
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

    // Create both tournaments
    let tournament1 = test_env
        .create_clan_tournament(clan.id.0, &low_stakes_tournament, &table_config, creator_id)
        .expect("Failed to create low stakes tournament");

    let tournament2 = test_env
        .create_clan_tournament(clan.id.0, &high_stakes_tournament, &table_config, creator_id)
        .expect("Failed to create high stakes tournament");

    // Verify both tournaments exist
    let clan_tournaments = test_env
        .get_clan_tournaments(clan.id.0)
        .expect("Failed to get clan tournaments");
    assert_eq!(clan_tournaments.len(), 2);
    assert!(clan_tournaments.contains(&tournament1.id));
    assert!(clan_tournaments.contains(&tournament2.id));

    // Verify tournament differences
    let tournament1_details = test_env.get_tournament(tournament1.id).unwrap();
    let tournament2_details = test_env.get_tournament(tournament2.id).unwrap();
    assert_ne!(tournament1_details.buy_in, tournament2_details.buy_in);
    assert_ne!(tournament1_details.max_players, tournament2_details.max_players);
}

#[test]
#[serial]
fn test_create_clan_tournament_insufficient_permissions() {
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // Create clan with owner and regular member
    let (clan, creator_id, _) = test_env.create_test_clan("Tournament Permission Clan", "tournament_perm_creator");

    // Add a regular member
    let (member_id, _) = test_env.create_user_and_join_clan(
        clan.id.0,
        "regular_tournament_member",
        0,
    );

    // Create tournament config
    let tournament_config = NewTournament {
        name: "Unauthorized Tournament".to_string(),
        description: "Should not be created".to_string(),
        hero_picture: "".to_string(),
        currency: CurrencyType::Real(Currency::ICP),
        buy_in: convert_to_e8s(10.0),
        starting_chips: 1000,
        speed_type: NewTournamentSpeedType::Regular(20),
        min_players: 4,
        max_players: 8,
        late_registration_duration_ns: 10,
        guaranteed_prize_pool: None,
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
        currency_type: CurrencyType::Real(Currency::ICP),
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

    // Regular member tries to create tournament (should fail)
    let result = test_env.create_clan_tournament(
        clan.id.0,
        &tournament_config,
        &table_config,
        member_id,
    );
    assert!(matches!(result, Err(ClanError::InsufficientPermissions)));

    // Owner creates tournament (should succeed)
    let result = test_env.create_clan_tournament(
        clan.id.0,
        &tournament_config,
        &table_config,
        creator_id,
    );
    assert!(result.is_ok());
}

#[test]
#[serial]
fn test_remove_clan_tournament() {
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // Create clan and tournament
    let (clan, creator_id, _) = test_env.create_test_clan("Remove Tournament Clan", "remove_tournament_creator");

    let tournament_config = NewTournament {
        name: "Tournament To Remove".to_string(),
        description: "Will be removed".to_string(),
        hero_picture: "".to_string(),
        currency: CurrencyType::Real(Currency::ICP),
        buy_in: convert_to_e8s(10.0),
        starting_chips: 1000,
        speed_type: NewTournamentSpeedType::Regular(20),
        min_players: 4,
        max_players: 8,
        late_registration_duration_ns: 10,
        guaranteed_prize_pool: None,
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
        currency_type: CurrencyType::Real(Currency::ICP),
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

    let tournament_id = test_env
        .create_clan_tournament(clan.id.0, &tournament_config, &table_config, creator_id)
        .expect("Failed to create tournament");

    // Verify tournament exists
    let clan_tournaments = test_env
        .get_clan_tournaments(clan.id.0)
        .expect("Failed to get clan tournaments");
    assert_eq!(clan_tournaments.len(), 1);

    // Remove tournament
    let result = test_env.remove_clan_tournament(clan.id.0, tournament_id.id, creator_id);
    assert!(result.is_ok());

    // Verify tournament removed
    let clan_tournaments = test_env
        .get_clan_tournaments(clan.id.0)
        .expect("Failed to get clan tournaments");
    assert_eq!(clan_tournaments.len(), 0);
}

#[test]
#[serial]
fn test_remove_nonexistent_tournament() {
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // Create clan
    let (clan, creator_id, _) = test_env.create_test_clan("Nonexistent Tournament Clan", "nonexist_tournament_creator");

    // Try to remove non-existent tournament
    let fake_tournament_id = TournamentId(Principal::self_authenticating("fake_tournament"));
    let result = test_env.remove_clan_tournament(clan.id.0, fake_tournament_id, creator_id);
    assert!(matches!(result, Err(ClanError::TournamentNotFound)));
}

#[test]
#[serial]
fn test_remove_tournament_insufficient_permissions() {
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // Create clan with tournament
    let (clan, creator_id, _) = test_env.create_test_clan("Remove Tournament Perm Clan", "remove_tournament_perm_creator");

    let tournament_config = NewTournament {
        name: "Protected Tournament".to_string(),
        description: "Cannot be removed by regular members".to_string(),
        hero_picture: "".to_string(),
        currency: CurrencyType::Real(Currency::ICP),
        buy_in: convert_to_e8s(10.0),
        starting_chips: 1000,
        speed_type: NewTournamentSpeedType::Regular(20),
        min_players: 4,
        max_players: 8,
        late_registration_duration_ns: 10,
        guaranteed_prize_pool: None,
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
        currency_type: CurrencyType::Real(Currency::ICP),
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

    let tournament_id = test_env
        .create_clan_tournament(clan.id.0, &tournament_config, &table_config, creator_id)
        .expect("Failed to create tournament");

    // Add regular member
    let (member_id, _) = test_env.create_user_and_join_clan(
        clan.id.0,
        "remove_tournament_member",
        0,
    );

    // Regular member tries to remove tournament (should fail)
    let result = test_env.remove_clan_tournament(clan.id.0, tournament_id.id, member_id);
    assert!(matches!(result, Err(ClanError::InsufficientPermissions)));

    // Verify tournament still exists
    let clan_tournaments = test_env
        .get_clan_tournaments(clan.id.0)
        .expect("Failed to get clan tournaments");
    assert_eq!(clan_tournaments.len(), 1);
}

#[test]
#[serial]
fn test_admin_can_manage_tournaments() {
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // Create clan
    let (clan, creator_id, _) = test_env.create_test_clan("Admin Tournament Clan", "admin_tournament_creator");

    // Add member and promote to admin
    let (admin_id, _) = test_env.create_user_and_join_clan(
        clan.id.0,
        "tournament_admin",
        0,
    );

    test_env
        .update_member_role(clan.id.0, admin_id, ClanRole::Admin, creator_id)
        .expect("Failed to promote to admin");

    // Admin creates tournament
    let tournament_config = NewTournament {
        name: "Admin Tournament".to_string(),
        description: "Created by admin".to_string(),
        hero_picture: "".to_string(),
        currency: CurrencyType::Real(Currency::ICP),
        buy_in: convert_to_e8s(15.0),
        starting_chips: 1500,
        speed_type: NewTournamentSpeedType::Turbo(30),
        min_players: 4,
        max_players: 8,
        late_registration_duration_ns: 15,
        guaranteed_prize_pool: None,
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
        currency_type: CurrencyType::Real(Currency::ICP),
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

    let tournament_id = test_env
        .create_clan_tournament(clan.id.0, &tournament_config, &table_config, admin_id)
        .expect("Admin should be able to create tournaments");

    // Admin removes tournament
    let result = test_env.remove_clan_tournament(clan.id.0, tournament_id.id, admin_id);
    assert!(result.is_ok(), "Admin should be able to remove tournaments");
}

#[test]
#[serial]
fn test_clan_members_can_join_clan_tournaments() {
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // Create clan
    let (clan, creator_id, _) = test_env.create_test_clan("Join Tournament Clan", "join_tournament_creator");

    // Create clan tournament
    let tournament_config = NewTournament {
        name: "Clan Tournament".to_string(),
        description: "For clan members only".to_string(),
        hero_picture: "".to_string(),
        currency: CurrencyType::Real(Currency::ICP),
        buy_in: convert_to_e8s(10.0),
        starting_chips: 1000,
        speed_type: NewTournamentSpeedType::Regular(20),
        min_players: 2,
        max_players: 8,
        late_registration_duration_ns: 10,
        guaranteed_prize_pool: None,
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
        currency_type: CurrencyType::Real(Currency::ICP),
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

    let tournament_id = test_env
        .create_clan_tournament(clan.id.0, &tournament_config, &table_config, creator_id)
        .expect("Failed to create clan tournament");

    // Add member to clan
    let member_id = WalletPrincipalId(Principal::self_authenticating("tournament_joiner"));
    let member_user = test_env
        .create_user("Tournament Joiner".to_string(), member_id)
        .expect("Failed to create user");

    test_env
        .join_clan(clan.id.0, member_user.users_canister_id, member_id, 0)
        .expect("Failed to join clan");

    // Give member tokens and approve tournament canister
    test_env.transfer_approve_tokens_for_testing(tournament_id.id.0, member_id, 1000.0, true);

    // Member joins clan tournament
    let result = test_env.join_tournament(tournament_id.id, member_user.users_canister_id, member_id);
    assert!(result.is_ok(), "Clan member should be able to join clan tournament");

    // Verify tournament has one player
    let tournament = test_env.get_tournament(tournament_id.id).unwrap();
    assert_eq!(tournament.current_players.len(), 1);
}

#[test]
#[serial]
fn test_get_empty_clan_tournaments() {
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // Create clan without any tournaments
    let (clan, _, _) = test_env.create_test_clan("Empty Tournament Clan", "empty_tournament_creator");

    // Get tournaments (should be empty)
    let clan_tournaments = test_env
        .get_clan_tournaments(clan.id.0)
        .expect("Failed to get clan tournaments");
    assert_eq!(clan_tournaments.len(), 0);
}
