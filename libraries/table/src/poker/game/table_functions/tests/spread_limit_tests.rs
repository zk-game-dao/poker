use candid::Principal;
use user::user::WalletPrincipalId;

use crate::poker::game::{
    table_functions::{
        table::{Table, TableId},
        tests::{create_user, get_table_config},
        types::{BetType, DealStage, SeatStatus},
    },
    types::GameType,
    utils::convert_to_e8s,
};

#[test]
fn test_spread_limit_insufficient_funds_when_raising() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(
            GameType::SpreadLimit(convert_to_e8s(1.0), convert_to_e8s(4.0)),
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
fn test_correct_blinds_spread_limit() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(
            GameType::SpreadLimit(convert_to_e8s(1.0), convert_to_e8s(5.0)),
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
fn test_invalid_opening_raise_spread_limit() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(
            GameType::SpreadLimit(convert_to_e8s(1.0), convert_to_e8s(5.0)),
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

    // Test that raising by more than the max limit is not allowed
    assert!(table
        .bet(user2.principal_id, BetType::Raised(convert_to_e8s(7.0)))
        .is_err());
}

#[test]
fn test_opening_raise_spread_limit() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(
            GameType::SpreadLimit(convert_to_e8s(1.0), convert_to_e8s(5.0)),
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
    assert_eq!(user2.balance.0, convert_to_e8s(98.0));
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
fn test_opening_call_spread_limit() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(
            GameType::SpreadLimit(convert_to_e8s(1.0), convert_to_e8s(5.0)),
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
    assert_eq!(user2.balance.0, convert_to_e8s(99.0));
}

#[test]
fn test_raise_amount_validation_spread_limit() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(
            GameType::SpreadLimit(convert_to_e8s(1.0), convert_to_e8s(5.0)),
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

    // Raise with valid amount within spread limit
    assert_eq!(
        table.bet(user2.principal_id, BetType::Raised(convert_to_e8s(3.0))),
        Ok(())
    );
    assert_eq!(
        table
            .user_table_data
            .get(&user2.principal_id)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(3.0)
    );

    // Attempt to raise with an invalid amount (exceeding spread limit)
    assert!(table
        .bet(user1.principal_id, BetType::Raised(convert_to_e8s(16.0)))
        .is_err());

    // Attempt to raise with an invalid amount (below spread limit)
    assert!(table
        .bet(user1.principal_id, BetType::Raised(convert_to_e8s(0.5)))
        .is_err());
}

#[test]
fn test_mutliple_raise_amount_validation_spread_limit() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(
            GameType::SpreadLimit(convert_to_e8s(1.0), convert_to_e8s(5.0)),
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

    // Raise with valid amount within spread limit
    assert_eq!(
        table.bet(user2.principal_id, BetType::Raised(convert_to_e8s(3.0))),
        Ok(())
    );
    assert_eq!(
        table
            .user_table_data
            .get(&user2.principal_id)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(3.0)
    );

    // Attempt to raise with an invalid amount (exceeding spread limit)
    assert_eq!(
        table.bet(user1.principal_id, BetType::Raised(convert_to_e8s(6.0))),
        Ok(())
    );

    // Attempt to raise with an invalid amount (below spread limit)
    assert!(table
        .bet(user1.principal_id, BetType::Raised(convert_to_e8s(0.5)))
        .is_err());
}

#[test]
fn test_check_action_when_no_raises_spread_limit() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(
            GameType::SpreadLimit(convert_to_e8s(1.0), convert_to_e8s(5.0)),
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

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));
    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(table.user_check(user1.principal_id, false), Ok(()));
}

#[test]
fn test_correct_pot_calculation_spread_limit() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(
            GameType::SpreadLimit(convert_to_e8s(1.0), convert_to_e8s(5.0)),
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
    // Small blind calls
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));
    // Big blind calls
    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    // The pot should now be 0.5 (small blind) + 1.0 (big blind) + 0.5 (small blind calls) +
    // 2.0 (other user raise) + 1.0 (big blind calls) + 1.0 (small blind calls the raise)
    assert_eq!(table.pot.0, convert_to_e8s(6.0));
}

#[test]
fn test_pot_calculation_with_folds_spread_limit() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(
            GameType::SpreadLimit(convert_to_e8s(1.0), convert_to_e8s(5.0)),
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

    // The pot should now be 0.5 (small blind) + 1.0 (big blind) + 0.5 (small blind calls) + 2.0 (other user raise) + 1.0 (small blind calls the raise)
    // Big blind's bet is not added since it folded
    assert_eq!(table.pot.0, convert_to_e8s(5.0));
}

#[test]
fn test_spread_limit_user_below_minimum() {
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(
            GameType::SpreadLimit(convert_to_e8s(1.0), convert_to_e8s(5.0)),
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
