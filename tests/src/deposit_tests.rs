use candid::Principal;
use serial_test::serial;
use table::poker::game::{
    table_functions::types::{BetType, DealStage},
    utils::convert_to_e8s,
};

use crate::TestEnv;

#[test]
#[serial]
fn deposit_mid_game() {
    let test_env = TestEnv::get();

    let (public_table, _) = test_env.get_test_icp_table();
    let (user_1, user_1_id, _) = test_env.create_test_user_with_icp_approval(
        "user1deposittest".to_string(),
        1000.0,
        public_table,
    );
    let (user_2, user_2_id, _) = test_env.create_test_user_with_icp_approval(
        "user2deposittest".to_string(),
        1000.0,
        public_table,
    );
    let (user_3, user_3_id, _) = test_env.create_test_user_with_icp_approval(
        "user3deposittest".to_string(),
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

    let public_table = test_env.get_table(public_table.id).unwrap();
    let current_player = public_table
        .get_player_at_seat(public_table.current_player_index)
        .unwrap();
    test_env
        .player_bet(public_table.id, current_player, BetType::Called)
        .unwrap();

    let mut public_table = test_env.get_table(public_table.id).unwrap();
    let mut deposit_player = Principal::anonymous();

    while public_table.sorted_users.is_none() {
        let current_player = public_table
            .get_player_at_seat(public_table.current_player_index)
            .unwrap();
        if public_table.deal_stage == DealStage::Flop {
            let current_player_canisters_id = if current_player == user_1_id {
                user_1
            } else if current_player == user_2_id {
                user_2
            } else {
                user_3
            };
            test_env
                .player_deposit(
                    public_table.id,
                    current_player_canisters_id,
                    current_player,
                    convert_to_e8s(100.0),
                )
                .unwrap();
            deposit_player = current_player;
        }
        test_env
            .player_check(public_table.id, current_player)
            .unwrap();
        public_table = test_env.get_table(public_table.id).unwrap();
        test_env.pocket_ic.tick();
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
        test_env.pocket_ic.tick();
    }
    test_env
        .start_betting_round_test_table(public_table.id)
        .unwrap();
    public_table = test_env.get_table(public_table.id).unwrap();

    test_env.pocket_ic.tick();

    println!(
        "Balance of deposit player: {}",
        public_table.users.get(&deposit_player).unwrap().balance
    );
    assert!(public_table.users.get(&deposit_player).unwrap().balance > convert_to_e8s(150.0));
}
