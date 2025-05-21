use candid::Principal;

use crate::poker::game::{
    table_functions::{
        table::Table,
        tests::{create_user, get_table_config},
        types::{BetType, DealStage, PlayerAction, SeatStatus},
    },
    types::GameType,
    utils::convert_to_e8s,
};

#[test]
fn test_all_in_side_pot_basic() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(50.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(150.0),
    );

    assert!(table.add_user(user1, 0, false).is_ok());
    assert!(table.add_user(user2, 1, false).is_ok());
    assert!(table.add_user(user3, 2, false).is_ok());

    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let mut other_uid = Principal::anonymous();
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if *user_id != big_blind_uid && *user_id != small_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    table.users.get_mut(&small_blind_uid).unwrap().balance = convert_to_e8s(50.0);
    table.users.get_mut(&big_blind_uid).unwrap().balance = convert_to_e8s(150.0);
    table.users.get_mut(&other_uid).unwrap().balance = convert_to_e8s(100.0);

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    // User 3 goes all in
    assert_eq!(
        table.bet(other_uid, BetType::Raised(convert_to_e8s(100.0)),),
        Ok(())
    );
    assert_eq!(
        table.user_table_data.get(&other_uid).unwrap().player_action,
        PlayerAction::AllIn
    );
    assert_eq!(
        table
            .user_table_data
            .get(&other_uid)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(100.0)
    );

    // User 1 calls
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));
    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );
    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(50.0)
    );

    // User 2 calls
    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));
    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn test_all_in_side_pot_basic_two() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(50.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(150.0),
    );

    assert!(table.add_user(user1, 0, false).is_ok());
    assert!(table.add_user(user2, 1, false).is_ok());
    assert!(table.add_user(user3, 2, false).is_ok());

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let mut other_uid = Principal::anonymous();
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if *user_id != big_blind_uid && *user_id != small_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    table.users.get_mut(&small_blind_uid).unwrap().balance = convert_to_e8s(100.0);
    table.users.get_mut(&big_blind_uid).unwrap().balance = convert_to_e8s(150.0);
    table.users.get_mut(&other_uid).unwrap().balance = convert_to_e8s(50.0);

    // User 2 goes all in
    assert_eq!(
        table.bet(other_uid, BetType::Raised(convert_to_e8s(50.0))),
        Ok(())
    );
    assert_eq!(
        table.user_table_data.get(&other_uid).unwrap().player_action,
        PlayerAction::AllIn
    );
    assert_eq!(
        table
            .user_table_data
            .get(&other_uid)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(50.0)
    );

    // User 3 raises to 100
    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(100.0)),),
        Ok(())
    );
    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );
    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(100.0)
    );

    // User 1 calls
    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn test_all_in_side_pot_basic_three() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(50.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(150.0),
    );

    assert!(table.add_user(user1, 0, false).is_ok());
    assert!(table.add_user(user2, 1, false).is_ok());
    assert!(table.add_user(user3, 2, false).is_ok());

    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let mut other_uid = Principal::anonymous();
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if *user_id != big_blind_uid && *user_id != small_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }
    table.users.get_mut(&small_blind_uid).unwrap().balance = convert_to_e8s(10.0);
    table.users.get_mut(&big_blind_uid).unwrap().balance = convert_to_e8s(4.0);
    table.users.get_mut(&other_uid).unwrap().balance = convert_to_e8s(8.0);

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

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

    assert_eq!(table.side_pots.len(), 0);

    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(2.0)),),
        Ok(())
    );

    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.users.get(&big_blind_uid).unwrap().balance, 0);

    assert_eq!(
        table
            .user_table_data
            .get(&big_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );

    assert_eq!(
        table.bet(other_uid, BetType::Raised(convert_to_e8s(3.5))),
        Ok(())
    );

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.side_pots.len(), 1);
    assert_eq!(table.side_pots[0].confirmed_pot, convert_to_e8s(3.0));
    assert_eq!(table.side_pots[0].user_principals.len(), 2);
}

#[test]
fn test_all_in_side_pot_basic_four() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(50.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(150.0),
    );

    assert!(table.add_user(user1, 0, false).is_ok());
    assert!(table.add_user(user2, 1, false).is_ok());
    assert!(table.add_user(user3, 2, false).is_ok());

    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let mut other_uid = Principal::anonymous();
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if *user_id != big_blind_uid && *user_id != small_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }
    table.users.get_mut(&small_blind_uid).unwrap().balance = convert_to_e8s(6.0);
    table.users.get_mut(&big_blind_uid).unwrap().balance = convert_to_e8s(4.0);
    table.users.get_mut(&other_uid).unwrap().balance = convert_to_e8s(10.0);

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

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

    assert_eq!(table.side_pots.len(), 0);

    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(2.0)),),
        Ok(())
    );

    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.users.get(&big_blind_uid).unwrap().balance, 0);

    assert_eq!(
        table
            .user_table_data
            .get(&big_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );

    assert_eq!(
        table.bet(other_uid, BetType::Raised(convert_to_e8s(2.5))),
        Ok(())
    );

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(
        table.users.get(&small_blind_uid).unwrap().balance,
        convert_to_e8s(1.5)
    );

    assert_eq!(table.side_pots.len(), 1);
    assert_eq!(table.side_pots[0].confirmed_pot, convert_to_e8s(1.0));
    assert_eq!(table.side_pots[0].user_principals.len(), 2);
}

#[test]
fn test_all_in_side_pot_basic_five() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(50.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(150.0),
    );

    assert!(table.add_user(user1, 0, false).is_ok());
    assert!(table.add_user(user2, 1, false).is_ok());
    assert!(table.add_user(user3, 2, false).is_ok());

    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let mut other_uid = Principal::anonymous();
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if *user_id != big_blind_uid && *user_id != small_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }
    println!("Small blind: {}", small_blind_uid);
    println!("Big blind: {}", big_blind_uid);
    println!("Other: {}", other_uid);
    table.users.get_mut(&small_blind_uid).unwrap().balance = convert_to_e8s(6.0);
    table.users.get_mut(&big_blind_uid).unwrap().balance = convert_to_e8s(4.0);
    table.users.get_mut(&other_uid).unwrap().balance = convert_to_e8s(10.0);

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

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

    assert_eq!(table.side_pots.len(), 0);

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(
        table.bet(other_uid, BetType::Raised(convert_to_e8s(2.0))),
        Ok(())
    );

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.users.get(&big_blind_uid).unwrap().balance, 0);

    assert_eq!(
        table
            .user_table_data
            .get(&big_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );

    assert_eq!(table.pot, convert_to_e8s(12.0));

    assert_eq!(
        table.bet(other_uid, BetType::Raised(convert_to_e8s(2.0))),
        Ok(())
    );

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );
}

#[test]
fn test_multiple_side_pots() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(50.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(150.0),
    );

    assert!(table.add_user(user1, 0, false).is_ok());
    assert!(table.add_user(user2, 1, false).is_ok());
    assert!(table.add_user(user3, 2, false).is_ok());

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let mut other_uid = Principal::anonymous();
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if *user_id != big_blind_uid && *user_id != small_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    table.users.get_mut(&small_blind_uid).unwrap().balance = convert_to_e8s(100.0);
    table.users.get_mut(&big_blind_uid).unwrap().balance = convert_to_e8s(150.0);
    table.users.get_mut(&other_uid).unwrap().balance = convert_to_e8s(50.0);

    // User 1 goes all in
    assert_eq!(
        table.bet(other_uid, BetType::Raised(convert_to_e8s(50.0))),
        Ok(())
    );
    assert_eq!(
        table.user_table_data.get(&other_uid).unwrap().player_action,
        PlayerAction::AllIn
    );

    assert_eq!(table.deal_stage, DealStage::Flop);

    // User 2 raises to 100
    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(100.0)),),
        Ok(())
    );
    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(100.0)
    );
    assert_eq!(table.deal_stage, DealStage::Flop);

    // User 3 raises to 150
    assert_eq!(
        table.bet(big_blind_uid, BetType::Raised(convert_to_e8s(150.0)),),
        Ok(())
    );
    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn test_all_in_side_pot_multiple_players() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(GameType::NoLimit(1), 4),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        50,
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        100,
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        150,
    );
    let user4 = create_user(
        Principal::from_text("bd3sg-teaaa-aaaaa-qaaba-cai").expect("Could not decode principal"),
        200,
    );

    assert!(table.add_user(user1, 0, false).is_ok());
    assert!(table.add_user(user2, 1, false).is_ok());
    assert!(table.add_user(user3, 2, false).is_ok());
    assert!(table.add_user(user4.clone(), 3, false).is_ok());

    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let mut other_uid = Principal::anonymous();
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if *user_id != big_blind_uid && *user_id != small_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }
    let mut user_4_uid = Principal::anonymous();
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if *user_id != big_blind_uid && *user_id != small_blind_uid && *user_id != other_uid {
                user_4_uid = *user_id;
                break;
            }
        }
    }

    table.users.get_mut(&small_blind_uid).unwrap().balance = 50;
    table.users.get_mut(&big_blind_uid).unwrap().balance = 100;
    table.users.get_mut(&other_uid).unwrap().balance = 150;
    table.users.get_mut(&user_4_uid).unwrap().balance = 200;

    assert_eq!(
        table.start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        Ok((vec![], vec![]))
    );

    // User 4 goes all in
    assert_eq!(table.bet(user_4_uid, BetType::Raised(200)), Ok(()));
    assert_eq!(
        table
            .user_table_data
            .get(&user_4_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );
    assert_eq!(
        table
            .user_table_data
            .get(&user_4_uid)
            .unwrap()
            .current_total_bet,
        200
    );

    // User 3 calls
    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));
    assert_eq!(
        table.user_table_data.get(&other_uid).unwrap().player_action,
        PlayerAction::AllIn
    );
    assert_eq!(
        table
            .user_table_data
            .get(&other_uid)
            .unwrap()
            .current_total_bet,
        150
    );

    // User 1 calls
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));
    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );
    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .current_total_bet,
        50
    );

    // User 2 calls
    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));
    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn test_betting_round_from_logs() {
    // Initialize the table
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );

    // Create users based on IDs from the logs
    let user0 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(10.0),
    ); // User IDs from logs start from 0
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(10.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(10.0),
    );

    // Add users to the table
    assert!(table.add_user(user0, 0, false).is_ok());
    assert!(table.add_user(user1, 1, false).is_ok());
    assert!(table.add_user(user2, 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = Principal::anonymous();
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if *user_id != big_blind_uid && *user_id != small_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    table.users.get_mut(&big_blind_uid).unwrap().balance = convert_to_e8s(4.0);

    // Start a betting round
    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    // User 0 calls
    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    // User 1 calls
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    // User 2 goes all in with an additional 2 units
    assert_eq!(
        table.bet(big_blind_uid, BetType::Raised(convert_to_e8s(2.0)),),
        Ok(())
    );

    // User 0 calls the all-in
    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    // Assess the state at the end of the round
    assert_eq!(table.pot, convert_to_e8s(12.0)); // As mentioned in the logs, the final pot amount is 12
    assert_eq!(
        table
            .user_table_data
            .get(&big_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );

    assert_eq!(table.deal_stage, DealStage::River);
}

#[test]
fn test_all_in_call() {
    // Initialize the table
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );

    // Create users based on IDs from the logs
    let user0 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(10.0),
    ); // User IDs from logs start from 0
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(10.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(10.0),
    );

    // Add users to the table
    assert!(table.add_user(user0, 0, false).is_ok());
    assert!(table.add_user(user1, 1, false).is_ok());
    assert!(table.add_user(user2, 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = Principal::anonymous();
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if *user_id != big_blind_uid && *user_id != small_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    table.users.get_mut(&big_blind_uid).unwrap().balance = convert_to_e8s(4.0);

    // Start a betting round
    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    // User 0 calls
    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    // User 1 calls
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert_eq!(table.pot, convert_to_e8s(6.0));

    assert_eq!(
        table.bet(other_uid, BetType::Raised(convert_to_e8s(4.0))),
        Ok(())
    );

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.pot, convert_to_e8s(12.0));
    assert_eq!(table.side_pots[0].confirmed_pot, convert_to_e8s(4.0));
}

#[test]
fn test_all_in_and_fold() {
    // Initialize the table
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );

    // Create users based on IDs from the logs
    let user0 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(10.0),
    ); // User IDs from logs start from 0
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(10.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(10.0),
    );

    // Add users to the table
    assert!(table.add_user(user0, 0, false).is_ok());
    assert!(table.add_user(user1, 1, false).is_ok());
    assert!(table.add_user(user2, 2, false).is_ok());

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = Principal::anonymous();
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if *user_id != big_blind_uid && *user_id != small_blind_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    table.users.get_mut(&big_blind_uid).unwrap().balance = convert_to_e8s(4.0);
    table.users.get_mut(&other_uid).unwrap().balance = convert_to_e8s(6.0);

    // Start a betting round
    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    // User 0 calls
    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    // User 1 calls
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    assert_eq!(table.pot, convert_to_e8s(6.0));

    assert_eq!(
        table.bet(other_uid, BetType::Raised(convert_to_e8s(4.0))),
        Ok(())
    );

    assert_eq!(
        table.user_table_data.get(&other_uid).unwrap().player_action,
        PlayerAction::AllIn
    );

    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(6.0))),
        Ok(())
    );

    assert_eq!(table.user_fold(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}
