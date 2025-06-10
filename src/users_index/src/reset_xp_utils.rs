use std::cell::RefCell;

use canister_functions::leaderboard_utils::{
    calculate_amount_to_transfer, calculate_amount_to_transfer_pure_poker, PERCENTAGE_PAYOUT,
};
use errors::user_error::UserError;
use ic_cdk_timers::TimerId;
use intercanister_call_wrappers::users_canister::{
    clear_experience_points_wrapper, clear_pure_poker_experience_points_wrapper, get_user_wrapper,
};

use crate::{CURRENCY_MANAGER, USER_INDEX_STATE};

thread_local! {
    static TIMER_ID: RefCell<Option<TimerId>> = const { RefCell::new(None) };
    static LAST_RESET_TIME: RefCell<Option<u64>> = const { RefCell::new(None) };
}

fn is_reset_time() -> bool {
    let now = ic_cdk::api::time();
    let days_since_epoch = now / (24 * 60 * 60 * 1_000_000_000);
    let day_of_week = (days_since_epoch + 4) % 7; // 0 = Sunday, 4 = Thursday

    let nanos_in_day = now % (24 * 60 * 60 * 1_000_000_000);
    let is_midnight = nanos_in_day < 60_000_000_000; // Check within first minute

    let should_reset = (day_of_week == 1 || day_of_week == 5) && is_midnight;

    // ic_cdk::println!("Day of week: {}, is midnight: {}, should reset: {}", day_of_week, is_midnight, should_reset);
    if should_reset {
        let mut already_reset = false;
        LAST_RESET_TIME.with(|last_reset| {
            if let Some(last_time) = *last_reset.borrow() {
                if now - last_time < 24 * 60 * 60 * 1_000_000_000 {
                    already_reset = true;
                }
            }
            if !already_reset {
                *last_reset.borrow_mut() = Some(now);
            }
        });
        !already_reset
    } else {
        false
    }
}

#[ic_cdk::heartbeat]
async fn heartbeat() {
    if is_reset_time() {
        match reset_all_experience_points().await {
            Ok(_) => ic_cdk::println!("Successfully reset experience points"),
            Err(e) => ic_cdk::println!("Failed to reset experience points: {:?}", e),
        }
        match reset_all_pure_poker_experience_points().await {
            Ok(_) => ic_cdk::println!("Successfully reset pure poker experience points"),
            Err(e) => ic_cdk::println!("Failed to reset pure poker experience points: {:?}", e),
        }
    }
}

async fn reset_all_experience_points() -> Result<(), UserError> {
    let leaderboard = {
        let user_index_state = USER_INDEX_STATE
            .lock()
            .map_err(|_| UserError::LockError)?
            .clone();
        user_index_state.get_experience_points_leaderboard().await
    }?;

    let users_index_state = {
        USER_INDEX_STATE
            .lock()
            .map_err(|_| UserError::LockError)?
            .clone()
    };

    let currency_manager = {
        CURRENCY_MANAGER
            .lock()
            .map_err(|_| UserError::LockError)?
            .clone()
    };

    let mut i = 0;
    for (user, xp) in leaderboard {
        if xp == 0 {
            break;
        }
        let users_canister = match users_index_state.get_users_canister_principal(user) {
            Some(canister) => canister,
            None => {
                ic_cdk::println!("User {} not found in users canisters", user.to_text());
                continue;
            }
        };
        let user_obj = match get_user_wrapper(*users_canister, user).await {
            Ok(user) => user,
            Err(e) => {
                ic_cdk::println!("Failed to get user {}: {:?}", user.to_text(), e);
                continue;
            }
        };

        // If user is not verified they are not eligible for payout.
        if user_obj.is_verified.unwrap_or(false) {
            let amount = calculate_amount_to_transfer(PERCENTAGE_PAYOUT[i]);
            ic_cdk::println!(
                "Transferring {} ICP to user {} with principal {}",
                amount,
                user_obj.user_name,
                user.to_text()
            );
            match currency_manager
                .withdraw(&currency::Currency::ICP, user, amount)
                .await
            {
                Ok(_) => (),
                Err(e) => ic_cdk::println!("Failed to transfer ICP: {:?}", e),
            }
            i += 1;
            if i >= PERCENTAGE_PAYOUT.len() {
                break; // Only transfer to top 5 users
            }
        }
    }

    let user_canisters = {
        let user_index_state = USER_INDEX_STATE.lock().map_err(|_| UserError::LockError)?;
        user_index_state.get_users_canisters()
    };

    for user_canister in user_canisters {
        if let Err(e) = clear_experience_points_wrapper(user_canister).await {
            ic_cdk::println!(
                "Failed to clear experience points for user canister {}: {:?}",
                user_canister.to_text(),
                e
            );
            continue;
        }
    }

    Ok(())
}

async fn reset_all_pure_poker_experience_points() -> Result<(), UserError> {
    let leaderboard = {
        let user_index_state = USER_INDEX_STATE
            .lock()
            .map_err(|_| UserError::LockError)?
            .clone();
        user_index_state
            .get_pure_poker_experience_points_leaderboard()
            .await
    }?;
    let users_index_state = {
        USER_INDEX_STATE
            .lock()
            .map_err(|_| UserError::LockError)?
            .clone()
    };

    let currency_manager = {
        CURRENCY_MANAGER
            .lock()
            .map_err(|_| UserError::LockError)?
            .clone()
    };

    let mut i = 0;
    for (user, xp) in leaderboard {
        if xp == 0 {
            break;
        }
        let users_canister = match users_index_state.get_users_canister_principal(user) {
            Some(canister) => canister,
            None => {
                ic_cdk::println!("User {} not found in users canisters", user.to_text());
                continue;
            }
        };
        let user_obj = match get_user_wrapper(*users_canister, user).await {
            Ok(user) => user,
            Err(e) => {
                ic_cdk::println!("Failed to get user {}: {:?}", user.to_text(), e);
                continue;
            }
        };

        // If user is not verified they are not eligible for payout.
        if user_obj.is_verified.unwrap_or(false) {
            let amount = calculate_amount_to_transfer_pure_poker(PERCENTAGE_PAYOUT[i]);
            ic_cdk::println!(
                "Transferring {} BTC to user {} with principal {}",
                amount,
                user_obj.user_name,
                user.to_text()
            );
            match currency_manager
                .withdraw(&currency::Currency::BTC, user, amount)
                .await
            {
                Ok(_) => (),
                Err(e) => ic_cdk::println!("Failed to transfer ICP: {:?}", e),
            }
            i += 1;
            if i >= PERCENTAGE_PAYOUT.len() {
                break; // Only transfer to top 5 users
            }
        }
    }

    let user_canisters = {
        let user_index_state = USER_INDEX_STATE.lock().map_err(|_| UserError::LockError)?;
        user_index_state.get_users_canisters()
    };

    for user_canister in user_canisters {
        if let Err(e) = clear_pure_poker_experience_points_wrapper(user_canister).await {
            ic_cdk::println!(
                "Failed to clear pure poker experience points for user canister {}: {:?}",
                user_canister.to_text(),
                e
            );
            continue;
        }
    }

    Ok(())
}
