use crate::poker::game::{
    table_functions::{
        ante::AnteType,
        table::Table,
        tests::{create_user, get_table_config, turn_tests::is_it_users_turn},
        types::{BetType, SeatStatus},
    },
    types::GameType,
    utils::convert_to_e8s,
};
use candid::Principal;

fn setup_table_with_ante(ante_type: AnteType) -> Table {
    let mut config = get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3);
    config.ante_type = Some(ante_type);
    Table::new(
        Principal::anonymous(),
        config,
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    )
}

#[test]
fn test_fixed_ante() {
    let mut table = setup_table_with_ante(AnteType::Fixed(convert_to_e8s(0.5)));
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").unwrap(),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").unwrap(),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").unwrap(),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    assert_eq!(
        table.start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        Ok((vec![], vec![]))
    );

    let small_blind = table.get_small_blind_user_principal().unwrap();
    let big_blind = table.get_big_blind_user_principal().unwrap();
    let mut other = Principal::anonymous();
    for user in table.seats.iter() {
        if let SeatStatus::Occupied(principal) = user {
            if principal != &small_blind && principal != &big_blind {
                other = *principal;
            }
        }
    }

    assert!(is_it_users_turn(&table, other));
    assert_eq!(table.bet(other, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, small_blind));
    assert_eq!(table.bet(small_blind, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, big_blind));
    assert_eq!(table.user_check(big_blind, false), Ok(()));

    // Each player should have paid the fixed ante (0.5)
    assert_eq!(table.pot, convert_to_e8s(7.5)); // 3 players * 0.5 ante + SB(1.0) + BB(2.0)

    // Check each player's balance is reduced by ante
    assert_eq!(
        table.users.get(&user1.principal_id).unwrap().balance,
        convert_to_e8s(97.5)
    );
    assert_eq!(
        table.users.get(&user2.principal_id).unwrap().balance,
        convert_to_e8s(97.5)
    );
    assert_eq!(
        table.users.get(&user3.principal_id).unwrap().balance,
        convert_to_e8s(97.5)
    );
}

#[test]
fn test_big_blind_ante() {
    let mut table = setup_table_with_ante(AnteType::BigBlindAnte);
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").unwrap(),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").unwrap(),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").unwrap(),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    assert_eq!(
        table.start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        Ok((vec![], vec![]))
    );

    let small_blind = table.get_small_blind_user_principal().unwrap();
    let big_blind = table.get_big_blind_user_principal().unwrap();
    let mut other = Principal::anonymous();
    for user in table.seats.iter() {
        if let SeatStatus::Occupied(principal) = user {
            if principal != &small_blind && principal != &big_blind {
                other = *principal;
            }
        }
    }

    assert!(is_it_users_turn(&table, other));
    assert_eq!(table.bet(other, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, small_blind));
    assert_eq!(table.bet(small_blind, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, big_blind));
    assert_eq!(table.user_check(big_blind, false), Ok(()));

    // Only dealer pays ante equal to big blind (2.0)
    assert_eq!(table.pot, convert_to_e8s(8.0)); // Dealer ante(2.0) + SB(1.0) + BB(2.0)

    // Check dealer's balance is reduced by big blind ante
    let dealer_principal = table.get_player_at_seat(table.dealer_position).unwrap();
    assert_eq!(
        table.users.get(&dealer_principal).unwrap().balance,
        convert_to_e8s(96.0)
    );
}

#[test]
fn test_percentage_ante() {
    let mut table = setup_table_with_ante(AnteType::PercentageOfBigBlind(50)); // 50% of BB
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").unwrap(),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").unwrap(),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").unwrap(),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    assert_eq!(
        table.start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        Ok((vec![], vec![]))
    );

    let small_blind = table.get_small_blind_user_principal().unwrap();
    let big_blind = table.get_big_blind_user_principal().unwrap();
    let mut other = Principal::anonymous();
    for user in table.seats.iter() {
        if let SeatStatus::Occupied(principal) = user {
            if principal != &small_blind && principal != &big_blind {
                other = *principal;
            }
        }
    }

    assert!(is_it_users_turn(&table, other));
    assert_eq!(table.bet(other, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, small_blind));
    assert_eq!(table.bet(small_blind, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, big_blind));
    assert_eq!(table.user_check(big_blind, false), Ok(()));

    // Each player pays 50% of BB (1.0 each) as ante
    assert_eq!(table.pot, convert_to_e8s(9.0)); // 3 players * 1.0 ante + SB(1.0) + BB(2.0)

    // Check each player's balance is reduced by percentage ante
    assert_eq!(
        table.users.get(&user1.principal_id).unwrap().balance,
        convert_to_e8s(97.0)
    );
    assert_eq!(
        table.users.get(&user2.principal_id).unwrap().balance,
        convert_to_e8s(97.0)
    );
    assert_eq!(
        table.users.get(&user3.principal_id).unwrap().balance,
        convert_to_e8s(97.0)
    );
}

#[test]
fn test_no_ante() {
    let mut table = setup_table_with_ante(AnteType::None);
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").unwrap(),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").unwrap(),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").unwrap(),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    assert_eq!(
        table.start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        Ok((vec![], vec![]))
    );

    let small_blind = table.get_small_blind_user_principal().unwrap();
    let big_blind = table.get_big_blind_user_principal().unwrap();
    let mut other = Principal::anonymous();
    for user in table.seats.iter() {
        if let SeatStatus::Occupied(principal) = user {
            if principal != &small_blind && principal != &big_blind {
                other = *principal;
            }
        }
    }

    assert!(is_it_users_turn(&table, other));
    assert_eq!(table.bet(other, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, small_blind));
    assert_eq!(table.bet(small_blind, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, big_blind));
    assert_eq!(table.user_check(big_blind, false), Ok(()));

    // Only blinds, no ante
    assert_eq!(table.pot, convert_to_e8s(6.0)); // SB(1.0) + BB(2.0)
}
