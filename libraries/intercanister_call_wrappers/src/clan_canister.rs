use std::collections::HashMap;

use clan::{
    Clan, ClanEvent, ClanId, ClanInvitation, ClanStats, CreateClanRequest, JoinRequest,
    member::ClanMember,
    subscriptions::{ClanRole, SubscriptionTier, SubscriptionTierId},
    treasury::ClanTreasury,
};
use errors::clan_error::ClanError;
use user::user::{UsersCanisterId, WalletPrincipalId};

pub async fn create_clan_wrapper(
    clan_canister: ClanId,
    request: CreateClanRequest,
    creator: WalletPrincipalId,
    creator_canister_id: UsersCanisterId,
) -> Result<Clan, ClanError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(clan_canister.0, "create_clan")
        .with_args(&(request, creator, creator_canister_id))
        .await;

    match call_result {
        Ok(clan_result) => match clan_result.candid() {
            Ok(clan) => clan,
            Err(err) => {
                ic_cdk::println!("Error decoding clan: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode clan: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in create_clan call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn get_clan_wrapper(clan_canister: ClanId) -> Result<Clan, ClanError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(clan_canister.0, "get_clan").await;

    match call_result {
        Ok(clan_result) => match clan_result.candid() {
            Ok(clan) => clan,
            Err(err) => {
                ic_cdk::println!("Error decoding clan: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode clan: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_clan call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

// Member management wrappers

pub async fn join_clan_wrapper(
    clan_canister: ClanId,
    users_canister_principal: UsersCanisterId,
    user_id: WalletPrincipalId,
    joining_fee_paid: u64,
) -> Result<(), ClanError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(clan_canister.0, "join_clan")
        .with_args(&(users_canister_principal, user_id, joining_fee_paid))
        .await;

    match call_result {
        Ok(result) => match result.candid() {
            Ok(result) => result,
            Err(err) => {
                ic_cdk::println!("Error decoding join_clan result: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode join_clan result: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in join_clan call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn leave_clan_wrapper(
    clan_canister: ClanId,
    user_id: WalletPrincipalId,
) -> Result<(), ClanError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(clan_canister.0, "leave_clan")
        .with_arg(user_id)
        .await;

    match call_result {
        Ok(result) => match result.candid() {
            Ok(result) => result,
            Err(err) => {
                ic_cdk::println!("Error decoding leave_clan result: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode leave_clan result: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in leave_clan call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn kick_member_wrapper(
    clan_canister: ClanId,
    user_id: WalletPrincipalId,
    kicked_by: WalletPrincipalId,
) -> Result<(), ClanError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(clan_canister.0, "kick_member")
        .with_args(&(user_id, kicked_by))
        .await;

    match call_result {
        Ok(result) => match result.candid() {
            Ok(result) => result,
            Err(err) => {
                ic_cdk::println!("Error decoding kick_member result: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode kick_member result: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in kick_member call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn get_clan_member_wrapper(
    clan_canister: ClanId,
    member_id: WalletPrincipalId,
) -> Result<ClanMember, ClanError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(clan_canister.0, "get_clan_member")
        .with_arg(member_id)
        .await;

    match call_result {
        Ok(member_result) => match member_result.candid() {
            Ok(member) => member,
            Err(err) => {
                ic_cdk::println!("Error decoding clan member: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode clan member: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_clan_member call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn get_clan_members_wrapper(clan_canister: ClanId) -> Result<Vec<ClanMember>, ClanError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(clan_canister.0, "get_clan_members").await;

    match call_result {
        Ok(members_result) => match members_result.candid() {
            Ok(members) => members,
            Err(err) => {
                ic_cdk::println!("Error decoding clan members: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode clan members: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_clan_members call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

// Role and permission management wrappers

pub async fn update_member_role_wrapper(
    clan_canister: ClanId,
    user_id: WalletPrincipalId,
    new_role: ClanRole,
    updated_by: WalletPrincipalId,
) -> Result<(), ClanError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(clan_canister.0, "update_member_role")
        .with_args(&(user_id, new_role, updated_by))
        .await;

    match call_result {
        Ok(result) => match result.candid() {
            Ok(result) => result,
            Err(err) => {
                ic_cdk::println!("Error decoding update_member_role result: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode update_member_role result: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in update_member_role call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn suspend_member_wrapper(
    clan_canister: ClanId,
    user_id: WalletPrincipalId,
    suspended_by: WalletPrincipalId,
    until: Option<u64>,
) -> Result<(), ClanError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(clan_canister.0, "suspend_member")
        .with_args(&(user_id, suspended_by, until))
        .await;

    match call_result {
        Ok(result) => match result.candid() {
            Ok(result) => result,
            Err(err) => {
                ic_cdk::println!("Error decoding suspend_member result: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode suspend_member result: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in suspend_member call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

// Subscription management wrappers

pub async fn upgrade_subscription_wrapper(
    clan_canister: ClanId,
    user_id: WalletPrincipalId,
    new_tier: SubscriptionTierId,
    paid_amount: u64,
    months: u32,
) -> Result<(), ClanError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(clan_canister.0, "upgrade_subscription")
        .with_args(&(user_id, new_tier, paid_amount, months))
        .await;

    match call_result {
        Ok(result) => match result.candid() {
            Ok(result) => result,
            Err(err) => {
                ic_cdk::println!("Error decoding upgrade_subscription result: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode upgrade_subscription result: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in upgrade_subscription call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn get_subscription_tiers_wrapper(
    clan_canister: ClanId,
) -> Result<Vec<SubscriptionTier>, ClanError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(clan_canister.0, "get_subscription_tiers").await;

    match call_result {
        Ok(tiers_result) => match tiers_result.candid() {
            Ok(tiers) => tiers,
            Err(err) => {
                ic_cdk::println!("Error decoding subscription tiers: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode subscription tiers: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_subscription_tiers call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn create_subscription_tier_wrapper(
    clan_canister: ClanId,
    tier: SubscriptionTier,
    created_by: WalletPrincipalId,
) -> Result<(), ClanError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(clan_canister.0, "create_subscription_tier")
            .with_args(&(tier, created_by))
            .await;

    match call_result {
        Ok(result) => match result.candid() {
            Ok(result) => result,
            Err(err) => {
                ic_cdk::println!("Error decoding create_subscription_tier result: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode create_subscription_tier result: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in create_subscription_tier call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn remove_subscription_tier_wrapper(
    clan_canister: ClanId,
    tier_id: SubscriptionTierId,
    removed_by: WalletPrincipalId,
) -> Result<(), ClanError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(clan_canister.0, "remove_subscription_tier")
            .with_args(&(tier_id, removed_by))
            .await;

    match call_result {
        Ok(result) => match result.candid() {
            Ok(result) => result,
            Err(err) => {
                ic_cdk::println!("Error decoding remove_subscription_tier result: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode remove_subscription_tier result: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in remove_subscription_tier call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

// Treasury and rewards wrappers

pub async fn get_clan_treasury_wrapper(clan_canister: ClanId) -> Result<ClanTreasury, ClanError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(clan_canister.0, "get_clan_treasury").await;

    match call_result {
        Ok(treasury_result) => match treasury_result.candid() {
            Ok(treasury) => treasury,
            Err(err) => {
                ic_cdk::println!("Error decoding clan treasury: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode clan treasury: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_clan_treasury call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn distribute_rewards_wrapper(
    clan_canister: ClanId,
    distribution: HashMap<WalletPrincipalId, u64>,
    distributed_by: WalletPrincipalId,
) -> Result<(), ClanError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(clan_canister.0, "distribute_rewards")
        .with_args(&(distribution, distributed_by))
        .await;

    match call_result {
        Ok(result) => match result.candid() {
            Ok(result) => result,
            Err(err) => {
                ic_cdk::println!("Error decoding distribute_rewards result: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode distribute_rewards result: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in distribute_rewards call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

// Statistics and events wrappers

pub async fn get_clan_statistics_wrapper(clan_canister: ClanId) -> Result<ClanStats, ClanError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(clan_canister.0, "get_clan_statistics").await;

    match call_result {
        Ok(stats_result) => match stats_result.candid() {
            Ok(stats) => stats,
            Err(err) => {
                ic_cdk::println!("Error decoding clan statistics: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode clan statistics: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_clan_statistics call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn get_clan_events_wrapper(
    clan_canister: ClanId,
    limit: Option<usize>,
) -> Result<Vec<ClanEvent>, ClanError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(clan_canister.0, "get_clan_events")
        .with_arg(limit)
        .await;

    match call_result {
        Ok(events_result) => match events_result.candid() {
            Ok(events) => events,
            Err(err) => {
                ic_cdk::println!("Error decoding clan events: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode clan events: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_clan_events call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

// Invitation and application management wrappers

pub async fn send_clan_invitation_wrapper(
    clan_canister: ClanId,
    invitee: WalletPrincipalId,
    invited_by: WalletPrincipalId,
    message: Option<String>,
    expires_in_hours: Option<u64>,
) -> Result<ClanInvitation, ClanError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(clan_canister.0, "send_clan_invitation")
        .with_args(&(invitee, invited_by, message, expires_in_hours))
        .await;

    match call_result {
        Ok(invitation_result) => match invitation_result.candid() {
            Ok(invitation) => invitation,
            Err(err) => {
                ic_cdk::println!("Error decoding clan invitation: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode clan invitation: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in send_clan_invitation call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn accept_clan_invitation_wrapper(
    clan_canister: ClanId,
    user_id: WalletPrincipalId,
    users_canister_principal: UsersCanisterId,
) -> Result<(), ClanError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(clan_canister.0, "accept_clan_invitation")
        .with_args(&(user_id, users_canister_principal))
        .await;

    match call_result {
        Ok(result) => match result.candid() {
            Ok(result) => result,
            Err(err) => {
                ic_cdk::println!("Error decoding accept_clan_invitation result: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode accept_clan_invitation result: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in accept_clan_invitation call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn submit_join_request_wrapper(
    clan_canister: ClanId,
    user_id: WalletPrincipalId,
    users_canister_principal: UsersCanisterId,
    message: Option<String>,
    referred_by: Option<WalletPrincipalId>,
) -> Result<(), ClanError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(clan_canister.0, "submit_join_request")
        .with_args(&(user_id, users_canister_principal, message, referred_by))
        .await;

    match call_result {
        Ok(result) => match result.candid() {
            Ok(result) => result,
            Err(err) => {
                ic_cdk::println!("Error decoding submit_join_request result: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode submit_join_request result: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in submit_join_request call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn approve_join_request_wrapper(
    clan_canister: ClanId,
    applicant: WalletPrincipalId,
    approved_by: WalletPrincipalId,
    users_canister_principal: UsersCanisterId,
) -> Result<(), ClanError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(clan_canister.0, "approve_join_request")
        .with_args(&(applicant, approved_by, users_canister_principal))
        .await;

    match call_result {
        Ok(result) => match result.candid() {
            Ok(result) => result,
            Err(err) => {
                ic_cdk::println!("Error decoding approve_join_request result: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode approve_join_request result: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in approve_join_request call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn get_pending_requests_wrapper(
    clan_canister: ClanId,
) -> Result<Vec<JoinRequest>, ClanError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(clan_canister.0, "get_pending_requests").await;

    match call_result {
        Ok(requests_result) => match requests_result.candid() {
            Ok(requests) => requests,
            Err(err) => {
                ic_cdk::println!("Error decoding pending requests: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode pending requests: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_pending_requests call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

// Administrative wrappers

pub async fn update_member_stats_wrapper(
    clan_canister: ClanId,
    member_id: WalletPrincipalId,
    games_played_delta: u64,
    tournaments_won_delta: u64,
    winnings_delta: u64,
    xp_delta: u64,
) -> Result<(), ClanError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(clan_canister.0, "update_member_stats")
        .with_args(&(
            member_id,
            games_played_delta,
            tournaments_won_delta,
            winnings_delta,
            xp_delta,
        ))
        .await;

    match call_result {
        Ok(result) => match result.candid() {
            Ok(result) => result,
            Err(err) => {
                ic_cdk::println!("Error decoding update_member_stats result: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode update_member_stats result: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in update_member_stats call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn process_subscription_renewals_wrapper(
    clan_canister: ClanId,
) -> Result<Vec<(WalletPrincipalId, String)>, ClanError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(clan_canister.0, "process_subscription_renewals").await;

    match call_result {
        Ok(renewals_result) => match renewals_result.candid() {
            Ok(renewals) => renewals,
            Err(err) => {
                ic_cdk::println!("Error decoding subscription renewals: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode subscription renewals: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in process_subscription_renewals call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

// Permission and access control wrappers

pub async fn has_tier_access_wrapper(
    clan_canister: ClanId,
    member_id: WalletPrincipalId,
    required_benefit: String,
) -> Result<bool, ClanError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(clan_canister.0, "has_tier_access")
        .with_args(&(member_id, required_benefit))
        .await;

    match call_result {
        Ok(access_result) => match access_result.candid() {
            Ok(has_access) => has_access,
            Err(err) => {
                ic_cdk::println!("Error decoding tier access result: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode tier access result: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in has_tier_access call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn can_access_table_stakes_wrapper(
    clan_canister: ClanId,
    member_id: WalletPrincipalId,
    table_stakes: u64,
) -> Result<bool, ClanError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(clan_canister.0, "can_access_table_stakes")
            .with_args(&(member_id, table_stakes))
            .await;

    match call_result {
        Ok(access_result) => match access_result.candid() {
            Ok(can_access) => can_access,
            Err(err) => {
                ic_cdk::println!("Error decoding table stakes access result: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode table stakes access result: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in can_access_table_stakes call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

// Cycle management wrappers

pub async fn request_cycles_wrapper(clan_canister: ClanId) -> Result<(), ClanError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(clan_canister.0, "request_cycles").await;

    match call_result {
        Ok(result) => match result.candid() {
            Ok(result) => result,
            Err(err) => {
                ic_cdk::println!("Error decoding request_cycles result: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode request_cycles result: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in request_cycles call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

// Additional utility wrappers for invited users and invitations

pub async fn get_invited_users_wrapper(
    clan_canister: ClanId,
) -> Result<HashMap<WalletPrincipalId, u64>, ClanError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(clan_canister.0, "get_invited_users").await;

    match call_result {
        Ok(invited_result) => match invited_result.candid() {
            Ok(invited) => invited,
            Err(err) => {
                ic_cdk::println!("Error decoding invited users: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode invited users: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_invited_users call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}

pub async fn reject_join_request_wrapper(
    clan_canister: ClanId,
    applicant: WalletPrincipalId,
    rejected_by: WalletPrincipalId,
) -> Result<(), ClanError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(clan_canister.0, "reject_join_request")
        .with_args(&(applicant.0, rejected_by))
        .await;

    match call_result {
        Ok(result) => match result.candid() {
            Ok(result) => result,
            Err(err) => {
                ic_cdk::println!("Error decoding reject_join_request result: {:?}", err);
                Err(ClanError::CanisterCallError(format!(
                    "Failed to decode reject_join_request result: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in reject_join_request call: {:?}", err);
            Err(ClanError::CanisterCallError(format!("{:?}", err)))
        }
    }
}
