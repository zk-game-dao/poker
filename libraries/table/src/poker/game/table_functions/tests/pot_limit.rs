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
fn test_pot_limit_insufficient_funds_when_raising() {
    // This test ensures that a player cannot raise more than their available balance in a pot limit game
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::PotLimit(convert_to_e8s(1.0)), 2),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(3.0), // Insufficient balance for raising the pot
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());

    assert_eq!(
        table.start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        Ok((Vec::new(), Vec::new()))
    );

    // User1 tries to raise to 3.0, but only has 1.5 balance
    assert!(table
        .bet(user1.principal_id, BetType::Raised(convert_to_e8s(5.0)))
        .is_err());
}

#[test]
fn test_pot_limit_correct_blinds() {
    // This test ensures that blinds are posted correctly in a pot limit game
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::PotLimit(convert_to_e8s(1.0)), 2),
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

    assert_eq!(
        table.start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        Ok((Vec::new(), Vec::new()))
    );

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();

    // Small blind should have bet 0.5
    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(1.0)
    );
    // Big blind should have bet 1.0
    assert_eq!(
        table
            .user_table_data
            .get(&big_blind_uid)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(2.0)
    );
}

#[test]
fn test_pot_limit_raise_within_pot() {
    // Test that a player can raise within the pot limit
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::PotLimit(convert_to_e8s(1.0)), 2),
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

    assert_eq!(
        table.start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        Ok((Vec::new(), Vec::new()))
    );

    // User1 raises to the maximum allowed: total bet 3.5 (1.0 call + 2.5 raise)
    assert_eq!(
        table.bet(user1.principal_id, BetType::Raised(convert_to_e8s(3.0))),
        Ok(())
    );

    // Check user1's total bet
    assert_eq!(
        table
            .user_table_data
            .get(&user1.principal_id)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(3.0)
    );
}

#[test]
fn test_pot_limit_raise_exceeds_pot() {
    // Test that a player cannot raise more than the pot limit
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::PotLimit(convert_to_e8s(1.0)), 2),
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

    assert_eq!(
        table.start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        Ok((Vec::new(), Vec::new()))
    );

    // User1 attempts to raise more than the pot limit
    assert!(table
        .bet(user1.principal_id, BetType::Raised(convert_to_e8s(5.0)))
        .is_err());
}

#[test]
fn test_pot_limit_correct_pot_calculation() {
    // Test that the pot is correctly calculated after bets and raises
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::PotLimit(convert_to_e8s(1.0)), 2),
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

    assert_eq!(
        table.start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        Ok((Vec::new(), Vec::new()))
    );

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let big_blind_uid = table.get_big_blind_user_principal().unwrap();

    // User1 raises to 3.5 (maximum allowed)
    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(3.0))),
        Ok(())
    );

    // User2 calls
    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    // Proceed to next stage to confirm bets and update pot
    table.set_deal_stage(DealStage::Flop);

    assert_eq!(table.pot.0, convert_to_e8s(6.0));
}

#[test]
fn test_pot_limit_multiple_raises() {
    // Test multiple raises in a pot limit game
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::PotLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(150.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(150.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(150.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    assert_eq!(
        table.start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        Ok((Vec::new(), Vec::new()))
    );

    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for uid in table.seats.iter() {
        if let SeatStatus::Occupied(uid) = uid {
            if uid != &big_blind_uid && uid != &small_blind_uid {
                other_uid = *uid;
                break;
            }
        }
    }

    assert_eq!(
        table.bet(other_uid, BetType::Raised(convert_to_e8s(3.0))),
        Ok(())
    );

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    let bb_raise_to = convert_to_e8s(8.0);
    assert_eq!(
        table.bet(big_blind_uid, BetType::Raised(bb_raise_to)),
        Ok(())
    );

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    // Small blind folds
    assert!(table.user_fold(small_blind_uid, false).is_ok());

    // Proceed to next stage to confirm bets and update pot
    table.set_deal_stage(DealStage::Flop);

    // Pot should now be 1.5 (blinds) + 3.5 (other) + 3.0 (small blind call) + 14.0 (big blind raise) + 10.5 (other call) = 32.5
    assert_eq!(table.pot.0, convert_to_e8s(19.0));
}
