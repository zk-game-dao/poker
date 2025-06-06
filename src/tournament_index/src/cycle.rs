use candid::Principal;
use canister_functions::cycle::top_up_canister;
use errors::{
    canister_management_error::CanisterManagementError,
    tournament_index_error::TournamentIndexError,
};

use crate::STATE;

const CYCLES_TOP_UP_AMOUNT: u128 = 3_000_000_000_000;

#[ic_cdk::update]
async fn request_cycles() -> Result<(), TournamentIndexError> {
    let cycles = ic_cdk::api::canister_cycle_balance();
    let caller = ic_cdk::api::msg_caller();
    ic_cdk::println!(
        "%%%%%%%%%%% Tournament Index: Requesting cycles: {} from caller: {}",
        cycles,
        caller.to_text()
    );
    if cycles < CYCLES_TOP_UP_AMOUNT {
        return Err(TournamentIndexError::ManagementCanisterError(
            CanisterManagementError::InsufficientCycles,
        ));
    }

    transfer_cycles(CYCLES_TOP_UP_AMOUNT as u128, caller).await
}

async fn transfer_cycles(
    cycles_amount: u128,
    caller: Principal,
) -> Result<(), TournamentIndexError> {
    {
        let tournament_index_state = STATE.lock().map_err(|_| TournamentIndexError::LockError)?;
        if !tournament_index_state.active_tournaments.contains(&caller) {
            return Err(TournamentIndexError::ManagementCanisterError(
                CanisterManagementError::Transfer(format!(
                    "Caller is not a valid destination: {}",
                    caller
                )),
            ));
        }
    }

    top_up_canister(caller, cycles_amount).await?;
    Ok(())
}
