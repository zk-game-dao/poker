use candid::{decode_one, encode_args, Principal};
use errors::{tournament_error::TournamentError, tournament_index_error::TournamentIndexError};
use table::poker::game::table_functions::table::TableConfig;
use tournaments::tournaments::types::{NewTournament, TournamentData};

use crate::TestEnv;

impl TestEnv {
    pub fn create_tournament(
        &self,
        tournament_config: &NewTournament,
        table_config: &TableConfig,
    ) -> Result<Principal, TournamentIndexError> {
        let result = self.pocket_ic.update_call(
            self.canister_ids.tournament_index,
            Principal::anonymous(),
            "create_tournament",
            encode_args((tournament_config.clone(), table_config.clone(), false)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let table_id: Result<Principal, TournamentIndexError> = decode_one(&arg).unwrap();
                table_id
            }
            _ => panic!("Failed to create tournament"),
        }
    }

    pub fn get_tournament(
        &self,
        tournament_id: Principal,
    ) -> Result<TournamentData, TournamentError> {
        let tournament_state = self.pocket_ic.query_call(
            tournament_id,
            Principal::anonymous(),
            "get_tournament",
            encode_args(()).unwrap(),
        );

        match tournament_state {
            Ok(arg) => {
                let tournament: Result<TournamentData, TournamentError> = decode_one(&arg).unwrap();
                tournament
            }
            _ => panic!("Failed to get tournament"),
        }
    }

    pub fn join_tournament(
        &self,
        tournament_id: Principal,
        users_canister_id: Principal,
        user_id: Principal,
    ) -> Result<(), TournamentError> {
        let result = self.pocket_ic.update_call(
            tournament_id,
            user_id,
            "user_join_tournament",
            encode_args((users_canister_id, user_id)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let res: Result<(), TournamentError> = decode_one(&arg).unwrap();
                res
            }
            _ => panic!("Failed to join tournament"),
        }
    }

    pub fn leave_tournament(
        &self,
        tournament_id: Principal,
        users_canister_id: Principal,
        user_id: Principal,
    ) -> Result<(), TournamentError> {
        let result = self.pocket_ic.update_call(
            tournament_id,
            user_id,
            "user_leave_tournament",
            encode_args((users_canister_id, user_id)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let res: Result<(), TournamentError> = decode_one(&arg).unwrap();
                res
            }
            _ => panic!("Failed to join tournament"),
        }
    }

    pub fn user_reentry_into_tournament(
        &self,
        tournament_id: Principal,
        user_principal: Principal,
        wallet_principal_id: Principal,
        table_id: Principal,
    ) -> Result<(), TournamentError> {
        let result = self.pocket_ic.update_call(
            tournament_id,
            user_principal,
            "user_reentry_into_tournament",
            encode_args((user_principal, wallet_principal_id, table_id)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let res: Result<(), TournamentError> = decode_one(&arg).unwrap();
                res
            }
            _ => panic!("Failed to rebuy into tournament"),
        }
    }

    pub fn user_refill_chips(
        &self,
        tournament_id: Principal,
        user_principal: Principal,
        table_id: Principal,
        wallet_principal_id: Principal,
    ) -> Result<(), TournamentError> {
        let result = self.pocket_ic.update_call(
            tournament_id,
            user_principal,
            "user_refill_chips",
            encode_args((user_principal, table_id, wallet_principal_id)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let res: Result<(), TournamentError> = decode_one(&arg).unwrap();
                res
            }
            _ => panic!("Failed to refill chips"),
        }
    }

    pub fn handle_user_losing(
        &self,
        tournament_id: Principal,
        user_principal: Principal,
        table_id: Principal,
    ) -> Result<(), TournamentError> {
        let result = self.pocket_ic.update_call(
            tournament_id,
            tournament_id,
            "handle_user_losing",
            encode_args((user_principal, table_id)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let res: Result<(), TournamentError> = decode_one(&arg).unwrap();
                res
            }
            _ => panic!("Failed to refill chips"),
        }
    }
}
