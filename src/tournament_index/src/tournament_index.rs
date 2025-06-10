// Add these definitions to tournament_index.rs
use std::collections::HashMap;

use candid::{CandidType, Principal};
use currency::Currency;
use errors::tournament_index_error::TournamentIndexError;
use intercanister_call_wrappers::tournament_canister::{create_tournament_wrapper, return_all_cycles_to_tournament_index_wrapper};
use serde::{Deserialize, Serialize};
use table::poker::game::table_functions::{table::TableConfig, types::CurrencyType};
use tournaments::tournaments::{
    spin_and_go::SpinGoMultiplier,
    tournament_type::{BuyInOptions, TournamentSizeType, TournamentType},
    types::{NewTournament, NewTournamentSpeedType, PayoutPercentage, TournamentData},
};

use crate::{create_tournament_canister, CURRENCY_MANAGER, STATE};

#[derive(Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct TournamentIndex {
    pub tournaments: HashMap<Principal, TournamentData>,
    pub active_tournaments: Vec<Principal>,
    pub completed_tournaments: Vec<Principal>,

    // Add fields for Spin and Go pools
    pub spin_go_pools: HashMap<u64, Vec<(Principal, Principal)>>, // Map buy-in amount to a queue of ready tournaments
    pub spin_go_templates: HashMap<u64, SpinGoTemplate>, // Store templates for different buy-in amounts
}

#[derive(Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct SpinGoTemplate {
    pub buy_in: u64,
    pub currency: Currency,
    pub hero_picture: String,
    pub starting_chips: u64,
}

impl Default for TournamentIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl TournamentIndex {
    pub fn new() -> Self {
        // Create default Spin and Go templates
        let mut spin_go_templates = HashMap::new();

        // 1 ICP Spin and Go template
        spin_go_templates.insert(
            10u64.pow(8),
            SpinGoTemplate {
                buy_in: 10u64.pow(8),
                currency: Currency::ICP,
                hero_picture: "https://pink-accessible-partridge-21.mypinata.cloud/ipfs/bafybeibe6lxse5eee6tccsod2rpun62vjhvopi3ewhas6bp6seq3azecdy".to_string(),
                starting_chips: 5000,
            },
        );

        // 2.5 ICP Spin and Go template
        spin_go_templates.insert(
            25 * 10u64.pow(7),
            SpinGoTemplate {
                buy_in: 25 * 10u64.pow(7), // 2.5 ICP
                currency: Currency::ICP,
                hero_picture: "https://pink-accessible-partridge-21.mypinata.cloud/ipfs/bafybeibe6lxse5eee6tccsod2rpun62vjhvopi3ewhas6bp6seq3azecdy".to_string(),
                starting_chips: 5000,
            },
        );

        Self {
            tournaments: HashMap::new(),
            active_tournaments: Vec::new(),
            completed_tournaments: Vec::new(),
            spin_go_pools: HashMap::new(),
            spin_go_templates,
        }
    }

    pub fn delete_all_tournaments_older_than_a_week(&mut self) {
        ic_cdk::println!("Deleting tournaments older than a week");
        // Get the current time
        let current_time = ic_cdk::api::time();

        // Calculate the timestamp for one week ago (7 days * 24 hours * 60 minutes * 60 seconds * 1_000_000_000 nanoseconds)
        let one_week_ago = current_time.saturating_sub(7 * 24 * 60 * 60 * 1_000_000_000);

        // Collect tournaments that are older than a week and are completed
        let tournaments_to_delete: Vec<Principal> = self
            .completed_tournaments
            .iter()
            .filter_map(|id| {
                if let Some(tournament) = self.tournaments.get(id) {
                    // Check if the tournament start time is older than a week
                    if tournament.start_time < one_week_ago {
                        return Some(*id);
                    }
                }
                None
            })
            .collect();

        // Log the number of tournaments to be deleted
        ic_cdk::println!(
            "Deleting {} tournaments older than a week",
            tournaments_to_delete.len()
        );

        // Spawn a task to delete each tournament
        for tournament_id in tournaments_to_delete {
            ic_cdk::futures::spawn(async move {
                // Try to return cycles first
                match return_all_cycles_to_tournament_index_wrapper(tournament_id).await {
                    Ok(_) => {
                        ic_cdk::println!("Successfully returned cycles from tournament {}", tournament_id);
                    },
                    Err(e) => {
                        // Log error but continue with deletion
                        ic_cdk::println!("Error returning cycles from tournament {}: {:?}", tournament_id, e);
                    }
                }

                // Now try to stop and delete the canister
                match canister_functions::stop_and_delete_canister(tournament_id).await {
                    Ok(_) => {
                        ic_cdk::println!(
                            "Successfully deleted tournament canister {}",
                            tournament_id
                        );

                        // Remove from state
                        if let Ok(mut state) = crate::STATE.lock() {
                            state.tournaments.remove(&tournament_id);
                            state.active_tournaments.retain(|&id| id != tournament_id);
                            state
                                .completed_tournaments
                                .retain(|&id| id != tournament_id);
                        } else {
                            ic_cdk::println!(
                                "Failed to acquire STATE lock when removing tournament {}",
                                tournament_id
                            );
                        }
                    }
                    Err(e) => {
                        ic_cdk::println!(
                            "Error deleting tournament canister {}: {:?}",
                            tournament_id,
                            e
                        );
                    }
                }
            });
        }
    }
}

// Create a new Spin and Go tournament and add it to the pool
pub async fn create_spin_go_tournament(buy_in: u64) -> Result<Principal, TournamentIndexError> {
    let template = {
        let state = STATE.lock().map_err(|_| TournamentIndexError::LockError)?;
        state.spin_go_templates.get(&buy_in).cloned().ok_or(
            TournamentIndexError::InvalidTournamentConfig(format!(
                "No template found for buy-in: {}",
                buy_in
            )),
        )?
    };

    // Create a new Spin and Go tournament based on the template
    let tournament_canister = create_tournament_canister().await?;

    let new_tournament = NewTournament {
        name: format!("Spin & Go: {}", template.buy_in),
        description: format!(
            "Spin & Go tournament with {} {} buy-in",
            template.buy_in / 1e8 as u64,
            template.currency
        ),
        hero_picture: template.hero_picture,
        tournament_type: TournamentType::SpinAndGo(
            TournamentSizeType::SingleTable(BuyInOptions::new_freezout()),
            SpinGoMultiplier {
                multiplier: 0, // Will be determined randomly when the tournament starts
                payout_structure: vec![],
            },
        ),
        start_time: 0, // Will start when full
        buy_in: template.buy_in,
        currency: CurrencyType::Real(template.currency),
        speed_type: NewTournamentSpeedType::HyperTurbo(20),
        max_players: 3,
        min_players: 3,
        starting_chips: template.starting_chips,
        late_registration_duration_ns: 0,
        payout_structure: vec![PayoutPercentage {
            position: 1,
            percentage: 100,
        }],
        require_proof_of_humanity: false,
    };

    let table_config = TableConfig::default_spin_and_go(100, tournament_canister);

    // Create tournament info
    let (tournament, prize_pool) =
        TournamentData::new_spin_and_go(tournament_canister, new_tournament, table_config.clone())
            .await?;

    // Prepare the tournament canister
    let currency_manager = {
        CURRENCY_MANAGER
            .lock()
            .map_err(|_| TournamentIndexError::LockError)?
            .clone()
    };

    match tournament.currency {
        CurrencyType::Real(currency) => {
            let balance = currency_manager
                .get_balance(&currency, ic_cdk::api::canister_self())
                .await
                .map_err(|e| TournamentIndexError::CanisterCallFailed(format!("{:?}", e)))?;

            if balance < prize_pool as u128 {
                return Err(TournamentIndexError::InsufficientLiquidity);
            } else if prize_pool > 3 * tournament.buy_in {
                currency_manager
                    .withdraw(
                        &currency,
                        tournament_canister,
                        prize_pool - 3 * tournament.buy_in,
                    )
                    .await
                    .map_err(|e| TournamentIndexError::CanisterCallFailed(format!("{:?}", e)))?;
            }
        }
        CurrencyType::Fake => {}
    }

    // Validate tournament configuration
    tournament.validate()?;

    let tournament = create_tournament_wrapper(tournament_canister, tournament, table_config, prize_pool).await?;

    {
        let mut state = STATE.lock().map_err(|_| TournamentIndexError::LockError)?;
        state
            .tournaments
            .insert(tournament_canister, tournament.clone());
        state.active_tournaments.push(tournament_canister);
    }

    Ok(tournament_canister)
}
