use std::collections::HashSet;

use candid::{decode_one, encode_args, Principal};
use serial_test::serial;
use table::poker::game::table_functions::table::TableConfig;

use crate::TestEnv;

#[test]
#[serial]
fn test_create_table() {
    let test_env = TestEnv::get();

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

    let public_table = test_env
        .create_table(&table_config)
        .expect("Failed to create table");

    assert!(test_env.pocket_ic.canister_exists(public_table.id));
    assert_eq!(public_table.config.name, table_config.name);

    // Call a function on the newly created table canister to verify its state
    let table_state = test_env.pocket_ic.query_call(
        public_table.id,
        Principal::anonymous(),
        "ping",
        encode_args(()).unwrap(),
    );

    match table_state {
        Ok(arg) => {
            let response: String = decode_one(&arg).unwrap();
            assert_eq!(response, "Ok");
        }
        _ => panic!("Failed to ping table"),
    }
}

#[test]
#[serial]
fn test_create_user() {
    let test_env = TestEnv::get();

    let user_id = Principal::self_authenticating("usertest");
    let user_canister = test_env
        .create_user("Test User".to_string(), user_id)
        .expect("Failed to create user");

    assert!(test_env
        .pocket_ic
        .canister_exists(user_canister.users_canister_id));

    let user = test_env
        .get_user(user_canister.users_canister_id, user_id)
        .expect("Failed to get user");

    assert_eq!(user.user_name, "Test User");
}

#[test]
fn test_creating_1100_users() {
    let test_env = TestEnv::get();
    let mut users = Vec::new();
    for i in 0..1100 {
        let user = test_env
            .create_user(
                format!("User {}", i),
                Principal::self_authenticating(format!("user{}", i)),
            )
            .unwrap();
        users.push((user.users_canister_id, user.principal_id));
        println!("Created user: {}", i);
    }
    assert_eq!(users.len(), 1100);
    let users_canister: HashSet<Principal> =
        HashSet::from_iter(users.iter().map(|(canister_id, _)| *canister_id));
    assert_eq!(users_canister.len(), 2);
}
