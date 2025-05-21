use serial_test::serial;
use table::poker::game::utils::convert_to_e8s;

use crate::TestEnv;

#[test]
#[serial]
fn test_basic_pause_unpause() {
    let test_env = TestEnv::get();
    let (public_table, _) = test_env.get_test_icp_table();

    // Setup 3 players
    let (user_1, user_1_id, _) = test_env.create_test_user_with_icp_approval(
        "user1pauseunpause".to_string(),
        1000.0,
        public_table,
    );
    let (user_2, user_2_id, _) = test_env.create_test_user_with_icp_approval(
        "user2pauseunpause".to_string(),
        1000.0,
        public_table,
    );
    let (user_3, user_3_id, _) = test_env.create_test_user_with_icp_approval(
        "user3pauseunpause".to_string(),
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
    let public_table = test_env
        .join_test_table(
            public_table.id,
            user_3,
            user_3_id,
            convert_to_e8s(100.0),
            2,
            false,
        )
        .unwrap();

    // Attempt to pause empty table
    let result = test_env.pause_table(public_table.id);
    assert!(result.is_ok());

    let table_state = test_env.get_table(public_table.id).unwrap();
    assert!(table_state.config.is_paused.unwrap_or(false));

    // Unpause table
    let result = test_env.unpause_table(public_table.id);
    println!("Unpause result: {:?}", result);
    assert!(result.is_ok());

    let table_state = test_env.get_table(public_table.id).unwrap();
    assert!(!table_state.config.is_paused.unwrap_or(false));
}

#[test]
#[serial]
fn test_pause_after_player_leaves() {
    let test_env = TestEnv::get();
    let (public_table, _) = test_env.get_test_icp_table();

    // Setup 3 players
    let (user_1, user_1_id, _) = test_env.create_test_user_with_icp_approval(
        "user1pauseleave".to_string(),
        1000.0,
        public_table,
    );
    let (user_2, user_2_id, _) = test_env.create_test_user_with_icp_approval(
        "user2pauseleave".to_string(),
        1000.0,
        public_table,
    );
    let (user_3, user_3_id, _) = test_env.create_test_user_with_icp_approval(
        "user3pauseleave".to_string(),
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
    let public_table = test_env
        .join_test_table(
            public_table.id,
            user_3,
            user_3_id,
            convert_to_e8s(100.0),
            2,
            false,
        )
        .unwrap();

    // Have one player leave
    test_env
        .leave_test_table(public_table.id, user_2, user_2_id)
        .unwrap();

    // Should be able to pause with 2 players
    let result = test_env.pause_table(public_table.id);
    assert!(result.is_ok());

    let table_state = test_env.get_table(public_table.id).unwrap();
    assert!(table_state.config.is_paused.unwrap_or(false));
}
