use serial_test::serial;
use table::poker::game::{
    table_functions::types::{BetType, DealStage, PlayerAction},
    utils::convert_to_e8s,
};

use crate::TestEnv;

#[test]
#[serial]
fn sitting_out_and_back_in() {
    let test_env = TestEnv::get();

    // Create a table configuration
    let (public_table, _) = test_env.get_test_icp_table();
    let (user_1, user_1_id, _1) = test_env.create_test_user_with_icp_approval(
        "user1sitoutandbackin".to_string(),
        1000.0,
        public_table,
    );
    let (user_2, user_2_id, _2) = test_env.create_test_user_with_icp_approval(
        "user2sitoutandbackin".to_string(),
        1000.0,
        public_table,
    );
    let (user_3, user_3_id, _3) = test_env.create_test_user_with_icp_approval(
        "user3sitoutandbackin".to_string(),
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

    let mut sat_out_player = current_player;
    let mut public_table = test_env.get_table(public_table.id).unwrap();
    while public_table.sorted_users.is_none() {
        let current_player = public_table
            .get_player_at_seat(public_table.current_player_index)
            .unwrap();
        test_env
            .player_check(public_table.id, current_player)
            .unwrap();
        public_table = test_env.get_table(public_table.id).unwrap();
        if public_table.deal_stage == DealStage::Turn {
            let current_player = public_table
                .get_player_at_seat(public_table.current_player_index)
                .unwrap();
            test_env.player_sitting_out_test_table(public_table.id, current_player);
            public_table = test_env.get_table(public_table.id).unwrap();
            assert_eq!(
                public_table
                    .user_table_data
                    .get(&current_player)
                    .unwrap()
                    .player_action,
                PlayerAction::Folded
            );
            sat_out_player = current_player;
        }
    }
    test_env
        .start_betting_round_test_table(public_table.id)
        .unwrap();
    public_table = test_env.get_table(public_table.id).unwrap();

    let current_player = public_table
        .get_player_at_seat(public_table.current_player_index)
        .unwrap();
    test_env
        .player_bet(public_table.id, current_player, BetType::Called)
        .unwrap();
    public_table = test_env.get_table(public_table.id).unwrap();

    for user in &public_table.user_table_data {
        println!("{} {:?}", user.0.to_text(), user.1.player_action);
    }

    while public_table.sorted_users.is_none() {
        let current_player = public_table
            .get_player_at_seat(public_table.current_player_index)
            .unwrap();
        test_env
            .player_check(public_table.id, current_player)
            .unwrap();
        public_table = test_env.get_table(public_table.id).unwrap();
        if public_table.deal_stage == DealStage::Turn {
            test_env
                .player_sitting_in_test_table(public_table.id, sat_out_player, user_1)
                .unwrap();
            assert_eq!(
                public_table
                    .user_table_data
                    .get(&sat_out_player)
                    .unwrap()
                    .player_action,
                PlayerAction::SittingOut
            );
        }
    }
    assert_eq!(
        public_table
            .user_table_data
            .get(&sat_out_player)
            .unwrap()
            .player_action,
        PlayerAction::SittingOut
    );
    test_env
        .start_betting_round_test_table(public_table.id)
        .unwrap();
    let public_table = test_env.get_table(public_table.id).unwrap();
    assert!(
        public_table
            .user_table_data
            .get(&sat_out_player)
            .unwrap()
            .player_action
            != PlayerAction::SittingOut
    );
}

#[test]
#[serial]
fn all_players_sitting_out_and_back_in() {
    let test_env = TestEnv::get();

    // Create a table configuration
    let (public_table, _) = test_env.get_test_icp_table();
    let (user_1, user_1_id, _1) = test_env.create_test_user_with_icp_approval(
        "user1allplayerssitoutandin".to_string(),
        1000.0,
        public_table,
    );
    let (user_2, user_2_id, _2) = test_env.create_test_user_with_icp_approval(
        "user2allplayerssitoutandin".to_string(),
        1000.0,
        public_table,
    );
    let (user_3, user_3_id, _3) = test_env.create_test_user_with_icp_approval(
        "user3allplayerssitoutandin".to_string(),
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
    test_env.player_sitting_out_test_table(public_table.id, user_1_id);

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
    test_env.player_sitting_out_test_table(public_table.id, user_2_id);

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
    assert_eq!(public_table.users.len(), 3);
    test_env.player_sitting_out_test_table(public_table.id, user_3_id);

    let public_table = test_env.get_table(public_table.id).unwrap();
    assert_eq!(public_table.deal_stage, DealStage::Fresh);
    test_env
        .player_sitting_in_test_table(public_table.id, user_1_id, user_1)
        .unwrap();
    let public_table = test_env.get_table(public_table.id).unwrap();

    assert_eq!(public_table.deal_stage, DealStage::Fresh);
    test_env
        .player_sitting_in_test_table(public_table.id, user_2_id, user_2)
        .unwrap();
    let public_table = test_env.get_table(public_table.id).unwrap();

    assert_eq!(public_table.deal_stage, DealStage::Flop);
}

#[test]
#[serial]
fn test_two_of_three_players_sitting_out() {
    let test_env = TestEnv::get();

    // Create a table configuration
    let (public_table, _) = test_env.get_test_icp_table();
    let (user_1, user_1_id, _1) = test_env.create_test_user_with_icp_approval(
        "user1twoofthreesitout".to_string(),
        1000.0,
        public_table,
    );
    let (user_2, user_2_id, _2) = test_env.create_test_user_with_icp_approval(
        "user2twoofthreesitout".to_string(),
        1000.0,
        public_table,
    );
    let (user_3, user_3_id, _3) = test_env.create_test_user_with_icp_approval(
        "user3twoofthreesitout".to_string(),
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
    test_env.player_sitting_out_test_table(public_table.id, user_1_id);

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
    test_env.player_sitting_out_test_table(public_table.id, user_2_id);

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
    assert_eq!(public_table.users.len(), 3);
    test_env.player_sitting_out_test_table(public_table.id, user_3_id);

    let public_table = test_env.get_table(public_table.id).unwrap();
    let dealer = public_table
        .get_player_at_seat(public_table.dealer_position)
        .unwrap();
    let all_users = vec![user_1, user_2, user_3];

    // Get the other two users by filtering out the dealer
    let other_users: Vec<_> = all_users
        .into_iter()
        .filter(|user| *user != dealer)
        .collect();
    let user_1 = other_users[0];
    let user_2 = other_users[1];
    test_env
        .player_sitting_in_test_table(public_table.id, user_1_id, user_1)
        .unwrap();
    test_env
        .player_sitting_in_test_table(public_table.id, user_2_id, user_2)
        .unwrap();
}
