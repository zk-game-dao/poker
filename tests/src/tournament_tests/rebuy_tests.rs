use std::time::Duration;

use candid::Principal;
use currency::Currency;
use table::poker::game::{
    table_functions::{
        table::{TableConfig, TableId, TableType},
        types::{BetType, CurrencyType, DealStage, SeatStatus},
    },
    utils::convert_to_e8s,
};
use tournaments::tournaments::{
    tournament_type::{BuyInOptions, NewTournamentOptions, TournamentSizeType, TournamentType},
    types::{NewTournament, NewTournamentSpeedType, PayoutPercentage, TournamentId, TournamentState},
};
use user::user::{UsersCanisterId, WalletPrincipalId};

use crate::TestEnv;

// Test utilities
impl TestEnv {
    fn setup_reentry_tournament(&self) -> (TournamentId, NewTournament) {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let now = std::time::SystemTime::now();
        self.pocket_ic.set_time(now.into());

        let new_tournament = NewTournament {
            name: "Test Tournament".to_string(),
            description: "Test Tournament Description".to_string(),
            hero_picture: "".to_string(),
            currency: CurrencyType::Real(Currency::ICP),
            buy_in: convert_to_e8s(10.0),
            starting_chips: 1000,
            speed_type: NewTournamentSpeedType::Regular(20),
            min_players: 5,
            max_players: 8,
            late_registration_duration_ns: 10,
            payout_structure: vec![PayoutPercentage {
                position: 1,
                percentage: 100,
            }],
            tournament_type: TournamentType::BuyIn(TournamentSizeType::SingleTable(
                BuyInOptions::new_reentry(NewTournamentOptions {
                    enable_reentry: true,
                    max_reentries: 1,
                    reentry_end_timestamp: u64::MAX,
                    reentry_price: convert_to_e8s(10.0),
                    reentry_chips: 1000,
                    addon_price: convert_to_e8s(10.0),
                    addon_chips: 2000,
                    addon_start_time: 0,
                    addon_end_time: 0,
                    max_addons: 0,
                    enable_rebuy: false,
                    rebuy_price: 0,
                    rebuy_chips: 0,
                    rebuy_end_timestamp: 0,
                    rebuy_window_seconds: 0,
                    max_rebuys: 0,
                    min_chips_for_rebuy: 0,
                })
                .unwrap(),
            )),
            start_time: current_time + 1_000_000_000, // 1 second in future
            require_proof_of_humanity: false,
        };
        let tournament_config = new_tournament;

        let table_config = TableConfig {
            name: "Test Table".to_string(),
            game_type: table::poker::game::types::GameType::NoLimit(0),
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
            table_type: Some(TableType::Tournament {
                tournament_id: Principal::anonymous(),
                is_final_table: true,
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

    fn register_players(
        &self,
        tournament_id: TournamentId,
        count: u8,
        name_suffix: &str,
    ) -> Vec<(WalletPrincipalId, UsersCanisterId)> {
        let mut players = Vec::new();

        for i in 0..count {
            let user = self
                .create_user(
                    format!("User {}", i),
                    WalletPrincipalId(Principal::self_authenticating(format!("user{}{}", i, name_suffix))),
                )
                .expect("Failed to create user");

            self.transfer_approve_tokens_for_testing(
                tournament_id.0,
                user.principal_id,
                1000.0,
                true,
            );

            self.join_tournament(tournament_id, user.users_canister_id, user.principal_id)
                .unwrap();
            players.push((user.principal_id, user.users_canister_id));
        }

        players
    }

    // Simulate a hand where target_player loses all chips
    fn all_but_one_players_all_in(&self, table_id: TableId) {
        let table = self.get_table(table_id).unwrap();
        for _ in table.users.users.keys() {
            let table = self.get_table(table_id).unwrap();
            let player =
                if let SeatStatus::Occupied(principal) = table.seats[table.current_player_index] {
                    principal
                } else {
                    WalletPrincipalId::default()
                };
            let player_balance = table.users.get(&player).unwrap().balance;
            let player_current_total_bet = table
                .user_table_data
                .get(&player)
                .unwrap()
                .current_total_bet;

            // Calculate raise amount (all remaining chips)
            let raise_amount = player_balance.0 - player_current_total_bet;

            // Have target player raise all their chips
            if self
                .player_bet(table_id, player, BetType::Raised(raise_amount))
                .is_err()
            {
                self.player_fold(table_id, player).unwrap();
            }
        }
    }

    fn all_players_called(&self, table_id: TableId) {
        let table = self.get_table(table_id).unwrap();
        let player_count = table.users.users.len();
        for _ in 0..player_count - 2 {
            let table = self.get_table(table_id).unwrap();
            let player =
                if let SeatStatus::Occupied(principal) = table.seats[table.current_player_index] {
                    principal
                } else {
                    WalletPrincipalId::default()
                };
            // Have target player call
            self.player_bet(table_id, player, BetType::Called).unwrap();
        }
        let table = self.get_table(table_id).unwrap();
        let player =
            if let SeatStatus::Occupied(principal) = table.seats[table.current_player_index] {
                principal
            } else {
                WalletPrincipalId::default()
            };
        self.player_fold(table_id, player).unwrap();

        let table = self.get_table(table_id).unwrap();
        let player =
            if let SeatStatus::Occupied(principal) = table.seats[table.current_player_index] {
                principal
            } else {
                WalletPrincipalId::default()
            };
        self.player_check(table_id, player).unwrap();
    }

    pub fn start_tournament(&self) {
        // Advance time past start_time
        self.pocket_ic
            .advance_time(Duration::from_nanos(120_000_000_000));

        // Ensure tournament starts
        for _ in 0..30 {
            self.pocket_ic.tick();
        }
    }
}

#[test]
fn test_reentry_into_tournament() {
    let test_env = TestEnv::get();

    // Setup tournament with reentry enabled
    let (tournament_id, _) = test_env.setup_reentry_tournament();

    // Register minimum required players
    let players = test_env
        .register_players(tournament_id, 5, "reentrytest")
        .into_iter()
        .collect::<Vec<_>>();

    // Start tournament
    test_env.start_tournament();
    let table_id = *test_env
        .get_tournament(tournament_id)
        .unwrap()
        .tables
        .keys()
        .next()
        .unwrap();
    let table = test_env.get_table(table_id).unwrap();
    assert_eq!(table.deal_stage, DealStage::Flop);

    // Verify tournament is running
    let tournament = test_env.get_tournament(tournament_id).unwrap();
    assert_eq!(tournament.state, TournamentState::Running);

    // Simulate player losing all chips
    let table = test_env.get_table(table_id).unwrap();
    println!("Deal stage: {:?}", table.deal_stage);
    test_env.all_players_called(table_id);

    let table = test_env.get_table(table_id).unwrap();
    println!("Deal stage: {:?}", table.deal_stage);
    test_env.all_but_one_players_all_in(table_id);

    // Verify player's chips are 0
    let table = test_env.get_table(table_id).unwrap();
    println!("Deal stage: {:?}", table.deal_stage);
    let _ = test_env.start_betting_round_test_table(table_id);
    let table = test_env.get_table(table_id).unwrap();
    println!("Deal stage: {:?}", table.deal_stage);
    let reentry_player = players
        .iter()
        .find(|&player| !table.users.users.contains_key(&player.0))
        .expect("No player was kicked");
    test_env.transfer_approve_tokens_for_testing(table_id.0, reentry_player.0, 10.0, true);

    let tournament = test_env.get_tournament(tournament_id).unwrap();
    assert_eq!(
        tournament
            .all_players
            .get(&reentry_player.0)
            .unwrap()
            .reentries,
        0
    );

    test_env
        .user_reentry_into_tournament(tournament_id, reentry_player.1, reentry_player.0, table_id)
        .unwrap();

    test_env
        .pocket_ic
        .advance_time(Duration::from_nanos(120_000_000_000));
    test_env.pocket_ic.tick();

    // Verify reentry was successful
    let tournament = test_env.get_tournament(tournament_id).unwrap();
    assert_eq!(
        tournament
            .current_players
            .get(&reentry_player.0)
            .unwrap()
            .reentries,
        1
    );
}
