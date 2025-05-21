use errors::tournament_error::TournamentError;

pub fn calculate_rake(amount: u64) -> Result<(u64, u64), TournamentError> {
    // let tournament = TOURNAMENT.lock().map_err(|_| TournamentError::LockError)?;
    // let tournament = tournament
    //     .as_ref()
    //     .ok_or(TournamentError::TournamentNotFound)?;
    // let rake_percentage = match &tournament.tournament_type {
    //     TournamentType::Freeroll(_) => return Ok((amount, 0)), // No rake for freerolls
    //     TournamentType::SpinAndGo(_, _) => 15,
    //     _ => 10,
    // };
    let rake_percentage = 15;

    // Calculate rake amount
    let rake_amount = (amount * rake_percentage as u64) / 100;
    let prize_pool_amount = amount - rake_amount;

    Ok((prize_pool_amount, rake_amount))
}
