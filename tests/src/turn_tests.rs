use serial_test::serial;
use table::poker::game::{
    table_functions::types::{BetType, DealStage, SeatStatus},
    utils::convert_to_e8s,
};

use crate::TestEnv;

#[test]
#[serial]
fn basic_turn_test_empty_seats() {
    let test_env = TestEnv::get();

    // Create a table configuration
    let (public_table, _) = test_env.get_test_icp_table();
    let (user_1, user_1_id, _1) = test_env.create_test_user_with_icp_approval(
        "user1basic_turn_test_empty_seats".to_string(),
        1000.0,
        public_table,
    );
    let (user_2, user_2_id, _2) = test_env.create_test_user_with_icp_approval(
        "user2basic_turn_test_empty_seats".to_string(),
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
            4,
            false,
        )
        .unwrap();
    assert_eq!(public_table.users.len(), 2);

    let public_table = test_env.get_table(public_table.id).unwrap();
    assert_eq!(public_table.dealer_position, 0);
    assert_eq!(public_table.deal_stage, DealStage::Flop);
}

#[test]
#[serial]
fn leaving_table_doesnt_change_current_player() {
    let test_env = TestEnv::get();

    // Create a table configuration
    let (public_table, _) = test_env.get_test_icp_table();
    let (user_1, user_1_id, _1) = test_env.create_test_user_with_icp_approval(
        "user1leaving_table_doesnt_change_current_player".to_string(),
        1000.0,
        public_table,
    );
    let (user_2, user_2_id, _2) = test_env.create_test_user_with_icp_approval(
        "user2leaving_table_doesnt_change_current_player".to_string(),
        1000.0,
        public_table,
    );
    let (user_3, user_3_id, _3) = test_env.create_test_user_with_icp_approval(
        "user3leaving_table_doesnt_change_current_player".to_string(),
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

    let mut public_table = test_env.get_table(public_table.id).unwrap();
    while public_table.sorted_users.is_none() {
        let current_player = public_table
            .get_player_at_seat(public_table.current_player_index)
            .unwrap();
        test_env
            .player_check(public_table.id, current_player)
            .unwrap();
        public_table = test_env.get_table(public_table.id).unwrap();
    }
    test_env
        .start_betting_round_test_table(public_table.id)
        .unwrap();
    public_table = test_env.get_table(public_table.id).unwrap();

    assert_eq!(public_table.users.len(), 3);

    let current_player = public_table
        .get_player_at_seat(public_table.current_player_index)
        .unwrap();
    test_env
        .player_bet(public_table.id, current_player, BetType::Called)
        .unwrap();
    let public_table = test_env.get_table(public_table.id).unwrap();
    let current_player = public_table
        .get_player_at_seat(public_table.current_player_index)
        .unwrap();
    let player_to_leave = public_table
        .seats
        .iter()
        .find(|seat| {
            if let SeatStatus::Occupied(principal) = seat {
                principal != &current_player
            } else {
                false
            }
        })
        .unwrap();
    let player_to_leave = match player_to_leave {
        SeatStatus::Occupied(p) => p,
        _ => panic!("Player to leave not found"),
    };
    let player_to_leave_canister = if player_to_leave == &user_1_id {
        user_1
    } else if player_to_leave == &user_2_id {
        user_2
    } else {
        user_3
    };
    test_env
        .leave_test_table(public_table.id, player_to_leave_canister, *player_to_leave)
        .unwrap();
    let public_table = test_env.get_table(public_table.id).unwrap();
    let current_player_post_leave = public_table
        .get_player_at_seat(public_table.current_player_index)
        .unwrap();
    assert_eq!(current_player, current_player_post_leave);
}
