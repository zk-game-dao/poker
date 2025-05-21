use crate::TestEnv;
use candid::{encode_args, Principal};
use currency::ckusdc_canister_interface::Account as Icrc1Account;
use currency::{
    ckusdc_canister_interface::ApproveArgs,
    icrc1_types::{Account, TransferArg, TransferErrorIcrc1},
};
use errors::user_error::UserError;
use ic_ledger_types::{AccountIdentifier, BlockIndex, Memo, Tokens, TransferArgs, TransferError};
use pocket_ic::PocketIc;
use table::poker::game::utils::convert_to_e8s;
use user::user::User;

pub fn transfer_tokens(
    pocket_ic: &PocketIc,
    amount: f64,
    to: Principal,
    ledger_canister: Principal,
    sender: Principal,
    minting: bool,
) -> BlockIndex {
    let fee = if minting {
        0
    } else {
        ic_ledger_types::DEFAULT_FEE.e8s()
    };
    let transfer_args = TransferArgs {
        memo: Memo(1), // You can set this to a meaningful value if needed
        amount: Tokens::from_e8s(convert_to_e8s(amount)),
        fee: Tokens::from_e8s(fee), // Standard fee is 10000 e8s
        from_subaccount: None,      // Use None if transferring from main account
        to: AccountIdentifier::new(&to, &ic_ledger_types::DEFAULT_SUBACCOUNT),
        created_at_time: None, // Let the ledger use the current time
    };

    let transfer_result = pocket_ic.update_call(
        ledger_canister,
        sender,
        "transfer",
        encode_args((transfer_args,)).unwrap(),
    );

    match transfer_result.expect("Failed to transfer funds") {
        pocket_ic::WasmResult::Reply(arg) => {
            let block_index: Result<BlockIndex, TransferError> = candid::decode_one(&arg).unwrap();
            block_index.unwrap()
        }
        _ => panic!("Failed to transfer funds"),
    }
}

pub fn transfer_icrc1_tokens(
    pocket_ic: &PocketIc,
    amount: f64,
    to: Principal,
    ledger_canister: Principal,
    sender: Principal,
    minting: bool,
) -> u128 {
    let fee = if minting {
        0
    } else {
        ic_ledger_types::DEFAULT_FEE.e8s()
    };

    let transfer_args = TransferArg {
        to: Account {
            owner: to,
            subaccount: None,
        },
        fee: Some(fee.into()),
        memo: None,
        from_subaccount: None,
        created_at_time: None,
        amount: convert_to_e8s(amount).into(),
    };

    let transfer_result = pocket_ic.update_call(
        ledger_canister,
        sender,
        "icrc1_transfer",
        encode_args((transfer_args,)).unwrap(),
    );

    match transfer_result.expect("Failed to transfer funds") {
        pocket_ic::WasmResult::Reply(arg) => {
            let block_index: Result<u128, TransferErrorIcrc1> = candid::decode_one(&arg).unwrap();
            block_index.unwrap()
        }
        _ => panic!("Failed to transfer funds"),
    }
}

// Add a function to approve tokens to a spender
pub fn approve_icrc1_tokens(
    pocket_ic: &PocketIc,
    amount: f64,
    spender: Principal,
    ledger_canister: Principal,
    owner: Principal,
) -> u128 {
    let approve_args = ApproveArgs {
        spender: Icrc1Account {
            owner: spender,
            subaccount: None,
        },
        amount: convert_to_e8s(amount).into(),
        fee: Some(ic_ledger_types::DEFAULT_FEE.e8s().into()),
        memo: None,
        from_subaccount: None,
        created_at_time: None,
        expected_allowance: None,
        expires_at: None,
    };

    let approve_result = pocket_ic.update_call(
        ledger_canister,
        owner,
        "icrc2_approve",
        encode_args((approve_args,)).unwrap(),
    );

    match approve_result.expect("Failed to approve tokens") {
        pocket_ic::WasmResult::Reply(arg) => {
            let block_index: Result<u128, TransferErrorIcrc1> = candid::decode_one(&arg).unwrap();
            block_index.unwrap()
        }
        _ => panic!("Failed to approve tokens"),
    }
}

// // Add a function to check allowance
// pub fn check_allowance(
//     pocket_ic: &PocketIc,
//     owner: Principal,
//     spender: Principal,
//     ledger_canister: Principal,
// ) -> Allowance {
//     let args = AllowanceArgs {
//         account: Account {
//             owner,
//             subaccount: None,
//         },
//         spender: Account {
//             owner: spender,
//             subaccount: None,
//         },
//     };

//     let result = pocket_ic.query_call(
//         ledger_canister,
//         owner,
//         "icrc2_allowance",
//         encode_args((args,)).unwrap(),
//     );

//     match result.expect("Failed to check allowance") {
//         pocket_ic::WasmResult::Reply(arg) => {
//             let allowance: Allowance = candid::decode_one(&arg).unwrap();
//             allowance
//         }
//         _ => panic!("Failed to check allowance"),
//     }
// }

impl TestEnv {
    pub fn deposit_to_test_user_index(&self, block_index: u64, user: Principal, amount: f64) {
        let user_state: Result<pocket_ic::WasmResult, pocket_ic::UserError> =
            self.pocket_ic.update_call(
                self.canister_ids.user_index,
                Principal::anonymous(),
                "deposit",
                encode_args((user, convert_to_e8s(amount), block_index)).unwrap(),
            );

        match user_state.expect("Failed to deposit funds") {
            pocket_ic::WasmResult::Reply(arg) => {
                let user: Result<User, UserError> = candid::decode_one(&arg).unwrap();
                let user = user.unwrap();
                assert_eq!(user.balance, convert_to_e8s(amount))
            }
            _ => panic!("Failed to deposit funds"),
        }
    }
}
