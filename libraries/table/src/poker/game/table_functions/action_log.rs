use candid::CandidType;
use serde::{Deserialize, Serialize};
use user::user::WalletPrincipalId;

use crate::poker::core::Card;

use super::{table::Table, types::DealStage};

/// All the different loggable actions a user can take.
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, Eq, PartialEq)]
pub enum ActionType {
    Join,
    Leave,
    Fold,
    Bet {
        amount: u64,
    },
    Call,
    Raise {
        amount: u64,
    },
    AllIn {
        amount: u64,
    },
    Check,
    Win {
        amount: u64,
    },
    PlayersHandsRankedMainPot {
        hands: Vec<(String, Vec<Card>, u64)>,
    },
    PlayersHandsRankedSidePot {
        hands: Vec<(String, Vec<Card>, u64)>,
    },
    BigBlind,
    SmallBlind,
    Kicked {
        reason: String,
    },
    Stage {
        stage: DealStage,
    },
    SidePotCreated,
}

/// A log of an action that a user has taken.
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ActionLog {
    /// The Unix timestamp of when the action was taken.
    pub timestamp: u64,
    /// The principal of the user who took the action.
    pub user_principal: Option<WalletPrincipalId>,
    /// The type of action that was taken.
    pub action_type: ActionType,
}

impl ActionLog {
    pub fn new(user_principal: Option<WalletPrincipalId>, action_type: ActionType) -> ActionLog {
        ActionLog {
            #[cfg(not(target_arch = "wasm32"))]
            timestamp: 0,
            #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
            timestamp: ic_cdk::api::time(),
            user_principal,
            action_type,
        }
    }
}

impl Table {
    /// Logs an action that a user has taken
    ///
    /// # Parameters
    ///
    /// - `user_principal` - The principal of the user who took the action.
    /// - `action_type` - The type of action that was taken.
    pub fn log_action(
        &mut self,
        user_principal: Option<WalletPrincipalId>,
        action_type: ActionType,
    ) {
        let action_log = ActionLog::new(user_principal, action_type);

        self.action_logs.push(action_log);
    }
}
