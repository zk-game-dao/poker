use std::time::Duration;

use candid::Principal;
use currency::Currency;
use table::poker::game::{
    table_functions::{
        table::{TableConfig, TableType},
        types::{BetType, CurrencyType, DealStage},
    },
    utils::convert_to_e8s,
};
use tournaments::tournaments::{
    blind_level::SpeedType, table_balancing::TableBalancer, tournament_type::{BuyInOptions, TournamentSizeType, TournamentType}, types::{NewTournament, NewTournamentSpeedType, TournamentId, TournamentState}, utils::calculate_rake
};
use user::user::{UsersCanisterId, WalletPrincipalId};

use crate::TestEnv;

// Test utilities
impl TestEnv {
    pub fn setup_payout_tournament(
        &self,
        min_players: u8,
        players_per_table: u8,
    ) -> (TournamentId, NewTournament) {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let now = std::time::SystemTime::now();
        self.pocket_ic.set_time(now.into());

        let tournament_config = NewTournament {
            name: "Test Tournament".to_string(),
            description: "Test Tournament Description".to_string(),
            hero_picture: "".to_string(),
            currency: CurrencyType::Real(Currency::ICP),
            buy_in: convert_to_e8s(10.0),
            guaranteed_prize_pool: None,
            starting_chips: 1000,
            speed_type: NewTournamentSpeedType::Regular(20),
            min_players: 5,
            max_players: 8,
            late_registration_duration_ns: 10,
            tournament_type: TournamentType::BuyIn(TournamentSizeType::MultiTable(
                BuyInOptions::new_freezout(),
                TableBalancer::new(
                    min_players,
                    players_per_table,
                    &SpeedType::new_regular(1000, 110),
                ),
            )),
            start_time: current_time + 1_000_000_000, // 1 second in future
            require_proof_of_humanity: false,
        };

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

    pub fn simulate_tournament_until_completion(
        &self,
        tournament_id: TournamentId,
        num_players: u8,
    ) -> Vec<(UsersCanisterId, WalletPrincipalId)> {
        let mut eliminated_players = Vec::new();
        let mut active_players = Vec::new();

        // Register players
        for i in 0..num_players {
            let user_id = WalletPrincipalId(self.pocket_ic.create_canister());
            let user = self
                .create_user(format!("User {}", i), user_id)
                .expect("Failed to create user");

            let amount = (1000000000 + ic_ledger_types::DEFAULT_FEE.e8s() * 2) as f64 / 1e8;
            println!(
                "User {}: {} - Amount: {}",
                i,
                user.users_canister_id.0.to_text(),
                amount
            );
            self.transfer_approve_tokens_for_testing(tournament_id.0, user_id, amount, true);

            self.join_tournament(tournament_id, user.users_canister_id, user_id)
                .unwrap();
            active_players.push((user.users_canister_id, user_id));
        }

        // Start tournament
        self.pocket_ic
            .advance_time(Duration::from_nanos(120_000_000_000));
        for _ in 0..30 {
            self.pocket_ic.tick();
        }

        let tournament = self.get_tournament(tournament_id).unwrap();
        let table_id = *tournament.tables.keys().next().unwrap();

        // Simulate hands until tournament completion
        while active_players.len() > 1 {
            let mut table = self.get_table(table_id).unwrap();
            let mut first_action_made = false;

            // Keep processing until we reach showdown
            while table.deal_stage != DealStage::Showdown {
                // Get the current player whose turn it is
                let current_player = table
                    .get_player_at_seat(table.current_player_index)
                    .expect("Failed to get current player");

                // Get their balance
                let user_balance = table
                    .users
                    .get(&current_player)
                    .map(|user| user.balance)
                    .unwrap_or_default();

                if user_balance > 0 {
                    if !first_action_made {
                        // First player goes all-in
                        self.player_bet(table_id, current_player, BetType::Raised(user_balance.0))
                            .unwrap();
                        first_action_made = true;
                    } else {
                        // Subsequent players call
                        self.player_bet(table_id, current_player, BetType::Called)
                            .unwrap();
                    }
                }

                // Update table state
                table = self.get_table(table_id).unwrap();
            }
            for _ in 0..6 {
                self.pocket_ic.tick();
            }

            // Update active players list based on who's left
            active_players.retain(|player| {
                let balance = table
                    .users
                    .get(&player.1)
                    .map(|user| user.balance)
                    .unwrap_or_default();
                if balance == 0 {
                    eliminated_players.push(*player);
                    false
                } else {
                    true
                }
            });

            // Give time for processing
            self.pocket_ic.advance_time(Duration::from_secs(70));
            for _ in 0..30 {
                self.pocket_ic.tick();
            }
        }

        // Add the final winner to eliminated players in last position
        if let Some(winner) = active_players.pop() {
            eliminated_players.push(winner);
        }

        eliminated_players
    }

    fn verify_payouts(
        &self,
        tournament_id: TournamentId,
        players: &[(UsersCanisterId, WalletPrincipalId)],
        expected_payouts: &[(usize, u64)],
    ) {
        // Get all players' balances
        let mut player_balances: Vec<_> = players
            .iter()
            .map(|player| {
                let balance = self.get_wallet_balance(player.1.0).unwrap();
                (balance.e8s(), *player)
            })
            .collect();

        // Sort by balance descending to match payout order
        player_balances.sort_by(|a, b| b.0.cmp(&a.0));

        println!(
            "Player balances: {:?}",
            player_balances
                .iter()
                .map(|b| (b.0 / 1e8 as u64, (b.1.0.0.to_text(), b.1.1.0.to_text())))
                .collect::<Vec<_>>()
        );

        // Transaction fee per transfer
        let tx_fee = ic_ledger_types::DEFAULT_FEE.e8s();

        // For each expected payout, verify that at least one player got approximately that amount
        for (_, expected_payout) in expected_payouts {
            let expected_with_fee = expected_payout.saturating_sub(tx_fee);
            assert!(
                player_balances
                    .iter()
                    .any(|(balance, _)| { *balance == expected_with_fee }),
                "No player received the expected payout of {} (with fee subtracted)",
                expected_with_fee
            );
        }

        // Verify tournament state
        let tournament = self.get_tournament(tournament_id).unwrap();
        assert_eq!(tournament.state, TournamentState::Completed);
    }
}

#[test]
fn test_winner_takes_all_payout() {
    let test_env = TestEnv::new(None);

    let (tournament_id, config) = test_env.setup_payout_tournament(2, 8);

    // Simulate tournament
    let players = test_env.simulate_tournament_until_completion(tournament_id, 3);
    let tournament = test_env.get_tournament(tournament_id).unwrap();
    assert_eq!(tournament.state, TournamentState::Completed);

    // Verify payouts
    let total_prize_pool = config.buy_in * 5;
    let (total_prize_pool, _rake) = calculate_rake(total_prize_pool).unwrap();
    let expected_payouts = vec![(0, total_prize_pool)];
    test_env.verify_payouts(tournament_id, &players, &expected_payouts);
}

#[test]
fn test_multi_player_payout() {
    let test_env = TestEnv::new(None);

    let (tournament_id, config) = test_env.setup_payout_tournament(2, 8);

    // Simulate tournament
    let players = test_env.simulate_tournament_until_completion(tournament_id, 8);

    // Calculate expected payouts
    let total_prize_pool = config.buy_in * 6;
    let (total_prize_pool, _rake) = calculate_rake(total_prize_pool).unwrap();
    let expected_payouts = vec![
        (0, total_prize_pool * 50 / 100),
        (1, total_prize_pool * 30 / 100),
        (2, total_prize_pool * 20 / 100),
    ];

    test_env.verify_payouts(tournament_id, &players, &expected_payouts);
}
