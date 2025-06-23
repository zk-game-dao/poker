use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::tournaments::tournament_type::TournamentType;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct PayoutStructure {
    pub payouts: Vec<PayoutPercentage>,
}

impl Default for PayoutStructure {
    fn default() -> Self {
        Self { payouts: Vec::new() }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct PayoutPercentage {
    pub position: u16,
    pub percentage: u8, // 0-100
}

pub fn calculate_dynamic_payout_structure(
    total_players: u32,
    tournament_type: &TournamentType,
) -> PayoutStructure {
    // Simple percentage tables based on total players
    let payouts = match tournament_type {
        TournamentType::SpinAndGo(_, _) => {
            vec![(1, 100)]
        }
        _ => {
            get_standard_payout_percentages(total_players)
        }
    };

    PayoutStructure {
        payouts: payouts
            .into_iter()
            .map(|(pos, pct)| PayoutPercentage {
                position: pos,
                percentage: pct,
            })
            .collect(),
    }
}

fn get_standard_payout_percentages(total_players: u32) -> Vec<(u16, u8)> {
    match total_players {
        1..=3 => vec![(1, 100)],
        4..=6 => vec![(1, 65), (2, 35)],
        7..=10 => vec![(1, 50), (2, 30), (3, 20)],
        11..=18 => vec![(1, 40), (2, 25), (3, 15), (4, 10), (5, 10)],
        19..=27 => vec![(1, 35), (2, 22), (3, 15), (4, 10), (5, 8), (6, 5), (7, 5)],
        28..=50 => vec![
            (1, 30), (2, 18), (3, 12), (4, 8), (5, 6),
            (6, 5), (7, 4), (8, 4), (9, 3), (10, 3),
            (11, 2), (12, 2), (13, 2), (14, 1)
        ],
        51..=100 => vec![
            (1, 28), (2, 16), (3, 11), (4, 8), (5, 6),
            (6, 5), (7, 4), (8, 4), (9, 3), (10, 3),
            (11, 2), (12, 2), (13, 2), (14, 2), (15, 2),
            (16, 1), (17, 1)
        ],
        101..=500 => vec![
            (1, 25), (2, 15), (3, 10), (4, 7), (5, 5),
            (6, 4), (7, 3), (8, 3), (9, 3), (10, 2),
            (11, 2), (12, 2), (13, 2), (14, 2), (15, 2),
            (16, 1), (17, 1), (18, 1), (19, 1), (20, 1),
            (21, 1), (22, 1), (23, 1), (24, 1), (25, 1),
            (26, 1), (27, 1), (28, 1)
        ],
        _ => vec![
            (1, 22), (2, 13), (3, 9), (4, 6), (5, 4),
            (6, 3), (7, 3), (8, 3), (9, 2), (10, 2),
            (11, 2), (12, 2), (13, 2), (14, 2), (15, 2),
            (16, 1), (17, 1), (18, 1), (19, 1), (20, 1),
            (21, 1), (22, 1), (23, 1), (24, 1), (25, 1),
            (26, 1), (27, 1), (28, 1), (29, 1), (30, 1),
            (31, 1), (32, 1), (33, 1), (34, 1), (35, 1),
            (36, 1), (37, 1), (38, 1)
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tournaments::{
        tournament_type::{BuyInOptions, TournamentSizeType, TournamentType},
        spin_and_go::SpinGoMultiplier,
    };
    
    #[test]
    fn test_basic_structures() {
        // 2 players: 65/35
        let structure = calculate_dynamic_payout_structure(2, &TournamentType::BuyIn(TournamentSizeType::SingleTable(BuyInOptions::new_freezout())));
        assert_eq!(structure.payouts.len(), 2);
        assert_eq!(structure.payouts[0].percentage, 65);
        assert_eq!(structure.payouts[1].percentage, 35);
        
        // 3 players: 50/30/20
        let structure = calculate_dynamic_payout_structure(3, &TournamentType::BuyIn(TournamentSizeType::SingleTable(BuyInOptions::new_freezout())));
        assert_eq!(structure.payouts.len(), 3);
        assert_eq!(structure.payouts[0].percentage, 50);
        assert_eq!(structure.payouts[1].percentage, 30);
        assert_eq!(structure.payouts[2].percentage, 20);
        
        // 6 players: 65/35 (top 2)
        let structure = calculate_dynamic_payout_structure(6, &TournamentType::SitAndGo(TournamentSizeType::SingleTable(BuyInOptions::new_freezout())));
        assert_eq!(structure.payouts.len(), 2);
        assert_eq!(structure.payouts[0].percentage, 65);
        assert_eq!(structure.payouts[1].percentage, 35);
        
        // 9 players: 50/30/20 (top 3)
        let structure = calculate_dynamic_payout_structure(9, &TournamentType::SitAndGo(TournamentSizeType::SingleTable(BuyInOptions::new_freezout())));
        assert_eq!(structure.payouts.len(), 3);
        assert_eq!(structure.payouts[0].percentage, 50);
        assert_eq!(structure.payouts[1].percentage, 30);
        assert_eq!(structure.payouts[2].percentage, 20);
    }
    
    #[test]
    fn test_spin_and_go() {
        let multiplier = SpinGoMultiplier {
            multiplier: 3,
            payout_structure: vec![PayoutPercentage { position: 1, percentage: 100 }],
        };
        
        let structure = calculate_dynamic_payout_structure(
            3, 
            &TournamentType::SpinAndGo(TournamentSizeType::SingleTable(BuyInOptions::new_freezout()), multiplier)
        );
        
        assert_eq!(structure.payouts.len(), 1);
        assert_eq!(structure.payouts[0].percentage, 100);
    }
    
    #[test]
    fn test_all_sum_to_100() {
        let test_cases = vec![1, 2, 3, 6, 9, 18, 27, 50, 100, 500, 1000];
        
        for players in test_cases {
            let structure = calculate_dynamic_payout_structure(
                players, 
                &TournamentType::BuyIn(TournamentSizeType::SingleTable(BuyInOptions::new_freezout()))
            );
            
            let total: u32 = structure.payouts.iter().map(|p| p.percentage as u32).sum();
            assert_eq!(total, 100, "Failed for {} players", players);
        }
    }
    
    #[test]
    fn test_positions_sequential() {
        let structure = calculate_dynamic_payout_structure(
            100, 
            &TournamentType::BuyIn(TournamentSizeType::SingleTable(BuyInOptions::new_freezout()))
        );
        
        for (i, payout) in structure.payouts.iter().enumerate() {
            assert_eq!(payout.position, (i + 1) as u16);
        }
    }
}
