use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub enum ReturnResult {
    DepositSuccessful,
    DepositQueued,
}
