use candid::{decode_one, encode_args, Principal};
use errors::table_index_error::TableIndexError;
use serial_test::serial;
use std::collections::{HashMap, HashSet};
use table::poker::game::table_functions::table::TableConfig;

use crate::{wasms, TestEnv};

// Helper for upgrading the table index canister
fn upgrade_table_index(env: &TestEnv) {
    env.pocket_ic
        .upgrade_canister(
            env.canister_ids.table_index,
            wasms::TABLE_INDEX.clone(),
            vec![], // empty args for upgrades
            None,   // Don't specify controller
        )
        .expect("Failed to upgrade table index canister");
}

// Create a standard test table configuration
fn create_test_table_config(name: &str) -> TableConfig {
    TableConfig {
        name: name.to_string(),
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

// Helper to get all public tables
fn get_all_public_tables(env: &TestEnv) -> Vec<(Principal, TableConfig)> {
    let result = env.pocket_ic.query_call(
        env.canister_ids.table_index,
        Principal::anonymous(),
        "get_all_public_tables",
        encode_args(()).unwrap(),
    );

    match result {
        Ok(arg) => {
            let tables: Result<Vec<(Principal, TableConfig)>, TableIndexError> =
                decode_one(&arg).unwrap();
            tables.expect("Failed to decode tables")
        }
        _ => panic!("Failed to get all public tables"),
    }
}

// Helper to get all table principals
fn get_all_table_principals(env: &TestEnv) -> Vec<Principal> {
    let result = env.pocket_ic.query_call(
        env.canister_ids.table_index,
        Principal::anonymous(),
        "get_all_table_principals",
        encode_args(()).unwrap(),
    );

    match result {
        Ok(arg) => {
            let tables: Result<Vec<Principal>, TableIndexError> = decode_one(&arg).unwrap();
            tables.expect("Failed to decode table principals")
        }
        _ => panic!("Failed to get all table principals"),
    }
}

// Helper to verify if a table exists by calling its ping method
fn verify_table_exists(env: &TestEnv, table_principal: Principal) -> bool {
    let result = env.pocket_ic.query_call(
        table_principal,
        Principal::anonymous(),
        "ping",
        encode_args(()).unwrap(),
    );

    matches!(result, Ok(_))
}

#[test]
#[serial]
fn test_table_index_upgrade_persistence() {
    // 1. Create a test environment
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // 2. Create several tables
    let num_tables = 5;
    let mut table_ids = Vec::new();

    for i in 0..num_tables {
        let config = create_test_table_config(&format!("Test Table {}", i));

        let table = test_env
            .create_table(&config)
            .expect("Failed to create table");

        table_ids.push(table.id);

        // Verify table was created successfully
        assert!(
            verify_table_exists(&test_env, table.id),
            "Table {} should exist after creation",
            table.id
        );
    }

    // 3. Get the current tables from the index
    let tables_before = get_all_public_tables(&test_env);
    let principals_before = get_all_table_principals(&test_env);

    println!("Before upgrade - Table count: {}", tables_before.len());
    println!("Principals before upgrade: {:?}", principals_before);

    // Create a map of principal -> config for easier comparison
    let mut table_configs_before = HashMap::new();
    for (principal, config) in &tables_before {
        table_configs_before.insert(*principal, config.clone());
    }

    // 4. Upgrade the table index canister
    upgrade_table_index(&test_env);

    // 5. Get tables after upgrade
    let tables_after = get_all_public_tables(&test_env);
    let principals_after = get_all_table_principals(&test_env);

    println!("After upgrade - Table count: {}", tables_after.len());
    println!("Principals after upgrade: {:?}", principals_after);

    // 6. Verify tables were preserved
    assert_eq!(
        tables_before.len(),
        tables_after.len(),
        "Table count should be the same before and after upgrade"
    );

    let principals_before_set: HashSet<Principal> = principals_before.into_iter().collect();
    let principals_after_set: HashSet<Principal> = principals_after.clone().into_iter().collect();

    assert_eq!(
        principals_before_set, principals_after_set,
        "Table principals should be the same before and after upgrade"
    );

    // 7. Verify each table still exists and is callable
    let mut valid_tables = Vec::new();
    let mut invalid_tables = Vec::new();

    for principal in &principals_after {
        if verify_table_exists(&test_env, *principal) {
            valid_tables.push(*principal);
        } else {
            invalid_tables.push(*principal);
            println!("Found invalid table: {}", principal);
        }
    }

    // Print status of invalid tables
    if !invalid_tables.is_empty() {
        println!(
            "Found {} invalid tables after upgrade",
            invalid_tables.len()
        );
        for principal in &invalid_tables {
            println!("Invalid table: {}", principal);
        }
    }

    // 8. Test purge_dud_tables functionality if invalid tables were found
    if !invalid_tables.is_empty() {
        panic!("Invalid tables found after upgrade, please investigate");
    }
}
