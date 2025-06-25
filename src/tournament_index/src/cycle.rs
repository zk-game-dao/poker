use canister_functions::cycle::top_up_canister;
use errors::canister_management_error::CanisterManagementError;
use tournaments::tournaments::types::TournamentId;

use crate::STATE;

const CYCLES_TOP_UP_AMOUNT: u128 = 3_000_000_000_000;

#[ic_cdk::update]
async fn request_cycles() -> Result<(), CanisterManagementError> {
    let cycles = ic_cdk::api::canister_cycle_balance();
    let caller = TournamentId(ic_cdk::api::msg_caller());
    ic_cdk::println!(
        "%%%%%%%%%%% Tournament Index: Requesting cycles: {} from caller: {}",
        cycles,
        caller.0.to_text()
    );
    if cycles < CYCLES_TOP_UP_AMOUNT {
        return Err(CanisterManagementError::InsufficientCycles);
    }

    transfer_cycles(CYCLES_TOP_UP_AMOUNT, caller).await
}

async fn transfer_cycles(
    cycles_amount: u128,
    caller: TournamentId,
) -> Result<(), CanisterManagementError> {
    {
        let tournament_index_state = STATE
            .lock()
            .map_err(|_| CanisterManagementError::LockError)?;
        if !tournament_index_state.tournaments.contains_key(&caller) {
            return Err(CanisterManagementError::Transfer(format!(
                "Caller is not a valid destination: {}",
                caller.0.to_text()
            )));
        }
    }

    top_up_canister(caller.0, cycles_amount).await?;
    Ok(())
}
