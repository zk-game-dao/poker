use crate::poker::core::{Card, Hand, Rank, Rankable};

/// Rank a hand by Vec\<Card\>
///
/// # Parameters
///
/// - `cards` - The cards to rank.
pub fn rank_hand(cards: Vec<Card>) -> Rank {
    let hand = Hand::new_with_cards(cards);
    hand.rank()
}

/// Increase the given `amount` by 1e8
///
/// # Parameters
///
/// - `amount` - The amount to increase
///
/// Used for testing purposes
pub fn convert_to_e8s(amount: f64) -> u64 {
    (amount * 1e8) as u64
}
