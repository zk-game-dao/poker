use candid::{decode_one, encode_args, Principal};
use currency::Currency;
use errors::{table_error::TableError, table_index_error::TableIndexError};
use pocket_ic::WasmResult;
use table::{
    poker::game::{
        table_functions::{table::TableConfig, types::BetType},
        types::PublicTable,
    },
    types::ReturnResult,
};
use table_index_types::filter::FilterOptions;

use crate::TestEnv;

use super::transfer::{approve_icrc1_tokens, transfer_icrc1_tokens, transfer_tokens};

impl TestEnv {
    pub fn create_table(&self, table_config: &TableConfig) -> Result<PublicTable, TableIndexError> {
        let result = self.pocket_ic.update_call(
            self.canister_ids.table_index,
            Principal::anonymous(),
            "create_table",
            encode_args((table_config.clone(), false)).unwrap(),
        );

        match result.expect("Failed to create table") {
            WasmResult::Reply(arg) => {
                let table_id: Result<PublicTable, TableIndexError> = decode_one(&arg).unwrap();
                table_id
            }
            _ => panic!("Failed to create table"),
        }
    }

    pub fn get_table(&self, table_id: Principal) -> Result<PublicTable, TableError> {
        let table_state = self.pocket_ic.query_call(
            table_id,
            Principal::anonymous(),
            "get_table",
            encode_args(()).unwrap(),
        );

        match table_state.expect("Failed to get table") {
            WasmResult::Reply(arg) => {
                let table: Result<PublicTable, TableError> = decode_one(&arg).unwrap();
                table
            }
            _ => panic!("Failed to get table"),
        }
    }

    pub fn get_tables(
        &self,
        filter_options: Option<FilterOptions>,
        page_number: u16,
        page_size: u16,
    ) -> Result<Vec<(Principal, TableConfig)>, TableIndexError> {
        let table_state = self.pocket_ic.update_call(
            self.canister_ids.table_index,
            Principal::anonymous(),
            "get_tables",
            encode_args((filter_options, page_number, page_size)).unwrap(),
        );

        match table_state.expect("Failed to get tables") {
            WasmResult::Reply(arg) => {
                let tables: Result<Vec<(Principal, TableConfig)>, TableIndexError> =
                    decode_one(&arg).unwrap();
                tables
            }
            _ => panic!("Failed to get tables"),
        }
    }

    pub fn join_test_table(
        &self,
        table_id: Principal,
        user: Principal,
        user_id: Principal,
        deposit_amount: u64,
        seat_index: u64,
        player_sitting_out: bool,
    ) -> Result<PublicTable, TableError> {
        let table_state: Result<pocket_ic::WasmResult, pocket_ic::UserError> =
            self.pocket_ic.update_call(
                table_id,
                user,
                "join_table",
                encode_args((
                    user,               // user_principal
                    user_id,            // wallet_principal_id
                    Some(seat_index),   // seat_index as Option<u64>
                    deposit_amount,     // deposit_amount
                    player_sitting_out, // player_sitting_out
                ))
                .unwrap(),
            );

        match table_state.expect("Failed to join table") {
            pocket_ic::WasmResult::Reply(arg) => {
                let table: Result<PublicTable, TableError> = candid::decode_one(&arg).unwrap();
                table
            }
            _ => panic!("Failed to join table"),
        }
    }

    pub fn pause_table(&self, table_id: Principal) -> Result<(), TableError> {
        let res: Result<pocket_ic::WasmResult, pocket_ic::UserError> = self.pocket_ic.update_call(
            table_id,
            Principal::anonymous(),
            "pause_table",
            encode_args(()).unwrap(),
        );

        match res.expect("Failed to pause table") {
            WasmResult::Reply(arg) => {
                let table: Result<(), TableError> = candid::decode_one(&arg).unwrap();
                table
            }
            _ => panic!("Failed to call pause_table"),
        }
    }

    pub fn unpause_table(&self, table_id: Principal) -> Result<(), TableError> {
        let res: Result<pocket_ic::WasmResult, pocket_ic::UserError> = self.pocket_ic.update_call(
            table_id,
            Principal::anonymous(),
            "resume_table",
            encode_args(()).unwrap(),
        );

        match res.expect("Failed to unpause table") {
            WasmResult::Reply(arg) => {
                let table: Result<(), TableError> = candid::decode_one(&arg).unwrap();
                table
            }
            _ => panic!("Failed to call unpause_table"),
        }
    }

    pub fn leave_test_table(
        &self,
        table_id: Principal,
        user: Principal,
        wallet_principal_id: Principal,
    ) -> Result<PublicTable, TableError> {
        let table_state: Result<pocket_ic::WasmResult, pocket_ic::UserError> =
            self.pocket_ic.update_call(
                table_id,
                wallet_principal_id,
                "leave_table",
                encode_args((user, wallet_principal_id)).unwrap(),
            );

        match table_state.expect("Failed to leave table") {
            pocket_ic::WasmResult::Reply(arg) => {
                let table: Result<PublicTable, TableError> = candid::decode_one(&arg).unwrap();
                table
            }
            _ => panic!("Failed to leave table"),
        }
    }

    pub fn start_betting_round_test_table(&self, table_id: Principal) -> Result<(), TableError> {
        let table_state: Result<pocket_ic::WasmResult, pocket_ic::UserError> =
            self.pocket_ic.update_call(
                table_id,
                Principal::anonymous(),
                "start_new_betting_round",
                candid::encode_args(()).unwrap(),
            );

        match table_state.expect("Failed to start betting round") {
            pocket_ic::WasmResult::Reply(arg) => {
                let table: Result<(), TableError> = candid::decode_one(&arg).unwrap();
                table
            }
            _ => panic!("Failed to start betting round"),
        }
    }

    pub fn player_sitting_out_test_table(&self, table_id: Principal, user_id: Principal) {
        let table_state: Result<pocket_ic::WasmResult, pocket_ic::UserError> =
            self.pocket_ic.update_call(
                table_id,
                user_id,
                "player_sitting_out",
                candid::encode_args((user_id,)).unwrap(),
            );

        match table_state.expect("Failed to sit out") {
            pocket_ic::WasmResult::Reply(arg) => {
                let table: Result<(), TableError> = candid::decode_one(&arg).unwrap();
                table.unwrap();
            }
            _ => panic!("Failed to sit out"),
        }
    }

    pub fn player_sitting_in_test_table(
        &self,
        table_id: Principal,
        user: Principal,
        users_canister_id: Principal,
    ) -> Result<(), TableError> {
        let table_state: Result<pocket_ic::WasmResult, pocket_ic::UserError> =
            self.pocket_ic.update_call(
                table_id,
                user,
                "player_sitting_in",
                candid::encode_args((users_canister_id, user, true)).unwrap(),
            );

        match table_state.expect("Failed to sit in") {
            pocket_ic::WasmResult::Reply(arg) => {
                let table: Result<(), TableError> = candid::decode_one(&arg).unwrap();
                table
            }
            _ => panic!("Failed to sit in"),
        }
    }

    pub fn player_bet(
        &self,
        table_id: Principal,
        user: Principal,
        bet_type: BetType,
    ) -> Result<(), TableError> {
        let table_state: Result<pocket_ic::WasmResult, pocket_ic::UserError> =
            self.pocket_ic.update_call(
                table_id,
                user,
                "place_bet",
                candid::encode_args((user, bet_type)).unwrap(),
            );

        match table_state.expect("Failed to make move") {
            pocket_ic::WasmResult::Reply(arg) => {
                let table: Result<(), TableError> = candid::decode_one(&arg).unwrap();
                table
            }
            _ => panic!("Failed to make move"),
        }
    }

    pub fn player_check(&self, table_id: Principal, user: Principal) -> Result<(), TableError> {
        let table_state: Result<pocket_ic::WasmResult, pocket_ic::UserError> =
            self.pocket_ic.update_call(
                table_id,
                user,
                "check",
                candid::encode_args((user,)).unwrap(),
            );

        match table_state.expect("Failed to make move") {
            pocket_ic::WasmResult::Reply(arg) => {
                let table: Result<(), TableError> = candid::decode_one(&arg).unwrap();
                table
            }
            _ => panic!("Failed to make move"),
        }
    }

    pub fn player_fold(&self, table_id: Principal, user: Principal) -> Result<(), TableError> {
        let table_state: Result<pocket_ic::WasmResult, pocket_ic::UserError> =
            self.pocket_ic.update_call(
                table_id,
                user,
                "fold",
                candid::encode_args((user, false)).unwrap(),
            );

        match table_state.expect("Failed to make move") {
            pocket_ic::WasmResult::Reply(arg) => {
                let table: Result<(), TableError> = candid::decode_one(&arg).unwrap();
                table
            }
            _ => panic!("Failed to make move"),
        }
    }

    pub fn player_deposit(
        &self,
        table_id: Principal,
        users_canister_id: Principal,
        user_id: Principal,
        amount: u64,
    ) -> Result<ReturnResult, TableError> {
        let table_state: Result<pocket_ic::WasmResult, pocket_ic::UserError> =
            self.pocket_ic.update_call(
                table_id,
                user_id,
                "deposit_to_table",
                candid::encode_args((users_canister_id, user_id, amount, false)).unwrap(),
            );

        match table_state.expect("Failed to make move") {
            pocket_ic::WasmResult::Reply(arg) => {
                let table: Result<ReturnResult, TableError> = candid::decode_one(&arg).unwrap();
                println!("Deposit result: {:?}", table);
                table
            }
            _ => panic!("Failed to make move"),
        }
    }

    pub fn player_withdraw(
        &self,
        table_id: Principal,
        user_principal: Principal,
        wallet_principal_id: Principal,
        amount: u64,
    ) -> Result<(), TableError> {
        let table_state: Result<pocket_ic::WasmResult, pocket_ic::UserError> =
            self.pocket_ic.update_call(
                table_id,
                wallet_principal_id,
                "withdraw_from_table",
                candid::encode_args((user_principal, wallet_principal_id, amount)).unwrap(),
            );

        match table_state.expect("Failed to make move") {
            pocket_ic::WasmResult::Reply(arg) => {
                let table: Result<(), TableError> = candid::decode_one(&arg).unwrap();
                table
            }
            _ => panic!("Failed to make move"),
        }
    }

    pub fn get_test_icp_table(&self) -> (Principal, TableConfig) {
        // Create a table configuration
        let table_config = TableConfig {
            name: "Test Table".to_string(),
            game_type: table::poker::game::types::GameType::NoLimit(1),
            seats: 6,
            timer_duration: 30,
            card_color: 0,
            color: 0,
            environment_color: 0,
            auto_start_timer: 10,
            max_inactive_turns: 3,
            currency_type: table::poker::game::table_functions::types::CurrencyType::Real(
                Currency::ICP,
            ),
            enable_rake: Some(false),
            max_seated_out_turns: None,
            is_private: Some(false),
            ante_type: None,
            table_type: None,
            is_shared_rake: None,
            require_proof_of_humanity: None,
            is_paused: None,
        };

        let public_table = self
            .create_table(&table_config)
            .expect("Failed to create table");
        (public_table.id, table_config)
    }

    pub fn get_test_fake_table(&self) -> (Principal, TableConfig) {
        // Create a table configuration
        let table_config = TableConfig {
            name: "Test Table".to_string(),
            game_type: table::poker::game::types::GameType::NoLimit(1),
            seats: 6,
            timer_duration: 30,
            card_color: 0,
            color: 0,
            environment_color: 0,
            auto_start_timer: 10,
            max_inactive_turns: 3,
            currency_type: table::poker::game::table_functions::types::CurrencyType::Fake,
            enable_rake: Some(false),
            max_seated_out_turns: None,
            is_private: Some(false),
            ante_type: None,
            table_type: None,
            is_shared_rake: None,
            require_proof_of_humanity: None,
            is_paused: None,
        };

        let public_table = self
            .create_table(&table_config)
            .expect("Failed to create table");
        (public_table.id, table_config)
    }

    pub fn get_ckusdc_test_table(&self) -> (Principal, TableConfig) {
        // Create a table configuration
        let table_config = TableConfig {
            name: "Test Table".to_string(),
            game_type: table::poker::game::types::GameType::NoLimit(1),
            seats: 6,
            timer_duration: 30,
            card_color: 0,
            color: 0,
            environment_color: 0,
            auto_start_timer: 10,
            max_inactive_turns: 3,
            currency_type: table::poker::game::table_functions::types::CurrencyType::Real(
                Currency::CKETHToken(currency::types::currency::CKTokenSymbol::USDC),
            ),
            enable_rake: Some(false),
            max_seated_out_turns: None,
            is_private: Some(false),
            ante_type: None,
            table_type: None,
            is_shared_rake: None,
            require_proof_of_humanity: None,
            is_paused: None,
        };

        let public_table = self
            .create_table(&table_config)
            .expect("Failed to create table");
        (public_table.id, table_config)
    }

    pub fn create_test_user_with_icp_approval(
        &self,
        name: String,
        amount: f64,
        table_id: Principal,
    ) -> (Principal, Principal, u128) {
        let user_id = Principal::self_authenticating(&name);
        let user_principal = self
            .create_user(name, user_id)
            .expect("Failed to create user");

        // Transfer tokens to the user
        transfer_tokens(
            &self.pocket_ic,
            amount,
            user_id,
            self.canister_ids.ledger,
            Principal::self_authenticating("minter"),
            true,
        );

        // Approve tokens to the table
        let approval_block = approve_icrc1_tokens(
            &self.pocket_ic,
            amount,
            table_id,
            self.canister_ids.ledger,
            user_id,
        );

        (user_principal.users_canister_id, user_id, approval_block)
    }

    // Create a test user with CKUSDC deposit using approvals
    pub fn create_test_user_with_ckusdc_approval(
        &self,
        name: String,
        amount: f64,
        table_id: Principal,
    ) -> (Principal, Principal, u128) {
        let user_id = Principal::self_authenticating(&name);
        let user_principal = self
            .create_user(name, user_id)
            .expect("Failed to create user");

        // Transfer tokens to the user
        transfer_icrc1_tokens(
            &self.pocket_ic,
            amount,
            user_id,
            self.canister_ids.ckusdc_ledger,
            Principal::self_authenticating("minter"),
            true,
        );

        // Approve tokens to the table
        let approval_block = approve_icrc1_tokens(
            &self.pocket_ic,
            amount,
            table_id,
            self.canister_ids.ckusdc_ledger,
            user_id,
        );

        (user_principal.users_canister_id, user_id, approval_block)
    }

    // Update player_deposit to use approval-based flow
    pub fn player_deposit_with_approval(
        &self,
        table_id: Principal,
        user: Principal,
        amount: u64,
    ) -> Result<table::types::ReturnResult, TableError> {
        let table_state: Result<pocket_ic::WasmResult, pocket_ic::UserError> =
            self.pocket_ic.update_call(
                table_id,
                user,
                "deposit_to_table",
                candid::encode_args((user, user, amount, false)).unwrap(),
            );

        match table_state.expect("Failed to deposit") {
            pocket_ic::WasmResult::Reply(arg) => {
                let result: Result<table::types::ReturnResult, TableError> =
                    candid::decode_one(&arg).unwrap();
                result
            }
            _ => panic!("Failed to deposit"),
        }
    }

    // Adding a helper function to pre-approve tokens for a specific table for all test usage
    pub fn approve_tokens_for_testing(
        &self,
        table_id: Principal,
        user: Principal,
        amount: f64,
        is_icp: bool,
    ) -> u128 {
        let ledger = if is_icp {
            self.canister_ids.ledger
        } else {
            self.canister_ids.ckusdc_ledger
        };

        approve_icrc1_tokens(&self.pocket_ic, amount, table_id, ledger, user)
    }

    // Adding a helper function to pre-approve tokens for a specific table for all test usage
    pub fn transfer_approve_tokens_for_testing(
        &self,
        table_id: Principal,
        user: Principal,
        amount: f64,
        is_icp: bool,
    ) -> u128 {
        let ledger = if is_icp {
            self.canister_ids.ledger
        } else {
            self.canister_ids.ckusdc_ledger
        };

        if is_icp {
            transfer_tokens(
                &self.pocket_ic,
                amount,
                user,
                self.canister_ids.ledger,
                Principal::self_authenticating("minter"),
                true,
            );
        } else {
            transfer_icrc1_tokens(
                &self.pocket_ic,
                amount,
                user,
                self.canister_ids.ckusdc_ledger,
                Principal::self_authenticating("minter"),
                true,
            );
        }

        approve_icrc1_tokens(&self.pocket_ic, amount, table_id, ledger, user)
    }
}
