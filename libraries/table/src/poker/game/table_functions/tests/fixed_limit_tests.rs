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
fn test_fixed_limit_insufficient_funds_when_raising() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(
            GameType::FixedLimit(convert_to_e8s(1.0), convert_to_e8s(2.0)),
            2,
        ),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(1.0),
    ); // Insufficient balance for raising
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    // User1 attempts to raise with insufficient funds
    assert!(table
        .bet(user1.principal_id, BetType::Raised(convert_to_e8s(2.0)))
        .is_err());
}

#[test]
fn test_correct_blinds_fixed_limit() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(
            GameType::FixedLimit(convert_to_e8s(1.0), convert_to_e8s(2.0)),
            2,
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
    assert!(table.deal_stage.eq(&DealStage::Flop));
    assert_eq!(
        table
            .user_table_data
            .get(&user1.principal_id)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(1.0)
    );
    assert_eq!(
        table
            .user_table_data
            .get(&user2.principal_id)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(0.5)
    );
}

#[test]
fn test_invalid_opening_raise_fixed_limit() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(
            GameType::FixedLimit(convert_to_e8s(1.0), convert_to_e8s(2.0)),
            2,
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

    // Test that raising by more than the fix limit is not allowed
    assert!(table
        .bet(user2.principal_id, BetType::Raised(convert_to_e8s(3.0)))
        .is_err());
}

#[test]
fn test_opening_raise_fixed_limit() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(
            GameType::FixedLimit(convert_to_e8s(1.0), convert_to_e8s(2.0)),
            2,
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

    assert_eq!(
        table.bet(user2.principal_id, BetType::Raised(convert_to_e8s(2.0))),
        Ok(())
    );

    let user2 = table.users.get(&user2.principal_id).unwrap();
    assert_eq!(user2.balance, convert_to_e8s(98.0));
    assert_eq!(
        table
            .user_table_data
            .get(&user2.principal_id)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(2.0)
    );
}

#[test]
fn test_opening_call_fixed_limit() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(
            GameType::FixedLimit(convert_to_e8s(1.0), convert_to_e8s(2.0)),
            2,
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

    assert_eq!(table.bet(user2.principal_id, BetType::Called), Ok(()));

    let user2 = table.users.get(&user2.principal_id).unwrap();

    assert_eq!(user2.balance, convert_to_e8s(99.0));
    // No need to check user2 table data for current total bet as the bets have been confirmed
    // and moved to the pot.
}

#[test]
fn test_raise_amount_validation_small_bet() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(
            GameType::FixedLimit(convert_to_e8s(1.0), convert_to_e8s(2.0)),
            2,
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
    assert_eq!(table.bet(user2.principal_id, BetType::Called), Ok(()));
    assert!(table.user_check(user1.principal_id, false).is_ok());

    // Raise with valid small bet amount
    assert_eq!(
        table.bet(user1.principal_id, BetType::Raised(convert_to_e8s(1.0))),
        Ok(())
    );
    assert_eq!(
        table
            .user_table_data
            .get(&user1.principal_id)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(1.0)
    );

    // Attempt to raise with an invalid amount (not matching small bet)
    assert!(table
        .bet(user2.principal_id, BetType::Raised(convert_to_e8s(0.5)))
        .is_err());
}

#[test]
fn test_raise_amount_validation_big_bet() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(
            GameType::FixedLimit(convert_to_e8s(1.0), convert_to_e8s(2.0)),
            2,
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
    assert_eq!(table.bet(user2.principal_id, BetType::Called), Ok(()));
    assert!(table.user_check(user1.principal_id, false).is_ok());

    // Move to turn or river stage to test big bet amount
    table.set_deal_stage(DealStage::River);

    // Raise with valid big bet amount
    assert_eq!(
        table.bet(user2.principal_id, BetType::Raised(convert_to_e8s(2.0))),
        Ok(())
    );
    assert_eq!(
        table
            .user_table_data
            .get(&user2.principal_id)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(2.0)
    );

    // Attempt to raise with an invalid amount (not matching big bet)
    assert!(table
        .bet(user1.principal_id, BetType::Raised(convert_to_e8s(1.5)))
        .is_err());
}

#[test]
fn test_raise_amount_validation_different_stages() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(
            GameType::FixedLimit(convert_to_e8s(1.0), convert_to_e8s(2.0)),
            2,
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
    assert_eq!(table.bet(user2.principal_id, BetType::Called), Ok(()));
    assert!(table.user_check(user1.principal_id, false).is_ok());

    println!("Pot: {}", table.pot);
    println!("User1 balance: {}", user1.balance);
    println!("User2 balance: {}", user2.balance);

    // Opening and Flop stages should allow small bet raises
    assert_eq!(
        table.bet(user2.principal_id, BetType::Raised(convert_to_e8s(1.0))),
        Ok(())
    );
    assert_eq!(
        table
            .user_table_data
            .get(&user2.principal_id)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(1.0)
    );

    // Move to Turn stage
    table.set_deal_stage(DealStage::Turn);

    // Turn and River stages should allow big bet raises
    assert!(table
        .bet(user1.principal_id, BetType::Raised(convert_to_e8s(2.0)))
        .is_ok());
    assert_eq!(
        table
            .user_table_data
            .get(&user1.principal_id)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(2.0)
    );
}

#[test]
fn test_raise_amount_validation_fixed_limit() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(
            GameType::FixedLimit(convert_to_e8s(1.0), convert_to_e8s(2.0)),
            2,
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
    assert_eq!(table.bet(user2.principal_id, BetType::Called), Ok(()));
    assert!(table.user_check(user1.principal_id, false).is_ok());

    // Opening stage should allow small bet raises
    assert!(table
        .bet(user2.principal_id, BetType::Raised(convert_to_e8s(1.0)))
        .is_ok());
    assert_eq!(
        table
            .user_table_data
            .get(&user2.principal_id)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(1.0)
    );

    // Attempt to raise with invalid amount during opening stage
    assert!(table
        .bet(user1.principal_id, BetType::Raised(convert_to_e8s(3.0)))
        .is_err());

    // Move to Turn stage
    table.set_deal_stage(DealStage::Turn);

    // Turn stage should allow big bet raises
    assert!(table
        .bet(user1.principal_id, BetType::Raised(convert_to_e8s(2.0)))
        .is_ok());
    assert_eq!(
        table
            .user_table_data
            .get(&user1.principal_id)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(2.0)
    );

    // Attempt to raise with invalid amount during turn stage
    assert!(table
        .bet(user2.principal_id, BetType::Raised(convert_to_e8s(1.0)))
        .is_err());
}

#[test]
fn test_check_action_when_no_raises_opening_stage() {
    let mut table = Table::new(
        Principal::anonymous(),
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
    let mut other_uid = Principal::anonymous();
    for uid in table.seats.iter() {
        if let SeatStatus::Occupied(uid) = uid {
            if uid != &big_blind_uid && uid != &small_blind_uid {
                other_uid = *uid;
            }
        }
    }
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));
    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(table.user_check(user1.principal_id, false), Ok(()));
}

#[test]
fn test_check_action_when_no_raises_flop_stage() {
    let mut table = Table::new(
        Principal::anonymous(),
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
    let mut other_uid = Principal::anonymous();
    for uid in table.seats.iter() {
        if let SeatStatus::Occupied(uid) = uid {
            if uid != &big_blind_uid && uid != &small_blind_uid {
                other_uid = *uid;
            }
        }
    }

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));
    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));
    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    // Move to flop stage
    table.set_deal_stage(DealStage::Flop);

    // Ensure that all users can check when no raises have been made in the flop stage
    assert!(table.user_check(user3.principal_id, false).is_ok());

    assert_eq!(
        table
            .user_table_data
            .get(&user3.principal_id)
            .unwrap()
            .player_action,
        PlayerAction::Checked
    );

    assert!(table.user_check(user1.principal_id, false).is_ok());

    assert_eq!(
        table
            .user_table_data
            .get(&user1.principal_id)
            .unwrap()
            .player_action,
        PlayerAction::Checked
    );

    assert_eq!(table.deal_stage, DealStage::Flop);
    assert!(table.user_check(user2.principal_id, false).is_ok());
    assert_eq!(table.deal_stage, DealStage::Turn);
}

#[test]
fn test_check_action_when_no_raises_turn_stage() {
    let mut table = Table::new(
        Principal::anonymous(),
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
    let mut other_uid = Principal::anonymous();
    for uid in table.seats.iter() {
        if let SeatStatus::Occupied(uid) = uid {
            if uid != &big_blind_uid && uid != &small_blind_uid {
                other_uid = *uid;
            }
        }
    }

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));
    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));
    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    // Move to turn stage
    table.set_deal_stage(DealStage::Turn);

    // Ensure that all users can check when no raises have been made in the turn stage
    assert!(table.user_check(user3.principal_id, false).is_ok());
    assert_eq!(
        table
            .user_table_data
            .get(&user3.principal_id)
            .unwrap()
            .player_action,
        PlayerAction::Checked
    );

    assert!(table.user_check(user1.principal_id, false).is_ok());
    assert_eq!(
        table
            .user_table_data
            .get(&user1.principal_id)
            .unwrap()
            .player_action,
        PlayerAction::Checked
    );

    assert!(table.user_check(user2.principal_id, false).is_ok());
    assert_eq!(table.deal_stage, DealStage::River);
}

#[test]
fn test_check_action_when_no_raises_river_stage() {
    let mut table = Table::new(
        Principal::anonymous(),
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
    let mut other_uid = Principal::anonymous();
    for uid in table.seats.iter() {
        if let SeatStatus::Occupied(uid) = uid {
            if uid != &big_blind_uid && uid != &small_blind_uid {
                other_uid = *uid;
            }
        }
    }

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));
    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));
    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    // Move to river stage
    table.set_deal_stage(DealStage::River);

    // Ensure that all users can check when no raises have been made in the river stage
    assert!(table.user_check(user3.principal_id, false).is_ok());
    assert_eq!(
        table
            .user_table_data
            .get(&user3.principal_id)
            .unwrap()
            .player_action,
        PlayerAction::Checked
    );

    assert!(table.user_check(user1.principal_id, false).is_ok());
    assert_eq!(
        table
            .user_table_data
            .get(&user1.principal_id)
            .unwrap()
            .player_action,
        PlayerAction::Checked
    );

    assert!(table.user_check(user2.principal_id, false).is_ok());
    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn test_multiple_raises_in_opening_round_two_users() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(
            GameType::FixedLimit(convert_to_e8s(1.0), convert_to_e8s(2.0)),
            2,
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

    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let small_blind_uid = table.get_small_blind_user_principal().unwrap();

    // Small blind calls
    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(2.0)),),
        Ok(())
    );

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

    println!("Big Blind: {}", big_blind_uid);
    println!("Small Blind: {}", small_blind_uid);
    // Small blind calls the re-raise
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));
    assert_eq!(table.pot, convert_to_e8s(6.0));
}

#[test]
fn test_multiple_raises_in_opening_round() {
    let mut table = Table::new(
        Principal::anonymous(),
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
    let mut other_uid = Principal::anonymous();
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

    // Small blind calls the re-raise
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));
    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(3.0)
    );

    println!("Big Blind: {}", big_blind_uid);
    println!("Small Blind: {}", small_blind_uid);
    println!("Other: {}", other_uid);
    // Other user calls the re-raise
    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(table.pot, convert_to_e8s(9.0));
}

#[test]
fn test_multiple_raises_within_limit_in_opening_round() {
    let mut table = Table::new(
        Principal::anonymous(),
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
    let mut other_uid = Principal::anonymous();
    for uid in table.seats.iter() {
        if let SeatStatus::Occupied(uid) = uid {
            if uid != &big_blind_uid && uid != &small_blind_uid {
                other_uid = *uid;
            }
        }
    }

    println!("Big Blind: {}", big_blind_uid);
    println!("Small Blind: {}", small_blind_uid);
    println!("Other: {}", other_uid);
    println!("User1 id: {}", user1.principal_id);
    println!("User2 id: {}", user2.principal_id);
    println!("User3 id: {}", user3.principal_id);

    println!(
        "User1 amount: {} {:?}",
        user1.balance,
        table
            .user_table_data
            .get(&user1.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User2 amount: {} {:?}",
        user2.balance,
        table
            .user_table_data
            .get(&user2.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User3 amount: {} {:?}",
        user3.balance,
        table
            .user_table_data
            .get(&user3.principal_id)
            .unwrap()
            .player_action
    );
    println!("Pot: {}", table.pot);
    println!("Deal stage: {:?}", table.deal_stage);
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

    println!(
        "User1 amount: {} {:?}",
        user1.balance,
        table
            .user_table_data
            .get(&user1.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User2 amount: {} {:?}",
        user2.balance,
        table
            .user_table_data
            .get(&user2.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User3 amount: {} {:?}",
        user3.balance,
        table
            .user_table_data
            .get(&user3.principal_id)
            .unwrap()
            .player_action
    );
    println!("Pot: {}", table.pot);
    println!("Deal stage: {:?}", table.deal_stage);
    // Small blind calls
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    println!(
        "User1 amount: {} {:?}",
        user1.balance,
        table
            .user_table_data
            .get(&user1.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User2 amount: {} {:?}",
        user2.balance,
        table
            .user_table_data
            .get(&user2.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User3 amount: {} {:?}",
        user3.balance,
        table
            .user_table_data
            .get(&user3.principal_id)
            .unwrap()
            .player_action
    );
    println!("Pot: {}", table.pot);
    println!("Deal stage: {:?}", table.deal_stage);
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

    println!(
        "User1 amount: {} {:?}",
        user1.balance,
        table
            .user_table_data
            .get(&user1.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User2 amount: {} {:?}",
        user2.balance,
        table
            .user_table_data
            .get(&user2.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User3 amount: {} {:?}",
        user3.balance,
        table
            .user_table_data
            .get(&user3.principal_id)
            .unwrap()
            .player_action
    );
    println!("Pot: {}", table.pot);
    println!("Deal stage: {:?}", table.deal_stage);
    // Other user calls the second re-raise
    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));
    // assert_eq!(table.user_table_data.get(&other_uid).unwrap().current_total_bet, 3.0);

    println!(
        "User1 amount: {} {:?}",
        user1.balance,
        table
            .user_table_data
            .get(&user1.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User2 amount: {} {:?}",
        user2.balance,
        table
            .user_table_data
            .get(&user2.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User3 amount: {} {:?}",
        user3.balance,
        table
            .user_table_data
            .get(&user3.principal_id)
            .unwrap()
            .player_action
    );
    println!("Pot: {}", table.pot);
    println!("Deal stage: {:?}", table.deal_stage);
    // Small blind calls
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    println!(
        "User1 amount: {} {:?}",
        user1.balance,
        table
            .user_table_data
            .get(&user1.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User2 amount: {} {:?}",
        user2.balance,
        table
            .user_table_data
            .get(&user2.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User3 amount: {} {:?}",
        user3.balance,
        table
            .user_table_data
            .get(&user3.principal_id)
            .unwrap()
            .player_action
    );
    println!("Pot: {}", table.pot);
    println!("Deal stage: {:?}", table.deal_stage);
    // Big blind calls the second re-raise
    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));
    assert_eq!(table.pot, convert_to_e8s(9.0));
}

#[test]
fn test_multiple_raises_within_limit_in_opening_round_small_blind_immediate_raise() {
    let mut table = Table::new(
        Principal::anonymous(),
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
    let mut other_uid = Principal::anonymous();
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

    // Big blind re-raises
    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(3.0)),),
        Ok(())
    );
    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(3.0)
    );

    // Big blind re-raises
    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));
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

    assert_eq!(table.pot, convert_to_e8s(9.0));
}

#[test]
fn test_correct_pot_calculation() {
    let mut table = Table::new(
        Principal::anonymous(),
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
    let mut other_uid = Principal::anonymous();
    for uid in table.seats.iter() {
        if let SeatStatus::Occupied(uid) = uid {
            if uid != &big_blind_uid && uid != &small_blind_uid {
                other_uid = *uid;
            }
        }
    }

    println!("Big Blind: {}", big_blind_uid);
    println!("Small Blind: {}", small_blind_uid);
    println!("Other: {}", other_uid);
    println!("User1 id: {}", user1.principal_id);
    println!("User2 id: {}", user2.principal_id);
    println!("User3 id: {}", user3.principal_id);

    println!(
        "User1 amount: {} {:?}",
        user1.balance,
        table
            .user_table_data
            .get(&user1.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User2 amount: {} {:?}",
        user2.balance,
        table
            .user_table_data
            .get(&user2.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User3 amount: {} {:?}",
        user3.balance,
        table
            .user_table_data
            .get(&user3.principal_id)
            .unwrap()
            .player_action
    );
    println!("Pot: {}", table.pot);
    println!("Deal stage: {:?}", table.deal_stage);
    assert_eq!(
        table.bet(other_uid, BetType::Raised(convert_to_e8s(2.0))),
        Ok(())
    );
    println!(
        "User1 amount: {} {:?}",
        user1.balance,
        table
            .user_table_data
            .get(&user1.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User2 amount: {} {:?}",
        user2.balance,
        table
            .user_table_data
            .get(&user2.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User3 amount: {} {:?}",
        user3.balance,
        table
            .user_table_data
            .get(&user3.principal_id)
            .unwrap()
            .player_action
    );
    println!("Pot: {}", table.pot);
    println!("Deal stage: {:?}", table.deal_stage);

    // Small blind calls
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    println!(
        "User1 amount: {} {:?}",
        user1.balance,
        table
            .user_table_data
            .get(&user1.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User2 amount: {} {:?}",
        user2.balance,
        table
            .user_table_data
            .get(&user2.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User3 amount: {} {:?}",
        user3.balance,
        table
            .user_table_data
            .get(&user3.principal_id)
            .unwrap()
            .player_action
    );
    println!("Pot: {}", table.pot);
    println!("Deal stage: {:?}", table.deal_stage);
    // Big blind calls
    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    println!(
        "User1 amount: {} {:?}",
        user1.balance,
        table
            .user_table_data
            .get(&user1.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User2 amount: {} {:?}",
        user2.balance,
        table
            .user_table_data
            .get(&user2.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User3 amount: {} {:?}",
        user3.balance,
        table
            .user_table_data
            .get(&user3.principal_id)
            .unwrap()
            .player_action
    );
    println!("Pot: {}", table.pot);
    println!("Deal stage: {:?}", table.deal_stage);
    // Small blind calls the raise
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    println!(
        "User1 amount: {} {:?}",
        user1.balance,
        table
            .user_table_data
            .get(&user1.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User2 amount: {} {:?}",
        user2.balance,
        table
            .user_table_data
            .get(&user2.principal_id)
            .unwrap()
            .player_action
    );
    println!(
        "User3 amount: {} {:?}",
        user3.balance,
        table
            .user_table_data
            .get(&user3.principal_id)
            .unwrap()
            .player_action
    );
    println!("Pot: {}", table.pot);
    println!("Deal stage: {:?}", table.deal_stage);

    // The pot should now be 0.5 (small blind) + 1.0 (big blind) + 0.5 (small blind calls) +
    // 2.0 (other user raise) + 1.0 (big blind calls) + 1.0 (small blind calls the raise)
    assert_eq!(table.pot, convert_to_e8s(6.0));
}

#[test]
fn test_pot_calculation_with_folds() {
    let mut table = Table::new(
        Principal::anonymous(),
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
    let mut other_uid = Principal::anonymous();
    for uid in table.seats.iter() {
        if let SeatStatus::Occupied(uid) = uid {
            if uid != &big_blind_uid && uid != &small_blind_uid {
                other_uid = *uid;
            }
        }
    }

    // Small blind calls
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));
    // Other user raises
    assert_eq!(
        table.bet(other_uid, BetType::Raised(convert_to_e8s(2.0))),
        Ok(())
    );
    // Big blind folds
    assert!(table.user_fold(big_blind_uid, false).is_ok());
    // Small blind calls the raise
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    // The pot should now be 0.5 (small blind) + 1.0 (big blind) + 0.5 (small blind calls) + 1.0 (other user raise) + 1.0 (small blind calls the raise)
    // Big blind's bet is not added since it folded
    assert_eq!(table.pot, convert_to_e8s(5.0));
}

#[test]
fn test_river_stage_double_raise_fixed_limit() {
    let mut table = Table::new(
        Principal::anonymous(),
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

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let mut other_uid = Principal::anonymous();
    for uid in table.seats.iter() {
        if let SeatStatus::Occupied(uid) = uid {
            if uid != &big_blind_uid && uid != &small_blind_uid {
                other_uid = *uid;
            }
        }
    }

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());
    assert!(table.bet(other_uid, BetType::Called).is_ok());
    assert!(table.bet(small_blind_uid, BetType::Called).is_ok());
    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    // Move to river stage
    table.set_deal_stage(DealStage::River);

    // User1 raises by 2
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

    // User2 raises on top by 2 (total bet 4.0, but should only be 2.0 due to fixed limit)
    let result = table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(4.0)));
    assert_eq!(result, Ok(()));
}

#[test]
fn test_fixed_limit_user_below_minimum() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(
            GameType::FixedLimit(convert_to_e8s(1.0), convert_to_e8s(2.0)),
            2,
        ),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(2.5),
    ); // Insufficient balance for calling
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    // User2 raises
    assert_eq!(table.bet(user2.principal_id, BetType::Called), Ok(()));

    assert!(table.bet(user1.principal_id, BetType::Called).is_ok());

    assert_eq!(table.deal_stage, DealStage::Turn);

    // User2 raises
    assert_eq!(
        table.bet(user2.principal_id, BetType::Raised(convert_to_e8s(1.0))),
        Ok(())
    );

    assert!(table.bet(user1.principal_id, BetType::Called).is_ok());

    assert_eq!(table.deal_stage, DealStage::Showdown);
}
