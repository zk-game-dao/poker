use candid::Principal;
use errors::log_store_error::LogStoreError;
use table::poker::game::table_functions::action_log::ActionLog;

pub async fn log_actions_wrapper(
    log_store_id: Principal,
    table_id: Principal,
    action_logs: Vec<ActionLog>,
) -> Result<(), LogStoreError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(log_store_id, "log_actions")
        .with_args(&(table_id, action_logs))
        .await;

    match call_result {
        Ok(kick_result) => match kick_result.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error kicking player: {:?}", err);
                Err(LogStoreError::CanisterCallError(format!(
                    "Failed to decode log_actions response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in kick_player call: {:?}", err);
            Err(LogStoreError::CanisterCallError(format!("{:?}", err)))
        }
    }
}
