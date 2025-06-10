use candid::Principal;
use canister_functions::cycle::check_and_top_up_canister;
use errors::table_error::TableError;
use ic_ledger_types::{AccountIdentifier, Subaccount};
use intercanister_call_wrappers::{
    table_index::update_table_player_count_wrapper,
    tournament_canister::update_player_count_tournament_wrapper,
};
use table::poker::game::table_functions::types::CurrencyType;
use tournaments::tournaments::types::UserTournamentAction;

use crate::{BACKEND_PRINCIPAL, CURRENCY_MANAGER, RAKE_WALLET_ADDRESS_PRINCIPAL, TABLE};

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
    let owner_principal = ic_cdk::api::canister_self();
    let default_subaccount = create_default_subaccount();

    let account_identifier = AccountIdentifier::new(&owner_principal, &default_subaccount);
    CanisterState {
        owner: owner_principal,
        default_subaccount,
        account_identifier,
    }
}

pub fn handle_cycle_check() {
    let cycles = ic_cdk::api::canister_cycle_balance();
    if cycles >= MINIMUM_CYCLE_THRESHOLD {
        return;
    }

    ic_cdk::futures::spawn(async {
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

        if let Err(e) = check_and_top_up_canister(
            ic_cdk::api::canister_self(),
            table_index,
            MINIMUM_CYCLE_THRESHOLD,
        )
        .await
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
            ic_cdk::api::canister_self().to_text()
        ),
        UserTournamentAction::Leave(uid) => ic_cdk::println!(
            "User {} left the table {}",
            uid.to_text(),
            ic_cdk::api::canister_self().to_text()
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
    ic_cdk::futures::spawn(async move {
        if let Err(e) = update_player_count_tournament_wrapper(
            backend_principal,
            ic_cdk::api::canister_self(),
            user_action,
        )
        .await
        {
            ic_cdk::println!("Failed to update player count in tournament: {:?}", e);
        }
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

    ic_cdk::futures::spawn(async move {
        if let Err(e) = update_table_player_count_wrapper(
            backend_principal,
            ic_cdk::api::canister_self(),
            user_count,
        )
        .await
        {
            ic_cdk::println!("Failed to update table player count: {:?}", e);
        }
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

pub fn get_user_index_principal(table_index_principal: Principal) -> Principal {
    if table_index_principal == Principal::from_text("zbspl-ziaaa-aaaam-qbe2q-cai").unwrap() {
        Principal::from_text("lvq5c-nyaaa-aaaam-qdswa-cai").unwrap()
    } else if table_index_principal == Principal::from_text("e4yx7-lqaaa-aaaah-qdslq-cai").unwrap()
    {
        Principal::from_text("m3tym-daaaa-aaaah-qqbsq-cai").unwrap()
    } else {
        Principal::from_text("txyno-ch777-77776-aaaaq-cai").unwrap()
    }
}

pub async fn handle_last_user_leaving() -> Result<(), TableError> {
    let table = {
        let mut table_lock = TABLE.lock().map_err(|_| TableError::LockError)?;
        let table = match table_lock.as_mut() {
            Some(table) => table,
            None => return Err(TableError::StateNotInitialized), // No table to process
        };
        table.rake_total = Some(0);
        table.clone()
    };
    let currency_manager = {
        let currency_manager = CURRENCY_MANAGER.lock().map_err(|_| TableError::LockError)?;
        currency_manager
            .clone()
            .ok_or(TableError::StateNotInitialized)?
    };

    match table.config.currency_type {
        CurrencyType::Real(currency) => {
            let balance = currency_manager
                .get_balance(&currency, ic_cdk::api::canister_self())
                .await
                .map_err(|e| TableError::CanisterCallError(format!("{:?}", e)))?;
            if balance > 0 {
                if let Some((rake_share_principal, _rake_share_account_id)) =
                    table.config.is_shared_rake
                {
                    let house_rake = balance / 2;
                    if let Err(e) = currency_manager
                        .withdraw_rake(&currency, *RAKE_WALLET_ADDRESS_PRINCIPAL, house_rake as u64)
                        .await
                    {
                        ic_cdk::println!("Error withdrawing rake: {:?}", e);
                    }
                    if let Err(e) = currency_manager
                        .withdraw(&currency, rake_share_principal, house_rake as u64)
                        .await
                    {
                        ic_cdk::println!("Error withdrawing rake: {:?}", e);
                    }
                } else if let Err(e) = currency_manager
                    .withdraw_rake(&currency, *RAKE_WALLET_ADDRESS_PRINCIPAL, balance as u64)
                    .await
                {
                    ic_cdk::println!("Error withdrawing rake: {:?}", e);
                }
            }
        }
        CurrencyType::Fake => {
            ic_cdk::println!("Table uses fake currency, no balance check needed.");
            return Ok(());
        }
    }

    Ok(())
}
