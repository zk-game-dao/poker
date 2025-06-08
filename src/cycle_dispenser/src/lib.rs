use candid::Principal;
use canister_functions::cycle::top_up_canister;
use errors::canister_management_error::CanisterManagementError;
use lazy_static::lazy_static;

pub type PlayerId = u64;
pub type TableId = u64;

const CYCLES_TOP_UP_AMOUNT: u128 = 20_000_000_000_000;

// Define a global instance of GameState wrapped in a Mutex for safe concurrent access.
lazy_static! {
    static ref VALID_CALLERS: Vec<Principal> = vec![
        *TOURNAMENT_INDEX_PROD,
        *TABLE_INDEX_PROD,
        *USERS_INDEX_PROD,
        *TOURNAMENT_INDEX_TEST,
        *TABLE_INDEX_TEST,
        *USERS_INDEX_TEST,
        *TABLE_INDEX_DEV,
        *TOURNAMENT_INDEX_DEV,
        *USERS_INDEX_DEV,
    ];
    static ref TOURNAMENT_INDEX_PROD: Principal =
        Principal::from_text("zocwf-5qaaa-aaaam-qdfaq-cai").unwrap();
    static ref TABLE_INDEX_PROD: Principal =
        Principal::from_text("zbspl-ziaaa-aaaam-qbe2q-cai").unwrap();
    static ref USERS_INDEX_PROD: Principal =
        Principal::from_text("lvq5c-nyaaa-aaaam-qdswa-cai").unwrap();
    static ref TOURNAMENT_INDEX_TEST: Principal =
        Principal::from_text("u2qna-fiaaa-aaaag-at3ea-cai").unwrap();
    static ref TABLE_INDEX_TEST: Principal =
        Principal::from_text("e4yx7-lqaaa-aaaah-qdslq-cai").unwrap();
    static ref USERS_INDEX_TEST: Principal =
        Principal::from_text("m3tym-daaaa-aaaah-qqbsq-cai").unwrap();
    static ref TABLE_INDEX_DEV: Principal =
        Principal::from_text("tqzl2-p7777-77776-aaaaa-cai").unwrap();
    static ref TOURNAMENT_INDEX_DEV: Principal =
        Principal::from_text("t63gs-up777-77776-aaaba-cai").unwrap();
    static ref USERS_INDEX_DEV: Principal =
        Principal::from_text("txyno-ch777-77776-aaaaq-cai").unwrap();
}

#[ic_cdk::init]
fn init() {
    let principal = ic_cdk::api::canister_self();
    ic_cdk::println!(
        "Cycle dispenser canister {} initialized",
        principal.to_text()
    );
}

#[ic_cdk::update]
async fn request_cycles() -> Result<(), CanisterManagementError> {
    let cycles = ic_cdk::api::canister_cycle_balance();
    let caller = ic_cdk::api::msg_caller();

    if cycles < CYCLES_TOP_UP_AMOUNT {
        ic_cdk::println!(
            "Not enough cycles in the canister. Current balance: {}",
            cycles
        );
        return Err(CanisterManagementError::InsufficientCycles);
    }

    transfer_cycles(CYCLES_TOP_UP_AMOUNT, caller).await
}

async fn transfer_cycles(
    cycles_amount: u128,
    caller: Principal,
) -> Result<(), CanisterManagementError> {
    let destination = if VALID_CALLERS.contains(&caller) {
        caller
    } else {
        return Err(CanisterManagementError::Transfer(format!(
            "Caller is not a valid destination: {}",
            caller
        )));
    };

    top_up_canister(destination, cycles_amount).await
}

ic_cdk::export_candid!();
