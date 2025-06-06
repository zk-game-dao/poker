use candid::Principal;
use errors::user_error::UserError;
use user::user::{User, UserAvatar};

pub async fn create_user_wrapper(
    user_canister: Principal,
    user_name: String,
    address: Option<String>,
    principal_id: Principal,
    avatar: Option<UserAvatar>,
    referrer: Option<Principal>,
) -> Result<(User, usize), UserError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(
        user_canister,
        "create_user",
    )
    .with_args(&(
        user_name,
        address,
        principal_id,
        avatar,
        None::<String>,
        referrer,
    ))
    .await;

    match call_result {
        Ok(user_result) => {
            match user_result.candid() {
                Ok(user) => user,
                Err(err) => {
                    ic_cdk::println!("Error decoding user: {:?}", err);
                    Err(UserError::CanisterCallFailed(format!(
                        "Failed to decode user: {:?}",
                        err
                    )))
                }
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in create_user call: {:?}", err);
            Err(UserError::CanisterCallFailed(format!(
                "{:?}",
                err
            )))
        }
    }
}

pub async fn update_user_wrapper(
    user_canister_principal_id: Principal,
    user_name: Option<String>,
    balance: Option<u64>,
    address: Option<String>,
    principal_id: Principal,
    wallet_principal_id: Option<String>,
    avatar: Option<UserAvatar>,
) -> Result<User, UserError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(
        user_canister_principal_id,
        "update_user",
    )
    .with_args(&(
        principal_id,
        user_name,
        balance,
        address,
        avatar,
        None::<bool>,
        None::<u16>,
        wallet_principal_id,
    ))
    .await;

    match call_result {
        Ok(user_result) => {
            match user_result.candid() {
                Ok(user) => Ok(user),
                Err(err) => {
                    ic_cdk::println!("Error decoding user: {:?}", err);
                    Err(UserError::CanisterCallFailed(format!(
                        "Failed to decode user: {:?}",
                        err
                    )))
                }
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in update_user call: {:?}", err);
            Err(UserError::CanisterCallFailed(format!(
                "{:?}",
                err
            )))
        }
    }
}

pub async fn get_user_wrapper(user_principal: Principal, user_id: Principal) -> Result<User, UserError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(user_principal, "get_user").with_arg(user_id).await;

    match call_result {
        Ok(user_result) => {
            match user_result.candid() {
                Ok(user) => user,
                Err(err) => {
                    ic_cdk::println!("Error decoding user: {:?}", err);
                    Err(UserError::CanisterCallFailed(format!(
                        "Failed to decode user: {:?}",
                        err
                    )))
                }
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_user call: {:?}", err);
            Err(UserError::CanisterCallFailed(format!(
                "{:?}",
                err
            )))
        }
    }
}

pub async fn add_users_active_table(
    users_canister_id: Principal,
    user_id: Principal,
    table_principal: Principal,
) -> Result<User, UserError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(
        users_canister_id,
        "add_active_table",
    )
    .with_args(&(table_principal, user_id))
    .await;

    match call_result {
        Ok(user_result) => match user_result.candid() {
            Ok(user) => user,
            Err(err) => {
                ic_cdk::println!("Error adding active table: {:?}", err);
                Err(UserError::CanisterCallFailed(format!(
                    "Failed to decode user after adding active table: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in add_active_table call: {:?}", err);
            Err(UserError::CanisterCallFailed(format!(
                "{:?}",
                err
            )))
        }
    }
}

pub async fn remove_users_active_table(
    users_canister_id: Principal,
    user_id: Principal,
    table_id: Principal,
) -> Result<User, UserError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(
        users_canister_id,
        "remove_active_table",
    )
    .with_args(&(table_id, user_id))
    .await;

    match call_result {
        Ok(user_result) => match user_result.candid() {
            Ok(user) => user,
            Err(err) => {
                ic_cdk::println!("Error removing active table: {:?}", err);
                Err(UserError::CanisterCallFailed(format!(
                    "Failed to decode user after removing active table: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in remove_active_table call: {:?}", err);
            Err(UserError::CanisterCallFailed(format!(
                "{:?}",
                err
            )))
        }
    }
}

#[ic_cdk::update]
pub async fn get_users_canister_principal_by_id_wrapper(index_principal: Principal, user_id: Principal) -> Result<Principal, UserError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(index_principal, "get_users_canister_principal_by_id").with_arg(user_id).await;

    match call_result {
        Ok(users_canister_result) => match users_canister_result.candid() {
            Ok(users_canister) => users_canister,
            Err(err) => {
                ic_cdk::println!("Error getting users canister principal: {:?}", err);
                Err(UserError::CanisterCallFailed(format!(
                    "Failed to decode users canister principal: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_users_canister_principal_by_id call: {:?}", err);
            Err(UserError::CanisterCallFailed(format!(
                "{:?}",
                err
            )))
        }
    }
}

pub async fn get_user_experience_points_wrapper(users_canister_id: Principal) -> Result<Vec<(Principal, u64)>, UserError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(
        users_canister_id,
        "get_user_experience_points",
    )
    .await;

    match call_result {
        Ok(points) => match points.candid() {
            Ok(points) => points,
            Err(err) => {
                ic_cdk::println!("Error decoding user experience points: {:?}", err);
                Err(UserError::CanisterCallFailed(format!(
                    "Failed to decode user experience points: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_user_experience_points call: {:?}", err);
            Err(UserError::CanisterCallFailed(format!(
                "{:?}",
                err
            )))
        }
    }
}

pub async fn get_pure_poker_user_experience_points_wrapper(
    users_canister_id: Principal,
) -> Result<Vec<(Principal, u64)>, UserError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(
        users_canister_id,
        "get_pure_poker_user_experience_points",
    )
    .await;

    match call_result {
        Ok(points) => match points.candid() {
            Ok(points) => points,
            Err(err) => {
                ic_cdk::println!("Error decoding pure poker user experience points: {:?}", err);
                Err(UserError::CanisterCallFailed(format!(
                    "Failed to decode pure poker user experience points: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_pure_poker_user_experience_points call: {:?}", err);
            Err(UserError::CanisterCallFailed(format!(
                "{:?}",
                err
            )))
        }
    }
}

pub async fn get_verified_user_experience_points_wrapper(
    users_canister: Principal,
) -> Result<Vec<(Principal, u64)>, UserError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(
        users_canister,
        "get_verified_user_experience_points",
    )
    .await;

    match call_result {
        Ok(points) => match points.candid() {
            Ok(points) => points,
            Err(err) => {
                ic_cdk::println!("Error decoding verified user experience points: {:?}", err);
                Err(UserError::CanisterCallFailed(format!(
                    "Failed to decode verified user experience points: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_verified_user_experience_points call: {:?}", err);
            Err(UserError::CanisterCallFailed(format!(
                "{:?}",
                err
            )))
        }
    }
}

pub async fn get_verified_pure_poker_user_experience_points_wrapper(
    users_canister: Principal,
) -> Result<Vec<(Principal, u64)>, UserError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(
        users_canister,
        "get_verified_pure_poker_user_experience_points",
    )
    .await;

    match call_result {
        Ok(points) => match points.candid() {
            Ok(points) => points,
            Err(err) => {
                ic_cdk::println!("Error decoding verified pure poker user experience points: {:?}", err);
                Err(UserError::CanisterCallFailed(format!(
                    "Failed to decode verified pure poker user experience points: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_verified_pure_poker_user_experience_points call: {:?}", err);
            Err(UserError::CanisterCallFailed(format!(
                "{:?}",
                err
            )))
        }
    }
}

pub async fn clear_experience_points_wrapper(
    user_canister: Principal,
) -> Result<(), UserError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(
        user_canister,
        "clear_experience_points",
    )
    .await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error decoding clear_experience_points response: {:?}", err);
                Err(UserError::CanisterCallFailed(format!(
                    "Failed to decode clear_experience_points response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in clear_experience_points call: {:?}", err);
            Err(UserError::CanisterCallFailed(format!(
                "{:?}",
                err
            )))
        }
    }
}

pub async fn clear_pure_poker_experience_points_wrapper(
    user_canister: Principal,
) -> Result<(), UserError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(
        user_canister,
        "clear_pure_poker_experience_points",
    )
    .await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error decoding clear_pure_poker_experience_points response: {:?}", err);
                Err(UserError::CanisterCallFailed(format!(
                    "Failed to decode clear_pure_poker_experience_points response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in clear_pure_poker_experience_points call: {:?}", err);
            Err(UserError::CanisterCallFailed(format!(
                "{:?}",
                err
            )))
        }
    }
}
