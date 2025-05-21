#![cfg(test)]

mod basic_tests;
mod cycle_tests;
mod deposit_tests;
mod env;
mod fake_currency_tests;
mod filter_tests;
mod leaderboard;
mod pause_unpause_tests;
mod pot;
mod sitting_in_and_out_tests;
mod table_tests;
mod tournament_tests;
mod transaction_history;
mod turn_tests;
mod upgrade_table_tests;
mod upgrade_tournament_tests;
mod utils;
mod wasms;

use std::collections::{HashMap, HashSet};

use candid::{CandidType, Nat, Principal};
use ic_ledger_types::{AccountIdentifier, Tokens, DEFAULT_SUBACCOUNT};
use pocket_ic::{PocketIc, PocketIcBuilder};
use utils::transfer::{transfer_icrc1_tokens, transfer_tokens};
use wasms::CanisterWasm;

const INIT_CYCLES_BALANCE: u128 = 10_000_000_000_000;

pub struct TestEnv {
    pub pocket_ic: PocketIc,
    pub canister_ids: CanisterIds,
}

pub struct CanisterIds {
    pub table_index: Principal,
    pub user_index: Principal,
    pub tournament_index: Principal,
    pub ledger: Principal,
    pub ckusdc_ledger: Principal,
    pub cycle_dispenser: Principal,
}

#[derive(CandidType)]
struct NnsLedgerCanisterInitArgs {
    minting_account: String,
    initial_values: HashMap<String, Tokens>,
    send_whitelist: HashSet<Principal>,
    transfer_fee: Option<Tokens>,
}

#[derive(CandidType)]
pub enum Icrc1LedgerCanisterInit {
    Init(Icrc1LedgerCanisterInitArgs),
}

#[derive(CandidType)]
pub struct Icrc1LedgerCanisterInitArgs {
    pub minting_account: Account,
    pub initial_balances: Vec<(Account, Nat)>,
    pub transfer_fee: Nat,
    pub token_symbol: String,
    pub token_name: String,
    pub metadata: Vec<(String, Vec<u8>)>,
    pub fee_collector_account: Option<Account>,
    pub decimals: Option<u8>,
    pub max_memo_length: Option<Nat>,
    pub feature_flags: Option<Vec<(String, bool)>>,
    pub maximum_number_of_accounts: Option<Nat>,
    pub accounts_overflow_trim_quantity: Option<Nat>,
    pub archive_options: ArchiveOptions,
}

#[derive(CandidType)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<[u8; 32]>,
}

#[derive(CandidType)]
pub struct ArchiveOptions {
    pub num_blocks_to_archive: u64,
    pub trigger_threshold: u64,
    pub max_transactions_per_response: Option<u64>,
    pub max_message_size_bytes: Option<u64>,
    pub cycles_for_archive_creation: Option<u64>,
    pub node_max_memory_size_bytes: Option<u64>,
    pub controller_id: Principal,
    pub more_controller_ids: Option<Vec<Principal>>,
}

impl Default for TestEnv {
    fn default() -> Self {
        Self::new(None)
    }
}

impl TestEnv {
    pub fn new(init_cycle_balance: Option<u128>) -> Self {
        let pocket_ic = PocketIcBuilder::new()
            .with_nns_subnet()
            .with_sns_subnet()
            .with_application_subnet()
            .build();

        let table_index = create_canister(
            &pocket_ic,
            (),
            wasms::TABLE_INDEX.clone(),
            init_cycle_balance,
        );
        let user_index = create_canister(
            &pocket_ic,
            (),
            wasms::USER_INDEX.clone(),
            init_cycle_balance,
        );
        let tournament_index = create_canister(
            &pocket_ic,
            (),
            wasms::TOURNAMENT_INDEX.clone(),
            init_cycle_balance,
        );
        let cycle_dispenser = create_canister(
            &pocket_ic,
            (),
            wasms::CYCLE_DISPENSER_CANISTER.clone(),
            Some(1_000_000_000_000_000),
        );

        let minting_account = AccountIdentifier::new(
            &Principal::self_authenticating("minter"),
            &DEFAULT_SUBACCOUNT,
        );
        let ledger_args = NnsLedgerCanisterInitArgs {
            minting_account: minting_account.to_string(),
            initial_values: HashMap::new(),
            send_whitelist: HashSet::new(),
            transfer_fee: Some(Tokens::from_e8s(10_000)),
        };
        let ledger = create_canister_with_id(
            &pocket_ic,
            "ryjl3-tyaaa-aaaaa-aaaba-cai",
            ledger_args,
            wasms::LEDGER.clone(),
        );
        transfer_tokens(
            &pocket_ic,
            1_000_000.0,
            user_index,
            ledger,
            Principal::self_authenticating("minter"),
            true,
        );

        let minting_principal = Principal::self_authenticating("minter");
        let init_args = Icrc1LedgerCanisterInit::Init(Icrc1LedgerCanisterInitArgs {
            minting_account: Account {
                owner: minting_principal,
                subaccount: None,
            },
            initial_balances: Vec::new(),
            transfer_fee: Nat::from(10_000u64),
            token_symbol: "LCKUSDC".to_string(),
            token_name: "Local CKUSDC".to_string(),
            metadata: Vec::new(),
            fee_collector_account: None,
            decimals: Some(6),
            max_memo_length: None,
            feature_flags: None,
            maximum_number_of_accounts: None,
            accounts_overflow_trim_quantity: None,
            archive_options: ArchiveOptions {
                num_blocks_to_archive: 2000,
                trigger_threshold: 2000,
                max_transactions_per_response: None,
                max_message_size_bytes: None,
                cycles_for_archive_creation: None,
                node_max_memory_size_bytes: None,
                controller_id: minting_principal,
                more_controller_ids: None,
            },
        });

        let ckusdc_ledger = create_canister_with_id(
            &pocket_ic,
            "xevnm-gaaaa-aaaar-qafnq-cai",
            init_args,
            wasms::ICRC1_LEDGER.clone(),
        );
        transfer_icrc1_tokens(
            &pocket_ic,
            1_000_000.0,
            user_index,
            ckusdc_ledger,
            minting_principal,
            true,
        );

        let canister_ids = CanisterIds {
            table_index,
            user_index,
            tournament_index,
            ledger,
            ckusdc_ledger,
            cycle_dispenser,
        };

        TestEnv {
            pocket_ic,
            canister_ids,
        }
    }
}

fn create_canister<A: CandidType>(
    pocket_ic: &PocketIc,
    args: A,
    canister_wasm: CanisterWasm,
    cycle_balance: Option<u128>,
) -> Principal {
    let canister_id = pocket_ic.create_canister();
    if let Some(cycle_balance) = cycle_balance {
        pocket_ic.add_cycles(canister_id, cycle_balance);
    } else {
        pocket_ic.add_cycles(canister_id, INIT_CYCLES_BALANCE);
    }
    let args = candid::encode_one(args).expect("Failed to encode args");
    pocket_ic.install_canister(canister_id, canister_wasm, args, None);
    canister_id
}

fn create_canister_with_id<A: CandidType>(
    pocket_ic: &PocketIc,
    canister_id: &str,
    args: A,
    canister_wasm: CanisterWasm,
) -> Principal {
    let canister_id = canister_id.try_into().expect("Invalid canister ID");
    pocket_ic
        .create_canister_with_id(None, None, canister_id)
        .expect("Create canister with ID failed");
    pocket_ic.add_cycles(canister_id, INIT_CYCLES_BALANCE);
    let args = candid::encode_one(args).expect("Failed to encode args");
    pocket_ic.install_canister(canister_id, canister_wasm, args, None);
    canister_id
}
