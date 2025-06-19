use candid::Principal;
use user::user::WalletPrincipalId;

use crate::poker::game::{
    table_functions::{
        action_log::ActionType,
        table::{Table, TableId},
        tests::{create_user, get_table_config, turn_tests::is_it_users_turn},
        types::{BetType, DealStage, PlayerAction, SeatStatus},
    },
    types::{GameType, TableStatus},
    utils::convert_to_e8s,
};

#[test]
fn test_table_new() {
    let table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 5),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    assert_eq!(table.config.name, "Test Table");
    assert_eq!(table.config.color, 0);
    assert_eq!(table.config.seats, 5);
    assert_eq!(table.big_blind.0, convert_to_e8s(2.0));
    assert_eq!(table.small_blind.0, convert_to_e8s(1.0));
    assert_eq!(table.config.timer_duration, 30);
    assert_eq!(table.status, TableStatus::Open);
}

#[test]
fn test_add_user() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 5),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user.clone(), 0, false).is_ok());
    assert_eq!(table.number_of_active_players(), 1);
    assert_eq!(
        table.get_player_at_seat(0).unwrap().0,
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal")
    );
}

#[test]
fn test_remove_user() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 5),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user.clone(), 0, false).is_ok());
    assert_eq!(table.number_of_active_players(), 1);
    table
        .remove_user(user.principal_id, ActionType::Leave)
        .unwrap();
    assert_eq!(table.number_of_active_players(), 0);
}

#[test]
fn test_is_full() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 2),
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
    assert!(table.is_full());
}

#[test]
fn test_rotate_dealer() {
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

    table.rotate_dealer().unwrap();
    assert_eq!(table.dealer_position, 1);
    table.rotate_dealer().unwrap();
    assert_eq!(table.dealer_position, 2);
    table.rotate_dealer().unwrap();
    assert_eq!(table.dealer_position, 0);
}

#[test]
fn test_get_blind_user_principals() {
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

    assert_eq!(
        table.get_small_blind_user_principal().unwrap().0,
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal")
    );
    assert_eq!(
        table.get_big_blind_user_principal().unwrap().0,
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal")
    );

    table.rotate_dealer().unwrap();
    assert_eq!(
        table.get_small_blind_user_principal().unwrap().0,
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal")
    );
    assert_eq!(
        table.get_big_blind_user_principal().unwrap().0,
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal")
    );
}

#[test]
fn test_deal_cards() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 2),
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

    table.set_deal_stage(DealStage::Opening);
    assert!(table.deal_cards(false).is_ok());
    assert_eq!(
        table
            .user_table_data
            .get(&user1.principal_id)
            .unwrap()
            .cards
            .len(),
        2
    );
    assert_eq!(
        table
            .user_table_data
            .get(&user2.principal_id)
            .unwrap()
            .cards
            .len(),
        2
    );
}

#[test]
fn test_log_action() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 2),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    table.log_action(Some(user1.principal_id), ActionType::Join);
    assert_eq!(table.action_logs.len(), 1);
    assert_eq!(
        table.action_logs[0].user_principal,
        Some(user1.principal_id)
    );
}

#[test]
fn test_bet() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 2),
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

    assert!(table.bet(user1.principal_id, BetType::BigBlind).is_ok());
    assert_eq!(
        table.users.get(&user1.principal_id).unwrap().balance.0,
        convert_to_e8s(98.0)
    );
    assert_eq!(
        table
            .get_user_table_data(user1.principal_id)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(2.0)
    );
}

#[test]
fn test_showdown() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 2),
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

    table.set_deal_stage(DealStage::River);
    assert!(table.deal_cards(false).is_ok());

    table.pot.0 = convert_to_e8s(20.0);
    assert!(table.showdown().is_ok());

    assert!(table.winners.is_some());
}

#[test]
fn test_folding_transfers_balance() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 2),
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

    assert_eq!(table.add_user(user1.clone(), 0, false), Ok(()));
    assert_eq!(table.add_user(user2.clone(), 1, false), Ok(()));

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert_eq!(
        table.bet(user2.principal_id, BetType::Raised(convert_to_e8s(50.0))),
        Ok(())
    );
    assert_eq!(table.user_fold(user1.principal_id, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
    let user2 = table.users.get(&user2.principal_id).unwrap();
    assert_eq!(user2.balance.0, convert_to_e8s(102.0));
}

#[test]
fn test_start_betting_round_after_folding() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 2),
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

    assert_eq!(table.add_user(user1.clone(), 0, false), Ok(()));
    assert_eq!(table.add_user(user2.clone(), 1, false), Ok(()));

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert_eq!(
        table.bet(user2.principal_id, BetType::Raised(convert_to_e8s(50.0))),
        Ok(())
    );
    assert_eq!(table.user_fold(user1.principal_id, false), Ok(()));

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());
}

#[test]
fn test_round_ends_after_calling_a_raise() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(
            GameType::FixedLimit(convert_to_e8s(1.0), convert_to_e8s(2.0)),
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

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for uid in table.seats.iter() {
        if let SeatStatus::Occupied(uid) = uid {
            if uid != &big_blind_uid && uid != &small_blind_uid {
                other_uid = *uid;
            }
        }
    }

    // Other user raises
    assert_eq!(
        table.bet(other_uid, BetType::Raised(convert_to_e8s(2.0))),
        Ok(())
    );
    assert_eq!(
        table
            .user_table_data
            .get(&other_uid)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(2.0)
    );

    // Small blind calls
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    // Big blind re-raises
    assert_eq!(
        table.bet(big_blind_uid, BetType::Raised(convert_to_e8s(3.0)),),
        Ok(())
    );
    assert_eq!(
        table
            .user_table_data
            .get(&big_blind_uid)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(3.0)
    );

    // Other user calls the second re-raise
    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));
    assert_eq!(
        table
            .user_table_data
            .get(&other_uid)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(3.0)
    );

    // Small blind calls
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    // Big blind calls the second re-raise
    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));
    assert_eq!(table.pot.0, convert_to_e8s(9.0));
    assert_eq!(table.deal_stage, DealStage::Turn);
}

#[test]
fn test_round_goes_to_preflop_after_start_betting_round() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(
            GameType::FixedLimit(convert_to_e8s(1.0), convert_to_e8s(2.0)),
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

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert_eq!(table.community_cards.len(), 0);
    assert_eq!(table.deal_stage, DealStage::Flop);
}

#[test]
fn test_correct_next_player() {
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

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for uid in table.seats.iter() {
        if let SeatStatus::Occupied(uid) = uid {
            if uid != &big_blind_uid && uid != &small_blind_uid {
                other_uid = *uid;
            }
        }
    }

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));
    assert_eq!(
        table
            .user_table_data
            .get(&other_uid)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(2.0)
    );

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));
    assert_eq!(table.pot.0, convert_to_e8s(6.0));

    assert_eq!(
        table
            .get_player_at_seat(table.current_player_index)
            .unwrap(),
        small_blind_uid
    );
}

#[test]
fn test_blinds_added_to_pot() {
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
    for uid in table.seats.iter() {
        if let SeatStatus::Occupied(uid) = uid {
            if uid != &small_blind_uid && uid != &big_blind_uid {
                other_uid = *uid;
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

    assert!(is_it_users_turn(&table, big_blind_uid));
    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.pot.0, convert_to_e8s(6.0));
}

#[test]
fn test_all_in_player_with_side_pot() {
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
    for uid in table.seats.iter() {
        if let SeatStatus::Occupied(uid) = uid {
            if uid != &small_blind_uid && uid != &big_blind_uid {
                other_uid = *uid;
            }
        }
    }

    table.users.get_mut(&other_uid).unwrap().balance.0 = convert_to_e8s(120.0);
    table.users.get_mut(&small_blind_uid).unwrap().balance.0 = convert_to_e8s(90.0);
    table.users.get_mut(&big_blind_uid).unwrap().balance.0 = convert_to_e8s(200.0);

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    assert!(is_it_users_turn(&table, other_uid));
    assert_eq!(
        table.bet(other_uid, BetType::Raised(convert_to_e8s(100.0))),
        Ok(())
    );

    assert!(is_it_users_turn(&table, small_blind_uid));
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));
    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );

    assert!(is_it_users_turn(&table, big_blind_uid));
    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );
    assert!(!is_it_users_turn(&table, small_blind_uid));
}
