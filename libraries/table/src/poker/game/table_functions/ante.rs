use candid::CandidType;
use serde::{Deserialize, Serialize};

use super::table::Table;

#[derive(Debug, Clone, Serialize, CandidType, Deserialize, PartialEq, Eq)]
pub enum AnteType {
    None,
    Fixed(u64),               // Fixed amount
    BigBlindAnte,             // One player pays the BB as ante
    PercentageOfBigBlind(u8), // Ante as percentage of BB
}

impl Table {
    pub fn get_ante_amount(&self) -> u64 {
        if let Some(ante_type) = &self.config.ante_type {
            match ante_type {
                AnteType::None => 0,
                AnteType::Fixed(amount) => *amount,
                AnteType::BigBlindAnte => self.big_blind,
                AnteType::PercentageOfBigBlind(percentage) => {
                    (self.big_blind * *percentage as u64) / 100
                }
            }
        } else {
            0
        }
    }

    pub fn is_big_blind_ante(&self) -> bool {
        if let Some(ante_type) = &self.config.ante_type {
            matches!(ante_type, AnteType::BigBlindAnte)
        } else {
            false
        }
    }
}
