use candid::CandidType;
use errors::tournament_error::TournamentError;
use serde::{Deserialize, Serialize};

// Import your PayoutPercentage from types.rs
use super::types::PayoutPercentage;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct MultiplierWithProbability {
    pub multiplier: u64,
    pub probability: u64, // Represented as parts per million for precision
    pub payout_structure: Vec<PayoutPercentage>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct SpinGoMultiplierDistribution {
    pub multipliers: Vec<MultiplierWithProbability>,
    pub total_probability: u64, // Should sum to 1,000,000 (100%)
}

impl SpinGoMultiplierDistribution {
    // Standard multiplier distribution commonly used in Spin & Go tournaments
    pub fn standard() -> Self {
        let multipliers = vec![
            // Most common - 2x multiplier (winner takes all)
            MultiplierWithProbability {
                multiplier: 2,
                probability: 750000, // 75%
                payout_structure: vec![PayoutPercentage {
                    position: 1,
                    percentage: 100,
                }],
            },
            // 3x multiplier
            MultiplierWithProbability {
                multiplier: 3,
                probability: 200000, // 17% changed to 20%
                payout_structure: vec![PayoutPercentage {
                    position: 1,
                    percentage: 100,
                }],
            },
            // 5x multiplier
            MultiplierWithProbability {
                multiplier: 5,
                probability: 50000, // 5%
                payout_structure: vec![PayoutPercentage {
                    position: 1,
                    percentage: 100,
                }],
            },
            // // 10x multiplier
            // MultiplierWithProbability {
            //     multiplier: 10,
            //     probability: 20000, // 2%
            //     payout_structure: vec![PayoutPercentage {
            //         position: 1,
            //         percentage: 100,
            //     }],
            // },
            // // 25x multiplier
            // MultiplierWithProbability {
            //     multiplier: 25,
            //     probability: 9000, // 0.9%
            //     payout_structure: vec![PayoutPercentage {
            //         position: 1,
            //         percentage: 100,
            //     }],
            // },
            // // 120x multiplier (split payout)
            // MultiplierWithProbability {
            //     multiplier: 120,
            //     probability: 900, // 0.09%
            //     payout_structure: vec![
            //         PayoutPercentage {
            //             position: 1,
            //             percentage: 80,
            //         },
            //         PayoutPercentage {
            //             position: 2,
            //             percentage: 10,
            //         },
            //         PayoutPercentage {
            //             position: 3,
            //             percentage: 10,
            //         },
            //     ],
            // },
            // // 250x multiplier (split payout)
            // MultiplierWithProbability {
            //     multiplier: 250,
            //     probability: 90, // 0.009%
            //     payout_structure: vec![
            //         PayoutPercentage {
            //             position: 1,
            //             percentage: 80,
            //         },
            //         PayoutPercentage {
            //             position: 2,
            //             percentage: 10,
            //         },
            //         PayoutPercentage {
            //             position: 3,
            //             percentage: 10,
            //         },
            //     ],
            // },
            // // 1,000x multiplier (split payout)
            // MultiplierWithProbability {
            //     multiplier: 1000,
            //     probability: 10, // 0.001%
            //     payout_structure: vec![
            //         PayoutPercentage {
            //             position: 1,
            //             percentage: 80,
            //         },
            //         PayoutPercentage {
            //             position: 2,
            //             percentage: 10,
            //         },
            //         PayoutPercentage {
            //             position: 3,
            //             percentage: 10,
            //         },
            //     ],
            // },
        ];

        // Verify total probability adds up to 1,000,000 (100%)
        let total = multipliers.iter().map(|m| m.probability).sum();

        Self {
            multipliers,
            total_probability: total,
        }
    }

    // Custom distribution with user-defined multipliers and probabilities
    pub fn custom(multipliers: Vec<MultiplierWithProbability>) -> Result<Self, TournamentError> {
        let total = multipliers.iter().map(|m| m.probability).sum();

        // Ensure probabilities sum to 1,000,000
        if total != 1_000_000 {
            return Err(TournamentError::InvalidConfiguration(format!(
                "Probabilities must sum to 1,000,000, got {}",
                total
            )));
        }

        // Ensure each payout structure sums to 100%
        for multiplier in &multipliers {
            let payout_sum: u8 = multiplier
                .payout_structure
                .iter()
                .map(|p| p.percentage)
                .sum();

            if payout_sum != 100 {
                return Err(TournamentError::InvalidConfiguration(format!(
                    "Payout percentages for multiplier {} must sum to 100, got {}",
                    multiplier.multiplier, payout_sum
                )));
            }
        }

        Ok(Self {
            multipliers,
            total_probability: total,
        })
    }

    // Select a random multiplier based on the probability distribution
    pub async fn select_random_multiplier(
        &self,
    ) -> Result<MultiplierWithProbability, TournamentError> {
        let raw_bytes = ic_cdk::management_canister::raw_rand().await;
        let raw_bytes = raw_bytes
            .map_err(|e| {
                TournamentError::CanisterCallError(format!(
                    "Failed to generate random bytes: {:?}",
                    e
                ))
            })?;

        // Convert to a u64 number between 0 and 999,999
        let random_value = u64::from_be_bytes([
            raw_bytes[0],
            raw_bytes[1],
            raw_bytes[2],
            raw_bytes[3],
            raw_bytes[4],
            raw_bytes[5],
            raw_bytes[6],
            raw_bytes[7],
        ]) % 1_000_000;

        // Find the multiplier based on the random value
        let mut cumulative_prob = 0;
        for multiplier in &self.multipliers {
            cumulative_prob += multiplier.probability;
            if random_value < cumulative_prob {
                return Ok(multiplier.clone());
            }
        }

        // Fallback to the first multiplier if something goes wrong
        // (shouldn't happen if probabilities sum to 1,000,000)
        Ok(self.multipliers[0].clone())
    }
}

// For convenience, create a new type to be used in TournamentType
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct SpinGoMultiplier {
    pub multiplier: u64,
    pub payout_structure: Vec<PayoutPercentage>,
}

impl From<MultiplierWithProbability> for SpinGoMultiplier {
    fn from(value: MultiplierWithProbability) -> Self {
        Self {
            multiplier: value.multiplier,
            payout_structure: value.payout_structure,
        }
    }
}
