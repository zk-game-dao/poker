use candid::Principal;
use user::user::WalletPrincipalId;

use crate::poker::game::{
    table_functions::{
        table::{Table, TableId},
        tests::{create_user, get_table_config, turn_tests::is_it_users_turn},
        types::{BetType, DealStage, SeatStatus},
    },
    types::GameType,
    utils::convert_to_e8s,
};

#[test]
fn test_pre_flop_betting_heads_up() {
    // Initialize table with 2 players
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 2),
        vec![1, 2],
    );

    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(50.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());

    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let small_blind_uid = table.get_small_blind_user_principal().unwrap();

    // Start betting round
    assert!(table.start_betting_round(vec![0, 1]).is_ok());

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, small_blind_uid));
    // Player 1 (Small Blind) calls $1 to complete to $2
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, big_blind_uid));
    // Player 2 (Big Blind) checks
    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    // The hand proceeds to the flop
    assert_eq!(table.deal_stage, DealStage::Turn);
}

#[test]
fn test_pre_flop_betting_with_raises_three_players() {
    // Initialize table with 3 players
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3],
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

    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let mut button_uid = WalletPrincipalId(Principal::anonymous());
    for uid in table.seats.iter() {
        if let SeatStatus::Occupied(uid) = uid {
            if uid != &big_blind_uid && uid != &small_blind_uid {
                button_uid = *uid;
                break;
            }
        }
    }
    // Start betting round
    assert!(table.start_betting_round(vec![0, 1, 2]).is_ok());

    // Player 3 (Button) raises to $5
    assert_eq!(
        table.bet(button_uid, BetType::Raised(convert_to_e8s(5.0))),
        Ok(())
    );

    // Player 1 (Small Blind) folds
    assert_eq!(table.user_fold(small_blind_uid, false), Ok(()));

    // Player 2 (Big Blind) re-raises to $10
    assert_eq!(
        table.bet(big_blind_uid, BetType::Raised(convert_to_e8s(10.0))),
        Ok(())
    );

    // Player 3 calls $5
    assert_eq!(table.bet(button_uid, BetType::Called), Ok(()));

    // Verify pot size
    assert_eq!(table.pot.0, convert_to_e8s(21.0)); // 10 + 10 + 1 from small blind

    // Move to flop
    assert_eq!(table.deal_stage, DealStage::Turn);
}
