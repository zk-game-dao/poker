use candid::Principal;

use crate::poker::game::{
    table_functions::{
        table::Table,
        tests::{create_user, get_table_config, turn_tests::is_it_users_turn},
        types::{BetType, DealStage, PlayerAction, SeatStatus},
    },
    types::GameType,
    utils::convert_to_e8s,
};

#[test]
fn test_all_in_basic() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1, 0, false).is_ok());
    assert!(table.add_user(user2, 1, false).is_ok());

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    let big_blind_uid: Principal = table.get_big_blind_user_principal().unwrap();
    let small_blind_uid = table.get_small_blind_user_principal().unwrap();

    // Small blind re-raises
    assert!(is_it_users_turn(&table, small_blind_uid));
    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(100.0)),),
        Ok(())
    );
    assert_eq!(table.deal_stage, DealStage::Flop);

    println!("Table deal stage {:?}", table.deal_stage);

    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(100.0)
    );

    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );

    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn test_all_in_one_player_larger_balance() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1, 0, false).is_ok());
    assert!(table.add_user(user2, 1, false).is_ok());

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let small_blind_uid = table.get_small_blind_user_principal().unwrap();

    table.users.get_mut(&big_blind_uid).unwrap().balance = convert_to_e8s(200.0);

    // Big blind re-raises
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

    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn test_multiple_users_all_in_basic() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8],
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

    // Big blind re-raises
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

    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));

    assert_eq!(
        table
            .user_table_data
            .get(&other_uid)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(100.0)
    );

    assert_eq!(
        table.user_table_data.get(&other_uid).unwrap().player_action,
        PlayerAction::AllIn
    );

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn test_multiple_users_all_in_basic_one_folds() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8],
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

    // Big blind re-raises
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

    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert_eq!(table.user_fold(other_uid, false), Ok(()));

    assert_eq!(
        table.user_table_data.get(&other_uid).unwrap().player_action,
        PlayerAction::Folded
    );

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn test_all_in_basic_current_player_index() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1, 0, false).is_ok());
    assert!(table.add_user(user2, 1, false).is_ok());

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let small_blind_uid = table.get_small_blind_user_principal().unwrap();

    // Big blind re-raises
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

    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );

    assert_eq!(table.current_player_index, 0);

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn test_all_in_multiple_all_ins() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3, 4, 5, 6, 7, 8],
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

    println!("---------------------------------------");
    println!("Big blind id: {}", big_blind_uid);
    println!("Small blind id: {}", small_blind_uid);
    println!("Other id: {}", other_uid);
    println!("---------------------------------------");

    table.users.get_mut(&small_blind_uid).unwrap().balance = convert_to_e8s(200.0);

    // Big blind re-raises
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(2.0)
    );

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert_eq!(table.user_fold(other_uid, false), Ok(()));

    assert_eq!(
        table.user_table_data.get(&other_uid).unwrap().player_action,
        PlayerAction::Folded
    );

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(200.0)),),
        Ok(())
    );

    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .current_total_bet,
        convert_to_e8s(200.0)
    );

    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );

    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));
}

#[test]
fn test_all_in_opening() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 4),
        vec![1, 2, 3, 4, 5, 6, 7, 8],
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
        Principal::self_authenticating("test4"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1, 0, false).is_ok());
    assert!(table.add_user(user2, 1, false).is_ok());
    assert!(table.add_user(user3, 2, false).is_ok());
    assert!(table.add_user(user4, 3, false).is_ok());

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let dealer_uid = if let SeatStatus::Occupied(user) = table.seats[table.dealer_position] {
        user
    } else {
        panic!("Dealer position is not occupied");
    };
    let mut other_uid = Principal::anonymous();
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if *user_id != big_blind_uid && *user_id != small_blind_uid && *user_id != dealer_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    println!("---------------------------------------");
    println!("Big blind id: {}", big_blind_uid);
    println!("Small blind id: {}", small_blind_uid);
    println!("Other id: {}", dealer_uid);
    println!("---------------------------------------");

    assert!(is_it_users_turn(&table, other_uid));
    // Big blind re-raises
    assert_eq!(table.user_fold(other_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, dealer_uid));

    assert_eq!(table.user_fold(dealer_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(
        table.bet(big_blind_uid, BetType::Raised(convert_to_e8s(100.0))),
        Ok(())
    );

    assert_eq!(
        table
            .user_table_data
            .get(&big_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );
    assert_eq!(
        table
            .user_table_data
            .get(&small_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::None
    );

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, small_blind_uid));

    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn test_all_players_all_in_opening() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 4),
        vec![1, 2, 3, 4, 5, 6, 7, 8],
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
        Principal::self_authenticating("test4"),
        convert_to_e8s(100.0),
    );

    assert!(table.add_user(user1, 0, false).is_ok());
    assert!(table.add_user(user2, 1, false).is_ok());
    assert!(table.add_user(user3, 2, false).is_ok());
    assert!(table.add_user(user4, 3, false).is_ok());

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    let dealer_uid = if let SeatStatus::Occupied(user) = table.seats[table.dealer_position] {
        user
    } else {
        panic!("Dealer position is not occupied");
    };
    let mut other_uid = Principal::anonymous();
    for user_id in table.seats.iter() {
        if let SeatStatus::Occupied(user_id) = user_id {
            if *user_id != big_blind_uid && *user_id != small_blind_uid && *user_id != dealer_uid {
                other_uid = *user_id;
                break;
            }
        }
    }

    println!("---------------------------------------");
    println!("Big blind id: {}", big_blind_uid);
    println!("Small blind id: {}", small_blind_uid);
    println!("Dealer id: {}", dealer_uid);
    println!("Other id: {}", other_uid);
    println!("---------------------------------------");

    println!(
        "Current player: {:?}",
        table
            .get_player_at_seat(table.current_player_index)
            .unwrap()
            .to_text()
    );

    assert!(is_it_users_turn(&table, other_uid));
    // Big blind re-raises
    assert_eq!(
        table.bet(other_uid, BetType::Raised(convert_to_e8s(100.0))),
        Ok(())
    );

    assert_eq!(
        table.user_table_data.get(&other_uid).unwrap().player_action,
        PlayerAction::AllIn
    );

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, dealer_uid));

    assert_eq!(table.bet(dealer_uid, BetType::Called), Ok(()));

    assert_eq!(
        table
            .user_table_data
            .get(&dealer_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );

    assert_eq!(table.deal_stage, DealStage::Flop);

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

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, big_blind_uid));

    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Showdown);
}

#[test]
fn test_all_in_with_side_pot_uneven_balances() {
    let mut table = Table::new(
        Principal::anonymous(),
        get_table_config(GameType::NoLimit(1), 4),
        vec![1, 2, 3, 4, 5, 6, 7, 8],
    );
    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        100,
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        100,
    );

    assert!(table.add_user(user1, 0, false).is_ok());
    assert!(table.add_user(user2, 1, false).is_ok());

    assert!(table
        .start_betting_round(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        .is_ok());

    let big_blind_uid = table.get_big_blind_user_principal().unwrap();
    let small_blind_uid = table.get_small_blind_user_principal().unwrap();
    table.users.get_mut(&big_blind_uid).unwrap().balance = 200;

    let user_3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        100,
    );

    table.add_user(user_3, 2, false).unwrap();

    assert!(is_it_users_turn(&table, small_blind_uid));
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, big_blind_uid));
    assert_eq!(table.bet(big_blind_uid, BetType::Raised(200)), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Flop);

    assert!(is_it_users_turn(&table, small_blind_uid));
    assert_eq!(table.bet(small_blind_uid, BetType::Called), Ok(()));

    for user in &table.user_table_data {
        println!("User: {:?}", user.1);
    }

    assert_eq!(table.deal_stage, DealStage::Showdown);
}
