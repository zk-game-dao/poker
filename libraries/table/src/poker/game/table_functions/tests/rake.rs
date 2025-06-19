use currency::{types::currency::CKTokenSymbol, Currency};

use crate::poker::game::{
    table_functions::{rake::{get_no_limit_config, interpolate_u64, Rake}, table::SmallBlind},
    types::GameType,
    utils::convert_to_e8s,
};

// Test interpolate_u64 function
#[test]
fn test_interpolate_u64() {
    // Test when x == x_min
    let x = 10;
    let x_min = 10;
    let x_max = 20;
    let y_min = 100;
    let y_max = 200;
    let result = interpolate_u64(x, x_min, x_max, y_min, y_max);
    assert_eq!(result, y_min);

    // Test when x == x_max
    let x = 20;
    let result = interpolate_u64(x, x_min, x_max, y_min, y_max);
    assert_eq!(result, y_max);

    // Test when x is between x_min and x_max
    let x = 15;
    let result = interpolate_u64(x, x_min, x_max, y_min, y_max);
    assert_eq!(result, 150);

    // Test when x_max == x_min (should return y_min)
    let x = 10;
    let x_min = 10;
    let x_max = 10;
    let y_min = 100;
    let y_max = 200;
    let result = interpolate_u64(x, x_min, x_max, y_min, y_max);
    assert_eq!(result, y_min);
}

// Test get_no_limit_config function
#[test]
fn test_get_no_limit_config() {
    // Small blind within the first mini stakes range (not micro)
    let small_blind = convert_to_e8s(0.05); // $0.05
    let rake = get_no_limit_config(small_blind).expect("Rake should be found");
    assert_eq!(rake.percentage_millipercent, 4500); // 4.5%

    // Expected caps calculated using interpolation
    let expected_cap_2_3_players = interpolate_u64(
        small_blind,
        convert_to_e8s(0.01),       // Changed range start
        convert_to_e8s(0.24999999), // Changed range end
        convert_to_e8s(0.05),
        convert_to_e8s(0.20),
    );
    let expected_cap_4_plus_players = interpolate_u64(
        small_blind,
        convert_to_e8s(0.01),       // Changed range start
        convert_to_e8s(0.24999999), // Changed range end
        convert_to_e8s(0.10),
        convert_to_e8s(0.50),
    );

    assert_eq!(rake.cap_2_3_players, expected_cap_2_3_players);
    assert_eq!(rake.cap_4_plus_players, expected_cap_4_plus_players);
}

// Test Rake::new function
#[test]
fn test_rake_new() {
    // No Limit game with valid small blind
    let small_blind = convert_to_e8s(0.05); // $0.05
    let game_type = GameType::NoLimit(small_blind);
    let rake = Rake::new(SmallBlind(small_blind), &game_type, &currency::Currency::ICP)
        .expect("Rake should be created");
    assert_eq!(rake.percentage_millipercent, 4500);

    // Fixed Limit game with valid small blind
    let small_blind = convert_to_e8s(0.05);
    let big_blind = convert_to_e8s(0.10);
    let game_type = GameType::FixedLimit(small_blind, big_blind);
    let rake = Rake::new(SmallBlind(small_blind), &game_type, &currency::Currency::ICP)
        .expect("Rake should be created");
    assert_eq!(rake.percentage_millipercent, 4500);
}

// Test Rake::calculate_rake function
#[test]
fn test_calculate_rake() {
    // Setup Rake
    let small_blind = convert_to_e8s(0.05); // $0.05
    let game_type = GameType::NoLimit(small_blind);
    let rake = Rake::new(SmallBlind(small_blind), &game_type, &currency::Currency::ICP)
        .expect("Rake should be created");

    // Pot size $1.00
    let pot = convert_to_e8s(1.00);

    // Number of players: 2 (2-3 players cap applies)
    let num_players = 2;

    // Calculate expected raw rake
    let raw_rake = (pot * rake.percentage_millipercent) / 100_000; // 4,500,000 units ($0.045)

    // Calculate expected cap
    let cap = rake.cap_2_3_players; // Interpolated cap from rake struct

    // Final rake should be min(raw_rake, cap)
    let expected_rake = raw_rake.min(cap);

    let calculated_rake = rake.calculate_rake(pot, num_players);
    assert_eq!(calculated_rake, expected_rake);

    // Number of players: 4 (4+ players cap applies)
    let num_players = 4;
    let cap = rake.cap_4_plus_players;
    let expected_rake = raw_rake.min(cap);

    let calculated_rake = rake.calculate_rake(pot, num_players);
    assert_eq!(calculated_rake, expected_rake);

    // Test with larger pot size
    let pot = convert_to_e8s(10.00); // $10.00
    let raw_rake = (pot * rake.percentage_millipercent) / 100_000; // 45,000,000 units ($0.45)
    let num_players = 2;
    let cap = rake.cap_2_3_players;
    let expected_rake = raw_rake.min(cap);

    let calculated_rake = rake.calculate_rake(pot, num_players);
    assert_eq!(calculated_rake, expected_rake);

    // Verify that cap is applied when raw_rake exceeds cap
    assert_eq!(calculated_rake, cap);

    // Test with small pot size
    let pot = convert_to_e8s(0.50); // $0.50
    let raw_rake = (pot * rake.percentage_millipercent) / 100_000; // 2,250,000 units ($0.0225)
    let expected_rake = raw_rake.min(cap);

    let calculated_rake = rake.calculate_rake(pot, num_players);
    assert_eq!(calculated_rake, expected_rake);
}

// Test for small blinds at the edges of ranges
#[test]
fn test_rake_edges() {
    // Small blind at the minimum of low stakes range
    let small_blind = convert_to_e8s(0.25); // $0.25
    let game_type = GameType::NoLimit(small_blind);
    let rake = Rake::new(SmallBlind(small_blind), &game_type, &currency::Currency::ICP)
        .expect("Rake should be created");
    assert_eq!(rake.percentage_millipercent, 4000); // 4.0%

    let expected_cap_2_3_players = interpolate_u64(
        small_blind,
        convert_to_e8s(0.25),
        convert_to_e8s(0.99), // Changed range end
        convert_to_e8s(0.30),
        convert_to_e8s(0.50),
    );
    let expected_cap_4_plus_players = interpolate_u64(
        small_blind,
        convert_to_e8s(0.25),
        convert_to_e8s(0.99), // Changed range end
        convert_to_e8s(0.75),
        convert_to_e8s(1.00),
    );

    assert_eq!(rake.cap_2_3_players, expected_cap_2_3_players);
    assert_eq!(rake.cap_4_plus_players, expected_cap_4_plus_players);
}

// Test with small blind not exactly matching any specific value
#[test]
fn test_rake_with_intermediate_small_blind() {
    let small_blind = convert_to_e8s(0.075); // $0.075
    let game_type = GameType::NoLimit(small_blind);
    let rake = Rake::new(SmallBlind(small_blind), &game_type, &currency::Currency::ICP)
        .expect("Rake should be created");
    assert_eq!(rake.percentage_millipercent, 4500); // 4.5%

    let expected_cap_2_3_players = interpolate_u64(
        small_blind,
        convert_to_e8s(0.01),
        convert_to_e8s(0.24999999), // Changed range end
        convert_to_e8s(0.05),
        convert_to_e8s(0.20),
    );
    let expected_cap_4_plus_players = interpolate_u64(
        small_blind,
        convert_to_e8s(0.01),
        convert_to_e8s(0.24999999), // Changed range end
        convert_to_e8s(0.10),
        convert_to_e8s(0.50),
    );

    assert_eq!(rake.cap_2_3_players, expected_cap_2_3_players);
    assert_eq!(rake.cap_4_plus_players, expected_cap_4_plus_players);
}

#[test]
fn test_rake_different_currencies() {
    // Test ETH (18 decimals)
    let small_blind_eth = 50_000_000_000_000_000; // 0.05 ETH
    let game_type = GameType::NoLimit(small_blind_eth);
    let rake_eth = Rake::new(
        SmallBlind(small_blind_eth),
        &game_type,
        &Currency::CKETHToken(CKTokenSymbol::ETH),
    )
    .expect("ETH rake should be created");
    assert_eq!(rake_eth.percentage_millipercent, 4500); // 4.5%

    // Test USDC (6 decimals)
    let small_blind_usdc = 50_000; // 0.05 USDC
    let game_type = GameType::NoLimit(small_blind_usdc);
    let rake_usdc = Rake::new(
        SmallBlind(small_blind_usdc),
        &game_type,
        &Currency::CKETHToken(CKTokenSymbol::USDC),
    )
    .expect("USDC rake should be created");
    assert_eq!(rake_usdc.percentage_millipercent, 4500); // 4.5%

    // Test BTC (8 decimals)
    let small_blind_btc = 5_000_000; // 0.05 BTC
    let game_type = GameType::NoLimit(small_blind_btc);
    let rake_btc =
        Rake::new(SmallBlind(small_blind_btc), &game_type, &Currency::BTC).expect("BTC rake should be created");
    assert_eq!(rake_btc.percentage_millipercent, 4500); // 4.5%
}

#[test]
fn test_rake_decimal_precision() {
    // Test precise ETH amounts (18 decimals)
    let small_blind_eth = 10_000_000_000_000; // 0.00001 ETH
    let game_type = GameType::NoLimit(small_blind_eth);
    let _rake_eth = Rake::new(
        SmallBlind(small_blind_eth),
        &game_type,
        &Currency::CKETHToken(CKTokenSymbol::ETH),
    )
    .expect("ETH rake should be created");

    // Test precise USDC amounts (6 decimals)
    let small_blind_usdc = 10; // 0.00001 USDC
    let game_type = GameType::NoLimit(small_blind_usdc);
    let _rake_usdc = Rake::new(
        SmallBlind(small_blind_usdc),
        &game_type,
        &Currency::CKETHToken(CKTokenSymbol::USDC),
    )
    .expect("USDC rake should be created");

    // Test precise BTC amounts (8 decimals)
    let small_blind_btc = 1_000; // 0.00001 BTC
    let game_type = GameType::NoLimit(small_blind_btc);
    let _rake_btc =
        Rake::new(SmallBlind(small_blind_btc), &game_type, &Currency::BTC).expect("BTC rake should be created");
}

#[test]
fn test_rake_edge_cases_different_currencies() {
    // Test ETH at range boundaries
    let small_blind_eth = 250_000_000_000_000_000; // 0.25 ETH
    let game_type = GameType::NoLimit(small_blind_eth);
    let rake_eth = Rake::new(
        SmallBlind(small_blind_eth),
        &game_type,
        &Currency::CKETHToken(CKTokenSymbol::ETH),
    )
    .expect("ETH rake should be created");
    assert_eq!(rake_eth.percentage_millipercent, 4000); // 4.0%

    // Test USDC at range boundaries
    let small_blind_usdc = 250_000; // 0.25 USDC
    let game_type = GameType::NoLimit(small_blind_usdc);
    let rake_usdc = Rake::new(
        SmallBlind(small_blind_usdc),
        &game_type,
        &Currency::CKETHToken(CKTokenSymbol::USDC),
    )
    .expect("USDC rake should be created");
    assert_eq!(rake_usdc.percentage_millipercent, 4000); // 4.0%

    // Test BTC at range boundaries
    let small_blind_btc = 25_000_000; // 0.25 BTC
    let game_type = GameType::NoLimit(small_blind_btc);
    let rake_btc =
        Rake::new(SmallBlind(small_blind_btc), &game_type, &Currency::BTC).expect("BTC rake should be created");
    assert_eq!(rake_btc.percentage_millipercent, 4000); // 4.0%
}
