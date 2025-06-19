use candid::Principal;
use user::user::WalletPrincipalId;

use crate::poker::game::{
    table_functions::{
        table::{Table, TableId},
        tests::{create_user, get_table_config},
        types::{BetType, DealStage, PlayerAction, SeatStatus},
    },
    types::GameType,
    utils::convert_to_e8s,
};

pub fn is_it_users_turn(table: &Table, user_id: WalletPrincipalId) -> bool {
    table
        .get_player_at_seat(table.current_player_index)
        .unwrap()
        == user_id
}

#[test]
fn basic_turn_test() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(
            GameType::SpreadLimit(convert_to_e8s(1.0), convert_to_e8s(4.0)),
            3,
        ),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));
}

#[test]
fn basic_turn_test_empty_seats() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(
            GameType::SpreadLimit(convert_to_e8s(1.0), convert_to_e8s(4.0)),
            6,
        ),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 4, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);
}

#[test]
fn basic_turn_test_two() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(
            GameType::SpreadLimit(convert_to_e8s(1.0), convert_to_e8s(4.0)),
            4,
        ),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user4 = create_user(
        Principal::from_text("bd3sg-teaaa-aaaaa-qaaba-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());
    assert!(table.add_user(user4.clone(), 3, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let other_uid = table.get_player_at_seat(table.dealer_position).unwrap();
    let mut user4 = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid && user_id != &other_uid {
                user4 = *user_id;
                break;
            }
        }
    }

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert!(is_it_users_turn(&table, user4));

    assert_eq!(table.bet(user4, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));
}

#[test]
fn basic_turn_test_three() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(
            GameType::SpreadLimit(convert_to_e8s(1.0), convert_to_e8s(4.0)),
            5,
        ),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user4 = create_user(
        Principal::from_text("bd3sg-teaaa-aaaaa-qaaba-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    let user5 = create_user(
        Principal::from_text("uyxh5-bi3za-gxbfs-op3gj-ere73-a6jhv-5jky3-zawef-b5r2s-k26un-sae")
            .expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());
    assert!(table.add_user(user4.clone(), 3, false).is_ok());
    assert!(table.add_user(user5.clone(), 4, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let other_uid = table.get_player_at_seat(table.dealer_position).unwrap();
    let mut user4 = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid && user_id != &other_uid {
                user4 = *user_id;
                break;
            }
        }
    }
    let mut user5 = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid
                && user_id != &big_blind_uid
                && user_id != &other_uid
                && user_id != &user4
            {
                user5 = *user_id;
                break;
            }
        }
    }

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert!(is_it_users_turn(&table, user4));

    assert_eq!(table.bet(user4, BetType::Called), Ok(()));
    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, user5));

    assert_eq!(table.bet(user5, BetType::Called), Ok(()));
    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));
    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));
    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));
    assert_eq!(table.deal_stage, DealStage::Turn);
}

#[test]
fn basic_turn_test_four() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(
            GameType::SpreadLimit(convert_to_e8s(1.0), convert_to_e8s(4.0)),
            6,
        ),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user4 = create_user(
        Principal::from_text("bd3sg-teaaa-aaaaa-qaaba-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    let user5 = create_user(
        Principal::from_text("uyxh5-bi3za-gxbfs-op3gj-ere73-a6jhv-5jky3-zawef-b5r2s-k26un-sae")
            .expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    let user6 = create_user(
        Principal::from_text("3kryc-rcxpr-ojfta-nasyy-tmfwd-73zt5-ajtev-zxwv5-dob3d-ymvow-4qe")
            .expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());
    assert!(table.add_user(user4.clone(), 3, false).is_ok());
    assert!(table.add_user(user5.clone(), 4, false).is_ok());
    assert!(table.add_user(user6.clone(), 5, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let other_uid = table.get_player_at_seat(table.dealer_position).unwrap();
    let mut user4 = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid && user_id != &other_uid {
                user4 = *user_id;
                break;
            }
        }
    }
    let mut user5 = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid
                && user_id != &big_blind_uid
                && user_id != &other_uid
                && user_id != &user4
            {
                user5 = *user_id;
                break;
            }
        }
    }
    let mut user6 = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid
                && user_id != &big_blind_uid
                && user_id != &other_uid
                && user_id != &user4
                && user_id != &user5
            {
                user6 = *user_id;
                break;
            }
        }
    }

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert!(is_it_users_turn(&table, user4));

    assert_eq!(table.bet(user4, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, user5));

    assert_eq!(table.bet(user5, BetType::Called), Ok(()));
    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, user6));

    assert_eq!(table.bet(user6, BetType::Called), Ok(()));
    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));
    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));
    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));
    assert_eq!(table.deal_stage, DealStage::Turn);
}

#[test]
fn basic_turn_test_with_raise() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(
        table.bet(other_uid, BetType::Raised(convert_to_e8s(10.0))),
        Ok(())
    );

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, big_blind_uid));
    assert_eq!(table.deal_stage, DealStage::Flop);

    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, small_blind_uid));
}

#[test]
fn basic_turn_test_with_fold() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(
        table.bet(other_uid, BetType::Raised(convert_to_e8s(10.0))),
        Ok(())
    );

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, big_blind_uid));
    assert_eq!(table.deal_stage, DealStage::Flop);

    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_check(small_blind_uid, false), Ok(()));

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_fold(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.user_check(other_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);
}

#[test]
fn basic_turn_test_check_own_raise() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());
    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.user_fold(other_uid, false), Ok(()));

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(5.0)),),
        Ok(())
    );

    assert!(is_it_users_turn(&table, big_blind_uid));
    assert_eq!(table.deal_stage, DealStage::Flop);

    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);
}

#[test]
fn two_users_check_to_showdown() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_check(small_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_check(small_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn two_users_check_to_showdown_two_rounds() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_check(small_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_check(small_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_check(small_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_check(small_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn all_users_check_to_showdown() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_check(small_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.user_check(other_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_check(small_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.user_check(other_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn all_users_check_to_showdown_two_rounds() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_check(small_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.user_check(other_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_check(small_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.user_check(other_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_check(small_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.user_check(other_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_check(small_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.user_check(other_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn turn_test_all_in() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    table.users.get_mut(&other_uid).unwrap().balance.0 = convert_to_e8s(4.0);
    table.users.get_mut(&small_blind_uid).unwrap().balance.0 = convert_to_e8s(10.0);

    println!("Small blind: {}", small_blind_uid.0.to_text());
    println!("Big blind: {}", big_blind_uid.0.to_text());
    println!("Other: {}", other_uid.0.to_text());

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(2.0)),),
        Ok(())
    );

    assert_eq!(table.deal_stage, DealStage::Turn);
    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);
    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(
        table.user_table_data.get(&other_uid).unwrap().player_action,
        PlayerAction::AllIn
    );

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_check(small_blind_uid, false), Ok(()));

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn turn_test_all_in_two() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    table.users.get_mut(&big_blind_uid).unwrap().balance.0 = convert_to_e8s(3.0);
    table.users.get_mut(&other_uid).unwrap().balance.0 = convert_to_e8s(10.0);

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(
        table.bet(other_uid, BetType::Raised(convert_to_e8s(2.5))),
        Ok(())
    );

    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(3.0)),),
        Ok(())
    );

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Flop);
    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);
    assert!(is_it_users_turn(&table, small_blind_uid));
}

#[test]
fn turn_test_fold_one() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    table.users.get_mut(&big_blind_uid).unwrap().balance.0 = convert_to_e8s(3.0);
    table.users.get_mut(&other_uid).unwrap().balance.0 = convert_to_e8s(10.0);

    assert_eq!(
        table.start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        Ok((vec![], vec![]))
    );

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_check(small_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_fold(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.user_check(other_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);
}

#[test]
fn turn_test_fold_and_sitting_out() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    table.users.get_mut(&big_blind_uid).unwrap().balance.0 = convert_to_e8s(3.0);
    table.users.get_mut(&other_uid).unwrap().balance.0 = convert_to_e8s(10.0);

    assert_eq!(
        table.start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        Ok((vec![], vec![]))
    );

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_fold(small_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_sitting_out(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn turn_test_all_in_cycle_to_showdown_one_fold_one_all_in() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    table.users.get_mut(&big_blind_uid).unwrap().balance.0 = convert_to_e8s(3.0);
    table.users.get_mut(&other_uid).unwrap().balance.0 = convert_to_e8s(10.0);

    assert_eq!(
        table.start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        Ok((vec![], vec![]))
    );

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(
        table.bet(other_uid, BetType::Raised(convert_to_e8s(10.0))),
        Ok(())
    );

    assert_eq!(
        table.user_table_data.get(&other_uid).unwrap().player_action,
        PlayerAction::AllIn
    );

    assert_eq!(table.user_fold(small_blind_uid, false), Ok(()));

    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn turn_test_all_in_and_sitting_out() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    table.users.get_mut(&other_uid).unwrap().balance.0 = convert_to_e8s(6.0);
    table.users.get_mut(&big_blind_uid).unwrap().balance.0 = convert_to_e8s(10.0);

    assert_eq!(
        table.start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        Ok((vec![], vec![]))
    );

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_sitting_out(small_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(
        table.bet(other_uid, BetType::Raised(convert_to_e8s(4.0))),
        Ok(())
    );

    assert_eq!(
        table.user_table_data.get(&other_uid).unwrap().player_action,
        PlayerAction::AllIn
    );

    println!("FROM HERE ------------------------------------");
    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn turn_test_check_after_raising() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(10.0)),),
        Ok(())
    );

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);
}

#[test]
fn turn_test_sitting_out() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_fold(small_blind_uid, false), Ok(()));

    table
        .set_player_action(small_blind_uid, PlayerAction::SittingOut)
        .unwrap();

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.user_check(other_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.user_check(other_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn turn_test_sitting_out_are_blinds_eaten() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert_eq!(table.user_fold(small_blind_uid, false), Ok(()));

    table
        .set_player_action(small_blind_uid, PlayerAction::SittingOut)
        .unwrap();

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.user_check(other_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.user_check(other_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);

    assert_eq!(
        table.users.get(&small_blind_uid).unwrap().balance.0,
        convert_to_e8s(98.0)
    );

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert_eq!(
        table.users.get(&small_blind_uid).unwrap().balance.0,
        convert_to_e8s(97.0)
    );

    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::SittingOut
    );
}

#[test]
fn turn_test_sitting_out_at_opening() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    println!("HERE -------------------------");
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Flop);

    while table.deal_stage != DealStage::Showdown {
        let current_player = table
            .get_player_at_seat(table.current_player_index)
            .unwrap();
        table.user_check(current_player, false).unwrap();
    }

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    let current_player = table
        .get_player_at_seat(table.current_player_index)
        .unwrap();
    table.user_sitting_out(current_player, false).unwrap();

    let current_player = table
        .get_player_at_seat(table.current_player_index)
        .unwrap();
    table.bet(current_player, BetType::Called).unwrap();

    assert_eq!(table.deal_stage, DealStage::Flop);
}

#[test]
fn turn_test_sitting_out_and_fold_at_opening() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Flop);

    while table.deal_stage != DealStage::Showdown {
        let current_player = table
            .get_player_at_seat(table.current_player_index)
            .unwrap();
        table.user_check(current_player, false).unwrap();
    }

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    let current_player = table
        .get_player_at_seat(table.current_player_index)
        .unwrap();
    table.user_sitting_out(current_player, false).unwrap();

    let current_player = table
        .get_player_at_seat(table.current_player_index)
        .unwrap();
    table.user_fold(current_player, false).unwrap();

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn turn_test_sitting_out_and_all_in() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    table.users.get_mut(&big_blind_uid).unwrap().balance.0 = convert_to_e8s(200.0);
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_sitting_out(small_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.user_check(other_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(
        table.bet(big_blind_uid, BetType::Raised(convert_to_e8s(100.0))),
        Ok(())
    );

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn turn_test_folded_and_all_in() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_fold(small_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(
        table.bet(big_blind_uid, BetType::Raised(convert_to_e8s(98.0))),
        Ok(())
    );

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn turn_test_folded_and_two_all_in() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.user_check(small_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.user_fold(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.user_check(other_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::River);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(98.0))),
        Ok(())
    );

    assert!(is_it_users_turn(&table, other_uid));

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn turn_test_three_sitting_out_and_two_back_in_at_opening() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());

    assert!(table.add_user(user2.clone(), 1, false).is_ok());

    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    table.user_sitting_out(small_blind_uid, false).unwrap();
    table.user_sitting_out(big_blind_uid, false).unwrap();
    table.user_sitting_out(other_uid, false).unwrap();

    table
        .set_player_action(small_blind_uid, PlayerAction::None)
        .unwrap();
    table
        .set_player_action(big_blind_uid, PlayerAction::None)
        .unwrap();

    let res = table.start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    println!("{:?}", res);
    assert!(res.is_ok());

    assert!(is_it_users_turn(&table, small_blind_uid));
}

#[test]
fn heads_up_all_in() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());

    assert!(table.add_user(user2.clone(), 1, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert_eq!(table.deal_stage, DealStage::Flop);
    assert!(is_it_users_turn(&table, small_blind_uid));
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Flop);
    assert!(is_it_users_turn(&table, big_blind_uid));
    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);
    assert!(is_it_users_turn(&table, small_blind_uid));
    assert_eq!(table.user_check(small_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);
    assert!(is_it_users_turn(&table, big_blind_uid));
    assert_eq!(
        table.bet(big_blind_uid, BetType::Raised(convert_to_e8s(98.0))),
        Ok(())
    );
    assert_eq!(
        table
            .get_user_table_data(big_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );

    assert_eq!(table.deal_stage, DealStage::Turn);
    assert!(is_it_users_turn(&table, small_blind_uid));
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn turn_test_one_sitting_out_and_one_all_in() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());

    assert!(table.add_user(user2.clone(), 1, false).is_ok());

    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if user_id != &small_blind_uid && user_id != &big_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    table.user_sitting_out(big_blind_uid, false).unwrap();

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert!(is_it_users_turn(&table, other_uid));
    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));
    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, small_blind_uid));
    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(100.0))),
        Ok(())
    );
    println!("Raised");
    assert_eq!(
        table
            .get_user_table_data(small_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );
    println!("All in");
    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, other_uid));
    println!("Other turn");
    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));
    println!("Called");

    assert_eq!(table.deal_stage, DealStage::Showdown);
}
