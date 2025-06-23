use serial_test::serial;

use crate::TestEnv;

#[test]
#[serial]
fn test_cycles_return_on_tournament_completion() {
    // Create test environment with ample cycles
    let test_env = TestEnv::new(Some(10_000_000_000_000));
    // Get initial cycle balance for tournament index
    let index_cycles_before_creation = test_env
        .pocket_ic
        .cycle_balance(test_env.canister_ids.tournament_index);
    println!(
        "Tournament index cycles before creation: {}",
        index_cycles_before_creation
    );

    let (tournament_id, _) = test_env.setup_payout_tournament(2, 8);
    // Get initial cycle balance for tournament index
    let index_cycles_before = test_env
        .pocket_ic
        .cycle_balance(test_env.canister_ids.tournament_index);

    println!("Tournament index cycles before: {}", index_cycles_before);
    assert!(
        index_cycles_before_creation > index_cycles_before,
        "Tournament index should have spent cycles to create tournament"
    );

    // Get cycle balance of tournament canister
    let tournament_cycles = test_env.pocket_ic.cycle_balance(tournament_id.0);
    println!("Tournament canister cycles: {}", tournament_cycles);
    assert!(
        tournament_cycles > 0,
        "Tournament canister should have cycles"
    );

    test_env.simulate_tournament_until_completion(tournament_id, 5);

    // Check tournament index cycles balance after tournament completion
    let index_cycles_after = test_env
        .pocket_ic
        .cycle_balance(test_env.canister_ids.tournament_index);

    println!("Tournament index cycles after: {}", index_cycles_after);

    // Verify tournament index received cycles back
    assert!(
        index_cycles_after > index_cycles_before,
        "Tournament index should have received cycles back: before={}, after={}",
        index_cycles_before,
        index_cycles_after
    );

    // Get the tournament canister's remaining cycles
    let remaining_tournament_cycles = test_env.pocket_ic.cycle_balance(tournament_id.0);
    println!(
        "Remaining tournament canister cycles: {}",
        remaining_tournament_cycles
    );

    // Verify tournament canister has minimal cycles left (most were returned)
    assert!(
        remaining_tournament_cycles < tournament_cycles,
        "Tournament canister should have returned most of its cycles"
    );

    // Final check of tournament index cycles
    let index_cycles_final = test_env
        .pocket_ic
        .cycle_balance(test_env.canister_ids.tournament_index);

    println!("Tournament index cycles final: {}", index_cycles_final);

    // Verify tournament index received even more cycles after forced return
    assert!(
        index_cycles_final >= index_cycles_after,
        "Tournament index should have received additional cycles: after={}, final={}",
        index_cycles_after,
        index_cycles_final
    );
}
