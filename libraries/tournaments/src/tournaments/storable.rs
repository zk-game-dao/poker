use candid::{Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

use super::types::TournamentData;

// Define a maximum size for TournamentData serialization
// Adjust this value based on your expected maximum tournament size
const MAX_VALUE_SIZE_TOURNAMENT: u32 = 5_000_000; // 5MB, adjust as needed

impl Storable for TournamentData {
    /// Serializes the TournamentData struct into a byte array.
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap_or_else(|e| {
            ic_cdk::println!("TournamentData serialization error: {:?}", e);
            vec![]
        }))
    }

    /// Deserializes a TournamentData struct from a byte array.
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_else(|e| {
            ic_cdk::println!("TournamentData deserialization error: {:?}", e);
            TournamentData::default()
        })
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE_TOURNAMENT,
        is_fixed_size: false,
    };
}
