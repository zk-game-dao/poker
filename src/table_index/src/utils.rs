use candid::Principal;
use errors::{table_error::TableError, table_index_error::TableIndexError};
use ic_ledger_types::{AccountIdentifier, Subaccount};
use table::poker::game::{table_functions::table::TableConfig, types::PublicTable};

use crate::TABLE_PLAYER_COUNTS;

pub async fn is_table_full(
    table: &TableConfig,
    table_id: &Principal,
) -> Result<bool, TableIndexError> {
    let player_counts = TABLE_PLAYER_COUNTS
        .lock()
        .map_err(|_| TableIndexError::LockError)?
        .clone();

    Ok(*player_counts.get(table_id).unwrap_or(&0) >= table.seats as usize)
}

pub async fn get_table_wrapper(table_id: Principal) -> Result<PublicTable, TableIndexError> {
    let (table,): (Result<PublicTable, TableError>,) =
        ic_cdk::call(table_id, "get_table", ()).await.map_err(|e| {
            TableIndexError::InterCanisterError(format!("Failed to get table canister: {:?}", e))
        })?;
    let table = table?;
    Ok(table)
}

pub struct CanisterState {
    pub owner: Principal,
    pub default_subaccount: Subaccount,
    pub account_identifier: AccountIdentifier,
}

pub fn create_default_subaccount() -> Subaccount {
    let bytes = [0u8; 32];

    Subaccount(bytes)
}

pub fn get_canister_state() -> CanisterState {
    let owner_principal = ic_cdk::api::id();
    let default_subaccount = create_default_subaccount();

    let account_identifier = AccountIdentifier::new(&owner_principal, &default_subaccount);
    CanisterState {
        owner: owner_principal,
        default_subaccount,
        account_identifier,
    }
}
