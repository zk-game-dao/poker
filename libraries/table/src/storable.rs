use candid::{Decode, Encode};

use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

use crate::poker::game::{table_functions::table::TableConfig, types::StorableTable};

const MAX_VALUE_SIZE_TABLE: u32 = 2_000_000_000;
const MAX_VALUE_SIZE_CONFIG: u32 = 1000;

// For a type to be used in a `StableBTreeMap`, it needs to implement the `Storable`
// trait, which specifies how the type can be serialized/deserialized.
//
// In this example, we're using candid to serialize/deserialize the struct, but you
// can use anything as long as you're maintaining backward-compatibility. The
// backward-compatibility allows you to change your struct over time (e.g. adding
// new fields).
//
// The `Storable` trait is already implemented for several common types (e.g. u64),
// so you can use those directly without implementing the `Storable` trait for them.
impl Storable for StorableTable {
    /// Serializes the struct into a byte array.
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap_or_else(|e| {
            ic_cdk::println!("Serialization error: {:?}", e);
            vec![]
        }))
    }

    /// Deserializes the struct from a byte array.
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_else(|e| {
            ic_cdk::println!("Deserialization error: {:?}", e);
            StorableTable::default()
        })
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE_TABLE,
        is_fixed_size: false,
    };
}

impl Storable for TableConfig {
    /// Serializes the struct into a byte array.
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap_or_else(|e| {
            ic_cdk::println!("Serialization error: {:?}", e);
            vec![]
        }))
    }

    /// Deserializes the struct from a byte array.
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_else(|e| {
            ic_cdk::println!("Deserialization error: {:?}", e);
            TableConfig::default()
        })
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE_CONFIG,
        is_fixed_size: false,
    };
}
