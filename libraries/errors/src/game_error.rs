use candid::CandidType;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, CandidType, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameError {
    #[error("invalid card value")]
    InvalidCardValue,

    #[error("card not found in deck")]
    CardNotFound,

    #[error("player not found")]
    PlayerNotFound,

    #[error("table not found")]
    TableNotFound,

    #[error("action not allowed: {reason}")]
    ActionNotAllowed { reason: String },

    #[error("game already full")]
    GameFull,

    #[error("user already exists")]
    UserAlreadyExists,

    #[error("user already in game")]
    UserAlreadyInGame,

    #[error("insufficient funds")]
    InsufficientFunds,

    #[error("no winner")]
    NoWinner,

    #[error("no cards left")]
    NoCardsLeft,

    #[error("Could not calculate rake")]
    CouldNotCalculateRake,

    #[error("error: {0}")]
    Other(String),

    #[error("Blind has insufficient funds")]
    BlindInsufficientFunds { user_id: u64 },

    #[error("Canister call failed: {0}")]
    CanisterCallFailed(String),
}
