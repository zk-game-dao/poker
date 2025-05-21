use std::collections::HashMap;

use candid::{CandidType, Principal};
use serde::Deserialize;
use table::poker::game::table_functions::table::TableConfig;

#[derive(Debug, Clone, PartialEq, CandidType, Deserialize)]
pub struct PublicTableIndex {
    pub tables: HashMap<Principal, TableConfig>,
}

impl Default for PublicTableIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl PublicTableIndex {
    pub fn new() -> Self {
        PublicTableIndex {
            tables: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, CandidType, Deserialize)]
pub struct PrivateTableIndex {
    pub tables: HashMap<Principal, TableConfig>,
}

impl Default for PrivateTableIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl PrivateTableIndex {
    pub fn new() -> Self {
        PrivateTableIndex {
            tables: HashMap::new(),
        }
    }
}
