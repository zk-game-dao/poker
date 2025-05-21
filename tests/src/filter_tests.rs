use serial_test::serial;
use table::poker::game::table_functions::table::TableConfig;

use crate::TestEnv;

#[test]
#[serial]
fn join_table() {
    let test_env = TestEnv::get();

    // Create a table configuration
    let table_config = TableConfig {
        name: "Test Table".to_string(),
        game_type: table::poker::game::types::GameType::NoLimit(1),
        seats: 6,
        timer_duration: 12321,
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

    let public_table_1 = test_env
        .create_table(&table_config)
        .expect("Failed to create table");

    // Create a table configuration
    let table_config = TableConfig {
        name: "Test Table".to_string(),
        game_type: table::poker::game::types::GameType::SpreadLimit(1, 2),
        seats: 6,
        timer_duration: 12321,
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

    let public_table_2 = test_env
        .create_table(&table_config)
        .expect("Failed to create table");

    // Create a table configuration
    let table_config = TableConfig {
        name: "Test Table".to_string(),
        game_type: table::poker::game::types::GameType::FixedLimit(1, 2),
        seats: 6,
        timer_duration: 12321,
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

    let public_table_3 = test_env
        .create_table(&table_config)
        .expect("Failed to create table");

    // Create a table configuration
    let table_config = TableConfig {
        name: "Test Table".to_string(),
        game_type: table::poker::game::types::GameType::NoLimit(1),
        seats: 6,
        timer_duration: 12321,
        card_color: 0,
        color: 0,
        environment_color: 0,
        auto_start_timer: 10,
        max_inactive_turns: 3,
        currency_type: table::poker::game::table_functions::types::CurrencyType::Fake,
        enable_rake: Some(false),
        max_seated_out_turns: None,
        is_private: Some(false),
        ante_type: None,
        table_type: None,
        is_shared_rake: None,
        require_proof_of_humanity: None,
        is_paused: None,
    };

    let public_table_4 = test_env
        .create_table(&table_config)
        .expect("Failed to create table");

    // Create a table configuration
    let table_config = TableConfig {
        name: "Test Table".to_string(),
        game_type: table::poker::game::types::GameType::SpreadLimit(1, 2),
        seats: 6,
        timer_duration: 12321,
        card_color: 0,
        color: 0,
        environment_color: 0,
        auto_start_timer: 10,
        max_inactive_turns: 3,
        currency_type: table::poker::game::table_functions::types::CurrencyType::Fake,
        enable_rake: Some(false),
        max_seated_out_turns: None,
        is_private: Some(false),
        ante_type: None,
        table_type: None,
        is_shared_rake: None,
        require_proof_of_humanity: None,
        is_paused: None,
    };

    let public_table_5 = test_env
        .create_table(&table_config)
        .expect("Failed to create table");

    // Create a table configuration
    let table_config = TableConfig {
        name: "Test Table".to_string(),
        game_type: table::poker::game::types::GameType::FixedLimit(1, 2),
        seats: 6,
        timer_duration: 12321,
        card_color: 0,
        color: 0,
        environment_color: 0,
        auto_start_timer: 10,
        max_inactive_turns: 3,
        currency_type: table::poker::game::table_functions::types::CurrencyType::Fake,
        enable_rake: Some(false),
        max_seated_out_turns: None,
        is_private: Some(false),
        ante_type: None,
        table_type: None,
        is_shared_rake: None,
        require_proof_of_humanity: None,
        is_paused: None,
    };

    let public_table_6 = test_env
        .create_table(&table_config)
        .expect("Failed to create table");

    let filter_options = table_index_types::filter::FilterOptions::new(
        Some(table::poker::game::types::GameType::NoLimit(1)),
        Some(12321),
        None,
        None,
        None,
        None,
        None,
        None,
    );

    let tables = test_env.get_tables(Some(filter_options), 0, 10).unwrap();
    assert_eq!(tables.len(), 2);
    assert_eq!(tables[0].1.game_type, public_table_1.config.game_type);
    assert_eq!(tables[1].1.game_type, public_table_4.config.game_type);
    assert_ne!(tables[0].0, tables[1].0);

    let filter_options = table_index_types::filter::FilterOptions::new(
        None,
        Some(12321),
        None,
        Some(
            table::poker::game::table_functions::types::CurrencyType::Real(currency::Currency::ICP),
        ),
        None,
        None,
        None,
        None,
    );
    let tables = test_env.get_tables(Some(filter_options), 0, 10).unwrap();
    assert_eq!(tables.len(), 3);
    assert_eq!(
        tables[0].1.currency_type,
        public_table_1.config.currency_type
    );
    assert_eq!(
        tables[1].1.currency_type,
        public_table_2.config.currency_type
    );
    assert_eq!(
        tables[2].1.currency_type,
        public_table_3.config.currency_type
    );
    assert_ne!(tables[0].0, tables[1].0);
    assert_ne!(tables[0].0, tables[2].0);
    assert_ne!(tables[1].0, tables[2].0);

    let filter_options = table_index_types::filter::FilterOptions::new(
        Some(table::poker::game::types::GameType::SpreadLimit(1, 2)),
        Some(12321),
        None,
        Some(
            table::poker::game::table_functions::types::CurrencyType::Real(currency::Currency::ICP),
        ),
        None,
        None,
        None,
        None,
    );
    let tables = test_env.get_tables(Some(filter_options), 0, 10).unwrap();
    assert_eq!(tables.len(), 1);
    assert_eq!(
        tables[0].1.currency_type,
        public_table_2.config.currency_type
    );
    assert_eq!(tables[0].1.game_type, public_table_2.config.game_type);

    let filter_options = table_index_types::filter::FilterOptions::new(
        None,
        Some(12321),
        None,
        Some(table::poker::game::table_functions::types::CurrencyType::Fake),
        None,
        None,
        None,
        None,
    );
    let tables = test_env.get_tables(Some(filter_options), 0, 10).unwrap();
    assert_eq!(tables.len(), 3);
    assert_eq!(
        tables[0].1.currency_type,
        public_table_4.config.currency_type
    );
    assert_eq!(
        tables[1].1.currency_type,
        public_table_5.config.currency_type
    );
    assert_eq!(
        tables[2].1.currency_type,
        public_table_6.config.currency_type
    );
    assert_ne!(tables[0].0, tables[1].0);
    assert_ne!(tables[0].0, tables[2].0);
    assert_ne!(tables[1].0, tables[2].0);
}
