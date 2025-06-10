use candid::{Nat, Principal};
use errors::canister_management_error::CanisterManagementError;
use ic_cdk::management_canister::{canister_status, deposit_cycles, CanisterStatusArgs, DepositCyclesArgs};

const MINIMUM_CYCLE_THRESHOLD: u128 = 100_000_000_000;
const TOP_UP_AMOUNT: u128 = 900_000_000_000;

/// Top up a canister with cycles.
///
/// # Parameters
///
/// - `canister_id`: The canister principal id to top up.
/// - `amount`: The amount of cycles to top up.
///
/// # Errors
///
/// - [`CanisterManagementError`] if the top up fails.
pub async fn top_up_canister(
    canister_id: Principal,
    amount: u128,
) -> Result<(), CanisterManagementError> {
    match deposit_cycles(&DepositCyclesArgs { canister_id }, amount).await {
        Ok(()) => Ok(()),
        Err(e) => Err(CanisterManagementError::ManagementCanisterError(format!(
            "Failed to top up canister: {}",
            e
        ))),
    }
}

/// Top up canisters if their cycles are below a certain threshold.
///
/// # Parameters
///
/// - `canisters`: A list of canister principal ids to top up.
///
/// # Errors
///
/// - [`CanisterManagementError`] if the top up fails.
pub async fn monitor_and_top_up_canisters(
    canisters: Vec<Principal>,
) -> Result<(), CanisterManagementError> {
    for canister_id in canisters {
        match canister_status(&CanisterStatusArgs { canister_id }).await {
            Ok(status) => {
                if status.cycles < MINIMUM_CYCLE_THRESHOLD {
                    let _ = top_up_canister(canister_id, TOP_UP_AMOUNT).await;
                }
            }
            Err(e) => {
                return Err(CanisterManagementError::ManagementCanisterError(format!(
                    "{:?}",
                    e
                )));
            }
        }
    }
    Ok(())
}

/// Gets the cycle balances of the canisters.
///
/// # Parameters
///
/// - `canisters`: A list of canister principal ids to get the cycle balances of.
///
/// # Returns
///
/// - A list of tuples containing the canister principal id and the cycle balance.
pub async fn get_cycle_balances(canisters: Vec<Principal>) -> Vec<(Principal, Nat)> {
    let mut cycle_balances = Vec::new();
    for canister_id in canisters {
        match canister_status(&CanisterStatusArgs { canister_id }).await {
            Ok(status) => {
                cycle_balances.push((canister_id, status.cycles));
            }
            Err(_) => {
                continue;
            }
        }
    }
    cycle_balances
}

/// Checks if the cycle balance is below a threshold and if it is requests a top up.
///
/// # Parameters
///
/// - `canister_id`: The canister principal id to request a top up from.
///
/// # Errors
///
/// - [`CanisterManagementError`] if the top up fails.
pub async fn check_and_top_up_canister(
    canister_id: Principal,
    cycle_dispenser_canister: Principal,
    minimum_cycle_threshold: u128,
) -> Result<(), CanisterManagementError> {
    match canister_status(&CanisterStatusArgs { canister_id }).await {
        Ok(status) => {
            if status.cycles < minimum_cycle_threshold {
                ic_cdk::println!(
                    "Requesting cycles for canister: {} with cycles: {}",
                    canister_id,
                    status.cycles
                );
                if let Err(e) = request_cycles_wrapper(cycle_dispenser_canister).await {
                    ic_cdk::println!("Error requesting cycles: {:?}", e);
                    return Err(e);
                }
            }
        }
        Err(e) => {
            ic_cdk::println!("Error checking canister status: {:?}", e);
            return Err(CanisterManagementError::ManagementCanisterError(format!(
                "{:?}",
                e
            )));
        }
    }
    Ok(())
}

pub async fn request_cycles_wrapper(
    cycle_dispenser_canister: Principal,
) -> Result<(), CanisterManagementError> {
    let call_result = ic_cdk::call::Call::unbounded_wait(
        cycle_dispenser_canister,
        "request_cycles",
    )
    .await;

    match call_result {
        Ok(res) => match res.candid() {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error requesting cycles: {:?}", err);
                Err(CanisterManagementError::CanisterCallError(format!(
                    "Failed to decode request_cycles response: {:?}",
                    err
                )))
            }
        },
        Err(err) => {
            ic_cdk::println!("Error in request_cycles call: {:?}", err);
            Err(CanisterManagementError::CanisterCallError(format!(
                "{:?}",
                err
            )))
        }
    }
}

