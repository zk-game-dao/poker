use candid::Principal;
use currency::Currency;
use user::user::WalletPrincipalId;
use std::time::Duration;
use table::poker::game::{
    table_functions::{
        table::{TableConfig, TableType},
        types::{CurrencyType, DealStage},
    },
    utils::convert_to_e8s,
};
use tournaments::tournaments::{
    blind_level::SpeedType,
    table_balancing::TableBalancer,
    tournament_type::{BuyInOptions, TournamentSizeType, TournamentType},
    types::{NewTournament, NewTournamentSpeedType, PayoutPercentage, TournamentId, TournamentState},
};

use crate::TestEnv;

impl TestEnv {
    fn setup_multi_table_tournament_with_levels(
        &self,
        players_per_table: u8,
        min_players: u8,
        max_players: u8,
    ) -> (TournamentId, NewTournament) {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let now = std::time::SystemTime::now();
        self.pocket_ic.set_time(now.into());

        let tournament_config = NewTournament {
            name: "Multi-Table Test Tournament".to_string(),
            description: "Test Tournament Description".to_string(),
            hero_picture: "".to_string(),
            currency: CurrencyType::Real(Currency::ICP),
            buy_in: convert_to_e8s(10.0),
            starting_chips: 1000,
            speed_type: NewTournamentSpeedType::Regular(110),
            min_players,
            max_players: max_players as u32,
            late_registration_duration_ns: 0,
            require_proof_of_humanity: false,
            payout_structure: vec![PayoutPercentage {
                position: 1,
                percentage: 100,
            }],
            tournament_type: TournamentType::BuyIn(TournamentSizeType::MultiTable(
                BuyInOptions::new_freezout(),
                TableBalancer::new(
                    min_players,
                    players_per_table,
                    &SpeedType::new_regular(1000, 110),
                ),
            )),
            start_time: current_time + 1_000_000_000, // 1 second in future
        };

        let table_config = TableConfig {
            name: "Test Table".to_string(),
            game_type: table::poker::game::types::GameType::NoLimit(10),
            seats: players_per_table,
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
            table_type: Some(TableType::Tournament {
                tournament_id: Principal::anonymous(),
                is_final_table: false,
            }),
            is_shared_rake: None,
            require_proof_of_humanity: None,
            is_paused: None,
        };

        let id = self
            .create_tournament(&tournament_config, &table_config)
            .unwrap();
        (id, tournament_config)
    }

    pub fn register_and_start_tournament(
        &self,
        tournament_id: TournamentId,
        player_count: u64,
    ) -> Vec<WalletPrincipalId> {
        let mut players = Vec::new();
        // Register enough players for multiple tables
        for i in 0..player_count {
            let user = self
                .create_user(
                    format!("User {}", i),
                    WalletPrincipalId(Principal::self_authenticating(format!("user{}final", i))),
                )
                .unwrap();

            self.transfer_approve_tokens_for_testing(
                tournament_id.0,
                user.principal_id,
                1000.0,
                true,
            );

            self.join_tournament(tournament_id, user.users_canister_id, user.principal_id)
                .unwrap();
            players.push(user.principal_id);
        }

        // Advance time to start tournament
        for _ in 0..6 {
            self.pocket_ic.advance_time(Duration::from_secs(30));
            for _ in 0..30 {
                self.pocket_ic.tick();
            }
        }
        players
    }
}

#[test]
fn test_multi_table_tournament_start() {
    let test_env = TestEnv::new(None);
    let (tournament_id, _) = test_env.setup_multi_table_tournament_with_levels(6, 2, 20);

    // Register enough players for multiple tables
    test_env.register_and_start_tournament(tournament_id, 12);

    // Verify tournament started properly
    let tournament = test_env.get_tournament(tournament_id).unwrap();
    assert_eq!(tournament.state, TournamentState::Running);
    assert_eq!(tournament.tables.len(), 2);

    // Verify all tables are active
    for table_id in tournament.tables.keys() {
        let table = test_env.get_table(*table_id).unwrap();
        assert_eq!(table.deal_stage, DealStage::Flop);
    }
}

#[test]
fn test_synchronized_blind_levels() {
    let test_env = TestEnv::new(None);
    let (tournament_id, _) = test_env.setup_multi_table_tournament_with_levels(4, 2, 20);

    test_env.register_and_start_tournament(tournament_id, 12);

    // Verify all tables have synchronized blinds
    let tournament = test_env.get_tournament(tournament_id).unwrap();
    let (small, big, _) = tournament.get_current_blinds();

    for table_id in tournament.tables.keys() {
        let table = test_env.get_table(*table_id).unwrap();
        assert_eq!(table.small_blind, small);
        assert_eq!(table.big_blind, big);
    }

    // Advance time to next blind level
    for _ in 0..6 {
        test_env.pocket_ic.advance_time(Duration::from_secs(210));
        for _ in 0..30 {
            test_env.pocket_ic.tick();
        }
    }

    let tournament = test_env.get_tournament(tournament_id).unwrap();
    let (small, big, _) = tournament.get_current_blinds();

    for table_id in tournament.tables.keys() {
        let table = test_env.get_table(*table_id).unwrap();
        assert_eq!(table.small_blind, small);
        assert_eq!(table.big_blind, big);
    }
}
