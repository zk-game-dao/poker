use candid::Principal;

use crate::poker::game::{
    table_functions::{
        action_log::ActionType,
        table::Table,
        tests::{create_user, get_table_config, turn_tests::is_it_users_turn},
        types::{BetType, DealStage, PlayerAction},
    },
    types::GameType,
    utils::convert_to_e8s,
};

pub fn print_action_logs(table: &Table) {
    for log in table.action_logs.iter() {
        println!(
            "{}: {:#?}",
            log.user_principal
                .unwrap_or_else(Principal::anonymous)
                .to_text(),
            log.action_type
        )
    }
}

#[test]
fn test_raising_action_log() {
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

    assert!(is_it_users_turn(&table, small_blind_uid));
    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(3.0))),
        Ok(())
    );
    assert_eq!(
        table.action_logs[table.action_logs.len() - 1].action_type,
        ActionType::Raise {
            amount: convert_to_e8s(3.0)
        }
    );
    assert!(is_it_users_turn(&table, big_blind_uid));
    assert_eq!(
        table.bet(big_blind_uid, BetType::Raised(convert_to_e8s(6.0))),
        Ok(())
    );
    assert_eq!(
        table.action_logs[table.action_logs.len() - 1].action_type,
        ActionType::Raise {
            amount: convert_to_e8s(6.0)
        }
    );
}

#[test]
fn test_raising_on_raising_action_log() {
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

    assert!(is_it_users_turn(&table, small_blind_uid));
    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(3.0))),
        Ok(())
    );
    assert_eq!(
        table.action_logs[table.action_logs.len() - 1].action_type,
        ActionType::Raise {
            amount: convert_to_e8s(3.0)
        }
    );
    assert!(is_it_users_turn(&table, big_blind_uid));
    assert_eq!(
        table.bet(big_blind_uid, BetType::Raised(convert_to_e8s(6.0))),
        Ok(())
    );
    assert_eq!(
        table.action_logs[table.action_logs.len() - 1].action_type,
        ActionType::Raise {
            amount: convert_to_e8s(6.0)
        }
    );
    assert!(is_it_users_turn(&table, small_blind_uid));
    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(12.0))),
        Ok(())
    );
    assert_eq!(
        table.action_logs[table.action_logs.len() - 1].action_type,
        ActionType::Raise {
            amount: convert_to_e8s(12.0)
        }
    );
    assert!(is_it_users_turn(&table, big_blind_uid));
    assert_eq!(
        table.bet(big_blind_uid, BetType::Raised(convert_to_e8s(24.0))),
        Ok(())
    );
    assert_eq!(
        table.action_logs[table.action_logs.len() - 1].action_type,
        ActionType::Raise {
            amount: convert_to_e8s(24.0)
        }
    );
}

#[test]
fn test_all_in_action_log() {
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

    let small_blind_uid = table.get_small_blind_user_principal().unwrap();

    assert!(is_it_users_turn(&table, small_blind_uid));
    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(100.0))),
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
        table.action_logs[table.action_logs.len() - 1].action_type,
        ActionType::AllIn {
            amount: convert_to_e8s(100.0)
        }
    );
}

#[test]
fn test_calling_all_in_action_log() {
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
    table.users.get_mut(&small_blind_uid).unwrap().balance = convert_to_e8s(200.0);

    assert!(is_it_users_turn(&table, small_blind_uid));
    assert_eq!(
        table.bet(small_blind_uid, BetType::Raised(convert_to_e8s(100.0))),
        Ok(())
    );
    assert_eq!(
        table.action_logs[table.action_logs.len() - 1].action_type,
        ActionType::Raise {
            amount: convert_to_e8s(100.0)
        }
    );

    assert!(is_it_users_turn(&table, big_blind_uid));
    assert_eq!(table.bet(big_blind_uid, BetType::Called), Ok(()));
    assert_eq!(
        table
            .user_table_data
            .get(&big_blind_uid)
            .unwrap()
            .player_action,
        PlayerAction::AllIn
    );
    println!("{:#?}", table.action_logs);
    assert_eq!(
        table.action_logs[table.action_logs.len() - 5].action_type,
        ActionType::AllIn {
            amount: convert_to_e8s(100.0)
        }
    );
    assert_eq!(
        table.action_logs[table.action_logs.len() - 1].action_type,
        ActionType::Stage {
            stage: DealStage::Showdown
        }
    );
}
