use serial_test::serial;
use table::poker::game::utils::convert_to_e8s;

use crate::TestEnv;

#[test]
#[serial]
fn join_table() {
    let test_env = TestEnv::get();

    // Create a table configuration
    let (public_table, _) = test_env.get_test_fake_table();
    let (user_1_canister, user_1) = test_env.create_test_user("user1jointable");
    let (user_2_canister, user_2) = test_env.create_test_user("user2jointable");

    let public_table = test_env
        .join_test_table(
            public_table,
            user_1_canister,
            user_1,
            convert_to_e8s(100.0),
            0,
            false,
        )
        .unwrap();
    assert_eq!(public_table.users.len(), 1);
    let public_table = test_env
        .join_test_table(
            public_table.id,
            user_2_canister,
            user_2,
            convert_to_e8s(100.0),
            1,
            false,
        )
        .unwrap();
    assert_eq!(public_table.users.len(), 2);
}

#[test]
#[serial]
fn join_table_incorrect_currency_type() {
    let test_env = TestEnv::get();

    // Create a table configuration
    let (public_table, _) = test_env.get_test_icp_table();
    let _ = public_table;
    let (user_1_canister, user_1) = test_env.create_test_user("user1jointableincorrectcurrency");

    let public_table = test_env.join_test_table(
        public_table,
        user_1_canister,
        user_1,
        convert_to_e8s(100.0),
        0,
        false,
    );
    assert!(public_table.is_err());
}

#[test]
#[serial]
fn withdraw_from_fake_table() {
    let test_env = TestEnv::get();

    // Create a table configuration
    let (public_table, _) = test_env.get_test_fake_table();
    let (user_1_canister, user_1) = test_env.create_test_user("user1withdrawfaketable");
    let (user_2_canister, user_2) = test_env.create_test_user("user2withdrawfaketable");

    let public_table = test_env
        .join_test_table(
            public_table,
            user_1_canister,
            user_1,
            convert_to_e8s(100.0),
            0,
            false,
        )
        .unwrap();
    assert_eq!(public_table.users.len(), 1);
    let public_table = test_env
        .join_test_table(
            public_table.id,
            user_2_canister,
            user_2,
            convert_to_e8s(100.0),
            1,
            false,
        )
        .unwrap();
    assert_eq!(public_table.users.len(), 2);

    let res = test_env.player_withdraw(public_table.id, user_1, user_1, convert_to_e8s(50.0));
    assert!(res.is_err());
}
