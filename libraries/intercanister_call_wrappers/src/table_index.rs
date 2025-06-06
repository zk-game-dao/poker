use candid::Principal;
use errors::table_index_error::TableIndexError;

pub async fn update_table_player_count_wrapper(
    backend_principal: Principal,
    table_id: Principal,
    user_count: usize,
) -> Result<(), TableIndexError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(
        backend_principal,
        "update_table_player_count"
    )
    .with_args(&(table_id, user_count))
    .await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error updating table player count: {:?}", err);
                Err(TableIndexError::CanisterCallError(format!(
                    "Failed to decode update_table_player_count response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in update_table_player_count call: {:?}", err);
            Err(TableIndexError::CanisterCallError(format!(
                "{:?}",
                err
            )))
        }
    }
}