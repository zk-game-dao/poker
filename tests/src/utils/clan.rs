use candid::{decode_one, encode_args, Principal};
use clan::{
    member::ClanMember,
    subscriptions::{ClanRole, SubscriptionBenefits, SubscriptionRequirements, SubscriptionTier, SubscriptionTierId},
    treasury::ClanTreasury,
    Clan, ClanEvent, ClanInvitation, ClanPrivacy, ClanStats, CreateClanRequest,
    JoinRequest,
};
use currency::Currency;
use errors::{clan_error::ClanError, clan_index_error::ClanIndexError};
use table::poker::game::{
    table_functions::table::TableConfig,
    types::PublicTable,
};
use tournaments::tournaments::types::{NewTournament, TournamentData, TournamentId};
use user::user::{UsersCanisterId, WalletPrincipalId};
use std::collections::HashMap;

use crate::TestEnv;

impl TestEnv {
    /// Create a clan canister and initialize it
    pub fn create_clan(
        &self,
        request: &CreateClanRequest,
        creator: WalletPrincipalId,
        creator_canister_id: UsersCanisterId,
    ) -> Result<Clan, ClanIndexError> {
        let result = self.pocket_ic.update_call(
            self.canister_ids.clan_index, // Assuming clan_index creates clan canisters
            creator.0,
            "create_clan",
            encode_args((request.clone(), creator, creator_canister_id)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let clan: Result<Clan, ClanIndexError> = decode_one(&arg).unwrap();
                clan
            }
            Err(e) => panic!("Failed to create clan {}", e),
        }
    }

    /// Get clan information
    pub fn get_clan(&self, clan_canister_id: Principal) -> Result<Clan, ClanError> {
        let result = self.pocket_ic.query_call(
            clan_canister_id,
            Principal::anonymous(),
            "get_clan",
            encode_args(()).unwrap(),
        );

        match result {
            Ok(arg) => {
                let clan: Result<Clan, ClanError> = decode_one(&arg).unwrap();
                clan
            }
            _ => panic!("Failed to get clan"),
        }
    }

    /// Get specific clan member
    pub fn get_clan_member(
        &self,
        clan_canister_id: Principal,
        member_id: WalletPrincipalId,
    ) -> Result<ClanMember, ClanError> {
        let result = self.pocket_ic.query_call(
            clan_canister_id,
            Principal::anonymous(),
            "get_clan_member",
            encode_args((member_id,)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let member: Result<ClanMember, ClanError> = decode_one(&arg).unwrap();
                member
            }
            _ => panic!("Failed to get clan member"),
        }
    }

    /// Get all clan members
    pub fn get_clan_members(&self, clan_canister_id: Principal) -> Result<Vec<ClanMember>, ClanError> {
        let result = self.pocket_ic.query_call(
            clan_canister_id,
            Principal::anonymous(),
            "get_clan_members",
            encode_args(()).unwrap(),
        );

        match result {
            Ok(arg) => {
                let members: Result<Vec<ClanMember>, ClanError> = decode_one(&arg).unwrap();
                members
            }
            _ => panic!("Failed to get clan members"),
        }
    }

    /// Get clan events
    pub fn get_clan_events(
        &self,
        clan_canister_id: Principal,
        limit: Option<usize>,
    ) -> Result<Vec<ClanEvent>, ClanError> {
        let result = self.pocket_ic.query_call(
            clan_canister_id,
            Principal::anonymous(),
            "get_clan_events",
            encode_args((limit,)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let events: Result<Vec<ClanEvent>, ClanError> = decode_one(&arg).unwrap();
                events
            }
            _ => panic!("Failed to get clan events"),
        }
    }

    /// Join a clan
    pub fn join_clan(
        &self,
        clan_canister_id: Principal,
        users_canister_principal: UsersCanisterId,
        user_id: WalletPrincipalId,
        joining_fee_paid: u64,
    ) -> Result<(), ClanError> {
        let result = self.pocket_ic.update_call(
            clan_canister_id,
            user_id.0,
            "join_clan",
            encode_args((users_canister_principal, user_id, joining_fee_paid)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let res: Result<(), ClanError> = decode_one(&arg).unwrap();
                res
            }
            _ => panic!("Failed to join clan"),
        }
    }

    /// Leave a clan
    pub fn leave_clan(
        &self,
        clan_canister_id: Principal,
        user_id: WalletPrincipalId,
    ) -> Result<(), ClanError> {
        let result = self.pocket_ic.update_call(
            clan_canister_id,
            user_id.0,
            "leave_clan",
            encode_args((user_id,)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let res: Result<(), ClanError> = decode_one(&arg).unwrap();
                res
            }
            _ => panic!("Failed to leave clan"),
        }
    }

    /// Kick a member from clan
    pub fn kick_member(
        &self,
        clan_canister_id: Principal,
        user_id: WalletPrincipalId,
        kicked_by: WalletPrincipalId,
    ) -> Result<(), ClanError> {
        let result = self.pocket_ic.update_call(
            clan_canister_id,
            kicked_by.0,
            "kick_member",
            encode_args((user_id, kicked_by)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let res: Result<(), ClanError> = decode_one(&arg).unwrap();
                res
            }
            _ => panic!("Failed to kick member"),
        }
    }

    /// Update member role
    pub fn update_member_role(
        &self,
        clan_canister_id: Principal,
        user_id: WalletPrincipalId,
        new_role: ClanRole,
        updated_by: WalletPrincipalId,
    ) -> Result<(), ClanError> {
        let result = self.pocket_ic.update_call(
            clan_canister_id,
            updated_by.0,
            "update_member_role",
            encode_args((user_id, new_role, updated_by)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let res: Result<(), ClanError> = decode_one(&arg).unwrap();
                res
            }
            _ => panic!("Failed to update member role"),
        }
    }

    /// Suspend a member
    pub fn suspend_member(
        &self,
        clan_canister_id: Principal,
        user_id: WalletPrincipalId,
        suspended_by: WalletPrincipalId,
        until: Option<u64>,
    ) -> Result<(), ClanError> {
        let result = self.pocket_ic.update_call(
            clan_canister_id,
            suspended_by.0,
            "suspend_member",
            encode_args((user_id, suspended_by, until)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let res: Result<(), ClanError> = decode_one(&arg).unwrap();
                res
            }
            _ => panic!("Failed to suspend member"),
        }
    }

    /// Create a clan table
    pub fn create_clan_table(
        &self,
        clan_canister_id: Principal,
        config: &TableConfig,
        created_by: WalletPrincipalId,
    ) -> Result<PublicTable, ClanError> {
        let result = self.pocket_ic.update_call(
            clan_canister_id,
            created_by.0,
            "create_clan_table",
            encode_args((config.clone(), created_by)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let table: Result<PublicTable, ClanError> = decode_one(&arg).unwrap();
                table
            }
            _ => panic!("Failed to create clan table"),
        }
    }

    /// Create a clan tournament
    pub fn create_clan_tournament(
        &self,
        clan_canister_id: Principal,
        new_tournament: &NewTournament,
        table_config: &TableConfig,
        created_by: WalletPrincipalId,
    ) -> Result<TournamentData, ClanError> {
        let result = self.pocket_ic.update_call(
            clan_canister_id,
            created_by.0,
            "create_clan_tournament",
            encode_args((new_tournament.clone(), table_config.clone(), created_by)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let tournament: Result<TournamentData, ClanError> = decode_one(&arg).unwrap();
                tournament
            }
            _ => panic!("Failed to create clan tournament"),
        }
    }

    /// Get clan tables
    pub fn get_clan_tables(
        &self,
        clan_canister_id: Principal,
    ) -> Result<Vec<table::poker::game::table_functions::table::TableId>, ClanError> {
        let result = self.pocket_ic.query_call(
            clan_canister_id,
            Principal::anonymous(),
            "get_clan_tables",
            encode_args(()).unwrap(),
        );

        match result {
            Ok(arg) => {
                let tables: Result<Vec<table::poker::game::table_functions::table::TableId>, ClanError> = decode_one(&arg).unwrap();
                tables
            }
            _ => panic!("Failed to get clan tables"),
        }
    }

    /// Get clan tournaments
    pub fn get_clan_tournaments(
        &self,
        clan_canister_id: Principal,
    ) -> Result<Vec<TournamentId>, ClanError> {
        let result = self.pocket_ic.query_call(
            clan_canister_id,
            Principal::anonymous(),
            "get_clan_tournaments",
            encode_args(()).unwrap(),
        );

        match result {
            Ok(arg) => {
                let tournaments: Result<Vec<TournamentId>, ClanError> = decode_one(&arg).unwrap();
                tournaments
            }
            _ => panic!("Failed to get clan tournaments"),
        }
    }

    /// Remove a clan table
    pub fn remove_clan_table(
        &self,
        clan_canister_id: Principal,
        table_id: table::poker::game::table_functions::table::TableId,
        removed_by: WalletPrincipalId,
    ) -> Result<(), ClanError> {
        let result = self.pocket_ic.update_call(
            clan_canister_id,
            removed_by.0,
            "remove_clan_table",
            encode_args((table_id, removed_by)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let res: Result<(), ClanError> = decode_one(&arg).unwrap();
                res
            }
            _ => panic!("Failed to remove clan table"),
        }
    }

    /// Remove a clan tournament
    pub fn remove_clan_tournament(
        &self,
        clan_canister_id: Principal,
        tournament_id: TournamentId,
        removed_by: WalletPrincipalId,
    ) -> Result<(), ClanError> {
        let result = self.pocket_ic.update_call(
            clan_canister_id,
            removed_by.0,
            "remove_clan_tournament",
            encode_args((tournament_id, removed_by)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let res: Result<(), ClanError> = decode_one(&arg).unwrap();
                res
            }
            _ => panic!("Failed to remove clan tournament"),
        }
    }

    /// Check if member can access table with specific stakes
    pub fn can_member_access_table(
        &self,
        clan_canister_id: Principal,
        member_id: WalletPrincipalId,
        table_stakes: u64,
    ) -> Result<bool, ClanError> {
        let result = self.pocket_ic.query_call(
            clan_canister_id,
            Principal::anonymous(),
            "can_member_access_table",
            encode_args((member_id, table_stakes)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let can_access: Result<bool, ClanError> = decode_one(&arg).unwrap();
                can_access
            }
            _ => panic!("Failed to check table access"),
        }
    }

    /// Check if member can access specific benefits
    pub fn can_member_access_benefits(
        &self,
        clan_canister_id: Principal,
        member_id: WalletPrincipalId,
        benefits: String,
    ) -> Result<bool, ClanError> {
        let result = self.pocket_ic.query_call(
            clan_canister_id,
            Principal::anonymous(),
            "can_member_access_benefits",
            encode_args((member_id, benefits)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let can_access: Result<bool, ClanError> = decode_one(&arg).unwrap();
                can_access
            }
            _ => panic!("Failed to check benefits access"),
        }
    }

    /// Upgrade member subscription
    pub fn upgrade_subscription(
        &self,
        clan_canister_id: Principal,
        user_id: WalletPrincipalId,
        new_tier: SubscriptionTierId,
        paid_amount: u64,
        months: u32,
    ) -> Result<(), ClanError> {
        let result = self.pocket_ic.update_call(
            clan_canister_id,
            user_id.0,
            "upgrade_subscription",
            encode_args((user_id, new_tier, paid_amount, months)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let res: Result<(), ClanError> = decode_one(&arg).unwrap();
                res
            }
            _ => panic!("Failed to upgrade subscription"),
        }
    }

    /// Create custom subscription tier
    pub fn create_custom_subscription_tier(
        &self,
        clan_canister_id: Principal,
        id: SubscriptionTierId,
        name: String,
        requirements: SubscriptionRequirements,
        benefits: SubscriptionBenefits,
        is_active: bool,
        tier_order: u32,
        creator: WalletPrincipalId,
    ) -> Result<(), ClanError> {
        let result = self.pocket_ic.update_call(
            clan_canister_id,
            creator.0,
            "create_custom_subscription_tier",
            encode_args((id, name, requirements, benefits, is_active, tier_order, creator)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let res: Result<(), ClanError> = decode_one(&arg).unwrap();
                res
            }
            _ => panic!("Failed to create custom subscription tier"),
        }
    }

    /// Get subscription tiers
    pub fn get_subscription_tiers(
        &self,
        clan_canister_id: Principal,
    ) -> Result<Vec<SubscriptionTier>, ClanError> {
        let result = self.pocket_ic.query_call(
            clan_canister_id,
            Principal::anonymous(),
            "get_subscription_tiers",
            encode_args(()).unwrap(),
        );

        match result {
            Ok(arg) => {
                let tiers: Result<Vec<SubscriptionTier>, ClanError> = decode_one(&arg).unwrap();
                tiers
            }
            _ => panic!("Failed to get subscription tiers"),
        }
    }

    /// Remove subscription tier
    pub fn remove_subscription_tier(
        &self,
        clan_canister_id: Principal,
        tier_id: SubscriptionTierId,
        removed_by: WalletPrincipalId,
    ) -> Result<(), ClanError> {
        let result = self.pocket_ic.update_call(
            clan_canister_id,
            removed_by.0,
            "remove_subscription_tier",
            encode_args((tier_id, removed_by)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let res: Result<(), ClanError> = decode_one(&arg).unwrap();
                res
            }
            _ => panic!("Failed to remove subscription tier"),
        }
    }

    /// Distribute rewards
    pub fn distribute_rewards(
        &self,
        clan_canister_id: Principal,
        distribution: HashMap<WalletPrincipalId, u64>,
        distributed_by: WalletPrincipalId,
    ) -> Result<(), ClanError> {
        let result = self.pocket_ic.update_call(
            clan_canister_id,
            distributed_by.0,
            "distribute_rewards",
            encode_args((distribution, distributed_by)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let res: Result<(), ClanError> = decode_one(&arg).unwrap();
                res
            }
            _ => panic!("Failed to distribute rewards"),
        }
    }

    /// Get clan treasury
    pub fn get_clan_treasury(&self, clan_canister_id: Principal) -> Result<ClanTreasury, ClanError> {
        let result = self.pocket_ic.query_call(
            clan_canister_id,
            Principal::anonymous(),
            "get_clan_treasury",
            encode_args(()).unwrap(),
        );

        match result {
            Ok(arg) => {
                let treasury: Result<ClanTreasury, ClanError> = decode_one(&arg).unwrap();
                treasury
            }
            _ => panic!("Failed to get clan treasury"),
        }
    }

    /// Get clan statistics
    pub fn get_clan_statistics(&self, clan_canister_id: Principal) -> Result<ClanStats, ClanError> {
        let result = self.pocket_ic.query_call(
            clan_canister_id,
            Principal::anonymous(),
            "get_clan_statistics",
            encode_args(()).unwrap(),
        );

        match result {
            Ok(arg) => {
                let stats: Result<ClanStats, ClanError> = decode_one(&arg).unwrap();
                stats
            }
            _ => panic!("Failed to get clan statistics"),
        }
    }

    /// Send clan invitation
    pub fn send_clan_invitation(
        &self,
        clan_canister_id: Principal,
        invitee: WalletPrincipalId,
        invited_by: WalletPrincipalId,
        message: Option<String>,
        expires_in_hours: Option<u64>,
    ) -> Result<ClanInvitation, ClanError> {
        let result = self.pocket_ic.update_call(
            clan_canister_id,
            invited_by.0,
            "send_clan_invitation",
            encode_args((invitee, invited_by, message, expires_in_hours)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let invitation: Result<ClanInvitation, ClanError> = decode_one(&arg).unwrap();
                invitation
            }
            _ => panic!("Failed to send clan invitation"),
        }
    }

    /// Accept clan invitation
    pub fn accept_clan_invitation(
        &self,
        clan_canister_id: Principal,
        user_id: WalletPrincipalId,
        users_canister_principal: UsersCanisterId,
    ) -> Result<(), ClanError> {
        let result = self.pocket_ic.update_call(
            clan_canister_id,
            user_id.0,
            "accept_clan_invitation",
            encode_args((user_id, users_canister_principal)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let res: Result<(), ClanError> = decode_one(&arg).unwrap();
                res
            }
            _ => panic!("Failed to accept clan invitation"),
        }
    }

    /// Submit join request
    pub fn submit_join_request(
        &self,
        clan_canister_id: Principal,
        user_id: WalletPrincipalId,
        users_canister_principal: UsersCanisterId,
        message: Option<String>,
        referred_by: Option<WalletPrincipalId>,
    ) -> Result<(), ClanError> {
        let result = self.pocket_ic.update_call(
            clan_canister_id,
            user_id.0,
            "submit_join_request",
            encode_args((user_id, users_canister_principal, message, referred_by)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let res: Result<(), ClanError> = decode_one(&arg).unwrap();
                res
            }
            _ => panic!("Failed to submit join request"),
        }
    }

    /// Approve join request
    pub fn approve_join_request(
        &self,
        clan_canister_id: Principal,
        applicant: WalletPrincipalId,
        approved_by: WalletPrincipalId,
        users_canister_principal: UsersCanisterId,
    ) -> Result<(), ClanError> {
        let result = self.pocket_ic.update_call(
            clan_canister_id,
            approved_by.0,
            "approve_join_request",
            encode_args((applicant, approved_by, users_canister_principal)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let res: Result<(), ClanError> = decode_one(&arg).unwrap();
                res
            }
            _ => panic!("Failed to approve join request"),
        }
    }

    /// Reject join request
    pub fn reject_join_request(
        &self,
        clan_canister_id: Principal,
        applicant: Principal,
        rejected_by: WalletPrincipalId,
    ) -> Result<(), ClanError> {
        let result = self.pocket_ic.update_call(
            clan_canister_id,
            rejected_by.0,
            "reject_join_request",
            encode_args((applicant, rejected_by)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let res: Result<(), ClanError> = decode_one(&arg).unwrap();
                res
            }
            _ => panic!("Failed to reject join request"),
        }
    }

    /// Get pending requests
    pub fn get_pending_requests(
        &self,
        clan_canister_id: Principal,
    ) -> Result<Vec<JoinRequest>, ClanError> {
        let result = self.pocket_ic.query_call(
            clan_canister_id,
            Principal::anonymous(),
            "get_pending_requests",
            encode_args(()).unwrap(),
        );

        match result {
            Ok(arg) => {
                let requests: Result<Vec<JoinRequest>, ClanError> = decode_one(&arg).unwrap();
                requests
            }
            _ => panic!("Failed to get pending requests"),
        }
    }

    /// Get invited users
    pub fn get_invited_users(
        &self,
        clan_canister_id: Principal,
    ) -> Result<HashMap<WalletPrincipalId, u64>, ClanError> {
        let result = self.pocket_ic.query_call(
            clan_canister_id,
            Principal::anonymous(),
            "get_invited_users",
            encode_args(()).unwrap(),
        );

        match result {
            Ok(arg) => {
                let users: Result<HashMap<WalletPrincipalId, u64>, ClanError> = decode_one(&arg).unwrap();
                users
            }
            _ => panic!("Failed to get invited users"),
        }
    }

    /// Helper function to create a basic clan for testing
    pub fn create_test_clan(
        &self,
        name: &str,
        creator_name: &str,
    ) -> (Clan, WalletPrincipalId, UsersCanisterId) {
        let creator_id = WalletPrincipalId(Principal::self_authenticating(creator_name));
        let creator_user = self
            .create_user(format!("{} User", creator_name), creator_id)
            .expect("Failed to create user");

        let clan_request = CreateClanRequest {
            name: name.to_string(),
            description: format!("Test clan: {}", name),
            tag: name.to_uppercase().chars().take(4).collect(),
            privacy: ClanPrivacy::Public,
            supported_currency: Currency::ICP,
            joining_fee: 0,
            require_proof_of_humanity: false,
            minimum_level_required: None,
            minimum_experience_points: None,
            member_limit: Some(100),
            avatar: None,
            website: None,
            discord: None,
            twitter: None,
        };

        let clan = self
            .create_clan(&clan_request, creator_id, creator_user.users_canister_id)
            .expect("Failed to create test clan");

        (clan, creator_id, creator_user.users_canister_id)
    }

    /// Helper function to create a clan with joining fee
    pub fn create_test_clan_with_fee(
        &self,
        name: &str,
        creator_name: &str,
        joining_fee: u64,
    ) -> (Clan, WalletPrincipalId, UsersCanisterId) {
        let creator_id = WalletPrincipalId(Principal::self_authenticating(creator_name));
        let creator_user = self
            .create_user(format!("{} User", creator_name), creator_id)
            .expect("Failed to create user");

        let clan_request = CreateClanRequest {
            name: name.to_string(),
            description: format!("Test clan with fee: {}", name),
            tag: name.to_uppercase().chars().take(4).collect(),
            privacy: ClanPrivacy::Public,
            supported_currency: Currency::ICP,
            joining_fee,
            require_proof_of_humanity: false,
            minimum_level_required: None,
            minimum_experience_points: None,
            member_limit: Some(100),
            avatar: None,
            website: None,
            discord: None,
            twitter: None,
        };

        let clan = self
            .create_clan(&clan_request, creator_id, creator_user.users_canister_id)
            .expect("Failed to create test clan with fee");

        (clan, creator_id, creator_user.users_canister_id)
    }

    /// Helper function to create a user and join them to a clan
    pub fn create_user_and_join_clan(
        &self,
        clan_canister_id: Principal,
        user_name: &str,
        joining_fee: u64,
    ) -> (WalletPrincipalId, UsersCanisterId) {
        let user_id = WalletPrincipalId(Principal::self_authenticating(user_name));
        let user = self
            .create_user(format!("{} User", user_name), user_id)
            .expect("Failed to create user");

        // If there's a joining fee, provide tokens
        if joining_fee > 0 {
            self.transfer_approve_tokens_for_testing(clan_canister_id, user_id, 1.0, true);
        }

        self.join_clan(clan_canister_id, user.users_canister_id, user_id, joining_fee)
            .expect("Failed to join clan");

        (user_id, user.users_canister_id)
    }
}
