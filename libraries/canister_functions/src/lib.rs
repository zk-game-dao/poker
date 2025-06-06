use candid::Principal;
use errors::canister_management_error::CanisterManagementError;
use ic_cdk::api::management_canister::main::{
    canister_status, create_canister, delete_canister, stop_canister, CanisterIdRecord,
    CanisterInstallMode, CanisterSettings, CanisterStatusType, CreateCanisterArgument,
    InstallCodeArgument,
};

pub mod cycle;
pub mod leaderboard_utils;
pub mod rake_constants;
pub mod rake_stats;

const CYCLE_TOP_UP_AMOUNT: u128 = 1_000_000_000_000;

/// Creates a new canister
///
/// # Parameters
///
/// - `controller_principals` - The list of controller principals for the new canister.
///
/// # Returns
///
/// - The principal of the new canister.
/// - A [`CanisterManagementError`] if the operation fails.
pub async fn create_canister_wrapper(
    mut controller_principals: Vec<Principal>,
    cycle_amount: Option<u128>,
) -> Result<Principal, CanisterManagementError> {
    // Step 1: Create a new canister
    let app_backend_principal = ic_cdk::api::canister_self();
    controller_principals.push(app_backend_principal);
    let create_canister_arg = CreateCanisterArgument {
        settings: Some(CanisterSettings {
            controllers: Some(controller_principals),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
            reserved_cycles_limit: None,
            log_visibility: None,
            wasm_memory_limit: None,
        }),
    };

    let cycle_amount = cycle_amount.unwrap_or(CYCLE_TOP_UP_AMOUNT);
    let create_canister_response = create_canister(create_canister_arg, cycle_amount)
        .await
        .map_err(|e| CanisterManagementError::CreateCanisterError(format!("{:?}", e)))?;

    Ok(create_canister_response.0.canister_id)
}

/// Installs the wasm code into a canister
///
/// # Parameters
///
/// - `canister_id` - The principal of the canister to install the wasm code into.
/// - `wasm_module` - The wasm module to install.
///
/// # Errors
///
/// - A [`CanisterManagementError`] if the operation fails.
#[allow(dependency_on_unit_never_type_fallback)]
pub async fn install_wasm_code(
    canister_id: Principal,
    wasm_module: Vec<u8>,
) -> Result<(), CanisterManagementError> {
    let install_code_arg = InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id,
        wasm_module,
        arg: candid::encode_args(()).unwrap(),
    };
    ic_cdk::call(
        Principal::management_canister(),
        "install_code",
        (install_code_arg,),
    )
    .await
    .map_err(|e| CanisterManagementError::InstallCodeError(format!("{:?}", e)))?;
    Ok(())
}

/// Upgrades the wasm code into a canister
///
/// # Parameters
///
/// - `canister_id` - The principal of the canister to install the wasm code into.
/// - `wasm_module` - The wasm module to install.
///
/// # Errors
///
/// - A [`CanisterManagementError`] if the operation fails.
#[allow(dependency_on_unit_never_type_fallback)]
pub async fn upgrade_wasm_code(
    canister_id: Principal,
    wasm_module: Vec<u8>,
) -> Result<(), CanisterManagementError> {
    let install_code_arg = InstallCodeArgument {
        mode: CanisterInstallMode::Upgrade(None),
        canister_id,
        wasm_module,
        arg: candid::encode_args(()).unwrap(),
    };
    ic_cdk::call(
        Principal::management_canister(),
        "install_code",
        (install_code_arg,),
    )
    .await
    .map_err(|e| CanisterManagementError::InstallCodeError(format!("{:?}", e)))?;
    Ok(())
}

/// Stops and deletes a canister
///
/// # Parameters
///
/// - `canister_id` - The principal of the canister to stop and delete.
///
/// # Errors
///
/// - A [`CanisterManagementError`] if the operation fails.
pub async fn stop_and_delete_canister(
    canister_id: Principal,
) -> Result<(), CanisterManagementError> {
    // Step 2: Stop the canister
    stop_canister(CanisterIdRecord { canister_id })
        .await
        .map_err(|e| CanisterManagementError::StopCanisterError(format!("{:?}", e)))?;

    let mut error_count = 0;
    loop {
        match canister_status(CanisterIdRecord { canister_id }).await {
            Ok(canister_status) => {
                if canister_status.0.status == CanisterStatusType::Stopped {
                    break;
                }
            }
            Err(e) => {
                error_count += 1;
                if error_count > 1000 {
                    return Err(CanisterManagementError::ManagementCanisterError(format!(
                        "{:?}",
                        e
                    )));
                }
                continue;
            }
        }
    }
    // Step 3: Delete the canister
    delete_canister(CanisterIdRecord { canister_id })
        .await
        .map_err(|e| CanisterManagementError::DeleteCanisterError(format!("{:?}", e)))?;

    Ok(())
}
