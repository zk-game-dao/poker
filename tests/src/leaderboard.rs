use std::time::{Duration, SystemTime, UNIX_EPOCH};

use candid::Principal;
use serial_test::serial;

use crate::{utils::transfer::transfer_tokens, TestEnv};

#[test]
#[serial]
fn test_experience_points_reset_and_payout() {
    let test_env = TestEnv::new(None);

    // Create users with initial experience points
    let users = vec![
        ("user1xppointreset", 1000),
        ("user2xppointreset", 800),
        ("user3xppointreset", 600),
        ("user4xppointreset", 400),
        ("user5xppointreset", 200),
        ("user6xppointreset", 100),
    ];

    let mut user_canisters = Vec::new();
    for (name, exp) in users {
        let user_id = Principal::self_authenticating(name);
        let user_canister = test_env
            .create_user(name.to_string(), user_id)
            .expect("Failed to create user");

        // Set experience points
        test_env
            .add_experience_points(
                user_canister.users_canister_id,
                user_id,
                currency::Currency::ICP.to_string(),
                exp,
            )
            .unwrap();

        user_canisters.push(user_canister.users_canister_id);
    }

    let usc = user_canisters[0];
    assert!(user_canisters.iter().all(|&canister| canister == usc));

    // Add balance to user index canister for payouts
    transfer_tokens(
        &test_env.pocket_ic,
        100.0, // More than enough for payouts
        test_env.canister_ids.user_index,
        test_env.canister_ids.ledger,
        test_env.canister_ids.user_index,
        false,
    );

    // Get timestamp for Sunday 00:00:00 UTC
    let sunday_midnight = {
        let current_time = test_env.pocket_ic.get_time();
        let now = current_time.as_nanos_since_unix_epoch() / 1_000_000_000;
        let days_since_epoch = now / 86400;
        let current_day = (days_since_epoch + 4) % 7;
        let days_to_sunday = (8 - current_day) % 7;
        let sunday = now + (days_to_sunday * 86400);
        let sunday_midnight = sunday - (sunday % 86400);
        UNIX_EPOCH + Duration::from_secs(sunday_midnight)
    };

    test_env.pocket_ic.set_time(sunday_midnight.into());

    for _ in 0..12 {
        test_env.pocket_ic.tick();
    }

    // Verify experience points were reset
    let exp = test_env
        .get_user_experience_points(user_canisters[0])
        .unwrap();
    for (_, exp) in exp {
        assert_eq!(exp, 0, "Experience points should be reset to 0");
    }

    test_env.pocket_ic.set_time(SystemTime::now().into());
}

#[test]
#[serial]
fn test_experience_points_reset_and_payout_btc() {
    let test_env = TestEnv::new(None);

    // Create users with initial experience points
    let users = vec![
        ("user1xppointresetbtc", 1000),
        ("user2xppointresetbtc", 800),
        ("user3xppointresetbtc", 600),
        ("user4xppointresetbtc", 400),
        ("user5xppointresetbtc", 200),
        ("user6xppointresetbtc", 100),
    ];

    let mut user_canisters = Vec::new();
    for (name, exp) in users {
        let user_id = Principal::self_authenticating(name);
        let user_canister = test_env
            .create_user(name.to_string(), user_id)
            .expect("Failed to create user");

        // Set experience points
        test_env
            .add_experience_points(
                user_canister.users_canister_id,
                user_id,
                currency::Currency::BTC.to_string(),
                exp,
            )
            .unwrap();

        user_canisters.push(user_canister.users_canister_id);
    }

    let usc = user_canisters[0];
    assert!(user_canisters.iter().all(|&canister| canister == usc));

    // Add balance to user index canister for payouts
    transfer_tokens(
        &test_env.pocket_ic,
        100.0, // More than enough for payouts
        test_env.canister_ids.user_index,
        test_env.canister_ids.ledger,
        test_env.canister_ids.user_index,
        false,
    );

    // Get timestamp for Sunday 00:00:00 UTC
    let sunday_midnight = {
        let current_time = test_env.pocket_ic.get_time();
        let now = current_time.as_nanos_since_unix_epoch() / 1_000_000_000;
        let days_since_epoch = now / 86400;
        let current_day = (days_since_epoch + 4) % 7;
        let days_to_sunday = (8 - current_day) % 7;
        let sunday = now + (days_to_sunday * 86400);
        let sunday_midnight = sunday - (sunday % 86400);
        UNIX_EPOCH + Duration::from_secs(sunday_midnight)
    };

    test_env.pocket_ic.set_time(sunday_midnight.into());

    for _ in 0..12 {
        test_env.pocket_ic.tick();
    }

    // Verify experience points were reset
    let exp = test_env
        .get_pure_poker_user_experience_points(user_canisters[0])
        .unwrap();
    for (_, exp) in exp {
        assert_eq!(exp, 0, "Experience points should be reset to 0");
    }

    test_env.pocket_ic.set_time(SystemTime::now().into());
}
