use candid::Principal;
use currency::Currency;
use errors::tournament_index_error::TournamentIndexError;

pub async fn request_withdrawal_wrapper(
    tournament_index: Principal,
    currency: Currency,
    amount: u64,
) -> Result<(), TournamentIndexError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(tournament_index, "request_withdrawal")
        .with_args(&(currency, amount))
        .await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error joining tournament: {:?}", err);
                Err(TournamentIndexError::CanisterCallError(format!(
                    "Failed to decode user_join_tournament response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in user_join_tournament call: {:?}", err);
            Err(TournamentIndexError::CanisterCallError(format!("{:?}", err)))
        }
    }
}