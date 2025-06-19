use candid::{decode_one, encode_args, Principal};
use errors::{tournament_error::TournamentError, tournament_index_error::TournamentIndexError};
use table::poker::game::table_functions::table::{TableConfig, TableId};
use tournaments::tournaments::types::{NewTournament, TournamentData, TournamentId};
use user::user::{UsersCanisterId, WalletPrincipalId};

use crate::TestEnv;

impl TestEnv {
    pub fn create_tournament(
        &self,
        tournament_config: &NewTournament,
        table_config: &TableConfig,
    ) -> Result<TournamentId, TournamentIndexError> {
        let result = self.pocket_ic.update_call(
            self.canister_ids.tournament_index,
            Principal::anonymous(),
            "create_tournament",
            encode_args((tournament_config.clone(), table_config.clone(), false)).unwrap(),
        );

        match result {
            Ok(arg) => {
                let table_id: Result<TournamentId, TournamentIndexError> = decode_one(&arg).unwrap();
                table_id
            }
            _ => panic!("Failed to create tournament"),
        }
    }

    pub fn get_tournament(
        &self,
        tournament_id: TournamentId,
    ) -> Result<TournamentData, TournamentError> {
        let tournament_state = self.pocket_ic.query_call(
            tournament_id.0,
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
        tournament_id: TournamentId,
        users_canister_id: UsersCanisterId,
        user_id: WalletPrincipalId,
    ) -> Result<(), TournamentError> {
        let result = self.pocket_ic.update_call(
            tournament_id.0,
            user_id.0,
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
        tournament_id: TournamentId,
        users_canister_id: UsersCanisterId,
        user_id: WalletPrincipalId,
    ) -> Result<(), TournamentError> {
        let result = self.pocket_ic.update_call(
            tournament_id.0,
            user_id.0,
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
        tournament_id: TournamentId,
        user_principal: UsersCanisterId,
        wallet_principal_id: WalletPrincipalId,
        table_id: TableId,
    ) -> Result<(), TournamentError> {
        let result = self.pocket_ic.update_call(
            tournament_id.0,
            user_principal.0,
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
        tournament_id: TournamentId,
        user_principal: UsersCanisterId,
        table_id: TableId,
        wallet_principal_id: WalletPrincipalId,
    ) -> Result<(), TournamentError> {
        let result = self.pocket_ic.update_call(
            tournament_id.0,
            user_principal.0,
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
        tournament_id: TournamentId,
        user_principal: UsersCanisterId,
        table_id: TableId,
    ) -> Result<(), TournamentError> {
        let result = self.pocket_ic.update_call(
            tournament_id.0,
            tournament_id.0,
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
