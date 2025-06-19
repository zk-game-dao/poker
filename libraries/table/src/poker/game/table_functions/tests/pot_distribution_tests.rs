use candid::Principal;
use user::user::WalletPrincipalId;

use crate::poker::{
    core::{Card, Suit, Value},
    game::{
        table_functions::{
            table::{Table, TableId},
            tests::{create_user, get_table_config, turn_tests::is_it_users_turn},
            types::{BetType, DealStage, SeatStatus},
        },
        types::GameType,
        utils::convert_to_e8s,
    },
};

#[test]
fn test_two_players_no_side_pot_with_bets() {
    // Initialize table with 2 players
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 2),
        vec![1, 2],
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

    let player1_uid = table.get_player_at_seat(0).unwrap();
    let player2_uid = table.get_player_at_seat(1).unwrap();

    // Start betting round
    assert_eq!(table.start_betting_round(vec![0, 1]), Ok((vec![], vec![])));

    // Player 1 bets 10
    assert_eq!(
        table.bet(player1_uid, BetType::Raised(convert_to_e8s(10.0))),
        Ok(())
    );

    // Player 2 calls
    assert_eq!(table.bet(player2_uid, BetType::Called), Ok(()));

    // Set community cards: [K♦, 9♠, 5♥, J♠, 3♦]
    table.community_cards = vec![
        Card::new(Value::King, Suit::Diamond),
        Card::new(Value::Nine, Suit::Spade),
        Card::new(Value::Five, Suit::Heart),
        Card::new(Value::Jack, Suit::Spade),
        Card::new(Value::Three, Suit::Diamond),
    ];

    // Set Player 1 hand: [A♠, K♠] (One pair: Kings)
    table.get_user_table_data_mut(player1_uid).unwrap().cards = vec![
        Card::new(Value::Ace, Suit::Spade),
        Card::new(Value::King, Suit::Spade),
    ];

    // Set Player 2 hand: [Q♣, J♦] (One pair: Jacks)
    table.get_user_table_data_mut(player2_uid).unwrap().cards = vec![
        Card::new(Value::Queen, Suit::Club),
        Card::new(Value::Jack, Suit::Diamond),
    ];

    // Perform showdown
    table.showdown().unwrap();

    // Verify that Player 1 receives the correct amount from the pot
    let player1_balance_after = table.users.get(&player1_uid).unwrap().balance;
    assert_eq!(player1_balance_after.0, convert_to_e8s(110.0)); // 100 initial - 10 for bet and 20 won from pot
}

#[test]
fn test_three_players_no_side_pot_with_bets() {
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

    let player1_uid = table.get_player_at_seat(0).unwrap();
    let player2_uid = table.get_player_at_seat(1).unwrap();
    let player3_uid = table.get_player_at_seat(2).unwrap();

    // Start betting round
    assert!(table.start_betting_round(vec![0, 1, 2]).is_ok());

    // Player 1 bets 10
    assert_eq!(
        table.bet(player1_uid, BetType::Raised(convert_to_e8s(10.0))),
        Ok(())
    );

    // Player 2 calls
    assert_eq!(table.bet(player2_uid, BetType::Called), Ok(()));

    // Player 3 calls
    assert_eq!(table.bet(player3_uid, BetType::Called), Ok(()));

    // Set community cards: [Q♠, Q♣, 7♥, 3♦, 9♠]
    table.community_cards = vec![
        Card::new(Value::Queen, Suit::Spade),
        Card::new(Value::Queen, Suit::Club),
        Card::new(Value::Seven, Suit::Heart),
        Card::new(Value::Three, Suit::Diamond),
        Card::new(Value::Nine, Suit::Spade),
    ];

    // Set Player 1 hand: [K♥, Q♦] (Three of a Kind: Queens)
    table.get_user_table_data_mut(player1_uid).unwrap().cards = vec![
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::Queen, Suit::Diamond),
    ];

    // Set Player 2 hand: [9♥, 9♦] (Full House: Nines over Queens)
    table.get_user_table_data_mut(player2_uid).unwrap().cards = vec![
        Card::new(Value::Nine, Suit::Heart),
        Card::new(Value::Nine, Suit::Diamond),
    ];

    // Set Player 3 hand: [A♠, 4♠] (One pair: Queens)
    table.get_user_table_data_mut(player3_uid).unwrap().cards = vec![
        Card::new(Value::Ace, Suit::Spade),
        Card::new(Value::Four, Suit::Spade),
    ];

    // Perform showdown
    table.showdown().unwrap();

    // Verify that Player 2 receives the correct amount from the pot
    let player2_balance_after = table.users.get(&player2_uid).unwrap().balance;
    assert_eq!(player2_balance_after.0, convert_to_e8s(120.0)); // 100 initial - 10 for bet and 30 won from pot
}

#[test]
fn test_four_players_one_side_pot_with_bets() {
    // Initialize table with 4 players
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 4),
        vec![1, 2, 3, 4],
    );

    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(80.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(50.0),
    );
    let user4 = create_user(
        Principal::from_text("bd3sg-teaaa-aaaaa-qaaba-cai").expect("Could not decode principal"),
        convert_to_e8s(40.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());
    assert!(table.add_user(user4.clone(), 3, false).is_ok());

    let player1_uid = table.get_player_at_seat(0).unwrap();
    let player2_uid = table.get_player_at_seat(1).unwrap();
    let player3_uid = table.get_player_at_seat(2).unwrap();
    let player4_uid = table.get_player_at_seat(3).unwrap();

    // Start betting round
    assert!(table.start_betting_round(vec![0, 1, 2, 3]).is_ok());

    // Player 1 bets 20
    assert_eq!(
        table.bet(player1_uid, BetType::Raised(convert_to_e8s(20.0))),
        Ok(())
    );

    assert_eq!(table.deal_stage, DealStage::Flop);

    // Player 2 calls
    assert_eq!(table.bet(player2_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Flop);

    // Player 3 goes all-in with 50
    assert_eq!(
        table.bet(player3_uid, BetType::Raised(convert_to_e8s(50.0))),
        Ok(())
    );

    assert_eq!(table.deal_stage, DealStage::Flop);

    // Player 4 goes all-in with 40
    assert_eq!(table.bet(player4_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Flop);

    // Player 1 calls
    assert_eq!(table.bet(player1_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Flop);

    // Player 2 calls
    assert_eq!(table.bet(player2_uid, BetType::Called), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    // Set community cards: [A♠, 5♦, 10♥, 7♣, 2♦]
    table.community_cards = vec![
        Card::new(Value::Ace, Suit::Spade),
        Card::new(Value::Five, Suit::Diamond),
        Card::new(Value::Ten, Suit::Heart),
        Card::new(Value::Seven, Suit::Club),
        Card::new(Value::Two, Suit::Diamond),
    ];

    // Set Player 1 hand: [A♥, K♣] (One pair: Aces)
    table.get_user_table_data_mut(player1_uid).unwrap().cards = vec![
        Card::new(Value::Ace, Suit::Heart),
        Card::new(Value::King, Suit::Club),
    ];

    // Set Player 2 hand: [10♠, 10♦] (Three of a Kind: Tens)
    table.get_user_table_data_mut(player2_uid).unwrap().cards = vec![
        Card::new(Value::Ten, Suit::Spade),
        Card::new(Value::Ten, Suit::Diamond),
    ];

    // Set Player 3 hand: [K♥, Q♠] (Ace High)
    table.get_user_table_data_mut(player3_uid).unwrap().cards = vec![
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::Queen, Suit::Spade),
    ];

    // Set Player 4 hand: [A♣, 9♣] (One pair: Aces)
    table.get_user_table_data_mut(player4_uid).unwrap().cards = vec![
        Card::new(Value::Ace, Suit::Club),
        Card::new(Value::Nine, Suit::Club),
    ];

    // Perform showdown
    table.showdown().unwrap();

    // Verify that Player 2 receives the correct amount from the main pot and side pot
    let player2_balance_after = table.users.get(&player2_uid).unwrap().balance;
    assert_eq!(player2_balance_after.0, convert_to_e8s(220.0)); // 80 initial - 20 for bet - 30 from bet + 160 won from main pot and + 30 from side pot
}

#[test]
fn test_five_players_multiple_side_pots_with_bets() {
    // Initialize table with 5 players
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(1), 5),
        vec![1, 2, 3, 4, 5],
    );

    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        100,
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        75,
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        50,
    );
    let user4 = create_user(
        Principal::from_text("bd3sg-teaaa-aaaaa-qaaba-cai").expect("Could not decode principal"),
        25,
    );
    let user5 = create_user(
        Principal::from_text("uyxh5-bi3za-gxbfs-op3gj-ere73-a6jhv-5jky3-zawef-b5r2s-k26un-sae")
            .expect("Could not decode principal"),
        25,
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());
    assert!(table.add_user(user4.clone(), 3, false).is_ok());
    assert!(table.add_user(user5.clone(), 4, false).is_ok());

    let player1_uid = table.get_player_at_seat(0).unwrap();
    let player2_uid = table.get_player_at_seat(1).unwrap();
    let player3_uid = table.get_player_at_seat(2).unwrap();
    let player4_uid = table.get_player_at_seat(3).unwrap();
    let player5_uid = table.get_player_at_seat(4).unwrap();

    // Start betting round
    assert!(table.start_betting_round(vec![0, 1, 2, 3, 4]).is_ok());

    // Player 1 bets 20
    assert_eq!(table.bet(player1_uid, BetType::Raised(20)), Ok(()));

    // Player 2 calls
    assert_eq!(table.bet(player2_uid, BetType::Called), Ok(()));

    // Player 3 goes all-in with 50
    assert_eq!(table.bet(player3_uid, BetType::Raised(50)), Ok(()));

    // Player 4 goes all-in with 25
    assert_eq!(table.bet(player4_uid, BetType::Called), Ok(()));

    // Player 5 goes all-in with 25
    assert_eq!(table.bet(player5_uid, BetType::Called), Ok(()));

    // Player 1 calls
    assert_eq!(table.bet(player1_uid, BetType::Called), Ok(()));

    // Player 2 calls
    assert_eq!(table.bet(player2_uid, BetType::Called), Ok(()));

    // Set community cards: [K♠, Q♥, J♦, 3♠, 9♣]
    table.community_cards = vec![
        Card::new(Value::King, Suit::Spade),
        Card::new(Value::Queen, Suit::Heart),
        Card::new(Value::Jack, Suit::Diamond),
        Card::new(Value::Three, Suit::Spade),
        Card::new(Value::Nine, Suit::Club),
    ];

    // Set Player 1 hand: [A♣, 10♦] (Straight: A to 10)
    table.get_user_table_data_mut(player1_uid).unwrap().cards = vec![
        Card::new(Value::Ace, Suit::Club),
        Card::new(Value::Ten, Suit::Diamond),
    ];

    // Set Player 2 hand: [K♦, K♥] (Three of a Kind: Kings)
    table.get_user_table_data_mut(player2_uid).unwrap().cards = vec![
        Card::new(Value::King, Suit::Diamond),
        Card::new(Value::King, Suit::Heart),
    ];

    // Set Player 3 hand: [Q♦, J♠] (Two Pair: Queens and Jacks)
    table.get_user_table_data_mut(player3_uid).unwrap().cards = vec![
        Card::new(Value::Queen, Suit::Diamond),
        Card::new(Value::Jack, Suit::Spade),
    ];

    // Set Player 4 hand: [A♠, 2♠] (Ace High)
    table.get_user_table_data_mut(player4_uid).unwrap().cards = vec![
        Card::new(Value::Ace, Suit::Spade),
        Card::new(Value::Two, Suit::Spade),
    ];

    // Set Player 5 hand: [K♣, 3♦] (Two Pair: Kings and Threes)
    table.get_user_table_data_mut(player5_uid).unwrap().cards = vec![
        Card::new(Value::King, Suit::Club),
        Card::new(Value::Three, Suit::Diamond),
    ];

    // Perform showdown
    table.showdown().unwrap();

    // Verify that Player 1 receives the correct amount from all pots
    let player1_balance_after = table.users.get(&player1_uid).unwrap().balance;
    assert_eq!(player1_balance_after.0, 250); // 100 initial - 50 for bet + 175 won from all pots
}

#[test]
fn test_three_players_one_all_in_multiple_side_pots_with_bets() {
    // Initialize table with 3 players
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 3),
        vec![1, 2, 3],
    );

    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(150.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(50.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());

    let player1_uid = table.get_player_at_seat(0).unwrap();
    let player2_uid = table.get_player_at_seat(1).unwrap();
    let player3_uid = table.get_player_at_seat(2).unwrap();

    // Start betting round
    assert!(table.start_betting_round(vec![0, 1, 2]).is_ok());

    // Player 1 bets 50
    assert_eq!(
        table.bet(player1_uid, BetType::Raised(convert_to_e8s(150.0))),
        Ok(())
    );

    // Player 2 calls
    assert_eq!(table.bet(player2_uid, BetType::Called), Ok(()));

    // Set community cards: [J♦, 9♥, 4♣, K♣, 6♠]
    table.community_cards = vec![Card::new(Value::Ace, Suit::Club)];

    // Set Player 1 hand: [A♦, A♠] (Three of a kind: Aces)
    table.get_user_table_data_mut(player1_uid).unwrap().cards = vec![
        Card::new(Value::Ace, Suit::Diamond),
        Card::new(Value::Ace, Suit::Spade),
    ];

    // Set Player 2 hand: [K♥, Q♠] (High card: King)
    table.get_user_table_data_mut(player2_uid).unwrap().cards = vec![
        Card::new(Value::Five, Suit::Heart),
        Card::new(Value::Two, Suit::Spade),
    ];

    // Set Player 3 hand: [K♦, 10♠] (High card: King)
    table.get_user_table_data_mut(player3_uid).unwrap().cards = vec![
        Card::new(Value::King, Suit::Diamond),
        Card::new(Value::Two, Suit::Spade),
    ];

    // Player 3 goes all-in with 50
    assert_eq!(table.bet(player3_uid, BetType::Called), Ok(()));

    // Verify that Player 1 receives the correct amount from the main pot and side pot
    let player1_balance_after = table.users.get(&player1_uid).unwrap().balance;
    println!("Player 1 balance after: {:?}", player1_balance_after);
    assert!(player1_balance_after.0 >= convert_to_e8s(300.0)); // 100 initial - 50 for bet + 200 won from both pots
}

#[test]
fn test_six_players_three_side_pots_with_bets() {
    // Initialize table with 6 players
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 6),
        vec![1, 2, 3, 4, 5, 6],
    );

    let user1 = create_user(
        Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").expect("Could not decode principal"),
        convert_to_e8s(200.0),
    );
    let user2 = create_user(
        Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").expect("Could not decode principal"),
        convert_to_e8s(150.0),
    );
    let user3 = create_user(
        Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").expect("Could not decode principal"),
        convert_to_e8s(100.0),
    );
    let user4 = create_user(
        Principal::from_text("bd3sg-teaaa-aaaaa-qaaba-cai").expect("Could not decode principal"),
        convert_to_e8s(75.0),
    );
    let user5 = create_user(
        Principal::from_text("uyxh5-bi3za-gxbfs-op3gj-ere73-a6jhv-5jky3-zawef-b5r2s-k26un-sae")
            .expect("Could not decode principal"),
        convert_to_e8s(50.0),
    );
    let user6 = create_user(
        Principal::from_text("3kryc-rcxpr-ojfta-nasyy-tmfwd-73zt5-ajtev-zxwv5-dob3d-ymvow-4qe")
            .expect("Could not decode principal"),
        convert_to_e8s(25.0),
    );

    assert!(table.add_user(user1.clone(), 0, false).is_ok());
    assert!(table.add_user(user2.clone(), 1, false).is_ok());
    assert!(table.add_user(user3.clone(), 2, false).is_ok());
    assert!(table.add_user(user4.clone(), 3, false).is_ok());
    assert!(table.add_user(user5.clone(), 4, false).is_ok());
    assert!(table.add_user(user6.clone(), 5, false).is_ok());

    let player1_uid = table.get_player_at_seat(0).unwrap();
    let player2_uid = table.get_player_at_seat(1).unwrap();
    let player3_uid = table.get_player_at_seat(2).unwrap();
    let player4_uid = table.get_player_at_seat(3).unwrap();
    let player5_uid = table.get_player_at_seat(4).unwrap();
    let player6_uid = table.get_player_at_seat(5).unwrap();

    // Start betting round
    assert!(table.start_betting_round(vec![0, 1, 2, 3, 4, 5]).is_ok());

    // Player 1 bets 50
    assert_eq!(
        table.bet(player1_uid, BetType::Raised(convert_to_e8s(50.0))),
        Ok(())
    );

    // Player 2 raises to 100
    assert_eq!(
        table.bet(player2_uid, BetType::Raised(convert_to_e8s(100.0))),
        Ok(())
    );

    // Player 3 calls
    assert_eq!(table.bet(player3_uid, BetType::Called), Ok(()));

    // Player 4 calls
    assert_eq!(table.bet(player4_uid, BetType::Called), Ok(()));

    // Player 5 calls
    assert_eq!(table.bet(player5_uid, BetType::Called), Ok(()));

    // Player 6 goes all-in with 25
    assert_eq!(table.bet(player6_uid, BetType::Called), Ok(()));

    assert!(is_it_users_turn(&table, player1_uid));

    assert_eq!(table.bet(player1_uid, BetType::Called), Ok(()));

    table.community_cards = vec![
        Card::new(Value::Two, Suit::Diamond),
        Card::new(Value::Jack, Suit::Spade),
        Card::new(Value::Five, Suit::Diamond),
        Card::new(Value::Queen, Suit::Spade),
        Card::new(Value::Ten, Suit::Heart),
    ];

    // Set Player 1 hand: [A♦, K♣] (Straight: 10 to A)
    table.get_user_table_data_mut(player1_uid).unwrap().cards = vec![
        Card::new(Value::Ace, Suit::Diamond),
        Card::new(Value::King, Suit::Club),
    ];

    // Set Player 2 hand: [Q♦, Q♥] (Three of a Kind: Queens)
    table.get_user_table_data_mut(player2_uid).unwrap().cards = vec![
        Card::new(Value::Queen, Suit::Diamond),
        Card::new(Value::Three, Suit::Heart),
    ];

    // Set Player 3 hand: [J♦, 10♦] (Two Pair: Jacks and Tens)
    table.get_user_table_data_mut(player3_uid).unwrap().cards = vec![
        Card::new(Value::Jack, Suit::Diamond),
        Card::new(Value::Two, Suit::Diamond),
    ];

    // Set Player 4 hand: [K♦, 9♠] (One pair: Nines)
    table.get_user_table_data_mut(player4_uid).unwrap().cards = vec![
        Card::new(Value::King, Suit::Diamond),
        Card::new(Value::Four, Suit::Spade),
    ];

    // Set Player 5 hand: [7♣, 6♠] (Straight: 6 to 10)
    table.get_user_table_data_mut(player5_uid).unwrap().cards = vec![
        Card::new(Value::Seven, Suit::Club),
        Card::new(Value::Three, Suit::Spade),
    ];

    // Set Player 6 hand: [Q♣, J♥] (Two Pair: Queens and Jacks)
    table.get_user_table_data_mut(player6_uid).unwrap().cards = vec![
        Card::new(Value::Queen, Suit::Club),
        Card::new(Value::Three, Suit::Heart),
    ];

    // Perform showdown
    table.showdown().unwrap();
    // println!("Sorted Users: {:#?}", table.sorted_users);

    // Verify that Player 1 receives the correct amount from all pots
    let player1_balance_after = table.users.get(&player1_uid).unwrap().balance;
    assert_eq!(player1_balance_after.0, convert_to_e8s(550.0)); // 200 initial - 100 for bet + 450 won from all pots
}

#[test]
fn test_four_players_split_pot_with_bets() {
    // Initialize table with 4 players
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 4),
        vec![1, 2, 3, 4],
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

    let player1_uid = table.get_player_at_seat(0).unwrap();
    let player2_uid = table.get_player_at_seat(1).unwrap();
    let player3_uid = table.get_player_at_seat(2).unwrap();
    let player4_uid = table.get_player_at_seat(3).unwrap();

    // Start betting round
    assert!(table.start_betting_round(vec![0, 1, 2, 3]).is_ok());

    // Player 1 bets 20
    assert_eq!(
        table.bet(player1_uid, BetType::Raised(convert_to_e8s(20.0))),
        Ok(())
    );

    // Player 2 calls
    assert_eq!(table.bet(player2_uid, BetType::Called), Ok(()));

    // Player 3 calls
    assert_eq!(table.bet(player3_uid, BetType::Called), Ok(()));

    // Player 4 calls
    assert_eq!(table.bet(player4_uid, BetType::Called), Ok(()));

    // Set community cards: [K♥, 9♠, 5♦, 5♣, 2♣]
    table.community_cards = vec![
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::Nine, Suit::Spade),
        Card::new(Value::Five, Suit::Diamond),
        Card::new(Value::Five, Suit::Club),
        Card::new(Value::Two, Suit::Club),
    ];

    // Set Player 1 hand: [A♥, K♦] (Two pair: Kings and Fives)
    table.get_user_table_data_mut(player1_uid).unwrap().cards = vec![
        Card::new(Value::Ace, Suit::Heart),
        Card::new(Value::King, Suit::Diamond),
    ];

    // Set Player 2 hand: [A♠, K♣] (Two pair: Kings and Fives)
    table.get_user_table_data_mut(player2_uid).unwrap().cards = vec![
        Card::new(Value::Ace, Suit::Spade),
        Card::new(Value::King, Suit::Club),
    ];

    // Set Player 3 hand: [Q♣, 9♥] (Two pair: Nines and Fives)
    table.get_user_table_data_mut(player3_uid).unwrap().cards = vec![
        Card::new(Value::Queen, Suit::Club),
        Card::new(Value::Nine, Suit::Heart),
    ];

    // Set Player 4 hand: [10♠, J♣] (High card: Jack)
    table.get_user_table_data_mut(player4_uid).unwrap().cards = vec![
        Card::new(Value::Ten, Suit::Spade),
        Card::new(Value::Jack, Suit::Club),
    ];

    // Perform showdown
    table.showdown().unwrap();

    // Verify that Players 1 and 2 split the pot equally
    let player1_balance_after = table.users.get(&player1_uid).unwrap().balance;
    let player2_balance_after = table.users.get(&player2_uid).unwrap().balance;

    // Since players 1 and 2 have identical hands and split the pot, each should receive half of the total pot amount
    assert_eq!(player1_balance_after.0, convert_to_e8s(120.0)); // 100 initial - 20 for bet + 30 split pot
    assert_eq!(player2_balance_after.0, convert_to_e8s(120.0)); // 100 initial - 20 for bet + 30 split pot
}

#[test]
fn test_correct_pot_side_pot_distribution() {
    // Initialize table with 4 players
    let mut table = Table::new(
        TableId(Principal::anonymous()),
        get_table_config(GameType::NoLimit(convert_to_e8s(1.0)), 4),
        vec![1, 2, 3, 4],
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

    // Start betting round
    assert!(table.start_betting_round(vec![0, 1, 2, 3]).is_ok());

    let big_blind_uid = table.big_blind_user_principal;
    let small_blind_uid = table.small_blind_user_principal;
    let mut other_uid = WalletPrincipalId(Principal::anonymous());
    for uid in table.seats.iter() {
        if let SeatStatus::Occupied(uid) = uid {
            if uid != &big_blind_uid && uid != &small_blind_uid {
                other_uid = *uid;
            }
        }
    }

    table.user_sitting_out(small_blind_uid, false).unwrap();

    assert_eq!(table.bet(other_uid, BetType::Called), Ok(()));
    assert_eq!(table.deal_stage, DealStage::Flop);

    assert_eq!(table.user_check(big_blind_uid, false), Ok(()));

    assert_eq!(table.deal_stage, DealStage::Turn);

    println!("Side Pots: {:#?}", table.side_pots);
    println!("Pot: {:#?}", table.pot);

    assert_eq!(table.pot.0, convert_to_e8s(5.0));

    assert_eq!(table.side_pots.len(), 0);
}
