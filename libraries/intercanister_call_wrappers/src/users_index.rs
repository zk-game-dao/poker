use candid::Principal;
use errors::user_error::UserError;
use user::user::{User, WalletPrincipalId};

pub async fn get_user_wrapper_index(
    user_index: Principal,
    user_id: WalletPrincipalId,
) -> Result<User, UserError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(user_index, "get_user")
        .with_arg(user_id)
        .await;

    match call_result {
        Ok(user_result) => match user_result.candid() {
            Ok(user) => user,
            Err(err) => {
                ic_cdk::println!("Error decoding user: {:?}", err);
                Err(UserError::CanisterCallFailed(format!(
                    "Failed to decode user: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in get_user call: {:?}", err);
            Err(UserError::CanisterCallFailed(format!("{:?}", err)))
        }
    }
}
