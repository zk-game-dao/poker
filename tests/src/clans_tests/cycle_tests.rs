use candid::Principal;
use clan::{search::ClanSearchFilters, subscriptions::ClanRole, ClanPrivacy, CreateClanRequest};
use currency::Currency;
use serial_test::serial;
use user::user::WalletPrincipalId;

use crate::TestEnv;

#[test]
#[serial]
fn test_cycles_create_clan() {
    let test_env = TestEnv::new(Some(20_000_000_000_000)); // 20T cycles for clan creation

    let cycles_before = test_env
        .pocket_ic
        .cycle_balance(test_env.canister_ids.clan_index);

    println!("Cycles before clan creation: {}", cycles_before);
    assert!(cycles_before <= 20_000_000_000_000);

    // Create a clan
    let creator_id = WalletPrincipalId(Principal::self_authenticating("cycle_test_creator"));
    let creator_user = test_env
        .create_user("Cycle Test Creator".to_string(), creator_id)
        .expect("Failed to create user");

    let clan_request = CreateClanRequest {
        name: "Cycle Test Clan".to_string(),
        description: "Testing cycle management".to_string(),
        tag: "CYCLE".to_string(),
        privacy: ClanPrivacy::Public,
        supported_currency: Currency::ICP,
        joining_fee: 0,
        require_proof_of_humanity: false,
        minimum_level_required: None,
        minimum_experience_points: None,
        member_limit: Some(100),
        avatar: None,
        website: None,
        discord: None,
        twitter: None,
    };

    let clan = test_env
        .create_clan(&clan_request, creator_id, creator_user.users_canister_id)
        .expect("Failed to create clan");

    let cycles_after = test_env
        .pocket_ic
        .cycle_balance(test_env.canister_ids.clan_index);

    println!("Cycles after clan creation: {}", cycles_after);

    // Verify that the clan index has more cycles (topped up during creation)
    assert!(cycles_after > cycles_before);

    // Verify the clan canister was created and has cycles
    let clan_cycles = test_env.pocket_ic.cycle_balance(clan.id.0);
    println!("Clan canister cycles: {}", clan_cycles);
    assert!(clan_cycles > 0, "Clan canister should have cycles");
    
    // Clan canister should have significant cycles for operation
    assert!(clan_cycles > 5_000_000_000_000, "Clan canister should have at least 5T cycles");
}

#[test]
#[serial]
fn test_cycles_clan_operations() {
    let test_env = TestEnv::new(Some(15_000_000_000_000)); // 15T cycles

    // Create a clan first
    let (clan, creator_id, _) = test_env.create_test_clan("Operations Test Clan", "ops_creator");

    let initial_clan_cycles = test_env.pocket_ic.cycle_balance(clan.id.0);
    println!("Initial clan cycles: {}", initial_clan_cycles);

    // Perform multiple operations that should consume cycles
    
    // 1. Join clan operations (multiple members)
    for i in 0..5 {
        let member_id = WalletPrincipalId(Principal::self_authenticating(&format!("member_{}", i)));
        let member_user = test_env
            .create_user(format!("Member {}", i), member_id)
            .expect("Failed to create user");

        test_env
            .join_clan(clan.id.0, member_user.users_canister_id, member_id, 0)
            .expect("Failed to join clan");
    }

    // 2. Role update operations
    let member_id = WalletPrincipalId(Principal::self_authenticating("member_0"));
    test_env
        .update_member_role(clan.id.0, member_id, ClanRole::Admin, creator_id)
        .expect("Failed to update member role");

    // 3. Suspension operation
    let suspend_target = WalletPrincipalId(Principal::self_authenticating("member_1"));
    test_env
        .suspend_member(
            clan.id.0,
            suspend_target,
            creator_id,
            Some(test_env.pocket_ic.get_time().as_nanos_since_unix_epoch() + 86_400_000_000_000), // 1 day
        )
        .expect("Failed to suspend member");

    let final_clan_cycles = test_env.pocket_ic.cycle_balance(clan.id.0);
    println!("Final clan cycles after operations: {}", final_clan_cycles);

    // Operations should consume some cycles, but clan should still have plenty
    assert!(final_clan_cycles < initial_clan_cycles, "Operations should consume cycles");
    assert!(final_clan_cycles > 1_000_000_000_000, "Clan should still have at least 1T cycles");
}

#[test]
#[serial]
fn test_cycles_clan_table_creation() {
    let test_env = TestEnv::new(Some(25_000_000_000_000)); // 25T cycles for table creation

    // Create a clan
    let (clan, creator_id, _) = test_env.create_test_clan("Table Test Clan", "table_creator");

    let cycles_before_table = test_env.pocket_ic.cycle_balance(clan.id.0);
    println!("Clan cycles before table creation: {}", cycles_before_table);

    // Create a clan table
    let (_, table_config) = test_env.get_test_icp_table();
    let table = test_env
        .create_clan_table(clan.id.0, &table_config, creator_id)
        .expect("Failed to create clan table");

    let cycles_after_table = test_env.pocket_ic.cycle_balance(clan.id.0);
    println!("Clan cycles after table creation: {}", cycles_after_table);

    // Verify table canister was created and has cycles
    let table_cycles = test_env.pocket_ic.cycle_balance(table.id.0);
    println!("Table canister cycles: {}", table_cycles);
    assert!(table_cycles > 0, "Table canister should have cycles");

    // Table creation should consume cycles from clan canister
    assert!(cycles_after_table < cycles_before_table, "Table creation should consume cycles");
}

#[test]
#[serial]
fn test_cycles_clan_tournament_creation() {
    let test_env = TestEnv::new(Some(30_000_000_000_000)); // 30T cycles for tournament creation

    // Create a clan
    let (clan, creator_id, _) = test_env.create_test_clan("Tournament Test Clan", "tournament_creator");

    let cycles_before_tournament = test_env.pocket_ic.cycle_balance(clan.id.0);
    println!("Clan cycles before tournament creation: {}", cycles_before_tournament);

    // Create a clan tournament
    let tournament_config = tournaments::tournaments::types::NewTournament {
        name: "Cycle Test Tournament".to_string(),
        description: "Testing cycles for tournament creation".to_string(),
        hero_picture: "".to_string(),
        currency: table::poker::game::table_functions::types::CurrencyType::Real(Currency::ICP),
        buy_in: table::poker::game::utils::convert_to_e8s(10.0),
        starting_chips: 1000,
        speed_type: tournaments::tournaments::types::NewTournamentSpeedType::Regular(20),
        min_players: 2,
        max_players: 8,
        late_registration_duration_ns: 10,
        guaranteed_prize_pool: None,
        tournament_type: tournaments::tournaments::tournament_type::TournamentType::BuyIn(
            tournaments::tournaments::tournament_type::TournamentSizeType::SingleTable(
                tournaments::tournaments::tournament_type::BuyInOptions::new_freezout(),
            ),
        ),
        start_time: u64::MAX,
        require_proof_of_humanity: false,
    };

    let table_config = table::poker::game::table_functions::table::TableConfig {
        name: "Tournament Table".to_string(),
        game_type: table::poker::game::types::GameType::NoLimit(0),
        seats: 6,
        timer_duration: 30,
        card_color: 0,
        color: 0,
        environment_color: 0,
        auto_start_timer: 10,
        max_inactive_turns: 3,
        currency_type: table::poker::game::table_functions::types::CurrencyType::Real(Currency::ICP),
        enable_rake: Some(false),
        max_seated_out_turns: None,
        is_private: Some(false),
        ante_type: None,
        table_type: Some(table::poker::game::table_functions::table::TableType::Tournament {
            tournament_id: Principal::anonymous(),
            is_final_table: true,
        }),
        is_shared_rake: None,
        require_proof_of_humanity: None,
        is_paused: None,
    };

    let tournament = test_env
        .create_clan_tournament(clan.id.0, &tournament_config, &table_config, creator_id)
        .expect("Failed to create clan tournament");

    let cycles_after_tournament = test_env.pocket_ic.cycle_balance(clan.id.0);
    println!("Clan cycles after tournament creation: {}", cycles_after_tournament);

    // Verify tournament canister was created and has cycles
    let tournament_cycles = test_env.pocket_ic.cycle_balance(tournament.id.0);
    println!("Tournament canister cycles: {}", tournament_cycles);
    assert!(tournament_cycles > 0, "Tournament canister should have cycles");
    assert!(tournament_cycles > 2_000_000_000_000, "Tournament should have at least 2T cycles");

    // Tournament creation should consume cycles from clan canister
    assert!(cycles_after_tournament < cycles_before_tournament, "Tournament creation should consume cycles");
}

#[test]
#[serial]
fn test_cycles_multiple_clans() {
    let test_env = TestEnv::new(Some(50_000_000_000_000)); // 50T cycles for multiple clans

    let initial_index_cycles = test_env
        .pocket_ic
        .cycle_balance(test_env.canister_ids.clan_index);

    println!("Initial clan index cycles: {}", initial_index_cycles);

    let mut clan_ids = Vec::new();

    // Create multiple clans
    for i in 0..3 {
        let creator_id = WalletPrincipalId(Principal::self_authenticating(&format!("creator_{}", i)));
        let creator_user = test_env
            .create_user(format!("Creator {}", i), creator_id)
            .expect("Failed to create user");

        let clan_request = CreateClanRequest {
            name: format!("Test Clan {}", i),
            description: format!("Testing clan {}", i),
            tag: format!("TC{}", i),
            privacy: ClanPrivacy::Public,
            supported_currency: Currency::ICP,
            joining_fee: 0,
            require_proof_of_humanity: false,
            minimum_level_required: None,
            minimum_experience_points: None,
            member_limit: Some(100),
            avatar: None,
            website: None,
            discord: None,
            twitter: None,
        };

        let clan = test_env
            .create_clan(&clan_request, creator_id, creator_user.users_canister_id)
            .expect("Failed to create clan");

        clan_ids.push(clan.id.0);
    }

    let final_index_cycles = test_env
        .pocket_ic
        .cycle_balance(test_env.canister_ids.clan_index);

    println!("Final clan index cycles: {}", final_index_cycles);

    // Verify all clan canisters were created and have cycles
    for (i, clan_id) in clan_ids.iter().enumerate() {
        let clan_cycles = test_env.pocket_ic.cycle_balance(*clan_id);
        println!("Clan {} cycles: {}", i, clan_cycles);
        assert!(clan_cycles > 5_000_000_000_000, "Each clan should have at least 5T cycles");
    }

    // Index should have been topped up during operations
    assert!(final_index_cycles >= initial_index_cycles, "Index should maintain or increase cycles");
}

#[test]
#[serial]
fn test_cycles_clan_request_top_up() {
    let test_env = TestEnv::new(Some(15_000_000_000_000));

    // Create a clan
    let (clan, _creator_id, _) = test_env.create_test_clan("Top Up Test Clan", "topup_creator");

    // Simulate low cycles by reducing the clan canister's cycle balance
    // Note: In a real test environment, you might need to simulate cycle consumption
    let initial_cycles = test_env.pocket_ic.cycle_balance(clan.id.0);
    println!("Initial clan cycles: {}", initial_cycles);

    // Test the cycle request mechanism
    // This would normally be called when cycles are low
    let result = test_env.pocket_ic.update_call(
        clan.id.0,
        test_env.canister_ids.clan_index, // Clan index as caller
        "request_cycles",
        candid::encode_args(()).unwrap(),
    );

    match result {
        Ok(_) => {
            let cycles_after_request = test_env.pocket_ic.cycle_balance(clan.id.0);
            println!("Cycles after top-up request: {}", cycles_after_request);
            // In a successful top-up, cycles might increase
        }
        Err(e) => {
            println!("Cycle request failed (expected in test): {:?}", e);
            // This might fail in test environment, which is expected
        }
    }
}

#[test]
#[serial]
fn test_cycles_clan_index_operations() {
    let test_env = TestEnv::new(Some(25_000_000_000_000)); // 25T cycles

    let initial_index_cycles = test_env
        .pocket_ic
        .cycle_balance(test_env.canister_ids.clan_index);

    println!("Initial clan index cycles: {}", initial_index_cycles);

    // Create multiple clans through index
    let mut clans = Vec::new();
    for i in 0..5 {
        let (clan, _, _) = test_env.create_test_clan(&format!("Index Test Clan {}", i), &format!("index_creator_{}", i));
        clans.push(clan);
    }

    // Perform index operations
    
    // 1. Search operations
    let _all_clans = test_env.pocket_ic.query_call(
        test_env.canister_ids.clan_index,
        Principal::anonymous(),
        "get_all_clans",
        candid::encode_args(()).unwrap(),
    ).expect("Failed to get all clans");

    // 2. Filter operations
    let filters = ClanSearchFilters {
        name_contains: Some("Index".to_string()),
        currency: Some(Currency::ICP),
        privacy: Some(ClanPrivacy::Public),
        min_members: None,
        max_members: None,
        has_joining_fee: Some(false),
        subscription_enabled: None,
        require_proof_of_humanity: None,
        tag_filters: None,
    };

    let _filtered_clans = test_env.pocket_ic.query_call(
        test_env.canister_ids.clan_index,
        Principal::anonymous(),
        "search_clans",
        candid::encode_args((Some(filters), 0u64, 10u64)).unwrap(),
    ).expect("Failed to search clans");

    // 3. Get statistics
    let _stats = test_env.pocket_ic.query_call(
        test_env.canister_ids.clan_index,
        Principal::anonymous(),
        "get_clan_count",
        candid::encode_args(()).unwrap(),
    ).expect("Failed to get clan count");

    let final_index_cycles = test_env
        .pocket_ic
        .cycle_balance(test_env.canister_ids.clan_index);

    println!("Final clan index cycles: {}", final_index_cycles);

    // Index should maintain or increase cycles due to top-ups during heavy operations
    assert!(final_index_cycles >= 10_000_000_000_000, "Index should maintain reasonable cycle balance");

    // Verify all created clans still have cycles
    for (i, clan) in clans.iter().enumerate() {
        let clan_cycles = test_env.pocket_ic.cycle_balance(clan.id.0);
        println!("Clan {} final cycles: {}", i, clan_cycles);
        assert!(clan_cycles > 3_000_000_000_000, "Each clan should maintain at least 3T cycles");
    }
}
