use crate::tournaments::{
    blind_level::SpeedType, table_balancing::TableBalancer, types::TableInfo,
};
use candid::Principal;
use std::{
    collections::{HashMap, HashSet},
    time::SystemTime,
};
use table::poker::game::table_functions::table::TableId;

fn get_current_time_ns() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("System time before UNIX epoch")
        .as_nanos() as u64
}

fn create_test_principal(id: &str) -> Principal {
    Principal::self_authenticating(id)
}

fn create_test_table_info(player_count: usize) -> TableInfo {
    let mut players = HashSet::new();
    for i in 0..player_count {
        players.insert(user::user::WalletPrincipalId(create_test_principal(
            &format!("user{}", i),
        )));
    }
    TableInfo {
        players,
        last_balance_time: None,
    }
}

#[test]
fn test_balanced_tables() {
    let balancer = TableBalancer::new(4, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    tables.insert(
        TableId(create_test_principal("table1")),
        create_test_table_info(6),
    );
    tables.insert(
        TableId(create_test_principal("table2")),
        create_test_table_info(6),
    );
    tables.insert(
        TableId(create_test_principal("table3")),
        create_test_table_info(6),
    );

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(
        moves.len(),
        0,
        "No moves should be made when tables are balanced"
    );
}

#[test]
fn test_understaffed_table() {
    let balancer = TableBalancer::new(4, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1"));
    let table2 = TableId(create_test_principal("table2"));

    tables.insert(table1, create_test_table_info(3)); // Understaffed
    tables.insert(table2, create_test_table_info(8)); // Overstaffed

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 2, "Should move one player");
    assert_eq!(
        moves[0],
        (table2, table1),
        "Should move from overstaffed to understaffed"
    );
    assert_eq!(
        moves[1],
        (table2, table1),
        "Should move from overstaffed to understaffed"
    );
}

#[test]
fn test_multiple_understaffed_tables() {
    let balancer = TableBalancer::new(4, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1"));
    let table2 = TableId(create_test_principal("table2"));
    let table3 = TableId(create_test_principal("table3"));

    tables.insert(table1, create_test_table_info(3)); // Understaffed
    tables.insert(table2, create_test_table_info(3)); // Understaffed
    tables.insert(table3, create_test_table_info(8)); // Overstaffed

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 3, "Should move two players");
    assert!(
        moves.iter().all(|(from, _)| *from == table1)
            || moves.iter().all(|(from, _)| *from == table2),
        "All moves should come from understaffed table"
    );
}

#[test]
fn test_recently_balanced_table() {
    let balancer = TableBalancer::new(4, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1"));
    let table2 = TableId(create_test_principal("table2"));

    let understaffed = create_test_table_info(3);
    tables.insert(table1, understaffed);

    let mut overstaffed = create_test_table_info(8);
    overstaffed.last_balance_time = Some(get_current_time_ns()); // Recently balanced
    tables.insert(table2, overstaffed);

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(
        moves.len(),
        0,
        "No moves should occur from recently balanced table"
    );
}

#[test]
fn test_large_imbalance() {
    let balancer = TableBalancer::new(4, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1"));
    let table2 = TableId(create_test_principal("table2"));

    tables.insert(table1, create_test_table_info(4)); // Minimum
    tables.insert(table2, create_test_table_info(8)); // Maximum

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 2, "Should move two players to balance");
    assert_eq!(moves[0], (table2, table1));
    assert_eq!(moves[1], (table2, table1));
}

#[test]
fn test_empty_table() {
    let balancer = TableBalancer::new(4, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1"));
    let table2 = TableId(create_test_principal("table2"));

    tables.insert(table1, create_test_table_info(0)); // Empty
    tables.insert(table2, create_test_table_info(8)); // Overstaffed

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 0, "Should not move players to empty table");
}

#[test]
fn test_single_table() {
    let balancer = TableBalancer::new(4, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    tables.insert(
        TableId(create_test_principal("table1")),
        create_test_table_info(6),
    );

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 0, "No moves possible with single table");
}

#[test]
fn test_near_balance() {
    let balancer = TableBalancer::new(4, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1"));
    let table2 = TableId(create_test_principal("table2"));

    tables.insert(table1, create_test_table_info(5));
    tables.insert(table2, create_test_table_info(6));

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 0, "No moves needed when difference is 1");
}

#[test]
fn test_near_balance_2() {
    let balancer = TableBalancer::new(4, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1"));
    let table2 = TableId(create_test_principal("table2"));
    let table3 = TableId(create_test_principal("table3"));

    tables.insert(table1, create_test_table_info(5));
    tables.insert(table2, create_test_table_info(6));
    tables.insert(table3, create_test_table_info(6));

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 0, "No moves needed when difference is 1");
}

#[test]
fn test_near_balance_3() {
    let balancer = TableBalancer::new(4, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1"));
    let table2 = TableId(create_test_principal("table2"));
    let table3 = TableId(create_test_principal("table3"));
    let table4 = TableId(create_test_principal("table4"));

    tables.insert(table1, create_test_table_info(5));
    tables.insert(table2, create_test_table_info(6));
    tables.insert(table3, create_test_table_info(6));
    tables.insert(table4, create_test_table_info(5));

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(
        moves.len(),
        5,
        "Should move 5 players to consolidate tables"
    );
    assert!(
        moves.iter().all(|(from, _)| *from == table1)
            || moves.iter().all(|(from, _)| *from == table4),
        "All moves should come from table1 or table4"
    );
}

#[test]
fn test_varied_player_counts_basic() {
    let balancer = TableBalancer::new(3, 7, &SpeedType::new_regular(1000, 100)); // Testing with min=3, max=7
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 3 players
    let table2 = TableId(create_test_principal("table2")); // 4 players
    let table3 = TableId(create_test_principal("table3")); // 5 players
    let table4 = TableId(create_test_principal("table4")); // 6 players
    let table5 = TableId(create_test_principal("table5")); // 7 players

    tables.insert(table1, create_test_table_info(3));
    tables.insert(table2, create_test_table_info(4));
    tables.insert(table3, create_test_table_info(5));
    tables.insert(table4, create_test_table_info(6));
    tables.insert(table5, create_test_table_info(7));

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 3, "Should make one move to balance");
    assert!(
        moves.iter().all(|(from, _)| *from == table1),
        "All moves should come from emptiest table"
    );
}

#[test]
fn test_low_min_high_max() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100)); // Testing with min=2, max=8
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 3 players
    let table2 = TableId(create_test_principal("table2")); // 5 players
    let table3 = TableId(create_test_principal("table3")); // 7 players

    tables.insert(table1, create_test_table_info(3));
    tables.insert(table2, create_test_table_info(5));
    tables.insert(table3, create_test_table_info(7));

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 3, "Should make two moves to balance");
    assert!(
        moves.iter().all(|(from, _)| *from == table1),
        "All moves should come from understaffed table to consolidate tables"
    );
}

#[test]
fn test_tight_range() {
    let balancer = TableBalancer::new(4, 6, &SpeedType::new_regular(1000, 100)); // Testing with min=4, max=6
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 3 players (understaffed)
    let table2 = TableId(create_test_principal("table2")); // 4 players
    let table3 = TableId(create_test_principal("table3")); // 7 players (overstaffed)

    tables.insert(table1, create_test_table_info(3));
    tables.insert(table2, create_test_table_info(4));
    tables.insert(table3, create_test_table_info(7));

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 2, "Should move two players");
    assert!(
        moves.iter().all(|(from, _)| *from == table3),
        "All moves should come from overstaffed table"
    );
}

#[test]
fn test_multiple_identical_tables() {
    let balancer = TableBalancer::new(3, 7, &SpeedType::new_regular(1000, 100)); // Testing with min=3, max=7
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 6 players
    let table2 = TableId(create_test_principal("table2")); // 6 players
    let table3 = TableId(create_test_principal("table3")); // 3 players

    tables.insert(table1, create_test_table_info(6));
    tables.insert(table2, create_test_table_info(6));
    tables.insert(table3, create_test_table_info(3));

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 2, "Should move one player");
    assert!(
        moves.iter().any(|(from, _)| *from == table1)
            && moves.iter().any(|(from, _)| *from == table2),
        "Moves should come from both table1 and table2"
    );
}

#[test]
fn test_near_maximum_variation() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100)); // Wide range
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 2 players
    let table2 = TableId(create_test_principal("table2")); // 4 players
    let table3 = TableId(create_test_principal("table3")); // 6 players
    let table4 = TableId(create_test_principal("table4")); // 8 players

    tables.insert(table1, create_test_table_info(2));
    tables.insert(table2, create_test_table_info(4));
    tables.insert(table3, create_test_table_info(6));
    tables.insert(table4, create_test_table_info(8));

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 2, "Should make maximum allowed moves");
    assert!(
        moves.iter().all(|(from, _)| *from == table1),
        "All moves should come from emptiest table"
    );
}

#[test]
fn test_small_variation() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100)); // Wide range
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 7 players
    let table2 = TableId(create_test_principal("table2")); // 5 players

    tables.insert(table1, create_test_table_info(7));
    tables.insert(table2, create_test_table_info(5));

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 1, "Should make maximum allowed moves");
    assert!(
        moves.iter().all(|(from, _)| *from == table1),
        "All moves should come from table1"
    );
}

#[test]
fn test_with_empty_and_full() {
    let balancer = TableBalancer::new(3, 7, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 0 players
    let table2 = TableId(create_test_principal("table2")); // 4 players
    let table3 = TableId(create_test_principal("table3")); // 7 players

    tables.insert(table1, create_test_table_info(0));
    tables.insert(table2, create_test_table_info(4));
    tables.insert(table3, create_test_table_info(7));

    let moves = balancer.get_balance_moves(&mut tables);
    assert!(
        moves.iter().all(|(_, to)| *to != table1),
        "No moves should go to the empty table"
    );
}

#[test]
fn test_min_equals_max() {
    let balancer = TableBalancer::new(5, 5, &SpeedType::new_regular(1000, 100)); // Testing edge case where min=max
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 4 players
    let table2 = TableId(create_test_principal("table2")); // 6 players

    tables.insert(table1, create_test_table_info(4));
    tables.insert(table2, create_test_table_info(6));

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 1, "Should move one player");
    assert_eq!(moves[0], (table2, table1));
}

#[test]
fn test_consolidation_basic_two() {
    let balancer = TableBalancer::new(2, 5, &SpeedType::new_regular(1000, 100)); // Testing edge case where min=max
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1"));
    let table2 = TableId(create_test_principal("table2"));

    tables.insert(table1, create_test_table_info(3));
    tables.insert(table2, create_test_table_info(1));

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 1, "Should move one player");
    assert_eq!(moves[0], (table2, table1));
}

#[test]
fn test_consolidation_basic_three() {
    let balancer = TableBalancer::new(2, 5, &SpeedType::new_regular(1000, 100)); // Testing edge case where min=max
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1"));
    let table2 = TableId(create_test_principal("table2"));

    tables.insert(table1, create_test_table_info(1));
    tables.insert(table2, create_test_table_info(1));

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 1, "Should move one player");
}

#[test]
fn test_consolidation_basic_four() {
    let balancer = TableBalancer::new(2, 3, &SpeedType::new_regular(1000, 100)); // Testing edge case where min=max
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1"));
    let table2 = TableId(create_test_principal("table2"));

    tables.insert(table1, create_test_table_info(2));
    tables.insert(table2, create_test_table_info(1));

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 1, "Should move one player");
    assert_eq!(moves[0], (table2, table1));
}

#[test]
fn test_all_tables_below_min() {
    let balancer = TableBalancer::new(4, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    tables.insert(
        TableId(create_test_principal("table1")),
        create_test_table_info(3),
    );
    tables.insert(
        TableId(create_test_principal("table2")),
        create_test_table_info(2),
    );
    tables.insert(
        TableId(create_test_principal("table3")),
        create_test_table_info(3),
    );

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(
        moves.len(),
        5,
        "Should consolidate to final table with 8 players"
    );
    assert!(
        moves
            .iter()
            .all(|(_, to)| *to == TableId(create_test_principal("table3")))
            || moves
                .iter()
                .all(|(_, to)| *to == TableId(create_test_principal("table1"))),
        "All moves should go to table1 or table3"
    );
}

#[test]
fn test_all_tables_above_max() {
    let balancer = TableBalancer::new(2, 5, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    tables.insert(
        TableId(create_test_principal("table1")),
        create_test_table_info(6),
    );
    tables.insert(
        TableId(create_test_principal("table2")),
        create_test_table_info(7),
    );
    tables.insert(
        TableId(create_test_principal("table3")),
        create_test_table_info(6),
    );

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 0, "No moves when no understaffed tables exist");
}

#[test]
fn test_consolidation_basic() {
    let balancer = TableBalancer::new(2, 6, &SpeedType::new_regular(1000, 100)); // min=2, max=6
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 2 players
    let table2 = TableId(create_test_principal("table2")); // 4 players
    let table3 = TableId(create_test_principal("table3")); // 4 players

    println!("table1: {:?}", table1.0.to_text());
    println!("table2: {:?}", table2.0.to_text());
    println!("table3: {:?}", table3.0.to_text());

    tables.insert(table1, create_test_table_info(2));
    tables.insert(table2, create_test_table_info(4));
    tables.insert(table3, create_test_table_info(4));

    let moves = balancer.get_balance_moves(&mut tables);
    // Current behavior: moves players to balance within existing tables
    assert_eq!(moves.len(), 2, "Should move two players to balance");
    // Expect moves from tables with 4 to table with 2
    assert!(
        moves.iter().all(|(from, _)| *from == table1),
        "All moves should come from table1"
    );
}

#[test]
fn test_consolidation_with_understaffed() {
    let balancer = TableBalancer::new(3, 7, &SpeedType::new_regular(1000, 100)); // min=3, max=7
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 2 players (understaffed)
    let table2 = TableId(create_test_principal("table2")); // 3 players
    let table3 = TableId(create_test_principal("table3")); // 5 players

    tables.insert(table1, create_test_table_info(2));
    tables.insert(table2, create_test_table_info(3));
    tables.insert(table3, create_test_table_info(5));

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 2, "Should move one player to reach minimum");
    assert!(
        moves.iter().all(|(from, _)| *from == table1),
        "All moves should come from table1 and go to table2"
    );
}

#[test]
fn test_consolidation_all_low() {
    let balancer = TableBalancer::new(4, 8, &SpeedType::new_regular(1000, 100)); // min=4, max=8
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 3 players
    let table2 = TableId(create_test_principal("table2")); // 3 players
    let table3 = TableId(create_test_principal("table3")); // 4 players

    tables.insert(table1, create_test_table_info(3));
    tables.insert(table2, create_test_table_info(3));
    tables.insert(table3, create_test_table_info(4));

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(
        moves.len(),
        3,
        "Should move 3 player to balance and consolidate tables"
    );
    assert!(
        moves.iter().all(|(from, _)| *from == table1)
            || moves.iter().all(|(from, _)| *from == table2),
        "All moves should come from either table1 or table2"
    );
}

#[test]
fn test_consolidation_near_max() {
    let balancer = TableBalancer::new(2, 5, &SpeedType::new_regular(1000, 100)); // min=2, max=5
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 2 players
    let table2 = TableId(create_test_principal("table2")); // 3 players
    let table3 = TableId(create_test_principal("table3")); // 5 players

    tables.insert(table1, create_test_table_info(2));
    tables.insert(table2, create_test_table_info(3));
    tables.insert(table3, create_test_table_info(5));

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 2, "Should move two players to balance");
    assert_eq!(
        moves[0],
        (table1, table2),
        "Should move from table3 to table1"
    );
    assert_eq!(
        moves[1],
        (table1, table2),
        "Should move from table3 to table1"
    );
    // Result: [5, 5]
}

#[test]
fn test_consolidation_multiple_sources() {
    let balancer = TableBalancer::new(3, 6, &SpeedType::new_regular(1000, 100)); // min=3, max=6
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 2 players (understaffed)
    let table2 = TableId(create_test_principal("table2")); // 4 players
    let table3 = TableId(create_test_principal("table3")); // 5 players

    tables.insert(table1, create_test_table_info(2));
    tables.insert(table2, create_test_table_info(4));
    tables.insert(table3, create_test_table_info(5));

    let moves = balancer.get_balance_moves(&mut tables);
    assert_eq!(moves.len(), 2, "Should move one player to reach minimum");
    assert!(
        moves.iter().all(|(from, _)| *from == table1),
        "All moves should be from table1"
    );
    // Result: [3, 4, 4]
}

#[test]
fn test_single_player_table_consolidation() {
    let balancer = TableBalancer::new(3, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 1 player (should be emptied)
    let table2 = TableId(create_test_principal("table2")); // 5 players
    let table3 = TableId(create_test_principal("table3")); // 7 players

    tables.insert(table1, create_test_table_info(1));
    tables.insert(table2, create_test_table_info(5));
    tables.insert(table3, create_test_table_info(7));

    let moves = balancer.get_balance_moves(&mut tables);

    // Check that exactly one move happened (the single player from table1)
    assert_eq!(moves.len(), 1, "Should move the single player from table1");
    assert!(
        moves.iter().all(|(from, _)| *from == table1),
        "All moves should come from table1"
    );
}

#[test]
fn test_consolidation_distribute_players() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 2 players (understaffed)
    let table2 = TableId(create_test_principal("table2")); // 3 players
    let table3 = TableId(create_test_principal("table3")); // 3 players

    println!("table1: {:?}", table1.0.to_text());
    println!("table2: {:?}", table2.0.to_text());
    println!("table3: {:?}", table3.0.to_text());

    tables.insert(table1, create_test_table_info(2));
    tables.insert(table2, create_test_table_info(3));
    tables.insert(table3, create_test_table_info(3));

    let moves = balancer.get_balance_moves(&mut tables);

    // We expect two moves, moving the players from table1 to the other tables
    assert_eq!(
        moves.len(),
        5,
        "Should move five players to form the final table"
    );

    println!(
        "moves: {:#?}",
        moves
            .iter()
            .map(|(a, b)| (a.0.to_text(), b.0.to_text()))
            .collect::<Vec<_>>()
    );

    // All moves should be from table1
    assert!(
        moves.iter().all(|(_, to)| *to == table2) || moves.iter().all(|(_, to)| *to == table3),
        "All moves should go to table2 or table3"
    );
}

#[test]
fn test_consolidation_distribute_players_two() {
    let balancer = TableBalancer::new(2, 6, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 4 players (understaffed)
    let table2 = TableId(create_test_principal("table2")); // 4 players
    let table3 = TableId(create_test_principal("table3")); // 4 players

    println!("table1: {:?}", table1.0.to_text());
    println!("table2: {:?}", table2.0.to_text());
    println!("table3: {:?}", table3.0.to_text());

    tables.insert(table1, create_test_table_info(4));
    tables.insert(table2, create_test_table_info(4));
    tables.insert(table3, create_test_table_info(4));

    let moves = balancer.get_balance_moves(&mut tables);

    // We expect two moves, moving the players from table1 to the other tables
    assert_eq!(
        moves.len(),
        4,
        "Should move four players to form the last two tables"
    );

    println!(
        "moves: {:#?}",
        moves
            .iter()
            .map(|(a, b)| (a.0.to_text(), b.0.to_text()))
            .collect::<Vec<_>>()
    );

    // All moves should be from only one of the tables
    assert!(
        moves.iter().all(|(from, _)| *from == table1)
            || moves.iter().all(|(from, _)| *from == table2)
            || moves.iter().all(|(from, _)| *from == table3),
        "All moves should come from either table1 or table2"
    );
}

#[test]
fn test_consolidation_medium_tables() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 4 players
    let table2 = TableId(create_test_principal("table2")); // 4 players
    let table3 = TableId(create_test_principal("table3")); // 8 players (maximum)

    tables.insert(table1, create_test_table_info(4));
    tables.insert(table2, create_test_table_info(4));
    tables.insert(table3, create_test_table_info(8));

    let moves = balancer.get_balance_moves(&mut tables);

    // Should consolidate the two medium tables onto one
    assert_eq!(
        moves.len(),
        4,
        "Should move all players from one table to the other"
    );

    // All moves should come from the same table (either table1 or table2)
    assert!(
        moves.iter().all(|(from, _)| *from == table1)
            || moves.iter().all(|(from, _)| *from == table2),
        "All moves should come from either table1 or table2"
    );

    // All moves should go to the other medium table
    let from_table = moves[0].0;
    let to_table = moves[0].1;
    assert!(
        (from_table == table1 && to_table == table2)
            || (from_table == table2 && to_table == table1),
        "Players should be moved from one medium table to the other"
    );

    // Final state should be [0, 8, 8] effectively closing one of the medium tables
}

#[test]
fn test_consolidation_multiple_small_tables() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 2 players
    let table2 = TableId(create_test_principal("table2")); // 2 players
    let table3 = TableId(create_test_principal("table3")); // 2 players

    tables.insert(table1, create_test_table_info(2));
    tables.insert(table2, create_test_table_info(2));
    tables.insert(table3, create_test_table_info(2));

    let moves = balancer.get_balance_moves(&mut tables);

    // Should consolidate onto one table
    assert_eq!(
        moves.len(),
        4,
        "Should move all players from two tables to one"
    );

    // Determine which table is the destination
    let destinations: Vec<TableId> = moves.iter().map(|(_, to)| *to).collect();
    let mut destination_counts = HashMap::new();
    for dest in destinations {
        *destination_counts.entry(dest).or_insert(0) += 1;
    }

    // Only one table should be the destination
    assert_eq!(
        destination_counts.len(),
        1,
        "There should be only one destination table"
    );

    // That table should receive 4 players from the other two tables
    assert_eq!(
        *destination_counts.values().next().unwrap(),
        4,
        "The destination table should receive 4 players"
    );

    // Final state should be [0, 0, 6] or similar, consolidating all players onto one table
}

#[test]
fn test_no_move_to_full_table() {
    // Test that players are not moved to a table that's already at maximum capacity
    let balancer = TableBalancer::new(3, 6, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 4 players
    let table2 = TableId(create_test_principal("table2")); // 6 players (already at max)

    tables.insert(table1, create_test_table_info(4));
    tables.insert(table2, create_test_table_info(6));

    let moves = balancer.get_balance_moves(&mut tables);

    // Verify no moves are made to the full table
    for (_, dest) in &moves {
        assert_ne!(*dest, table2, "Should not move players to a full table");
    }
}

#[test]
fn test_no_move_to_near_full_table() {
    // Test that we don't move more players to a table than would fit
    let balancer = TableBalancer::new(3, 7, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 3 players
    let table2 = TableId(create_test_principal("table2")); // 6 players (can only take 1 more)
    let table3 = TableId(create_test_principal("table3")); // 5 players

    tables.insert(table1, create_test_table_info(3));
    tables.insert(table2, create_test_table_info(6));
    tables.insert(table3, create_test_table_info(5));

    let moves = balancer.get_balance_moves(&mut tables);

    // Count how many players are moved to table2
    let moves_to_table2 = moves.iter().filter(|(_, dest)| *dest == table2).count();
    println!("moves: {:?}", moves_to_table2);

    // Verify we don't move more than the available capacity
    assert!(
        moves_to_table2 <= 1,
        "Should not move more than 1 player to a nearly full table"
    );
}

#[test]
fn test_all_tables_full() {
    // Test behavior when all tables are at maximum capacity
    let balancer = TableBalancer::new(3, 6, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    tables.insert(
        TableId(create_test_principal("table1")),
        create_test_table_info(6),
    );
    tables.insert(
        TableId(create_test_principal("table2")),
        create_test_table_info(6),
    );
    tables.insert(
        TableId(create_test_principal("table3")),
        create_test_table_info(6),
    );

    let moves = balancer.get_balance_moves(&mut tables);

    // No moves should be made since all tables are full
    assert_eq!(
        moves.len(),
        0,
        "No moves should be made when all tables are full"
    );
}

#[test]
fn test_unbalanced_full_tables() {
    // Test with some full and some understaffed tables
    let balancer = TableBalancer::new(3, 6, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 2 players (understaffed)
    let table2 = TableId(create_test_principal("table2")); // 6 players (full)
    let table3 = TableId(create_test_principal("table3")); // 6 players (full)

    tables.insert(table1, create_test_table_info(2));
    tables.insert(table2, create_test_table_info(6));
    tables.insert(table3, create_test_table_info(6));

    let moves = balancer.get_balance_moves(&mut tables);

    // Verify we're moving players from full tables to understaffed
    for (_, dest) in &moves {
        assert_ne!(*dest, table2, "Should not move to full table2");
        assert_ne!(*dest, table3, "Should not move to full table3");
        assert_eq!(*dest, table1, "Should move to understaffed table1");
    }
}

#[test]
fn test_mixed_capacities() {
    // Test with tables of different capacities
    let balancer = TableBalancer::new(3, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 3 players (min capacity)
    let table2 = TableId(create_test_principal("table2")); // 8 players (max capacity)
    let table3 = TableId(create_test_principal("table3")); // 7 players (near max)
    let table4 = TableId(create_test_principal("table4")); // 5 players (mid capacity)

    tables.insert(table1, create_test_table_info(3));
    tables.insert(table2, create_test_table_info(8));
    tables.insert(table3, create_test_table_info(7));
    tables.insert(table4, create_test_table_info(5));

    let moves = balancer.get_balance_moves(&mut tables);

    // No moves should be made to table2 (already at max)
    assert!(
        moves.iter().all(|(_, dest)| *dest != table2),
        "No players should be moved to the full table"
    );

    // Count moves to table3 (near max)
    let moves_to_table3 = moves.iter().filter(|(_, dest)| *dest == table3).count();
    println!("moves: {:?}", moves_to_table3);

    // Verify we don't exceed capacity
    assert!(
        moves_to_table3 <= 1,
        "Should not move more than 1 player to a nearly full table"
    );
}

#[test]
fn test_full_and_empty_tables() {
    // Test with some full tables and an empty table
    let balancer = TableBalancer::new(3, 6, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 0 players (empty)
    let table2 = TableId(create_test_principal("table2")); // 6 players (full)
    let table3 = TableId(create_test_principal("table3")); // 6 players (full)

    tables.insert(table1, create_test_table_info(0));
    tables.insert(table2, create_test_table_info(6));
    tables.insert(table3, create_test_table_info(6));

    let moves = balancer.get_balance_moves(&mut tables);

    // Verify we're not moving players to empty tables but are moving from full tables
    for (source, dest) in &moves {
        assert!(
            *source == table2 || *source == table3,
            "Should move from full tables"
        );
        assert_ne!(*dest, table2, "Should not move to full table2");
        assert_ne!(*dest, table3, "Should not move to full table3");
    }
}

#[test]
fn test_full_table_with_recently_balanced() {
    // Test with a full table that was recently balanced
    let balancer = TableBalancer::new(3, 6, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 3 players
    let table2 = TableId(create_test_principal("table2")); // 6 players (full)

    tables.insert(table1, create_test_table_info(3));

    // Mark the full table as recently balanced
    let mut full_table_info = create_test_table_info(6);
    full_table_info.last_balance_time = Some(get_current_time_ns());
    tables.insert(table2, full_table_info);

    let moves = balancer.get_balance_moves(&mut tables);

    // No moves should be made involving the recently balanced table
    for (source, dest) in &moves {
        assert_ne!(
            *source, table2,
            "Should not move from recently balanced full table"
        );
        assert_ne!(*dest, table2, "Should not move to full table");
    }
}

#[test]
fn test_final_table_consolidation_with_full_table() {
    // Test the final table consolidation logic when one table is already full
    let balancer = TableBalancer::new(3, 6, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 2 players
    let table2 = TableId(create_test_principal("table2")); // 2 players
    let table3 = TableId(create_test_principal("table3")); // 6 players (full)

    tables.insert(table1, create_test_table_info(2));
    tables.insert(table2, create_test_table_info(2));
    tables.insert(table3, create_test_table_info(6));

    let moves = balancer.get_balance_moves(&mut tables);

    // Should consolidate the smaller tables and not involve the full table
    assert!(
        moves.iter().all(|(_, dest)| *dest != table3),
        "No players should be moved to the full table"
    );

    // The moves should be between table1 and table2
    for (source, dest) in &moves {
        assert!(
            (*source == table1 && *dest == table2) || (*source == table2 && *dest == table1),
            "Moves should only be between the non-full tables"
        );
    }
}

#[test]
fn test_respect_max_players_in_multiple_moves() {
    // Test that multiple moves in the same balancing operation don't exceed table capacity
    let balancer = TableBalancer::new(3, 6, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 8 players (overstaffed)
    let table2 = TableId(create_test_principal("table2")); // 5 players (one space left)
    let table3 = TableId(create_test_principal("table3")); // 3 players (min capacity)

    tables.insert(table1, create_test_table_info(8));
    tables.insert(table2, create_test_table_info(5));
    tables.insert(table3, create_test_table_info(3));

    let moves = balancer.get_balance_moves(&mut tables);

    // Count moves to each destination table
    let mut moves_to_table2 = 0;
    let mut moves_to_table3 = 0;

    for (_, dest) in &moves {
        if *dest == table2 {
            moves_to_table2 += 1;
        } else if *dest == table3 {
            moves_to_table3 += 1;
        }
    }

    // Table2 can only accept 1 more player to reach max capacity of 6
    assert!(
        moves_to_table2 <= 1,
        "Should not move more than 1 player to table2"
    );

    // Table3 can accept up to 3 more players to reach max capacity of 6
    assert!(
        moves_to_table3 <= 3,
        "Should not move more than 3 players to table3"
    );
}

#[test]
fn test_no_move_to_near_full_table_multiple_sources() {
    let balancer = TableBalancer::new(3, 7, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    // Three tables with different counts
    let table1 = TableId(create_test_principal("table1")); // 9 players (overstaffed)
    let table2 = TableId(create_test_principal("table2")); // 6 players (can only take 1 more)
    let table3 = TableId(create_test_principal("table3")); // 8 players (overstaffed)
    let table4 = TableId(create_test_principal("table4")); // 2 players (understaffed)

    tables.insert(table1, create_test_table_info(9));
    tables.insert(table2, create_test_table_info(6));
    tables.insert(table3, create_test_table_info(8));
    tables.insert(table4, create_test_table_info(2));

    let moves = balancer.get_balance_moves(&mut tables);

    // Count how many players are moved to table2
    let moves_to_table2 = moves.iter().filter(|(_, dest)| *dest == table2).count();

    // Verify we don't move more than the available capacity
    assert!(
        moves_to_table2 <= 1,
        "Should not move more than 1 player to a nearly full table"
    );
}

#[test]
fn test_no_move_to_near_full_table_consecutive_balances() {
    let balancer = TableBalancer::new(3, 7, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 3 players
    let table2 = TableId(create_test_principal("table2")); // 6 players (can only take 1 more)
    let table3 = TableId(create_test_principal("table3")); // 9 players (overstaffed)

    tables.insert(table1, create_test_table_info(3));
    tables.insert(table2, create_test_table_info(6));
    tables.insert(table3, create_test_table_info(9));

    // First balancing operation
    let moves1 = balancer.get_balance_moves(&mut tables);

    // Apply the moves to our tables (simulate actual movement)
    for (source, dest) in &moves1 {
        if let Some(source_table) = tables.get_mut(source) {
            if !source_table.players.is_empty() {
                let player = source_table.players.iter().next().cloned().unwrap();
                if let Some(dest_table) = tables.get_mut(dest) {
                    dest_table.players.insert(player);
                }
            }
        }
    }

    // Second balancing operation
    let moves2 = balancer.get_balance_moves(&mut tables);

    // Count moves to table2 in both operations
    let moves_to_table2 = moves1
        .iter()
        .chain(moves2.iter())
        .filter(|(_, dest)| *dest == table2)
        .count();

    // Verify we don't exceed capacity even across multiple balance operations
    assert!(
        moves_to_table2 <= 1,
        "Should not move more than 1 player to a nearly full table across balance operations"
    );
}

#[test]
fn test_near_full_with_multiple_destinations() {
    let balancer = TableBalancer::new(3, 6, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 10 players (overstaffed)
    let table2 = TableId(create_test_principal("table2")); // 5 players (one space left)
    let table3 = TableId(create_test_principal("table3")); // 5 players (one space left)
    let table4 = TableId(create_test_principal("table4")); // 4 players (two spaces left)

    tables.insert(table1, create_test_table_info(10));
    tables.insert(table2, create_test_table_info(5));
    tables.insert(table3, create_test_table_info(5));
    tables.insert(table4, create_test_table_info(4));

    let moves = balancer.get_balance_moves(&mut tables);

    // Count moves to each destination
    let moves_to_table2 = moves.iter().filter(|(_, dest)| *dest == table2).count();
    let moves_to_table3 = moves.iter().filter(|(_, dest)| *dest == table3).count();

    // Verify we don't exceed capacity
    assert!(
        moves_to_table2 <= 1,
        "Should not move more than 1 player to table2"
    );
    assert!(
        moves_to_table3 <= 1,
        "Should not move more than 1 player to table3"
    );

    // Also verify that most moves go to the table with most space
    let moves_to_table4 = moves.iter().filter(|(_, dest)| *dest == table4).count();
    assert!(
        moves_to_table4 >= moves_to_table2,
        "Should prefer moving to tables with more space"
    );
    assert!(
        moves_to_table4 >= moves_to_table3,
        "Should prefer moving to tables with more space"
    );
}

#[test]
fn test_mixed_capacities_with_consolidation() {
    let balancer = TableBalancer::new(3, 6, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 2 players (understaffed)
    let table2 = TableId(create_test_principal("table2")); // 6 players (max capacity)
    let table3 = TableId(create_test_principal("table3")); // 5 players (near max)
    let table4 = TableId(create_test_principal("table4")); // 2 players (understaffed)

    tables.insert(table1, create_test_table_info(2));
    tables.insert(table2, create_test_table_info(6));
    tables.insert(table3, create_test_table_info(5));
    tables.insert(table4, create_test_table_info(2));

    let moves = balancer.get_balance_moves(&mut tables);

    // No moves should be made to table2 (already at max)
    assert!(
        moves.iter().all(|(_, dest)| *dest != table2),
        "No players should be moved to the full table"
    );

    // Count moves to table3 (near max)
    let moves_to_table3 = moves.iter().filter(|(_, dest)| *dest == table3).count();

    // Verify we don't exceed capacity
    assert!(
        moves_to_table3 <= 1,
        "Should not move more than 1 player to a nearly full table"
    );

    // Given the two understaffed tables, check if we're consolidating
    let moves_between_understaffed = moves
        .iter()
        .filter(|(source, dest)| {
            (*source == table1 && *dest == table4) || (*source == table4 && *dest == table1)
        })
        .count();

    assert!(
        moves_between_understaffed > 0,
        "Should consolidate understaffed tables"
    );
}

#[test]
fn test_balanced_tables_with_uneven_distribution() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 4 players
    let table2 = TableId(create_test_principal("table2")); // 7 players
    let table3 = TableId(create_test_principal("table3")); // 7 players

    tables.insert(table1, create_test_table_info(4));
    tables.insert(table2, create_test_table_info(7));
    tables.insert(table3, create_test_table_info(7));

    let moves = balancer.get_balance_moves(&mut tables);

    // Should move players from the fuller tables to balance distribution
    assert_eq!(moves.len(), 2, "Should move two players to balance tables");

    // All moves should be to table1 (the table with fewer players)
    assert!(
        moves.iter().all(|(_, to)| *to == table1),
        "All moves should go to the table with fewer players"
    );

    // Should move one player from each of the fuller tables
    let sources: HashSet<_> = moves.iter().map(|(from, _)| *from).collect();
    assert_eq!(
        sources.len(),
        2,
        "Should move players from both of the fuller tables"
    );
    assert!(
        sources.contains(&table2) && sources.contains(&table3),
        "Should move one player from each of the fuller tables"
    );

    // Final expected distribution would be [6, 6, 6]
}

#[test]
fn test_consolidation_with_655_distribution() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 6 players
    let table2 = TableId(create_test_principal("table2")); // 5 players
    let table3 = TableId(create_test_principal("table3")); // 5 players

    tables.insert(table1, create_test_table_info(6));
    tables.insert(table2, create_test_table_info(5));
    tables.insert(table3, create_test_table_info(5));

    let moves = balancer.get_balance_moves(&mut tables);

    // Should consolidate to two tables by moving all players from one table
    assert_eq!(
        moves.len(),
        5,
        "Should move 5 players to consolidate tables"
    );

    // All moves should be from a single table
    let from_table = moves[0].0;
    assert!(
        moves.iter().all(|(from, _)| *from == from_table),
        "All moves should come from the same table"
    );

    // The destination should not be the same as the source
    let to_tables: HashSet<_> = moves.iter().map(|(_, to)| *to).collect();
    assert!(
        !to_tables.contains(&from_table),
        "Destination should be different from source"
    );

    // Final expected distribution would be either [0, 8, 8] or [8, 0, 8] or [8, 8, 0]
}

#[test]
fn test_consolidation_with_555_distribution() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 5 players
    let table2 = TableId(create_test_principal("table2")); // 5 players
    let table3 = TableId(create_test_principal("table3")); // 5 players

    tables.insert(table1, create_test_table_info(5));
    tables.insert(table2, create_test_table_info(5));
    tables.insert(table3, create_test_table_info(5));

    let moves = balancer.get_balance_moves(&mut tables);

    // Should consolidate to two tables by moving all players from one table
    assert_eq!(
        moves.len(),
        5,
        "Should move 5 players to consolidate tables"
    );

    // All moves should be from a single table
    let from_table = moves[0].0;
    assert!(
        moves.iter().all(|(from, _)| *from == from_table),
        "All moves should come from the same table"
    );

    // The destination should not be the same as the source
    let to_tables: HashSet<_> = moves.iter().map(|(_, to)| *to).collect();
    assert!(
        !to_tables.contains(&from_table),
        "Destination should be different from source"
    );

    // Check that we're not exceeding the max capacity of 8 for any table
    let target_table = *to_tables.iter().next().unwrap();
    let moves_to_target = moves.iter().filter(|(_, to)| *to == target_table).count();

    assert!(
        moves_to_target <= 3,
        "Should not move more than 3 players to the target table to avoid exceeding max capacity"
    );

    // Final expected distribution would be [0, 8, 7] or [8, 0, 7] or [8, 7, 0] or some equivalent
}

#[test]
fn test_log_scenario_april_29_192936() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 1 player
    let table2 = TableId(create_test_principal("table2")); // 6 players
    let table3 = TableId(create_test_principal("table3")); // 7 players

    tables.insert(table1, create_test_table_info(1));
    tables.insert(table2, create_test_table_info(6));
    tables.insert(table3, create_test_table_info(7));

    let moves = balancer.get_balance_moves(&mut tables);

    // Check that we're generating moves for this scenario
    assert!(
        !moves.is_empty(),
        "Moves should be generated for tables with 1, 6, and 7 players"
    );

    assert!(
        moves[0].0 == table1,
        "Should move the single player from table1"
    );
    assert!(moves[0].1 == table2, "Should move to table2");
}

// Test for log scenario - April 29, 19:32:57
// Tables with 1, 8, 6, and 7 players
#[test]
fn test_log_scenario_april_29_193257() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 1 player
    let table2 = TableId(create_test_principal("table2")); // 8 players
    let table3 = TableId(create_test_principal("table3")); // 6 players
    let table4 = TableId(create_test_principal("table4")); // 7 players

    tables.insert(table1, create_test_table_info(1));
    tables.insert(table2, create_test_table_info(8));
    tables.insert(table3, create_test_table_info(6));
    tables.insert(table4, create_test_table_info(7));

    let moves = balancer.get_balance_moves(&mut tables);

    // Check that we're generating moves for this scenario
    assert!(
        !moves.is_empty(),
        "Moves should be generated for tables with 1, 8, 6, and 7 players"
    );

    assert!(
        moves[0].0 == table1,
        "Should move the single player from table1"
    );
    assert!(moves[0].1 == table3, "Should move to table3");
}

// Test for log scenario - April 29, 19:37:01
// Tables with 4 and 8 players
#[test]
fn test_log_scenario_april_29_193701() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 4 players
    let table2 = TableId(create_test_principal("table2")); // 8 players

    tables.insert(table1, create_test_table_info(4));
    tables.insert(table2, create_test_table_info(8));

    let moves = balancer.get_balance_moves(&mut tables);

    // Check that we're generating moves for this scenario
    assert!(moves.len() == 2);

    assert!(
        moves
            .iter()
            .all(|(from, to)| *from == table2 && *to == table1),
        "All moves should come from table2 to table1"
    );
}

// Test for log scenario - April 29, 19:44:10
// Tables with 7, 5, and 6 players
#[test]
fn test_log_scenario_april_29_194410() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    let table1 = TableId(create_test_principal("table1")); // 7 players
    let table2 = TableId(create_test_principal("table2")); // 5 players
    let table3 = TableId(create_test_principal("table3")); // 6 players

    tables.insert(table1, create_test_table_info(7));
    tables.insert(table2, create_test_table_info(5));
    tables.insert(table3, create_test_table_info(6));

    let moves = balancer.get_balance_moves(&mut tables);

    assert!(
        moves.len() == 1,
        "Should move the single player from table1"
    );
    assert!(moves[0].0 == table1, "Should move from table1");
}

// -------------

#[test]
fn test_consolidation_5_to_4_tables() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    // Create 5 tables with varying player counts
    let table1 = TableId(create_test_principal("table1")); // 5 players
    let table2 = TableId(create_test_principal("table2")); // 5 players
    let table3 = TableId(create_test_principal("table3")); // 6 players
    let table4 = TableId(create_test_principal("table4")); // 6 players
    let table5 = TableId(create_test_principal("table5")); // 6 players

    tables.insert(table1, create_test_table_info(5));
    tables.insert(table2, create_test_table_info(5));
    tables.insert(table3, create_test_table_info(6));
    tables.insert(table4, create_test_table_info(6));
    tables.insert(table5, create_test_table_info(6));

    let moves = balancer.get_balance_moves(&mut tables);

    // Should consolidate from 5 to 4 tables by emptying one table
    assert!(
        !moves.is_empty(),
        "Should generate moves to consolidate tables"
    );

    // All moves should be from a single table (either table1 or table2, the less populated ones)
    let from_table = moves[0].0;
    assert!(
        from_table == table1 || from_table == table2,
        "All moves should come from one of the less populated tables"
    );

    assert!(
        moves.iter().all(|(from, _)| *from == from_table),
        "All moves should come from the same table"
    );

    // Count total number of players moved (should equal the number of players in the source table)
    assert_eq!(
        moves.len(),
        5,
        "Should move all 5 players from the source table"
    );

    // Check that we don't exceed max capacity for any destination table
    let mut destination_count = HashMap::new();
    for (_, dest) in &moves {
        *destination_count.entry(*dest).or_insert(0) += 1;
    }

    for (dest, count) in destination_count {
        let dest_current_players = match tables.get(&dest) {
            Some(table_info) => table_info.players.len(),
            None => 0,
        };

        assert!(
            dest_current_players + count <= 8,
            "Destination table should not exceed max capacity of 8 players"
        );
    }
}

#[test]
fn test_consolidation_6_to_5_tables() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    // Create 6 tables with varying player counts
    let table1 = TableId(create_test_principal("table1")); // 4 players
    let table2 = TableId(create_test_principal("table2")); // 4 players
    let table3 = TableId(create_test_principal("table3")); // 5 players
    let table4 = TableId(create_test_principal("table4")); // 5 players
    let table5 = TableId(create_test_principal("table5")); // 5 players
    let table6 = TableId(create_test_principal("table6")); // 6 players

    tables.insert(table1, create_test_table_info(4));
    tables.insert(table2, create_test_table_info(4));
    tables.insert(table3, create_test_table_info(5));
    tables.insert(table4, create_test_table_info(5));
    tables.insert(table5, create_test_table_info(5));
    tables.insert(table6, create_test_table_info(6));

    let moves = balancer.get_balance_moves(&mut tables);

    // Should consolidate from 6 to 5 tables by emptying one table
    assert!(
        !moves.is_empty(),
        "Should generate moves to consolidate tables"
    );

    // All moves should be from one of the less populated tables (table1 or table2)
    let from_table = moves[0].0;
    assert!(
        from_table == table1 || from_table == table2,
        "All moves should come from one of the less populated tables"
    );

    assert!(
        moves.iter().all(|(from, _)| *from == from_table),
        "All moves should come from the same table"
    );

    // Count total number of players moved (should equal the number of players in the source table)
    assert_eq!(
        moves.len(),
        4,
        "Should move all 4 players from the source table"
    );

    // Check that we don't exceed max capacity for any destination table
    let mut destination_count = HashMap::new();
    for (_, dest) in &moves {
        *destination_count.entry(*dest).or_insert(0) += 1;
    }

    for (dest, count) in destination_count {
        let dest_current_players = match tables.get(&dest) {
            Some(table_info) => table_info.players.len(),
            None => 0,
        };

        assert!(
            dest_current_players + count <= 8,
            "Destination table should not exceed max capacity of 8 players"
        );
    }
}

#[test]
fn test_consolidation_with_near_max_tables() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    // Create tables with some near max capacity
    let table1 = TableId(create_test_principal("table1")); // 4 players
    let table2 = TableId(create_test_principal("table2")); // 7 players (near max)
    let table3 = TableId(create_test_principal("table3")); // 7 players (near max)
    let table4 = TableId(create_test_principal("table4")); // 5 players
    let table5 = TableId(create_test_principal("table5")); // 5 players

    tables.insert(table1, create_test_table_info(4));
    tables.insert(table2, create_test_table_info(7));
    tables.insert(table3, create_test_table_info(7));
    tables.insert(table4, create_test_table_info(5));
    tables.insert(table5, create_test_table_info(5));

    let moves = balancer.get_balance_moves(&mut tables);

    // Should consolidate by emptying the least populated table
    assert!(
        !moves.is_empty(),
        "Should generate moves to consolidate tables"
    );

    // All moves should be from the least populated table
    let from_table = moves[0].0;
    assert_eq!(
        from_table, table1,
        "All moves should come from the least populated table"
    );

    assert!(
        moves.iter().all(|(from, _)| *from == from_table),
        "All moves should come from the same table"
    );

    // Check that no players are moved to tables that are already near max
    for (_, dest) in &moves {
        assert!(
            *dest != table2 && *dest != table3,
            "Should not move players to tables that are already near max capacity"
        );
    }
}

#[test]
fn test_gradual_consolidation_with_multiple_candidates() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    // Create multiple tables with the same low player count
    let table1 = TableId(create_test_principal("table1")); // 3 players
    let table2 = TableId(create_test_principal("table2")); // 3 players
    let table3 = TableId(create_test_principal("table3")); // 3 players
    let table4 = TableId(create_test_principal("table4")); // 6 players
    let table5 = TableId(create_test_principal("table5")); // 6 players

    tables.insert(table1, create_test_table_info(3));
    tables.insert(table2, create_test_table_info(3));
    tables.insert(table3, create_test_table_info(3));
    tables.insert(table4, create_test_table_info(6));
    tables.insert(table5, create_test_table_info(6));

    let moves = balancer.get_balance_moves(&mut tables);

    // Should consolidate by emptying one of the least populated tables
    assert!(
        !moves.is_empty(),
        "Should generate moves to consolidate tables"
    );

    // All moves should be from one of the least populated tables
    let from_table = moves[0].0;
    assert!(
        from_table == table1 || from_table == table2 || from_table == table3,
        "All moves should come from one of the least populated tables"
    );

    assert!(
        moves.iter().all(|(from, _)| *from == from_table),
        "All moves should come from the same table"
    );

    // Count total number of players moved (should equal the number of players in the source table)
    assert_eq!(
        moves.len(),
        3,
        "Should move all 3 players from the source table"
    );
}

#[test]
fn test_consolidation_with_exact_fit() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    // Create a scenario where consolidation results in exactly filled tables
    let table1 = TableId(create_test_principal("table1")); // 3 players
    let table2 = TableId(create_test_principal("table2")); // 5 players
    let table3 = TableId(create_test_principal("table3")); // 5 players
    let table4 = TableId(create_test_principal("table4")); // 5 players
    let table5 = TableId(create_test_principal("table5")); // 6 players

    tables.insert(table1, create_test_table_info(3));
    tables.insert(table2, create_test_table_info(5));
    tables.insert(table3, create_test_table_info(5));
    tables.insert(table4, create_test_table_info(5));
    tables.insert(table5, create_test_table_info(6));

    let moves = balancer.get_balance_moves(&mut tables);

    // Should consolidate by emptying the least populated table
    assert!(
        !moves.is_empty(),
        "Should generate moves to consolidate tables"
    );

    // All moves should be from the least populated table
    let from_table = moves[0].0;
    assert_eq!(
        from_table, table1,
        "All moves should come from the least populated table"
    );

    assert!(
        moves.iter().all(|(from, _)| *from == from_table),
        "All moves should come from the same table"
    );

    // Count total number of players moved
    assert_eq!(
        moves.len(),
        3,
        "Should move all 3 players from the source table"
    );

    // Check distribution of moves - ideally moves are distributed to maintain balance
    let destination_counts = moves.iter().fold(HashMap::new(), |mut acc, (_, dest)| {
        *acc.entry(*dest).or_insert(0) += 1;
        acc
    });

    // Verify moves distribute players in a balanced way
    for (dest, count) in &destination_counts {
        let current_count = tables.get(dest).map_or(0, |info| info.players.len());
        assert!(
            current_count + count <= 8,
            "Destination table should not exceed max capacity"
        );
    }
}

#[test]
fn test_consolidation_with_even_distribution() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    // Create 5 tables with even distribution
    let table1 = TableId(create_test_principal("table1")); // 5 players
    let table2 = TableId(create_test_principal("table2")); // 5 players
    let table3 = TableId(create_test_principal("table3")); // 5 players
    let table4 = TableId(create_test_principal("table4")); // 5 players
    let table5 = TableId(create_test_principal("table5")); // 5 players

    tables.insert(table1, create_test_table_info(5));
    tables.insert(table2, create_test_table_info(5));
    tables.insert(table3, create_test_table_info(5));
    tables.insert(table4, create_test_table_info(5));
    tables.insert(table5, create_test_table_info(5));

    let moves = balancer.get_balance_moves(&mut tables);

    // Should consolidate from 5 to 4 tables
    assert!(
        !moves.is_empty(),
        "Should generate moves to consolidate tables"
    );

    // All moves should be from a single table
    let from_table = moves[0].0;
    assert!(
        moves.iter().all(|(from, _)| *from == from_table),
        "All moves should come from the same table"
    );

    // Count total number of players moved
    assert_eq!(
        moves.len(),
        5,
        "Should move all 5 players from the source table"
    );

    // Check that destination tables receive a balanced number of players
    let destination_counts = moves.iter().fold(HashMap::new(), |mut acc, (_, dest)| {
        *acc.entry(*dest).or_insert(0) += 1;
        acc
    });

    let max_players_added = destination_counts.values().max().cloned().unwrap_or(0);
    let min_players_added = destination_counts.values().min().cloned().unwrap_or(0);

    assert!(
        max_players_added - min_players_added <= 1,
        "Players should be distributed evenly across destination tables"
    );

    // The final distribution should be close to [0, 6, 6, 6, 7] or something similar
}

#[test]
fn test_consolidation_7_to_6_tables() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    // Create 7 tables
    let table1 = TableId(create_test_principal("table1")); // 4 players
    let table2 = TableId(create_test_principal("table2")); // 4 players
    let table3 = TableId(create_test_principal("table3")); // 5 players
    let table4 = TableId(create_test_principal("table4")); // 5 players
    let table5 = TableId(create_test_principal("table5")); // 5 players
    let table6 = TableId(create_test_principal("table6")); // 6 players
    let table7 = TableId(create_test_principal("table7")); // 6 players

    tables.insert(table1, create_test_table_info(4));
    tables.insert(table2, create_test_table_info(4));
    tables.insert(table3, create_test_table_info(5));
    tables.insert(table4, create_test_table_info(5));
    tables.insert(table5, create_test_table_info(5));
    tables.insert(table6, create_test_table_info(6));
    tables.insert(table7, create_test_table_info(6));

    let moves = balancer.get_balance_moves(&mut tables);

    // Should consolidate from 7 to 6 tables
    assert!(
        !moves.is_empty(),
        "Should generate moves to consolidate tables"
    );

    // All moves should be from one of the least populated tables
    let from_table = moves[0].0;
    assert!(
        from_table == table1 || from_table == table2,
        "All moves should come from one of the least populated tables"
    );

    assert!(
        moves.iter().all(|(from, _)| *from == from_table),
        "All moves should come from the same table"
    );

    // Count total number of players moved
    assert_eq!(
        moves.len(),
        4,
        "Should move all 4 players from the source table"
    );
}

#[test]
fn test_consolidation_with_one_player_difference() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    // Create tables with a one-player difference
    let table1 = TableId(create_test_principal("table1")); // 5 players
    let table2 = TableId(create_test_principal("table2")); // 6 players
    let table3 = TableId(create_test_principal("table3")); // 6 players
    let table4 = TableId(create_test_principal("table4")); // 6 players
    let table5 = TableId(create_test_principal("table5")); // 6 players

    tables.insert(table1, create_test_table_info(5));
    tables.insert(table2, create_test_table_info(6));
    tables.insert(table3, create_test_table_info(6));
    tables.insert(table4, create_test_table_info(6));
    tables.insert(table5, create_test_table_info(6));

    let moves = balancer.get_balance_moves(&mut tables);

    // Should consolidate from 5 to 4 tables by emptying the least populated table
    assert!(
        !moves.is_empty(),
        "Should generate moves to consolidate tables"
    );

    // All moves should be from the least populated table
    let from_table = moves[0].0;
    assert_eq!(
        from_table, table1,
        "All moves should come from the least populated table"
    );

    assert!(
        moves.iter().all(|(from, _)| *from == from_table),
        "All moves should come from the same table"
    );

    // Count total number of players moved
    assert_eq!(
        moves.len(),
        5,
        "Should move all 5 players from the source table"
    );
}

#[test]
fn test_consolidation_with_multiple_min_and_max_tables() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    // Create tables with both minimum and maximum player counts
    let table1 = TableId(create_test_principal("table1")); // 2 players (minimum)
    let table2 = TableId(create_test_principal("table2")); // 2 players (minimum)
    let table3 = TableId(create_test_principal("table3")); // 5 players
    let table4 = TableId(create_test_principal("table4")); // 8 players (maximum)
    let table5 = TableId(create_test_principal("table5")); // 8 players (maximum)

    tables.insert(table1, create_test_table_info(2));
    tables.insert(table2, create_test_table_info(2));
    tables.insert(table3, create_test_table_info(5));
    tables.insert(table4, create_test_table_info(8));
    tables.insert(table5, create_test_table_info(8));

    let moves = balancer.get_balance_moves(&mut tables);

    // Should consolidate the two minimum tables
    assert!(
        !moves.is_empty(),
        "Should generate moves to consolidate tables"
    );

    // All moves should be from one of the minimum tables
    let from_table = moves[0].0;
    assert!(
        from_table == table1 || from_table == table2,
        "All moves should come from one of the minimum tables"
    );

    assert!(
        moves.iter().all(|(from, _)| *from == from_table),
        "All moves should come from the same table"
    );

    // No moves should go to the maximum tables
    for (_, dest) in &moves {
        assert!(
            *dest != table4 && *dest != table5,
            "No players should be moved to tables that are already at max capacity"
        );
    }

    // Should move all players from one minimum table to the other or to the medium table
    assert_eq!(
        moves.len(),
        2,
        "Should move both players from the source table"
    );
}

#[test]
fn test_scenario_with_uneven_distribution_4_tables() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    // Create an uneven distribution of players
    let table1 = TableId(create_test_principal("table1")); // 3 players
    let table2 = TableId(create_test_principal("table2")); // 5 players
    let table3 = TableId(create_test_principal("table3")); // 7 players
    let table4 = TableId(create_test_principal("table4")); // 7 players

    tables.insert(table1, create_test_table_info(3));
    tables.insert(table2, create_test_table_info(5));
    tables.insert(table3, create_test_table_info(7));
    tables.insert(table4, create_test_table_info(7));

    let moves = balancer.get_balance_moves(&mut tables);

    // Should consolidate to 3 tables
    assert!(
        !moves.is_empty(),
        "Should generate moves to consolidate tables"
    );

    // All moves should be from the least populated table
    let from_table = moves[0].0;
    assert_eq!(
        from_table, table1,
        "All moves should come from the least populated table"
    );

    assert!(
        moves.iter().all(|(from, _)| *from == from_table),
        "All moves should come from the same table"
    );

    // The moves should be to the second-least populated table
    assert!(
        moves.iter().all(|(_, to)| *to == table2),
        "All moves should go to the second-least populated table"
    );

    // Should move all players from the least populated table
    assert_eq!(
        moves.len(),
        3,
        "Should move all 3 players from the source table"
    );
}

#[test]
fn test_scenario_57777_distribution() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    // Create a scenario with one less populated table and many tables at capacity
    let table1 = TableId(create_test_principal("table1")); // 5 players
    let table2 = TableId(create_test_principal("table2")); // 7 players
    let table3 = TableId(create_test_principal("table3")); // 7 players
    let table4 = TableId(create_test_principal("table4")); // 7 players
    let table5 = TableId(create_test_principal("table5")); // 7 players
    let table6 = TableId(create_test_principal("table6")); // 7 players

    tables.insert(table1, create_test_table_info(5));
    tables.insert(table2, create_test_table_info(7));
    tables.insert(table3, create_test_table_info(7));
    tables.insert(table4, create_test_table_info(7));
    tables.insert(table5, create_test_table_info(7));
    tables.insert(table6, create_test_table_info(7));

    let moves = balancer.get_balance_moves(&mut tables);

    // Should consolidate to 4 tables
    assert!(
        !moves.is_empty(),
        "Should generate moves to consolidate tables"
    );

    // All moves should be from the least populated table
    let from_table = moves[0].0;
    assert_eq!(
        from_table, table1,
        "All moves should come from the least populated table"
    );

    assert!(
        moves.iter().all(|(from, _)| *from == from_table),
        "All moves should come from the same table"
    );

    // Should move all players from the least populated table
    assert_eq!(
        moves.len(),
        5,
        "Should move all 5 players from the source table"
    );

    // Players should be distributed optimally among destinations
    let destination_counts = moves.iter().fold(HashMap::new(), |mut acc, (_, dest)| {
        *acc.entry(*dest).or_insert(0) += 1;
        acc
    });

    // Each destination table should receive at most 1 player to avoid exceeding max capacity
    for count in destination_counts.values() {
        assert!(
            count <= &1,
            "Each destination table should receive at most 1 player"
        );
    }

    // Should distribute to multiple tables
    assert!(
        destination_counts.len() > 1,
        "Should distribute players to multiple destination tables"
    );
}

#[test]
fn test_scenario_55788_distribution() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    // Create a scenario with mixed distribution including max tables
    let table1 = TableId(create_test_principal("table1")); // 5 players
    let table2 = TableId(create_test_principal("table2")); // 5 players
    let table3 = TableId(create_test_principal("table3")); // 6 players
    let table4 = TableId(create_test_principal("table4")); // 8 players (max)
    let table5 = TableId(create_test_principal("table5")); // 8 players (max)

    tables.insert(table1, create_test_table_info(5));
    tables.insert(table2, create_test_table_info(5));
    tables.insert(table3, create_test_table_info(6));
    tables.insert(table4, create_test_table_info(8));
    tables.insert(table5, create_test_table_info(8));

    let moves = balancer.get_balance_moves(&mut tables);

    // Should consolidate to 4 tables
    assert!(
        !moves.is_empty(),
        "Should generate moves to consolidate tables"
    );

    // All moves should be from one of the least populated tables
    let from_table = moves[0].0;
    assert!(
        from_table == table1 || from_table == table2,
        "All moves should come from one of the least populated tables"
    );

    assert!(
        moves.iter().all(|(from, _)| *from == from_table),
        "All moves should come from the same table"
    );

    // No moves should go to the maximum tables
    for (_, dest) in &moves {
        assert!(
            *dest != table4 && *dest != table5,
            "No players should be moved to tables that are already at max capacity"
        );
    }

    // All moves should go to the middle table
    assert!(
        moves.iter().all(|(_, to)| *to == table3
            || (*to == table1 && from_table == table2)
            || (*to == table2 && from_table == table1)),
        "All moves should go to available tables"
    );

    // Should move all players from the source table
    assert_eq!(
        moves.len(),
        5,
        "Should move all 5 players from the source table"
    );
}

#[test]
fn test_specific_edge_case_2367_distribution() {
    let balancer = TableBalancer::new(2, 8, &SpeedType::new_regular(1000, 100));
    let mut tables = HashMap::new();

    // Create a specific edge case scenario
    let table1 = TableId(create_test_principal("table1")); // 2 players (min)
    let table2 = TableId(create_test_principal("table2")); // 3 players
    let table3 = TableId(create_test_principal("table3")); // 6 players
    let table4 = TableId(create_test_principal("table4")); // 7 players

    tables.insert(table1, create_test_table_info(2));
    tables.insert(table2, create_test_table_info(3));
    tables.insert(table3, create_test_table_info(6));
    tables.insert(table4, create_test_table_info(7));

    let moves = balancer.get_balance_moves(&mut tables);

    // Should consolidate by emptying one of the smaller tables
    assert!(
        !moves.is_empty(),
        "Should generate moves to consolidate tables"
    );

    // All moves should be from the smallest table
    let from_table = moves[0].0;
    assert_eq!(
        from_table, table1,
        "All moves should come from the smallest table"
    );

    assert!(
        moves.iter().all(|(from, _)| *from == from_table),
        "All moves should come from the same table"
    );

    // Moves should go to the second smallest table
    assert!(
        moves.iter().all(|(_, to)| *to == table2),
        "All moves should go to the second smallest table"
    );

    // Should move both players from the smallest table
    assert_eq!(
        moves.len(),
        2,
        "Should move both players from the smallest table"
    );
}
