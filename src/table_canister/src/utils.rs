use candid::Principal;
use canister_functions::cycle::check_and_top_up_canister;
use errors::{
    table_error::TableError, table_index_error::TableIndexError, tournament_error::TournamentError,
    user_error::UserError,
};
use ic_ledger_types::{AccountIdentifier, Subaccount};
use table::poker::game::table_functions::types::CurrencyType;
use tournaments::tournaments::types::UserTournamentAction;
use user::user::TransactionType;

use crate::{BACKEND_PRINCIPAL, TABLE};

const MINIMUM_CYCLE_THRESHOLD: u128 = 350_000_000_000;

pub type PlayerId = u64;
pub type TableId = u64;

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

pub fn handle_cycle_check() {
    let cycles = ic_cdk::api::canister_balance();
    if cycles as u128 >= MINIMUM_CYCLE_THRESHOLD {
        return;
    }

    ic_cdk::spawn(async {
        let table_index_result = BACKEND_PRINCIPAL.lock();
        let table_index = match table_index_result {
            Ok(lock) => match *lock {
                Some(index) => index,
                None => {
                    ic_cdk::println!("User not found");
                    return; // or perform some error handling
                }
            },
            Err(_) => {
                ic_cdk::println!("Lock error occurred");
                return; // or handle the lock error
            }
        };

        if let Err(e) =
            check_and_top_up_canister(ic_cdk::api::id(), table_index, MINIMUM_CYCLE_THRESHOLD).await
        {
            ic_cdk::println!("Failed to top up canister: {:?}", e);
        }
    });
}

pub fn update_player_count_tournament(user_action: UserTournamentAction) -> Result<(), TableError> {
    match &user_action {
        UserTournamentAction::Join(uid) => ic_cdk::println!(
            "User {} joined the table {}",
            uid.to_text(),
            ic_cdk::api::id().to_text()
        ),
        UserTournamentAction::Leave(uid) => ic_cdk::println!(
            "User {} left the table {}",
            uid.to_text(),
            ic_cdk::api::id().to_text()
        ),
    }
    let backend_principal = BACKEND_PRINCIPAL.lock();
    let backend_principal = match backend_principal {
        Ok(lock) => match *lock {
            Some(principal) => principal,
            None => {
                ic_cdk::println!("Backend principal not found");
                return Ok(());
            }
        },
        Err(_) => {
            ic_cdk::println!("Lock error occurred");
            return Ok(());
        }
    };
    ic_cdk::spawn(async move {
        let (_,): (Result<(), TournamentError>,) = match ic_cdk::call(
            backend_principal,
            "update_player_count_tournament",
            (ic_cdk::api::id(), user_action),
        )
        .await
        .map_err(|e| TableError::CanisterCallError(format!("{:?}", e)))
        {
            Ok(res) => res,
            Err(e) => {
                ic_cdk::println!("Failed to update player count: {:?}", e);
                return;
            }
        };
    });
    Ok(())
}

pub fn update_table_player_count(user_count: usize) -> Result<(), TableError> {
    let backend_principal = BACKEND_PRINCIPAL.lock();
    let backend_principal = match backend_principal {
        Ok(lock) => match *lock {
            Some(principal) => principal,
            None => {
                ic_cdk::println!("Backend principal not found");
                return Ok(());
            }
        },
        Err(_) => {
            ic_cdk::println!("Lock error occurred");
            return Ok(());
        }
    };

    ic_cdk::spawn(async move {
        let (_,): (Result<(), TableIndexError>,) = match ic_cdk::call(
            backend_principal,
            "update_table_player_count",
            (ic_cdk::api::id(), user_count),
        )
        .await
        .map_err(|e| TableError::CanisterCallError(format!("{:?}", e)))
        {
            Ok(res) => res,
            Err(e) => {
                ic_cdk::println!("Failed to update table player count: {:?}", e);
                return;
            }
        };
    });

    Ok(())
}

pub fn handle_table_validity_check() -> Result<(), TableError> {
    let table = TABLE.lock().map_err(|_| TableError::LockError)?;
    let table = table.as_ref().ok_or(TableError::TableNotFound)?;

    if table.config.currency_type == CurrencyType::Fake {
        return Err(TableError::InvalidRequest(
            "Table uses fake currency".to_string(),
        ));
    }

    Ok(())
}

pub async fn log_user_transaction(
    users_canister_principal: Principal,
    user_id: Principal,
    amount: u64,
    transaction_type: TransactionType,
    timestamp: Option<u64>,
    currency: Option<String>,
) -> Result<(), TableError> {
    let (ret,): (Result<(), UserError>,) = ic_cdk::call(
        users_canister_principal,
        "log_transaction",
        (user_id, amount, transaction_type, timestamp, currency),
    )
    .await
    .map_err(|e| TableError::CanisterCallError(format!("{:?} {}", e.0, e.1)))?;
    ret?;
    Ok(())
}

pub fn get_user_index_principal(table_index_principal: Principal) -> Principal {
    if table_index_principal == Principal::from_text("zbspl-ziaaa-aaaam-qbe2q-cai").unwrap() {
        Principal::from_text("lvq5c-nyaaa-aaaam-qdswa-cai").unwrap()
    } else if table_index_principal == Principal::from_text("e4yx7-lqaaa-aaaah-qdslq-cai").unwrap() {
        Principal::from_text("m3tym-daaaa-aaaah-qqbsq-cai").unwrap()
    } else {
        Principal::from_text("txyno-ch777-77776-aaaaq-cai").unwrap()
    }
}
