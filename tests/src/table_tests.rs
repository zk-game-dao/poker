use serial_test::serial;
use table::poker::game::utils::convert_to_e8s;

use crate::TestEnv;

#[test]
#[serial]
fn join_table() {
    let test_env = TestEnv::get();

    // Create a table configuration
    let (public_table, _) = test_env.get_test_icp_table();
    let (user_1, user_1_id, _1) = test_env.create_test_user_with_icp_approval(
        "user1join_table".to_string(),
        1000.0,
        public_table,
    );
    let (user_2, user_2_id, _2) = test_env.create_test_user_with_icp_approval(
        "user2join_table".to_string(),
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
    let public_table = test_env
        .join_test_table(
            public_table.id,
            user_2,
            user_2_id,
            convert_to_e8s(100.0),
            1,
            false,
        )
        .unwrap();
    assert_eq!(public_table.users.len(), 2);

    let _ = test_env
        .leave_test_table(public_table.id, user_1, user_1_id)
        .unwrap();
    let _ = test_env
        .leave_test_table(public_table.id, user_2, user_2_id)
        .unwrap();
}

#[test]
#[serial]
fn leave_table() {
    let test_env = TestEnv::get();

    // Create a table configuration
    let (public_table, _) = test_env.get_test_icp_table();
    let (user_1, user_1_id, _1) = test_env.create_test_user_with_icp_approval(
        "user1leavetable".to_string(),
        1000.0,
        public_table,
    );
    let (user_2, user_2_id, _2) = test_env.create_test_user_with_icp_approval(
        "user2leavetable".to_string(),
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
    assert!(!user.active_tables.is_empty());
    assert_eq!(user.active_tables[0], public_table.id);
    let public_table = test_env
        .join_test_table(
            public_table.id,
            user_2,
            user_2_id,
            convert_to_e8s(100.0),
            1,
            false,
        )
        .unwrap();
    assert_eq!(public_table.users.len(), 2);
    let user = test_env.get_user(user_2, user_2_id).unwrap();
    assert!(!user.active_tables.is_empty());
    assert_eq!(user.active_tables[0], public_table.id);

    let public_table = test_env.leave_test_table(public_table.id, user_1, user_1_id);
    let public_table = public_table.unwrap();
    let public_table = test_env.leave_test_table(public_table.id, user_2, user_2_id);
    let public_table = public_table.unwrap();
    let public_table = test_env.get_table(public_table.id).unwrap();
    assert_eq!(public_table.users.len(), 0);
}

#[test]
#[serial]
fn join_ckusdc_table() {
    let test_env = TestEnv::get();

    // Create a table configuration
    let (public_table, _) = test_env.get_ckusdc_test_table();
    let (user_1, user_1_id, _1) = test_env.create_test_user_with_ckusdc_approval(
        "user1joinckusdctable".to_string(),
        1000.0,
        public_table,
    );
    let (user_2, user_2_id, _2) = test_env.create_test_user_with_ckusdc_approval(
        "user2joinckusdctable".to_string(),
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
    let public_table = test_env
        .join_test_table(
            public_table.id,
            user_2,
            user_2_id,
            convert_to_e8s(100.0),
            1,
            false,
        )
        .unwrap();
    assert_eq!(public_table.users.len(), 2);
}

#[test]
#[serial]
fn leave_ckusdc_table() {
    let test_env = TestEnv::get();

    // Create a table configuration
    let (public_table, _) = test_env.get_ckusdc_test_table();
    let (user_1, user_1_id, _1) = test_env.create_test_user_with_ckusdc_approval(
        "user1leaveckusdctable".to_string(),
        1000.0,
        public_table,
    );
    let (user_2, user_2_id, _2) = test_env.create_test_user_with_ckusdc_approval(
        "user2leaveckusdctable".to_string(),
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
    assert_eq!(user.active_tables.len(), 1);
    assert_eq!(user.active_tables[0], public_table.id);
    let public_table = test_env
        .join_test_table(
            public_table.id,
            user_2,
            user_2_id,
            convert_to_e8s(100.0),
            1,
            false,
        )
        .unwrap();
    assert_eq!(public_table.users.len(), 2);
    let user = test_env.get_user(user_2, user_2_id).unwrap();
    assert_eq!(user.active_tables.len(), 1);
    assert_eq!(user.active_tables[0], public_table.id);

    let public_table = test_env.leave_test_table(public_table.id, user_1, user_1_id);
    let public_table = public_table.unwrap();
    assert!(!public_table.users.is_empty());
    let public_table = test_env.leave_test_table(public_table.id, user_2, user_2_id);
    let public_table = public_table.unwrap();
    let public_table = test_env.get_table(public_table.id).unwrap();
    assert_eq!(public_table.users.len(), 0);
}
