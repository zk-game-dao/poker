use canister_functions::cycle::check_and_top_up_canister;

use crate::BACKEND_PRINCIPAL;

const MINIMUM_CYCLE_THRESHOLD: u128 = 350_000_000_000;

/// Handle cycle checking and top-up if needed
pub fn handle_cycle_check() {
    let cycles = ic_cdk::api::canister_cycle_balance();
    if cycles >= MINIMUM_CYCLE_THRESHOLD {
        return;
    }

    ic_cdk::futures::spawn(async {
        let clan_index_result = BACKEND_PRINCIPAL.lock();
        let clan_index = match clan_index_result {
            Ok(lock) => match *lock {
                Some(index) => index,
                None => {
                    ic_cdk::println!("Clan index not found");
                    return;
                }
            },
            Err(_) => {
                ic_cdk::println!("Lock error occurred while getting clan index");
                return;
            }
        };

        if let Err(e) = check_and_top_up_canister(
            ic_cdk::api::canister_self(),
            clan_index,
            MINIMUM_CYCLE_THRESHOLD,
        )
        .await
        {
            ic_cdk::println!("Failed to top up clan canister: {:?}", e);
        }
    });
}
