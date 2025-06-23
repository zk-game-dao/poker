use candid::Principal;
use currency::Currency;
use serial_test::serial;
use table::poker::game::{
    table_functions::{
        table::{TableConfig, TableType},
        types::CurrencyType,
    },
    utils::convert_to_e8s,
};
use tournaments::tournaments::{
    tournament_type::{BuyInOptions, TournamentSizeType, TournamentType}, types::{NewTournament, NewTournamentSpeedType}
};
use user::user::WalletPrincipalId;

use crate::{TestEnv, INIT_CYCLES_BALANCE};

#[test]
#[serial]
fn test_cycles_create_table() {
    let test_env = TestEnv::new(Some(1_000_000_000_000));

    // Create a table configuration
    let table_config = TableConfig {
        name: "Test Table".to_string(),
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
    };

    let cycles_before = test_env
        .pocket_ic
        .cycle_balance(test_env.canister_ids.table_index);

    assert!(cycles_before <= INIT_CYCLES_BALANCE);

    let _public_table = test_env
        .create_table(&table_config)
        .expect("Failed to create table");

    let cycles_after = test_env
        .pocket_ic
        .cycle_balance(test_env.canister_ids.table_index);

    println!("Cycles before: {}", cycles_before);
    println!("Cycles after: {}", cycles_after);

    assert!(cycles_after > INIT_CYCLES_BALANCE);
}

#[test]
#[serial]
fn test_cycles_create_user() {
    let test_env = TestEnv::new(Some(1_000_000_000_000));

    let cycles_before = test_env
        .pocket_ic
        .cycle_balance(test_env.canister_ids.user_index);

    assert!(cycles_before <= INIT_CYCLES_BALANCE);

    let _user_1 = test_env
        .create_user(
            "User 1".to_string(),
            WalletPrincipalId(Principal::self_authenticating("user1cyclestest")),
        )
        .expect("Failed to create user");

    let cycles_after = test_env
        .pocket_ic
        .cycle_balance(test_env.canister_ids.user_index);

    println!("Cycles before: {}", cycles_before);
    println!("Cycles after: {}", cycles_after);

    assert!(cycles_after > INIT_CYCLES_BALANCE);
}

#[test]
#[serial]
fn test_cycles_create_tournament() {
    let test_env = TestEnv::new(Some(5_000_000_000_000));

    // Create a table configuration
    let tournament_config = NewTournament {
        name: "Test Tournament".to_string(),
        description: "Test Tournament Description".to_string(),
        hero_picture: "".to_string(),
        currency: CurrencyType::Real(Currency::ICP),
        buy_in: convert_to_e8s(10.0),
        guaranteed_prize_pool: None,
        starting_chips: 1000,
        speed_type: NewTournamentSpeedType::Regular(20),
        min_players: 5,
        max_players: 8,
        late_registration_duration_ns: 10,
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

    let cycles_before = test_env
        .pocket_ic
        .cycle_balance(test_env.canister_ids.tournament_index);

    println!("Cycles before: {:?}", cycles_before);

    assert!(cycles_before <= INIT_CYCLES_BALANCE);

    let tournament_id = test_env
        .create_tournament(&tournament_config, &table_config)
        .expect("Failed to create tournament");

    // Verify the tournament canister was created
    assert!(test_env.pocket_ic.canister_exists(tournament_id.0));

    // Check if cycles were transferred to the new tournament canister
    let tournament_cycles = test_env.pocket_ic.cycle_balance(tournament_id.0);
    println!("Tournament canister cycles: {:?}", tournament_cycles);
    assert!(
        tournament_cycles > 0,
        "Tournament canister should have cycles"
    );

    // Check if cycles were deducted from tournament index
    let cycles_after = test_env
        .pocket_ic
        .cycle_balance(test_env.canister_ids.tournament_index);
    println!("Cycles before: {:?}", cycles_before);
    println!("Cycles after: {:?}", cycles_after);

    // Verify the tournament data is stored
    assert!(cycles_before < cycles_after);
}

// #[test]
// #[serial]
// fn test_tournament_request_cycles() {
//     let test_env = TestEnv::new(Some(10_000_000_000_000));

//     // Create a table configuration
//     let table_config = TableConfig {
//         name: "Test Tournament Table".to_string(),
//         game_type: table::poker::game::types::GameType::NoLimit(1),
//         seats: 6,
//         timer_duration: 30,
//         card_color: 0,
//         color: 0,
//         environment_color: 0,
//         auto_start_timer: 10,
//         max_inactive_turns: 3,
//         currency_type: CurrencyType::Fake,
//         enable_rake: Some(false),
//         max_seated_out_turns: None,
//         is_private: Some(false),
//         ante_type: None,
//         table_type: None,
//         is_shared_rake: None,
//         require_proof_of_humanity: None,
//         is_paused: None,
//     };

//     // Create a tournament configuration
//     let tournament_config = NewTournament {
//         name: "Cycle Test Tournament".to_string(),
//         max_players: 6,
//         min_players: 2,
//         buy_in: 0,
//         prize_structure: vec![70, 30],
//         speed_type: NewTournamentSpeedType::Turbo,
//         tournament_type: TournamentType::SitAndGo(None),
//         start_time: ic_cdk::api::time() + 1_000_000_000,
//         starting_chips: 1000,
//         description: "Tournament for cycle request test".to_string(),
//         is_private: false,
//         registration_cut_off: None,
//     };

//     let tournament_id = test_env
//         .create_tournament(&tournament_config, &table_config)
//         .expect("Failed to create tournament");

//     // Get initial cycle balance
//     let initial_cycles = test_env.pocket_ic.cycle_balance(tournament_id);
//     println!("Initial tournament cycles: {}", initial_cycles);

//     // Use some cycles to drop below threshold
//     // This is a bit of a hack for testing - in a real environment the canister would use cycles over time
//     test_env.pocket_ic.add_cycles(tournament_id, -1_000_000_000_000);
//     let reduced_cycles = test_env.pocket_ic.cycle_balance(tournament_id);
//     println!("Reduced tournament cycles: {}", reduced_cycles);
//     assert!(reduced_cycles < initial_cycles);

//     // Request cycles from the tournament index
//     let result = test_env.request_tournament_cycles(tournament_id);
//     assert!(result.is_ok(), "Failed to request cycles: {:?}", result);

//     // Check if cycles were topped up
//     let final_cycles = test_env.pocket_ic.cycle_balance(tournament_id);
//     println!("Final tournament cycles: {}", final_cycles);
//     assert!(final_cycles > reduced_cycles, "Tournament should have received additional cycles");
// }

// #[test]
// #[serial]
// fn test_cycles_user_canister_to_index() {
//     let test_env = TestEnv::get();

//     let user = create_canister(&test_env.pocket_ic, (), wasms::USER.clone(), Some(85_000_000_000));
//     let user = test_env.pocket_ic.update_call(user, test_env.canister_ids.user_index, "create_user", encode_args(("TEST", (), Principal::self_authenticating("user1cycletestcanistertoindex"), ()),).unwrap());

//     let user = match user.expect("Failed to create user") {
//         WasmResult::Reply(arg) => {
//             let user_canister: Result<User, UserError> = decode_one(&arg).unwrap();
//             user_canister
//         }
//         _ => panic!("Failed to create user"),
//     }.unwrap();

//     let cycles_before = test_env.pocket_ic.cycle_balance(user.canister_id);
//     println!("Cycles before: {}", cycles_before);

//     let user = test_env.add_active_table(user.canister_id, Principal::self_authenticating("table1cycletestcanistertoindex")).unwrap();

//     let cycles_after = test_env.pocket_ic.cycle_balance(user.canister_id);
//     println!("Cycles after: {}", cycles_after);

//     assert!(cycles_after > cycles_before);
// }

// #[test]
// #[serial]
// fn test_cycles_table_canister_to_index() {
//     let test_env = TestEnv::get();

//     let table_config = TableConfig {
//         name: "Test Table".to_string(),
//         game_type: table::poker::game::types::GameType::NoLimit(1),
//         seats: 6,
//         timer_duration: 30,
//         card_color: 0,
//         color: 0,
//         environment_color: 0,
//         auto_start_timer: 2,
//         max_inactive_turns: 3,
//     };
//     let bytes: Vec<u8> = vec![0,1,2,3];

//     let table = create_canister(&test_env.pocket_ic, (), wasms::TABLE.clone(), Some(90_000_000_000));
//     let table = test_env.pocket_ic.update_call(table, test_env.canister_ids.table_index, "create_table", encode_args((table_config, bytes),).unwrap());

//     let public_table = match table.expect("Failed to create table") {
//         WasmResult::Reply(arg) => {
//             let table_canister: Result<PublicTable, TableError> = decode_one(&arg).unwrap();
//             table_canister
//         }
//         _ => panic!("Failed to create user"),
//     }.unwrap();

//     let cycles_before = test_env.pocket_ic.cycle_balance(public_table.id);
//     println!("Cycles before: {}", cycles_before);

//     let user_1 = test_env
//         .create_user(
//             "User 1".to_string(),
//             Principal::self_authenticating("user1depositmidgame"),
//         )
//         .expect("Failed to create user");
//     let block_index_user_1 = transfer_tokens(
//         &test_env.pocket_ic,
//         100.0,
//         public_table.id,
//         test_env.canister_ids.ledger,
//         test_env.canister_ids.user_index,
//         false,
//     );

//     let public_table = test_env.join_test_table(public_table.id, user_1, convert_to_e8s(100.0), 0, block_index_user_1);
//     assert_eq!(public_table.users.len(), 1);

//     let cycles_after = test_env.pocket_ic.cycle_balance(public_table.id);
//     println!("Cycles after: {}", cycles_after);

//     assert!(cycles_after > cycles_before);
// }
