use serial_test::serial;
use table::poker::game::utils::convert_to_e8s;
use user::user::TransactionType;

use crate::TestEnv;

#[test]
#[serial]
fn join_table_transaction_history() {
    let test_env = TestEnv::get();

    // Create a table configuration
    let (public_table, _) = test_env.get_test_icp_table();
    let (user_1, user_1_id, _) = test_env.create_test_user_with_icp_approval(
        "user1join_table_transaction_history".to_string(),
        1000.0,
        public_table,
    );

    let public_table = test_env
        .join_test_table(
            public_table,
            user_1,
            user_1_id,
            convert_to_e8s(100.0),
            0,
            false,
        )
        .unwrap();
    assert_eq!(public_table.users.len(), 1);
    let user = test_env.get_user(user_1, user_1_id).unwrap();
    assert!(user.transaction_history.is_some());
    assert_eq!(user.transaction_history.clone().unwrap().len(), 1);
    assert_eq!(
        user.transaction_history.clone().unwrap()[0].amount,
        convert_to_e8s(100.0)
    );
    assert_eq!(
        user.transaction_history.clone().unwrap()[0].transaction_type,
        TransactionType::TableDeposit {
            table_id: public_table.id
        }
    );
}

#[test]
#[serial]
fn leave_table_transaction_history() {
    let test_env = TestEnv::get();

    // Create a table configuration
    let (public_table, _) = test_env.get_test_icp_table();
    let (user_1, user_1_id, _) = test_env.create_test_user_with_icp_approval(
        "user1leave_table_transaction_history".to_string(),
        1000.0,
        public_table,
    );

    let public_table = test_env
        .join_test_table(
            public_table,
            user_1,
            user_1_id,
            convert_to_e8s(100.0),
            0,
            false,
        )
        .unwrap();
    assert_eq!(public_table.users.len(), 1);
    let user = test_env.get_user(user_1, user_1_id).unwrap();
    assert!(user.transaction_history.is_some());
    assert_eq!(user.transaction_history.clone().unwrap().len(), 1);
    assert_eq!(
        user.transaction_history.clone().unwrap()[0].amount,
        convert_to_e8s(100.0)
    );
    assert_eq!(
        user.transaction_history.clone().unwrap()[0].transaction_type,
        TransactionType::TableDeposit {
            table_id: public_table.id
        }
    );
    let public_table = test_env
        .leave_test_table(public_table.id, user_1, user.principal_id)
        .unwrap();
    let user = test_env.get_user(user_1, user_1_id).unwrap();
    assert_eq!(user.transaction_history.clone().unwrap().len(), 2);
    assert_eq!(
        user.transaction_history.clone().unwrap()[1].amount,
        convert_to_e8s(99.9999)
    );
    assert_eq!(
        user.transaction_history.clone().unwrap()[1].transaction_type,
        TransactionType::TableWithdraw {
            table_id: public_table.id
        }
    );
}

#[test]
#[serial]
fn deposit_table_transaction_history() {
    let test_env = TestEnv::get();

    // Create a table configuration
    let (public_table, _) = test_env.get_test_icp_table();
    let (user_1, user_1_id, _) = test_env.create_test_user_with_icp_approval(
        "user1deposit_table_transaction_history".to_string(),
        1000.0,
        public_table,
    );

    let public_table = test_env
        .join_test_table(
            public_table,
            user_1,
            user_1_id,
            convert_to_e8s(100.0),
            0,
            false,
        )
        .unwrap();
    assert_eq!(public_table.users.len(), 1);
    let user = test_env.get_user(user_1, user_1_id).unwrap();
    assert!(user.transaction_history.is_some());
    assert_eq!(user.transaction_history.clone().unwrap().len(), 1);
    assert_eq!(
        user.transaction_history.clone().unwrap()[0].amount,
        convert_to_e8s(100.0)
    );
    assert_eq!(
        user.transaction_history.clone().unwrap()[0].transaction_type,
        TransactionType::TableDeposit {
            table_id: public_table.id
        }
    );

    let _ = test_env
        .player_deposit(public_table.id, user_1, user_1_id, convert_to_e8s(50.0))
        .unwrap();
    let user = test_env.get_user(user_1, user_1_id).unwrap();
    assert_eq!(user.transaction_history.clone().unwrap().len(), 2);
    assert_eq!(
        user.transaction_history.clone().unwrap()[1].amount,
        convert_to_e8s(50.0)
    );
    assert_eq!(
        user.transaction_history.clone().unwrap()[1].transaction_type,
        TransactionType::TableDeposit {
            table_id: public_table.id
        }
    );
}

#[test]
#[serial]
fn withdraw_table_transaction_history() {
    let test_env = TestEnv::get();

    // Create a table configuration
    let (public_table, _) = test_env.get_test_icp_table();
    let (user_1, user_1_id, _) = test_env.create_test_user_with_icp_approval(
        "user1withdraw_table_transaction_history".to_string(),
        1000.0,
        public_table,
    );

    let public_table = test_env
        .join_test_table(
            public_table,
            user_1,
            user_1_id,
            convert_to_e8s(100.0),
            0,
            false,
        )
        .unwrap();
    assert_eq!(public_table.users.len(), 1);
    let user = test_env.get_user(user_1, user_1_id).unwrap();
    assert!(user.transaction_history.is_some());
    assert_eq!(user.transaction_history.clone().unwrap().len(), 1);
    assert_eq!(
        user.transaction_history.clone().unwrap()[0].amount,
        convert_to_e8s(100.0)
    );
    assert_eq!(
        user.transaction_history.clone().unwrap()[0].transaction_type,
        TransactionType::TableDeposit {
            table_id: public_table.id
        }
    );

    test_env
        .player_withdraw(public_table.id, user_1, user_1_id, convert_to_e8s(50.0))
        .unwrap();
    let user = test_env.get_user(user_1, user_1_id).unwrap();
    assert_eq!(user.transaction_history.clone().unwrap().len(), 2);
    assert_eq!(
        user.transaction_history.clone().unwrap()[1].amount,
        convert_to_e8s(49.9999)
    );
    assert_eq!(
        user.transaction_history.clone().unwrap()[1].transaction_type,
        TransactionType::TableWithdraw {
            table_id: public_table.id
        }
    );
}
