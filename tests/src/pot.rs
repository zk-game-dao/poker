use serial_test::serial;
use table::poker::game::{
    table_functions::types::{BetType, DealStage},
    utils::convert_to_e8s,
};

use crate::TestEnv;

#[test]
#[serial]
fn pot_split_when_player_leaves_table() {
    let test_env = TestEnv::get();

    // Create a table configuration
    let (public_table, _) = test_env.get_test_icp_table();
    let (user_1, user_1_id, _1) =
        test_env.create_test_user_with_icp_approval("user1".to_string(), 1000.0, public_table);
    let (user_2, user_2_id, _2) =
        test_env.create_test_user_with_icp_approval("user2".to_string(), 1000.0, public_table);
    let (user_3, user_3_id, _3) =
        test_env.create_test_user_with_icp_approval("user3".to_string(), 1000.0, public_table);

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

    let current_player = public_table
        .get_player_at_seat(public_table.current_player_index)
        .unwrap();
    test_env
        .player_bet(public_table.id, current_player, BetType::Called)
        .unwrap();

    let mut public_table = test_env.get_table(public_table.id).unwrap();
    while public_table.sorted_users.is_none() {
        let current_player = public_table
            .get_player_at_seat(public_table.current_player_index)
            .unwrap();
        test_env
            .player_check(public_table.id, current_player)
            .unwrap();
        public_table = test_env.get_table(public_table.id).unwrap();
        if public_table.deal_stage == DealStage::Turn
            && public_table.users.users.contains_key(&user_3_id)
        {
            test_env
                .leave_test_table(public_table.id, user_3, user_3_id)
                .unwrap();
        }
        println!(
            "User 1 balance: {}",
            public_table.users.get(&user_1_id).unwrap().balance.0
        );
        println!(
            "User 2 balance: {}",
            public_table.users.get(&user_2_id).unwrap().balance.0
        );
        if public_table.users.users.contains_key(&user_3_id) {
            println!(
                "User 3 balance: {}",
                public_table.users.get(&user_3_id).unwrap().balance.0
            );
        }
    }
    let public_table = test_env.get_table(public_table.id).unwrap();
    println!(
        "User 1 balance: {}",
        public_table.users.get(&user_1_id).unwrap().balance.0
    );
    println!(
        "User 2 balance: {}",
        public_table.users.get(&user_2_id).unwrap().balance.0
    );
    assert!(
        public_table.users.get(&user_1_id).unwrap().balance
            != public_table.users.get(&user_2_id).unwrap().balance
    );
}
