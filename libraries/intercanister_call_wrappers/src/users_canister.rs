use candid::Principal;
use errors::user_error::UserError;
use user::user::User;


pub async fn get_user(user_principal: Principal, user_id: Principal) -> Result<User, UserError> {
    let call_result =
        ic_cdk::call::Call::unbounded_wait(user_principal, "get_user").with_arg(user_id).await;

    match call_result {
        Ok(user_result) => {
            match user_result.candid() {
                Ok(user) => user,
                Err(err) => {
                    ic_cdk::println!("Error decoding user: {:?}", err);
                    Err(UserError::LockError)
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
) -> Result<User, UserError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(
        users_canister_id,
        "remove_active_table",
    )
    .with_args(&(ic_cdk::api::canister_self(), user_id))
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
