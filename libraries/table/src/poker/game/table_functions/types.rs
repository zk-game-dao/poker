use candid::{CandidType, Principal};
use currency::Currency;
use serde::{Deserialize, Serialize};
use user::user::{User, WalletPrincipalId};

use crate::poker::core::Card;

#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq)]
pub enum SeatStatus {
    Empty,
    Reserved {
        principal: WalletPrincipalId,
        timestamp: u64,
    },
    Occupied(WalletPrincipalId),
    QueuedForNextRound(WalletPrincipalId, Box<User>, bool),
}

/// Indicates whether the currency used on the table is real or fake.
#[derive(Debug, Clone, Serialize, CandidType, Deserialize, PartialEq, Eq, Copy)]
pub enum CurrencyType {
    Real(Currency),
    Fake,
}

/// The different stages of a deal.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum DealStage {
    /// Start a new deal.
    Fresh = 0,
    /// Place the blinds.
    Blinds = 1,
    /// Deal 2 cards to each player.
    Opening = 2,
    /// Deal 3 community cards.
    Flop = 3,
    /// Deal 1 community card.
    Turn = 4,
    /// Deal 1 community card.
    River = 5,
    /// Showdown stage.
    Showdown = 6,
}

/// The different actions a player can take.
/// The u64 corresponds to the amount of the raised bet.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, CandidType)]
pub enum PlayerAction {
    /// Player is in queue to join game.
    Joining,
    /// Sitting out of the hand.
    SittingOut,
    /// Abandons the hand.
    Folded,
    /// Keeps the hand, but doesn't bet.
    Checked,
    /// Matches the current bet.
    Called,
    /// Raises the bet
    Raised(u64),
    /// Bets the remaining balance.
    AllIn,
    /// No action has been taken.
    None,
}

/// The different types of bets a player can make.
/// The u64 corresponds to the amount of the raised bet.
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq)]
pub enum BetType {
    /// Matches the current bet.
    Called,
    /// Raises the bet
    Raised(u64),
    /// Bet is equal to the big blind.
    BigBlind,
    /// Bet is equal to the small blind.
    SmallBlind,
    /// Bet is equal to the ante.
    Ante(u64),
}

/// Data for a user at the table.
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct UserTableData {
    /// The hand of the user.
    pub cards: Vec<Card>,
    /// The action the user has taken.
    pub player_action: PlayerAction,
    /// The total bet of the stage.
    pub current_total_bet: u64,
    /// The total bet of the deal.
    pub total_bet: u64,
    /// The card requests others users have made.
    pub show_card_requests: Vec<CardRequestData>,
    /// How many turns the user has been inactive for.
    pub inactive_turns: u16,
    /// How many turns the user has been seated out for.
    pub seated_out_turns: u16,
    /// How many user experience points the user has accumulated.
    pub experience_points: u64,
    /// Whether the user has auto check fold enabled.
    pub auto_check_fold: bool,
}

/// Data for a card request.
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct CardRequestData {
    /// The principal of the user that wants to view the cards.
    pub user_principal: Principal,
    /// The amount the user is willing to pay to view the cards.
    pub amount: u64,
    /// Whether the cards will be shown.
    pub show_cards: bool,
}

impl Default for UserTableData {
    fn default() -> Self {
        Self::new()
    }
}

impl UserTableData {
    pub fn new() -> UserTableData {
        UserTableData {
            cards: Vec::new(),
            player_action: PlayerAction::None,
            current_total_bet: 0,
            total_bet: 0,
            show_card_requests: Vec::new(),
            inactive_turns: 0,
            seated_out_turns: 0,
            experience_points: 0,
            auto_check_fold: false,
        }
    }

    /// Resets the user table data to the default values.
    pub fn reset(&mut self) {
        self.cards.clear();
        self.player_action = PlayerAction::None;
        self.current_total_bet = 0;
        self.total_bet = 0;
        self.show_card_requests.clear();
        self.experience_points = 0;
        self.auto_check_fold = false;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct Notifications {
    pub id_counter: u64,
    pub notifications: Vec<Notification>,
}

impl Default for Notifications {
    fn default() -> Self {
        Self::new()
    }
}

impl Notifications {
    pub fn new() -> Notifications {
        Notifications {
            id_counter: 0,
            notifications: Vec::new(),
        }
    }

    pub fn add_notification(
        &mut self,
        user_principal: WalletPrincipalId,
        message: NotificationMessage,
    ) {
        let notification = Notification::new(self.id_counter, user_principal, message);
        self.id_counter += 1;
        self.notifications.push(notification);
    }

    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    pub fn clear_notifications_older_than(&mut self, timestamp: u64) {
        self.notifications
            .retain(|notification| notification.timestamp >= timestamp);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct Notification {
    pub id: u64,
    pub timestamp: u64,
    pub user_principal: WalletPrincipalId,
    pub message: NotificationMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum NotificationMessage {
    UserTurnStarted,
}

impl Notification {
    pub fn new(
        id: u64,
        user_principal: WalletPrincipalId,
        message: NotificationMessage,
    ) -> Notification {
        #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
        {
            Notification {
                id,
                timestamp: ic_cdk::api::time(),
                user_principal,
                message,
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            Notification {
                id,
                timestamp: 0,
                user_principal,
                message,
            }
        }
    }
}
