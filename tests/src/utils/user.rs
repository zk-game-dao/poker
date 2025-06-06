use candid::{decode_one, encode_args, Principal};
use currency::Currency;
use errors::user_error::UserError;
use ic_ledger_types::{AccountIdentifier, Tokens};
use user::user::User;

use crate::TestEnv;

use super::transfer::{transfer_icrc1_tokens, transfer_tokens};

impl TestEnv {
    pub fn create_user(&self, name: String, user_id: Principal) -> Result<User, UserError> {
        let user_canister = self.pocket_ic.update_call(
            self.canister_ids.user_index,
            user_id,
            "create_user",
            encode_args((name, (), user_id, ())).unwrap(),
        );

        match user_canister {
            Ok(arg) => {
                let user_canister: Result<User, UserError> = decode_one(&arg).unwrap();
                user_canister
            }
            _ => panic!("Failed to create user"),
        }
    }

    pub fn get_user(
        &self,
        user_principal: Principal,
        user_id: Principal,
    ) -> Result<User, UserError> {
        let user = self.pocket_ic.update_call(
            user_principal,
            Principal::anonymous(),
            "get_user",
            encode_args((user_id,)).unwrap(),
        );

        match user {
            Ok(arg) => {
                let user: Result<User, UserError> = decode_one(&arg).unwrap();
                user
            }
            _ => panic!("Failed to create user"),
        }
    }

    pub fn add_active_table(
        &self,
        user_principal: Principal,
        table_id: Principal,
    ) -> Result<User, UserError> {
        let user = self.pocket_ic.update_call(
            user_principal,
            Principal::anonymous(),
            "add_active_table",
            encode_args((table_id,)).unwrap(),
        );

        match user {
            Ok(arg) => {
                let user: Result<User, UserError> = decode_one(&arg).unwrap();
                user
            }
            _ => panic!("Failed to add active table"),
        }
    }

    pub fn add_experience_points(
        &self,
        users_canister_id: Principal,
        user_id: Principal,
        currency: Currency,
        exp: u64,
    ) -> Result<User, UserError> {
        let user = self.pocket_ic.update_call(
            users_canister_id,
            Principal::anonymous(),
            "add_experience_points",
            encode_args((exp, currency, user_id)).unwrap(),
        );

        match user {
            Ok(arg) => {
                let user: Result<User, UserError> = decode_one(&arg).unwrap();
                user
            }
            _ => panic!("Failed to add experience points"),
        }
    }

    pub fn get_user_experience_points(
        &self,
        user_principal: Principal,
    ) -> Result<Vec<(Principal, u64)>, UserError> {
        let exp = self.pocket_ic.query_call(
            user_principal,
            Principal::anonymous(),
            "get_user_experience_points",
            encode_args(()).unwrap(),
        );

        match exp {
            Ok(arg) => {
                let exp: Result<Vec<(Principal, u64)>, UserError> = decode_one(&arg).unwrap();
                exp
            }
            _ => panic!("Failed to get experience points"),
        }
    }

    pub fn get_pure_poker_user_experience_points(
        &self,
        user_principal: Principal,
    ) -> Result<Vec<(Principal, u64)>, UserError> {
        let exp = self.pocket_ic.query_call(
            user_principal,
            Principal::anonymous(),
            "get_pure_poker_user_experience_points",
            encode_args(()).unwrap(),
        );

        match exp {
            Ok(arg) => {
                let exp: Result<Vec<(Principal, u64)>, UserError> = decode_one(&arg).unwrap();
                exp
            }
            _ => panic!("Failed to get experience points"),
        }
    }

    pub fn get_wallet_balance(&self, principal: Principal) -> Result<Tokens, String> {
        let account = AccountIdentifier::new(&principal, &ic_ledger_types::DEFAULT_SUBACCOUNT);
        let args = ic_ledger_types::AccountBalanceArgs { account };

        let exp = self.pocket_ic.query_call(
            self.canister_ids.ledger,
            Principal::anonymous(),
            "account_balance",
            encode_args((args,)).unwrap(),
        );

        match exp {
            Ok(arg) => {
                let balance = decode_one::<Tokens>(&arg)
                    .map_err(|e| format!("Failed to decode balance: {}", e))?;
                Ok(balance)
            }
            _ => panic!("Failed to get balance"),
        }
    }

    pub fn create_test_user(&self, user_name: &str) -> (Principal, Principal) {
        let user_id = Principal::self_authenticating(user_name);
        let user_canister = self
            .create_user(user_name.to_string().clone(), user_id)
            .unwrap();
        (user_canister.users_canister_id, user_id)
    }

    pub fn create_test_user_with_icp_deposit(
        &self,
        user_name: String,
        amount: f64,
        to: Principal,
    ) -> (Principal, u64) {
        let (_, user_id) = self.create_test_user(&user_name);
        let block_index_user_1 = transfer_tokens(
            &self.pocket_ic,
            amount,
            to,
            self.canister_ids.ledger,
            self.canister_ids.user_index,
            false,
        );
        (user_id, block_index_user_1)
    }

    pub fn create_test_user_with_ckusdc_deposit(
        &self,
        user_name: String,
        amount: f64,
        to: Principal,
    ) -> (Principal, u128) {
        let (_, user_id) = self.create_test_user(&user_name);
        let block_index_user_1 = transfer_icrc1_tokens(
            &self.pocket_ic,
            amount,
            to,
            self.canister_ids.ckusdc_ledger,
            self.canister_ids.user_index,
            false,
        );
        (user_id, block_index_user_1)
    }
}
