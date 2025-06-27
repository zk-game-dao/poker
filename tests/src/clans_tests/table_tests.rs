use candid::Principal;
use clan::subscriptions::ClanRole;
use currency::Currency;
use errors::clan_error::ClanError;
use serial_test::serial;
use table::poker::game::{
    table_functions::types::CurrencyType,
    utils::convert_to_e8s,
};
use user::user::WalletPrincipalId;

use crate::TestEnv;

#[test]
#[serial]
fn test_create_clan_table_success() {
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // Create clan
    let (clan, creator_id, _) = test_env.create_test_clan("Table Test Clan", "table_creator");

    // Get a test table config (using the same pattern as regular table tests)
    let (_, table_config) = test_env.get_test_icp_table();

    // Create clan table using the config
    let result = test_env.create_clan_table(clan.id.0, &table_config, creator_id);
    println!("Create clan table result: {:?}", result);
    assert!(result.is_ok(), "Failed to create clan table");

    let table = result.unwrap();
    assert_eq!(table.config.currency_type, CurrencyType::Real(Currency::ICP));

    // Verify table is added to clan's table list
    let clan_tables = test_env
        .get_clan_tables(clan.id.0)
        .expect("Failed to get clan tables");
    assert_eq!(clan_tables.len(), 1);
    assert!(clan_tables.contains(&table.id));
}

#[test]
#[serial]
fn test_create_clan_table_insufficient_permissions() {
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // Create clan with owner and regular member
    let (clan, creator_id, _) = test_env.create_test_clan("Permission Test Clan", "perm_creator");

    // Add a regular member
    let (member_id, _) = test_env.create_user_and_join_clan(
        clan.id.0,
        "regular_member",
        0,
    );

    // Get table config
    let (_, table_config) = test_env.get_test_icp_table();

    // Regular member tries to create table (should fail)
    let result = test_env.create_clan_table(clan.id.0, &table_config, member_id);
    assert!(matches!(result, Err(ClanError::InsufficientPermissions)));

    // Owner creates table (should succeed)
    let result = test_env.create_clan_table(clan.id.0, &table_config, creator_id);
    assert!(result.is_ok());
}

#[test]
#[serial]
fn test_remove_clan_table() {
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // Create clan and table
    let (clan, creator_id, _) = test_env.create_test_clan("Remove Table Clan", "remove_creator");

    let (_, table_config) = test_env.get_test_icp_table();
    let table = test_env
        .create_clan_table(clan.id.0, &table_config, creator_id)
        .expect("Failed to create table");

    // Verify table exists
    let clan_tables = test_env
        .get_clan_tables(clan.id.0)
        .expect("Failed to get clan tables");
    assert_eq!(clan_tables.len(), 1);

    // Remove table
    let result = test_env.remove_clan_table(clan.id.0, table.id, creator_id);
    assert!(result.is_ok());

    // Verify table removed
    let clan_tables = test_env
        .get_clan_tables(clan.id.0)
        .expect("Failed to get clan tables");
    assert_eq!(clan_tables.len(), 0);
}

#[test]
#[serial]
fn test_remove_table_insufficient_permissions() {
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // Create clan with table
    let (clan, creator_id, _) = test_env.create_test_clan("Remove Perm Clan", "remove_perm_creator");

    let (_, table_config) = test_env.get_test_icp_table();
    let table = test_env
        .create_clan_table(clan.id.0, &table_config, creator_id)
        .expect("Failed to create table");

    // Add regular member
    let (member_id, _) = test_env.create_user_and_join_clan(
        clan.id.0,
        "remove_member",
        0,
    );

    // Regular member tries to remove table (should fail)
    let result = test_env.remove_clan_table(clan.id.0, table.id, member_id);
    assert!(matches!(result, Err(ClanError::InsufficientPermissions)));

    // Verify table still exists
    let clan_tables = test_env
        .get_clan_tables(clan.id.0)
        .expect("Failed to get clan tables");
    assert_eq!(clan_tables.len(), 1);
}

#[test]
#[serial]
fn test_admin_can_manage_tables() {
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // Create clan
    let (clan, creator_id, _) = test_env.create_test_clan("Admin Table Clan", "admin_creator");

    // Add member and promote to admin
    let (admin_id, _) = test_env.create_user_and_join_clan(
        clan.id.0,
        "table_admin",
        0,
    );

    test_env
        .update_member_role(clan.id.0, admin_id, ClanRole::Admin, creator_id)
        .expect("Failed to promote to admin");

    // Admin creates table
    let (_, table_config) = test_env.get_test_icp_table();
    let table = test_env
        .create_clan_table(clan.id.0, &table_config, admin_id)
        .expect("Admin should be able to create tables");

    // Admin removes table
    let result = test_env.remove_clan_table(clan.id.0, table.id, admin_id);
    assert!(result.is_ok(), "Admin should be able to remove tables");
}

#[test]
#[serial]
fn test_clan_members_can_join_clan_tables() {
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // Create clan
    let (clan, creator_id, _) = test_env.create_test_clan("Join Table Clan", "join_creator");

    // Create clan table
    let (_, table_config) = test_env.get_test_icp_table();
    let clan_table = test_env
        .create_clan_table(clan.id.0, &table_config, creator_id)
        .expect("Failed to create clan table");

    // Add member with ICP approval for table
    let member_id = WalletPrincipalId(Principal::self_authenticating("table_joiner"));
    let member_user = test_env
        .create_user("Table Joiner".to_string(), member_id)
        .expect("Failed to create user");

    test_env
        .join_clan(clan.id.0, member_user.users_canister_id, member_id, 0)
        .expect("Failed to join clan");

    // Give member ICP and approve table canister
    let (_, member_id_typed, _) = test_env.create_test_user_with_icp_approval(
        "table_joiner 1".to_string(),
        1000.0,
        clan_table.id,
    );

    // Member joins clan table
    let updated_table = test_env
        .join_test_table(
            clan_table.id,
            member_user.users_canister_id,
            member_id_typed,
            convert_to_e8s(100.0),
            0,
            false,
        )
        .expect("Failed to join clan table");

    assert_eq!(updated_table.users.len(), 1);

    // Clean up - member leaves table
    let _ = test_env
        .leave_test_table(updated_table.id, member_user.users_canister_id, member_id_typed)
        .expect("Failed to leave table");
}

#[test]
#[serial]
fn test_table_access_based_on_subscription() {
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // Create clan
    let (clan, _, _) = test_env.create_test_clan("Subscription Table Clan", "sub_creator");

    // Add a regular member
    let (member_id, _) = test_env.create_user_and_join_clan(
        clan.id.0,
        "sub_member",
        0,
    );

    // Test access with different stake levels
    let low_stakes = 10000; // Should be accessible to basic members
    let high_stakes = 200000; // Should require premium subscription

    // Member should be able to access low stakes
    let can_access_low = test_env
        .can_member_access_table(clan.id.0, member_id, low_stakes)
        .expect("Failed to check table access");
    assert!(can_access_low, "Member should access low stakes tables");

    // Member should NOT be able to access high stakes without premium subscription
    let can_access_high = test_env
        .can_member_access_table(clan.id.0, member_id, high_stakes)
        .expect("Failed to check table access");
    assert!(!can_access_high, "Member should NOT access high stakes tables without premium");
}

#[test]
#[serial]
fn test_get_empty_clan_tables() {
    let test_env = TestEnv::new(Some(100_000_000_000_000));

    // Create clan without any tables
    let (clan, _, _) = test_env.create_test_clan("Empty Table Clan", "empty_creator");

    // Get tables (should be empty)
    let clan_tables = test_env
        .get_clan_tables(clan.id.0)
        .expect("Failed to get clan tables");
    assert_eq!(clan_tables.len(), 0);
}
