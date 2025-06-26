use std::collections::HashSet;

use candid::{decode_one, encode_args, Principal};
use clan::{
    member::MemberStatus, subscriptions::ClanRole, Clan, ClanPrivacy, CreateClanRequest
};
use currency::Currency;
use errors::clan_error::ClanError;
use serial_test::serial;
use user::user::{User, WalletPrincipalId};

use crate::TestEnv;

#[test]
#[serial]
fn test_create_clan_success() {
    let test_env = TestEnv::new(None);

    let creator_id = WalletPrincipalId(Principal::self_authenticating("creator"));
    let creator_user = test_env
        .create_user("Creator User".to_string(), creator_id)
        .expect("Failed to create user");

    let clan_request = CreateClanRequest {
        name: "Test Clan".to_string(),
        description: "A test clan for testing purposes".to_string(),
        tag: "TEST".to_string(),
        privacy: ClanPrivacy::Public,
        supported_currency: Currency::ICP,
        joining_fee: 1000,
        require_proof_of_humanity: false,
        minimum_level_required: None,
        minimum_experience_points: None,
        member_limit: Some(100),
        avatar: None,
        website: Some("https://testclan.com".to_string()),
        discord: Some("testclan_discord".to_string()),
        twitter: Some("@testclan".to_string()),
    };

    let clan = test_env
        .create_clan(&clan_request, creator_id, creator_user.users_canister_id)
        .expect("Failed to create clan");

    assert_eq!(clan.name, "Test Clan");
    assert_eq!(clan.description, "A test clan for testing purposes");
    assert_eq!(clan.privacy, ClanPrivacy::Public);
    assert_eq!(clan.supported_currency, Currency::ICP);
    assert_eq!(clan.joining_fee, 1000);
    assert!(!clan.require_proof_of_humanity);
    assert_eq!(clan.member_limit, 100);
    assert_eq!(clan.members.len(), 1); // Creator should be added automatically
    assert_eq!(
        clan.members.get(&creator_id).unwrap().role,
        ClanRole::Owner
    );
    assert_eq!(clan.website, Some("https://testclan.com".to_string()));
    assert_eq!(clan.discord, Some("testclan_discord".to_string()));
    assert_eq!(clan.twitter, Some("@testclan".to_string()));
}

#[test]
#[serial]
fn test_create_clan_invalid_name() {
    let test_env = TestEnv::new(None);

    let creator_id = WalletPrincipalId(Principal::self_authenticating("creator_invalid"));
    let creator_user = test_env
        .create_user("Creator User".to_string(), creator_id)
        .expect("Failed to create user");

    // Test empty name
    let clan_request = CreateClanRequest {
        name: "".to_string(), // Empty name should fail
        description: "A test clan".to_string(),
        tag: "TEST".to_string(),
        privacy: ClanPrivacy::Public,
        supported_currency: Currency::ICP,
        joining_fee: 0,
        require_proof_of_humanity: false,
        minimum_level_required: None,
        minimum_experience_points: None,
        member_limit: None,
        avatar: None,
        website: None,
        discord: None,
        twitter: None,
    };

    let result = test_env.create_clan(&clan_request, creator_id, creator_user.users_canister_id);
    assert!(result.is_err());

    // Test name too long
    let clan_request = CreateClanRequest {
        name: "A".repeat(51), // 51 characters should fail (max is 50)
        description: "A test clan".to_string(),
        tag: "TEST".to_string(),
        privacy: ClanPrivacy::Public,
        supported_currency: Currency::ICP,
        joining_fee: 0,
        require_proof_of_humanity: false,
        minimum_level_required: None,
        minimum_experience_points: None,
        member_limit: None,
        avatar: None,
        website: None,
        discord: None,
        twitter: None,
    };

    let result = test_env.create_clan(&clan_request, creator_id, creator_user.users_canister_id);
    assert!(result.is_err());
}

#[test]
#[serial]
fn test_create_clan_invalid_description() {
    let test_env = TestEnv::new(None);

    let creator_id = WalletPrincipalId(Principal::self_authenticating("creator_desc"));
    let creator_user = test_env
        .create_user("Creator User".to_string(), creator_id)
        .expect("Failed to create user");

    // Test description too long
    let clan_request = CreateClanRequest {
        name: "Test Clan".to_string(),
        description: "A".repeat(501), // 501 characters should fail (max is 500)
        tag: "TEST".to_string(),
        privacy: ClanPrivacy::Public,
        supported_currency: Currency::ICP,
        joining_fee: 0,
        require_proof_of_humanity: false,
        minimum_level_required: None,
        minimum_experience_points: None,
        member_limit: None,
        avatar: None,
        website: None,
        discord: None,
        twitter: None,
    };

    let result = test_env.create_clan(&clan_request, creator_id, creator_user.users_canister_id);
    assert!(result.is_err());
}

#[test]
#[serial]
fn test_join_clan_public() {
    let test_env = TestEnv::new(None);

    // Create clan
    let creator_id = WalletPrincipalId(Principal::self_authenticating("creator_public"));
    let creator_user = test_env
        .create_user("Creator User 1".to_string(), creator_id)
        .expect("Failed to create user");

    let clan_request = CreateClanRequest {
        name: "Public Clan".to_string(),
        description: "A public test clan".to_string(),
        tag: "PUB".to_string(),
        privacy: ClanPrivacy::Public,
        supported_currency: Currency::ICP,
        joining_fee: 0, // No fee for easier testing
        require_proof_of_humanity: false,
        minimum_level_required: None,
        minimum_experience_points: None,
        member_limit: Some(10),
        avatar: None,
        website: None,
        discord: None,
        twitter: None,
    };

    let clan_canister = test_env
        .create_clan(&clan_request, creator_id, creator_user.users_canister_id)
        .expect("Failed to create clan");

    // Create new user to join clan
    let joiner_id = WalletPrincipalId(Principal::self_authenticating("joiner"));
    let joiner_user = test_env
        .create_user("Joiner User".to_string(), joiner_id)
        .expect("Failed to create user");

    // Join clan
    let result = test_env.join_clan(
        clan_canister.id.0,
        joiner_user.users_canister_id,
        joiner_id,
        0, // No joining fee
    );
    assert!(result.is_ok());

    // Verify membership
    let clan = test_env.get_clan(clan_canister.id.0).expect("Failed to get clan");
    assert_eq!(clan.members.len(), 2); // Creator + joiner
    assert!(clan.is_member(&joiner_id));
    assert_eq!(
        clan.members.get(&joiner_id).unwrap().role,
        ClanRole::Member
    );
}

#[test]
#[serial]
fn test_join_clan_with_fee() {
    let test_env = TestEnv::new(None);

    // Create clan with joining fee
    let creator_id = WalletPrincipalId(Principal::self_authenticating("creator_fee"));
    let creator_user = test_env
        .create_user("Creator User 2".to_string(), creator_id)
        .expect("Failed to create user");

    let joining_fee = 1000000; // 0.01 ICP in e8s
    let clan_request = CreateClanRequest {
        name: "Fee Clan".to_string(),
        description: "A clan with joining fee".to_string(),
        tag: "FEE".to_string(),
        privacy: ClanPrivacy::Public,
        supported_currency: Currency::ICP,
        joining_fee,
        require_proof_of_humanity: false,
        minimum_level_required: None,
        minimum_experience_points: None,
        member_limit: Some(10),
        avatar: None,
        website: None,
        discord: None,
        twitter: None,
    };

    let clan_canister = test_env
        .create_clan(&clan_request, creator_id, creator_user.users_canister_id)
        .expect("Failed to create clan");

    // Create new user with tokens
    let joiner_id = WalletPrincipalId(Principal::self_authenticating("joiner_fee"));
    let joiner_user = test_env
        .create_user("Joiner User 2".to_string(), joiner_id)
        .expect("Failed to create user");

    // Give user tokens and approve clan canister
    test_env.transfer_approve_tokens_for_testing(
        clan_canister.id.0,
        joiner_id,
        0.1, // 0.1 ICP
        true,
    );

    // Join clan with fee
    let result = test_env.join_clan(
        clan_canister.id.0,
        joiner_user.users_canister_id,
        joiner_id,
        joining_fee,
    );
    assert!(result.is_ok());

    // Verify membership and treasury
    let clan = test_env.get_clan(clan_canister.id.0).expect("Failed to get clan");
    assert!(clan.is_member(&joiner_id));
    assert_eq!(clan.treasury.balance, joining_fee);
    assert_eq!(clan.treasury.total_joining_fees_collected, joining_fee);
}

#[test]
#[serial]
fn test_join_clan_insufficient_fee() {
    let test_env = TestEnv::new(None);

    // Create clan with joining fee
    let creator_id = WalletPrincipalId(Principal::self_authenticating("creator_insufficient"));
    let creator_user = test_env
        .create_user("Creator User 3".to_string(), creator_id)
        .expect("Failed to create user");

    let joining_fee = 1000000; // 0.01 ICP
    let clan_request = CreateClanRequest {
        name: "Fee Clan".to_string(),
        description: "A clan with joining fee".to_string(),
        tag: "FEE".to_string(),
        privacy: ClanPrivacy::Public,
        supported_currency: Currency::ICP,
        joining_fee,
        require_proof_of_humanity: false,
        minimum_level_required: None,
        minimum_experience_points: None,
        member_limit: Some(10),
        avatar: None,
        website: None,
        discord: None,
        twitter: None,
    };

    let clan_canister = test_env
        .create_clan(&clan_request, creator_id, creator_user.users_canister_id)
        .expect("Failed to create clan");

    let joiner_id = WalletPrincipalId(Principal::self_authenticating("joiner_insufficient"));
    let joiner_user = test_env
        .create_user("Joiner User 3".to_string(), joiner_id)
        .expect("Failed to create user");

    // Try to join with insufficient fee
    let result = test_env.join_clan(
        clan_canister.id.0,
        joiner_user.users_canister_id,
        joiner_id,
        joining_fee - 1, // Less than required
    );
    assert!(result.is_err());

    // Verify user not added to clan
    let clan = test_env.get_clan(clan_canister.id.0).expect("Failed to get clan");
    assert!(!clan.is_member(&joiner_id));
}

#[test]
#[serial]
fn test_leave_clan() {
    let test_env = TestEnv::new(None);

    // Setup clan and member
    let (clan_canister, _, joiner_id, _) = setup_clan_with_member(&test_env, "leave_test_joiner");

    // Leave clan
    let result = test_env.leave_clan(clan_canister.id.0, joiner_id);
    assert!(result.is_ok());

    // Verify member removed
    let clan = test_env.get_clan(clan_canister.id.0).expect("Failed to get clan");
    assert!(!clan.is_member(&joiner_id));
    assert_eq!(clan.members.len(), 1); // Only creator left
}

#[test]
#[serial]
fn test_kick_member() {
    let test_env = TestEnv::new(None);

    // Setup clan and member
    let (clan_canister, creator_id, joiner_id, _) = setup_clan_with_member(&test_env, "kick_test_joiner");

    // Creator kicks member
    let result = test_env.kick_member(clan_canister.id.0, joiner_id, creator_id);
    assert!(result.is_ok());

    // Verify member removed
    let clan = test_env.get_clan(clan_canister.id.0).expect("Failed to get clan");
    assert!(!clan.is_member(&joiner_id));
}

#[test]
#[serial]
fn test_kick_member_insufficient_permissions() {
    let test_env = TestEnv::new(None);

    // Setup clan and member
    let (clan_canister, creator_id, joiner_id, _) = setup_clan_with_member(&test_env, "kick_test_joiner_insufficient");

    // Regular member tries to kick creator (should fail)
    let result = test_env.kick_member(clan_canister.id.0, creator_id, joiner_id);
    assert!(matches!(result, Err(ClanError::InsufficientPermissions)));

    // Verify creator still in clan
    let clan = test_env.get_clan(clan_canister.id.0).expect("Failed to get clan");
    assert!(clan.is_member(&creator_id));
}

#[test]
#[serial]
fn test_cannot_kick_owner() {
    let test_env = TestEnv::new(None);

    // Setup clan with two members
    let (clan_canister, creator_id, joiner_id, _) = setup_clan_with_member(&test_env, "owner_kick_test_joiner");

    // Promote joiner to admin
    let result = test_env.update_member_role(
        clan_canister.id.0,
        joiner_id,
        ClanRole::Admin,
        creator_id,
    );
    assert!(result.is_ok());

    // Admin tries to kick owner (should fail)
    let result = test_env.kick_member(clan_canister.id.0, creator_id, joiner_id);
    assert!(result.is_err());
}

#[test]
#[serial]
fn test_update_member_role() {
    let test_env = TestEnv::new(None);

    // Setup clan and member
    let (clan_canister, creator_id, joiner_id, _) = setup_clan_with_member(&test_env, "update_role_test_joiner");

    // Promote member to admin
    let result = test_env.update_member_role(
        clan_canister.id.0,
        joiner_id,
        ClanRole::Admin,
        creator_id,
    );
    assert!(result.is_ok());

    // Verify role updated
    let clan = test_env.get_clan(clan_canister.id.0).expect("Failed to get clan");
    assert_eq!(clan.members.get(&joiner_id).unwrap().role, ClanRole::Admin);
}

#[test]
#[serial]
fn test_update_member_role_insufficient_permissions() {
    let test_env = TestEnv::new(None);

    // Setup clan and member
    let (clan_canister, _, joiner_id, _) = setup_clan_with_member(&test_env, "update_role_insufficient_joiner");

    // Regular member tries to promote themselves
    let result = test_env.update_member_role(
        clan_canister.id.0,
        joiner_id,
        ClanRole::Admin,
        joiner_id, // Member trying to promote themselves
    );
    assert!(result.is_err());
}

#[test]
#[serial]
fn test_suspend_member() {
    let test_env = TestEnv::new(None);

    // Setup clan and member
    let (clan_canister, creator_id, joiner_id, _) = setup_clan_with_member(&test_env, "suspend_test_joiner");

    let suspension_end = ic_cdk::api::time() + 86_400_000_000_000; // 1 day from now

    // Suspend member
    let result = test_env.suspend_member(
        clan_canister.id.0,
        joiner_id,
        creator_id,
        Some(suspension_end),
    );
    assert!(result.is_ok());

    // Verify suspension
    let clan = test_env.get_clan(clan_canister.id.0).expect("Failed to get clan");
    let member = clan.members.get(&joiner_id).unwrap();
    assert!(matches!(
        member.status,
        MemberStatus::Suspended { until: Some(_) }
    ));
}

#[test]
#[serial]
fn test_clan_at_capacity() {
    let test_env = TestEnv::new(None);

    // Create clan with limit of 2 members
    let creator_id = WalletPrincipalId(Principal::self_authenticating("creator_capacity"));
    let creator_user = test_env
        .create_user("Creator User 4".to_string(), creator_id)
        .expect("Failed to create user");

    let clan_request = CreateClanRequest {
        name: "Small Clan".to_string(),
        description: "A clan with low capacity".to_string(),
        tag: "SMALL".to_string(),
        privacy: ClanPrivacy::Public,
        supported_currency: Currency::ICP,
        joining_fee: 0,
        require_proof_of_humanity: false,
        minimum_level_required: None,
        minimum_experience_points: None,
        member_limit: Some(2), // Only creator + 1 member
        avatar: None,
        website: None,
        discord: None,
        twitter: None,
    };

    let clan_canister = test_env
        .create_clan(&clan_request, creator_id, creator_user.users_canister_id)
        .expect("Failed to create clan");

    // Add first member (should succeed)
    let member1_id = WalletPrincipalId(Principal::self_authenticating("member1"));
    let member1_user = test_env
        .create_user("Member 1".to_string(), member1_id)
        .expect("Failed to create user");

    let result = test_env.join_clan(
        clan_canister.id.0,
        member1_user.users_canister_id,
        member1_id,
        0,
    );
    assert!(result.is_ok());

    // Try to add second member (should fail - clan is full)
    let member2_id = WalletPrincipalId(Principal::self_authenticating("member2"));
    let member2_user = test_env
        .create_user("Member 2".to_string(), member2_id)
        .expect("Failed to create user");

    let result = test_env.join_clan(
        clan_canister.id.0,
        member2_user.users_canister_id,
        member2_id,
        0,
    );
    assert!(matches!(result, Err(ClanError::ClanFull(_))));
}

#[test]
#[serial]
fn test_user_already_member() {
    let test_env = TestEnv::new(None);

    // Setup clan and member
    let (clan_canister, _, joiner_id, joiner_user) = setup_clan_with_member(&test_env, "already_member_joiner");

    // Try to join again
    let result = test_env.join_clan(
        clan_canister.id.0,
        joiner_user.users_canister_id,
        joiner_id,
        0,
    );
    assert!(matches!(result, Err(ClanError::UserAlreadyMember)));
}

#[test]
#[serial]
fn test_get_clan_members() {
    let test_env = TestEnv::new(None);

    // Setup clan and member
    let (clan_canister, creator_id, joiner_id, _) = setup_clan_with_member(&test_env, "get_members_joiner");

    // Get members
    let members = test_env
        .get_clan_members(clan_canister.id.0)
        .expect("Failed to get clan members");

    assert_eq!(members.len(), 2);
    
    // Check both members are present
    let member_ids: HashSet<_> = members.iter().map(|m| m.principal_id).collect();
    assert!(member_ids.contains(&creator_id));
    assert!(member_ids.contains(&joiner_id));
}

#[test]
#[serial]
fn test_get_specific_clan_member() {
    let test_env = TestEnv::new(None);

    // Setup clan and member
    let (clan_canister, creator_id, _, _) = setup_clan_with_member(&test_env, "get_specific_member_joiner");

    // Get specific member
    let member = test_env
        .get_clan_member(clan_canister.id.0, creator_id)
        .expect("Failed to get clan member");

    assert_eq!(member.principal_id, creator_id);
    assert_eq!(member.role, ClanRole::Owner);
}

#[test]
#[serial]
fn test_get_nonexistent_member() {
    let test_env = TestEnv::new(None);

    // Setup clan
    let (clan_canister, _, _, _) = setup_clan_with_member(&test_env, "nonexistent_member_joiner");

    let nonexistent_id = WalletPrincipalId(Principal::self_authenticating("nonexistent"));

    // Try to get nonexistent member
    let result = test_env.get_clan_member(clan_canister.id.0, nonexistent_id);
    assert!(matches!(result, Err(ClanError::MemberNotFound)));
}

#[test]
#[serial]
fn test_ping_clan_canister() {
    let test_env = TestEnv::new(None);

    // Create a simple clan
    let creator_id = WalletPrincipalId(Principal::self_authenticating("ping_creator"));
    let creator_user = test_env
        .create_user("Creator User 5".to_string(), creator_id)
        .expect("Failed to create user");

    let clan_request = CreateClanRequest {
        name: "Ping Clan".to_string(),
        description: "Test ping".to_string(),
        tag: "PING".to_string(),
        privacy: ClanPrivacy::Public,
        supported_currency: Currency::ICP,
        joining_fee: 0,
        require_proof_of_humanity: false,
        minimum_level_required: None,
        minimum_experience_points: None,
        member_limit: None,
        avatar: None,
        website: None,
        discord: None,
        twitter: None,
    };

    let clan_canister = test_env
        .create_clan(&clan_request, creator_id, creator_user.users_canister_id)
        .expect("Failed to create clan");

    // Test ping
    let ping_result = test_env.pocket_ic.query_call(
        clan_canister.id.0,
        Principal::anonymous(),
        "ping",
        encode_args(()).unwrap(),
    );

    match ping_result {
        Ok(arg) => {
            let response: String = decode_one(&arg).unwrap();
            assert_eq!(response, "Ok");
        }
        _ => panic!("Failed to ping clan canister"),
    }
}

// Helper function to set up a clan with one member
fn setup_clan_with_member(
    test_env: &TestEnv,
    user_name: &str,
) -> (
    Clan,
    WalletPrincipalId,
    WalletPrincipalId,
    User,
) {
    // Create clan
    let creator_id = WalletPrincipalId(Principal::self_authenticating(user_name));
    let creator_user = test_env
        .create_user("Creator User 6".to_string(), creator_id)
        .expect("Failed to create user");

    let clan_request = CreateClanRequest {
        name: "Test Setup Clan".to_string(),
        description: "A clan for testing setup".to_string(),
        tag: "SETUP".to_string(),
        privacy: ClanPrivacy::Public,
        supported_currency: Currency::ICP,
        joining_fee: 0,
        require_proof_of_humanity: false,
        minimum_level_required: None,
        minimum_experience_points: None,
        member_limit: Some(10),
        avatar: None,
        website: None,
        discord: None,
        twitter: None,
    };

    let clan_canister = test_env
        .create_clan(&clan_request, creator_id, creator_user.users_canister_id)
        .expect("Failed to create clan");

    // Add member
    let joiner_id = WalletPrincipalId(Principal::self_authenticating("setup_joiner"));
    let joiner_user = test_env
        .create_user("Joiner User 6".to_string(), joiner_id)
        .expect("Failed to create user");

    test_env
        .join_clan(
            clan_canister.id.0,
            joiner_user.users_canister_id,
            joiner_id,
            0,
        )
        .expect("Failed to join clan");

    (clan_canister, creator_id, joiner_id, joiner_user)
}
