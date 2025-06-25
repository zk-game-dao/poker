use std::collections::HashMap;

use candid::{CandidType, Principal};
use errors::game_error::GameError;
use serde::{Deserialize, Serialize};
use user::user::{User, UsersCanisterId, WalletPrincipalId};

use crate::poker::{
    core::{Card, FlatDeck, Hand, Rank},
    game::table_functions::table::{BigBlind, Pot, SmallBlind, TableId},
};

use super::{
    table_functions::{
        action_log::{ActionLog, ActionType},
        ante::AnteType,
        side_pot::SidePot,
        table::{Table, TableConfig},
        types::{DealStage, Notifications, SeatStatus, UserTableData},
    },
    users::Users,
};

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum QueueItem {
    SittingIn(WalletPrincipalId, bool),
    Deposit(WalletPrincipalId, UsersCanisterId, u64),
    SittingOut(WalletPrincipalId),
    RemoveUser(WalletPrincipalId, ActionType),
    LeaveTableToMove(UsersCanisterId, WalletPrincipalId, TableId),
    UpdateBlinds(SmallBlind, BigBlind, Option<AnteType>),
    PauseTable,
    PauseTableForAddon(u64),
}

/// The TableStatus enum determines
/// if a table is joinable or not.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum TableStatus {
    /// The table is open and players can join.
    Open,
    /// The table is private or full.
    Closed,
    /// The table is paused.
    Paused,
    Reserved,
}

/// The different variants of Texas Hold'em.
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum GameType {
    /// Players can bet any amount.
    NoLimit(u64),
    /// Players can only bet a fixed amount.
    FixedLimit(u64, u64), // (small_bet, big_bet)
    // Players must bet between min_bet and max_bet
    SpreadLimit(u64, u64), // (min_bet, max_bet)
    // Players can bet up to the amount in the pot.
    PotLimit(u64),
}

/// A struct that holds a user's hand and rank,
/// used for comparing players' hands.
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct UserCards {
    pub id: WalletPrincipalId,
    pub cards: Hand,
    pub rank: Rank,
    pub amount_won: u64,
}

impl UserCards {
    pub fn new(id: WalletPrincipalId, cards: Hand, rank: Rank, amount_won: u64) -> UserCards {
        UserCards {
            id,
            cards,
            rank,
            amount_won,
        }
    }
}

/// The Table struct that's send to the frontend.
///
/// Some fields are not included in the public table,
/// like the deck and the timer.
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PublicTable {
    pub id: TableId,
    pub config: TableConfig,
    pub seats: Vec<SeatStatus>,
    pub community_cards: Vec<Card>,
    pub pot: Pot,
    pub side_pots: Vec<SidePot>,
    pub status: TableStatus,
    pub deal_stage: DealStage,
    pub big_blind: BigBlind,
    pub small_blind: SmallBlind,
    pub dealer_position: usize,
    pub current_player_index: usize,
    pub winners: Option<Vec<User>>,
    pub sorted_users: Option<Vec<UserCards>>,
    pub action_logs: Vec<ActionLog>,
    pub user_table_data: HashMap<WalletPrincipalId, UserTableData>,
    pub highest_bet: u64,
    pub last_raise: u64,
    pub round_ticker: u64,
    pub last_timer_started_timestamp: u64,
    pub users: Users,
    pub queue: Vec<QueueItem>,
}

impl PublicTable {
    pub fn is_full(&self) -> bool {
        self.seats
            .iter()
            .all(|seat: &SeatStatus| matches!(seat, SeatStatus::Occupied(_)))
    }

    pub fn is_game_ongoing(&self) -> bool {
        self.deal_stage != DealStage::Opening
            && self.deal_stage != DealStage::Fresh
            && self.sorted_users.is_none()
    }

    pub fn is_user_queued_to_leave(&self, user: WalletPrincipalId) -> bool {
        self.queue.iter().any(|item| {
            matches!(item, QueueItem::RemoveUser(principal, _) if *principal == user)
                | matches!(item, QueueItem::LeaveTableToMove(_, principal, _) if *principal == user)
        })
    }

    pub fn get_free_seat_index(&self) -> Option<usize> {
        self.seats
            .iter()
            .position(|seat| matches!(seat, SeatStatus::Empty))
    }

    pub fn get_big_blind_user_index(&self) -> Option<usize> {
        let mut index = (self.dealer_position + 1) % self.seats.len();
        let mut found_small_blind = false;

        for _ in 0..self.seats.len() {
            if let SeatStatus::Occupied(user_principal) = self.seats[index] {
                if found_small_blind && self.user_table_data.contains_key(&user_principal) {
                    return Some(index);
                }
                found_small_blind = true;
            }
            index = (index + 1) % self.seats.len();
        }
        None
    }

    pub fn get_big_blind_user_principal(&self) -> Option<WalletPrincipalId> {
        let mut index = (self.dealer_position + 1) % self.seats.len();

        let mut found_small_blind = false;
        for _ in 0..self.config.seats * 2 {
            if let SeatStatus::Occupied(user_principal) = self.seats[index] {
                if found_small_blind && self.user_table_data.contains_key(&user_principal) {
                    return Some(user_principal);
                }
                found_small_blind = true;
            }
            index = (index + 1) % self.seats.len();
        }
        None
    }
}

impl Default for PublicTable {
    fn default() -> Self {
        PublicTable {
            id: TableId(Principal::anonymous()),
            config: TableConfig::default(),
            seats: Vec::new(),
            community_cards: vec![],
            pot: Pot(0),
            side_pots: vec![],
            status: TableStatus::Open,
            deal_stage: DealStage::Opening,
            big_blind: BigBlind(0),
            small_blind: SmallBlind(0),
            dealer_position: 0,
            current_player_index: 0,
            winners: None,
            sorted_users: None,
            action_logs: vec![],
            user_table_data: HashMap::new(),
            highest_bet: 0,
            last_raise: 0,
            round_ticker: 0,
            last_timer_started_timestamp: 0,
            users: Users::new(),
            queue: Vec::new(),
        }
    }
}

impl From<Table> for PublicTable {
    fn from(table: Table) -> PublicTable {
        PublicTable {
            id: table.id,
            config: table.config,
            seats: table.seats.clone(),
            community_cards: table.community_cards.clone(),
            pot: table.pot,
            side_pots: table.side_pots.clone(),
            status: table.status,
            deal_stage: table.deal_stage,
            big_blind: table.big_blind,
            small_blind: table.small_blind,
            dealer_position: table.dealer_position,
            current_player_index: table.current_player_index,
            winners: table.winners.clone(),
            sorted_users: table.sorted_users.clone(),
            action_logs: table.action_logs.clone(),
            user_table_data: table.user_table_data.clone(),
            highest_bet: table.highest_bet,
            last_raise: table.last_raise,
            round_ticker: table.round_ticker,
            last_timer_started_timestamp: table.last_timer_started_timestamp,
            users: table.users,
            queue: table.queue,
        }
    }
}

impl From<&Table> for PublicTable {
    fn from(table: &Table) -> PublicTable {
        PublicTable {
            id: table.id,
            config: table.config.clone(),
            seats: table.seats.clone(),
            community_cards: table.community_cards.clone(),
            pot: table.pot,
            side_pots: table.side_pots.clone(),
            status: table.status,
            deal_stage: table.deal_stage,
            big_blind: table.big_blind,
            small_blind: table.small_blind,
            dealer_position: table.dealer_position,
            current_player_index: table.current_player_index,
            winners: table.winners.clone(),
            sorted_users: table.sorted_users.clone(),
            action_logs: table.action_logs.clone(),
            user_table_data: table.user_table_data.clone(),
            highest_bet: table.highest_bet,
            last_raise: table.last_raise,
            round_ticker: table.round_ticker,
            last_timer_started_timestamp: table.last_timer_started_timestamp,
            users: table.users.clone(),
            queue: table.queue.clone(),
        }
    }
}

impl From<&mut Table> for PublicTable {
    fn from(table: &mut Table) -> PublicTable {
        PublicTable {
            id: table.id,
            config: table.config.clone(),
            seats: table.seats.clone(),
            community_cards: table.community_cards.clone(),
            pot: table.pot,
            side_pots: table.side_pots.clone(),
            status: table.status,
            deal_stage: table.deal_stage,
            big_blind: table.big_blind,
            small_blind: table.small_blind,
            dealer_position: table.dealer_position,
            current_player_index: table.current_player_index,
            winners: table.winners.clone(),
            sorted_users: table.sorted_users.clone(),
            action_logs: table.action_logs.clone(),
            user_table_data: table.user_table_data.clone(),
            highest_bet: table.highest_bet,
            last_raise: table.last_raise,
            round_ticker: table.round_ticker,
            last_timer_started_timestamp: table.last_timer_started_timestamp,
            users: table.users.clone(),
            queue: table.queue.clone(),
        }
    }
}

/// The Table struct that's stored in the canister.
///
/// During a table canister upgrade, the table is stored
/// in memory and then restored after the upgrade.
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct StorableTable {
    pub id: TableId,
    pub config: TableConfig,
    pub seats: Vec<SeatStatus>,
    pub community_cards: Vec<Card>,
    pub deck: FlatDeck,
    pub pot: Pot,
    pub side_pots: Vec<SidePot>,
    pub status: TableStatus,
    pub deal_stage: DealStage,
    pub big_blind: BigBlind,
    pub small_blind: SmallBlind,
    pub big_blind_user_principal: WalletPrincipalId,
    pub small_blind_user_principal: WalletPrincipalId,
    pub dealer_position: usize,
    pub current_player_index: usize,
    pub winners: Option<Vec<User>>,
    pub sorted_users: Option<Vec<UserCards>>,
    pub action_logs: Vec<ActionLog>,
    pub user_table_data: HashMap<WalletPrincipalId, UserTableData>,
    pub highest_bet: u64,
    pub highest_bet_in_pot: u64,
    pub last_raise: u64,
    pub last_raise_principal: WalletPrincipalId,
    pub is_side_pot_active: bool,
    pub round_ticker: u64,
    pub last_timer_started_timestamp: u64,
    pub users: Users,
    pub queue: Vec<QueueItem>,
}

impl Default for StorableTable {
    fn default() -> Self {
        StorableTable {
            id: TableId(Principal::anonymous()),
            config: TableConfig::default(),
            seats: Vec::new(),
            community_cards: vec![],
            deck: FlatDeck::new(vec![1, 2, 3, 4, 5, 6, 7, 8]),
            pot: Pot(0),
            side_pots: vec![],
            status: TableStatus::Open,
            deal_stage: DealStage::Opening,
            big_blind: BigBlind(0),
            small_blind: SmallBlind(0),
            big_blind_user_principal: WalletPrincipalId(Principal::anonymous()),
            small_blind_user_principal: WalletPrincipalId(Principal::anonymous()),
            dealer_position: 0,
            current_player_index: 0,
            winners: None,
            sorted_users: None,
            action_logs: vec![],
            user_table_data: HashMap::new(),
            highest_bet: 0,
            highest_bet_in_pot: 0,
            last_raise: 0,
            last_raise_principal: WalletPrincipalId(Principal::anonymous()),
            is_side_pot_active: false,
            round_ticker: 0,
            last_timer_started_timestamp: 0,
            users: Users::new(),
            queue: Vec::new(),
        }
    }
}

impl From<StorableTable> for Table {
    fn from(storable_table: StorableTable) -> Table {
        Table {
            id: storable_table.id,
            config: storable_table.config,
            seats: storable_table.seats,
            community_cards: storable_table.community_cards,
            deck: storable_table.deck,
            pot: storable_table.pot,
            side_pots: storable_table.side_pots,
            status: storable_table.status,
            deal_stage: storable_table.deal_stage,
            big_blind: storable_table.big_blind,
            small_blind: storable_table.small_blind,
            big_blind_user_principal: storable_table.big_blind_user_principal,
            small_blind_user_principal: storable_table.small_blind_user_principal,
            dealer_position: storable_table.dealer_position,
            current_player_index: storable_table.current_player_index,
            winners: storable_table.winners,
            sorted_users: storable_table.sorted_users,
            action_logs: storable_table.action_logs,
            user_table_data: storable_table.user_table_data,
            highest_bet: storable_table.highest_bet,
            highest_bet_in_pot: storable_table.highest_bet_in_pot,
            last_raise: storable_table.last_raise,
            last_raise_principal: storable_table.last_raise_principal,
            is_side_pot_active: storable_table.is_side_pot_active,
            round_ticker: storable_table.round_ticker,
            last_timer_started_timestamp: storable_table.last_timer_started_timestamp,
            users: storable_table.users,
            timer: None,
            notifications: Notifications::new(),
            queue: storable_table.queue,
            rake_config: None,
            rake_total: None,
        }
    }
}

impl From<Table> for StorableTable {
    fn from(table: Table) -> StorableTable {
        StorableTable {
            id: table.id,
            config: table.config,
            seats: table.seats,
            community_cards: table.community_cards,
            deck: table.deck,
            pot: table.pot,
            side_pots: table.side_pots,
            status: table.status,
            deal_stage: table.deal_stage,
            big_blind: table.big_blind,
            small_blind: table.small_blind,
            big_blind_user_principal: table.big_blind_user_principal,
            small_blind_user_principal: table.small_blind_user_principal,
            dealer_position: table.dealer_position,
            current_player_index: table.current_player_index,
            winners: table.winners,
            sorted_users: table.sorted_users,
            action_logs: table.action_logs,
            user_table_data: table.user_table_data,
            highest_bet: table.highest_bet,
            highest_bet_in_pot: table.highest_bet_in_pot,
            last_raise: table.last_raise,
            last_raise_principal: table.last_raise_principal,
            is_side_pot_active: table.is_side_pot_active,
            round_ticker: table.round_ticker,
            last_timer_started_timestamp: table.last_timer_started_timestamp,
            users: table.users,
            queue: table.queue,
        }
    }
}

impl PublicTable {
    pub fn get_player_at_seat(&self, index: usize) -> Result<WalletPrincipalId, GameError> {
        match self.seats.get(index) {
            Some(SeatStatus::Occupied(principal)) => Ok(*principal),
            _ => Err(GameError::PlayerNotFound),
        }
    }
}
