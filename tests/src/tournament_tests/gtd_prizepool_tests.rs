use std::time::Duration;

use tournaments::tournaments::{types::TournamentState, utils::calculate_rake};

use crate::TestEnv;

#[test]
fn test_winner_basic_gtd_prizepool() {
    let test_env = TestEnv::new(None);

    let (tournament_id, _config) = test_env.setup_payout_tournament(2, 8, Some(100e8 as u64));

    // Simulate tournament
    let players = test_env.simulate_tournament_until_completion(tournament_id, 3);
    let tournament = test_env.get_tournament(tournament_id).unwrap();
    for _ in 0..6 {
        test_env.pocket_ic.advance_time(Duration::from_secs(60)); // 2 seconds
        for _ in 0..6 {
            test_env.pocket_ic.tick();
        }
        test_env.pocket_ic.tick();
    }
    assert_eq!(tournament.state, TournamentState::Completed);

    // Verify payouts
    let total_prize_pool = 100e8 as u64;
    let expected_payouts = vec![(0, total_prize_pool)];
    test_env.verify_payouts(tournament_id, &players, &expected_payouts);
}

#[test]
fn test_winner_basic_gtd_prizepool_surpassed() {
    let test_env = TestEnv::new(None);

    let (tournament_id, _config) = test_env.setup_payout_tournament(2, 8, Some(20e8 as u64));

    // Simulate tournament
    let players = test_env.simulate_tournament_until_completion(tournament_id, 8);
    let tournament = test_env.get_tournament(tournament_id).unwrap();
    for _ in 0..6 {
        test_env.pocket_ic.advance_time(Duration::from_secs(60)); // 2 seconds
        for _ in 0..6 {
            test_env.pocket_ic.tick();
        }
        test_env.pocket_ic.tick();
    }
    assert_eq!(tournament.state, TournamentState::Completed);

    // Verify payouts
    let total_prize_pool = 80e8 as u64;
    let (total_prize_pool, _rake) = calculate_rake(total_prize_pool).unwrap();
    let expected_payouts = vec![
        (1, total_prize_pool * 50 / 100),
        (2, total_prize_pool * 30 / 100),
        (3, total_prize_pool * 20 / 100),
    ];
    test_env.verify_payouts(tournament_id, &players, &expected_payouts);
}
