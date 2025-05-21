use std::borrow::Cow;

use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, CandidType, Serialize, Deserialize)]
pub struct RakeStats {
    pub total_rake_collected: u64,
    pub total_rake_shared: u64,
}

impl RakeStats {
    pub fn new() -> Self {
        Self {
            total_rake_collected: 0,
            total_rake_shared: 0,
        }
    }

    pub fn add_rake(&mut self, amount: u64, is_shared: bool) {
        self.total_rake_collected += amount;
        if is_shared {
            self.total_rake_shared += amount / 2; // Assuming 50/50 split as per current implementation
        }
    }
}

const MAX_VALUE_SIZE_RAKE_STATS: u32 = 1000;

impl Storable for RakeStats {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap_or_else(|e| {
            ic_cdk::println!("RakeStats serialization error: {:?}", e);
            vec![]
        }))
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_else(|e| {
            ic_cdk::println!("RakeStats deserialization error: {:?}", e);
            RakeStats::default()
        })
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE_RAKE_STATS,
        is_fixed_size: false,
    };
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct TableRakeStats {
    pub table_id: Principal,
    pub total_rake_collected: u64,
    pub total_rake_shared: u64,
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct GlobalRakeStats {
    pub total_rake_collected: u64,
    pub total_rake_shared: u64,
    pub table_stats: Vec<TableRakeStats>,
}
