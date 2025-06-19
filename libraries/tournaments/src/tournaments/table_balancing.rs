use std::collections::{HashMap, HashSet};

use candid::CandidType;
use errors::tournament_error::TournamentError;
use serde::{Deserialize, Serialize};
use table::poker::game::table_functions::table::TableId;

use super::{
    blind_level::SpeedType,
    types::{TableInfo, TournamentData},
};

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
use ic_cdk::api::time as get_time;

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
fn get_time() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("System time before UNIX epoch")
        .as_nanos() as u64
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct TableBalancer {
    pub min_players_per_table: u8,
    pub max_players_per_table: u8,
    pub balance_interval_ns: u64,
}

impl TableBalancer {
    pub fn new(min_players: u8, max_players: u8, speed_type: &SpeedType) -> Self {
        Self {
            min_players_per_table: min_players,
            max_players_per_table: max_players,
            balance_interval_ns: get_balance_interval(speed_type),
        }
    }

    pub fn get_balance_moves(
        &self,
        tables: &mut HashMap<TableId, TableInfo>,
    ) -> Vec<(TableId, TableId)> {
        let mut table_sizes: Vec<(TableId, usize)> = tables
            .iter()
            .map(|(id, table)| (*id, table.players.len()))
            .collect();

        table_sizes.sort_by_key(|(_, count)| *count);

        // Skip recently balanced tables
        let current_time = get_time();
        let recently_balanced: HashSet<TableId> = tables
            .iter()
            .filter_map(|(id, table)| {
                if let Some(last_time) = table.last_balance_time {
                    if current_time - last_time < self.balance_interval_ns {
                        return Some(*id);
                    }
                }
                None
            })
            .collect();

        let mut affected_tables: HashSet<TableId> = HashSet::new();
        let mut moves = Vec::new();

        // Get non-empty tables
        let mut non_empty_tables: Vec<(TableId, usize)> = table_sizes
            .iter()
            .filter(|(id, count)| *count > 0 && !recently_balanced.contains(id))
            .map(|(id, count)| (*id, *count))
            .collect();

        let player_count = non_empty_tables
            .iter()
            .map(|(_, count)| *count)
            .sum::<usize>();

        // remove empty tables from table sizes
        let all_tables: Vec<(TableId, usize)> = table_sizes
            .iter()
            .filter(|(_, count)| *count > 0)
            .map(|(id, count)| (*id, *count))
            .collect();

        // PRIORITY 0: Create final table
        if moves.is_empty()
            && non_empty_tables.len() >= 2
            && player_count <= self.max_players_per_table as usize
        {
            return self.create_final_table(&all_tables, &mut affected_tables);
        }

        // // Skip balancing if the difference between min and max tables is 1 or less
        if non_empty_tables.len() > 1 && !self.should_consolidate_tables(&non_empty_tables) {
            let min_count = non_empty_tables
                .first()
                .map(|(_, count)| *count)
                .unwrap_or(0);
            let max_count = non_empty_tables
                .last()
                .map(|(_, count)| *count)
                .unwrap_or(0);

            if max_count - min_count <= 1 && !self.should_consolidate_tables(&non_empty_tables) {
                return moves;
            }
        }

        // PRIORITY 1: Consider consolidating smaller tables
        if moves.is_empty()
            && table_sizes.len() >= 2
            && self.should_consolidate_tables(&non_empty_tables)
        {
            println!("Consolidating tables");
            moves.extend(self.consolidate_small_tables(
                &non_empty_tables,
                &recently_balanced,
                &mut affected_tables,
            ));
        }

        // PRIORITY 2: Balance tables with large differences
        if moves.is_empty() && non_empty_tables.len() > 1 {
            moves.extend(
                self.balance_large_differences(&mut non_empty_tables, &mut affected_tables),
            );
        }

        // Cap at maximum 5 moves
        if moves.len() > 5 {
            moves.truncate(5);
        }

        // Update last_balance_time for affected tables
        for table_id in &affected_tables {
            if let Some(table) = tables.get_mut(table_id) {
                table.last_balance_time = Some(current_time);
            }
        }

        moves
    }

    // Priority 0: Create final table
    fn create_final_table(
        &self,
        all_tables: &[(TableId, usize)],
        affected_tables: &mut HashSet<TableId>,
    ) -> Vec<(TableId, TableId)> {
        let mut moves = Vec::new();

        if all_tables.len() < 2 {
            return moves; // Nothing to do with just one table
        }

        // Sort tables by player count (ascending)
        let mut sorted_tables = all_tables.to_vec();
        sorted_tables.sort_by_key(|(_, count)| *count);

        // First check if we need to consolidate to a final table
        let total_players: usize = sorted_tables.iter().map(|(_, count)| *count).sum();

        if total_players <= self.max_players_per_table as usize {
            // We can fit all players on a single table

            // Find the table with the most players to be our destination
            let (dest_id, _) = match sorted_tables.last() {
                Some(table) => table,
                None => return moves, // This shouldn't happen, but just in case
            };

            // Move all players from other tables to the destination table
            for (source_id, source_count) in &sorted_tables {
                if *source_id != *dest_id && *source_count > 0 {
                    // Add moves for all players in this source table
                    for _ in 0..*source_count {
                        moves.push((*source_id, *dest_id));
                        affected_tables.insert(*source_id);
                        affected_tables.insert(*dest_id);

                        // Cap at maximum 5 moves
                        if moves.len() >= 5 {
                            return moves;
                        }
                    }
                }
            }
        }

        moves
    }

    // PRIORITY 1: Consolidate small tables
    fn consolidate_small_tables(
        &self,
        non_empty_tables: &[(TableId, usize)],
        recently_balanced: &HashSet<TableId>,
        affected_tables: &mut HashSet<TableId>,
    ) -> Vec<(TableId, TableId)> {
        let mut moves = Vec::new();

        // Find tables that are small but not empty
        let small_tables: Vec<(TableId, usize)> = non_empty_tables
            .iter()
            .filter(|(id, count)| {
                *count <= self.max_players_per_table as usize && // Include tables slightly above min
                !recently_balanced.contains(id)
            })
            .map(|(id, count)| (*id, *count))
            .collect();

        if small_tables.is_empty() {
            return moves;
        }

        // Start with the smallest table
        let (smallest_id, smallest_count) = small_tables[0];

        // Find tables that can receive players
        let receivers: Vec<(TableId, usize)> = non_empty_tables
            .iter()
            .filter(|(id, count)| {
                *id != smallest_id
                    && !recently_balanced.contains(id)
                    && *count < self.max_players_per_table as usize
            })
            .map(|(id, count)| (*id, *count))
            .collect();

        let mut remaining = smallest_count;

        // Distribute to receivers, starting with emptier tables
        for (dest_id, dest_count) in &receivers {
            if remaining == 0 {
                break;
            }

            let space = self.max_players_per_table as usize - dest_count;
            let to_move = std::cmp::min(space, remaining);

            if to_move > 0 {
                // Add moves
                for _ in 0..to_move {
                    moves.push((smallest_id, *dest_id));
                    affected_tables.insert(smallest_id);
                    affected_tables.insert(*dest_id);

                    // Limit moves to 5
                    if moves.len() >= 5 {
                        return moves;
                    }
                }

                remaining -= to_move;
            }

            if moves.len() >= 5 || remaining == 0 {
                break;
            }
        }

        moves
    }

    // PRIORITY 2: Balance tables with large differences
    fn balance_large_differences(
        &self,
        non_empty_tables: &mut [(TableId, usize)],
        affected_tables: &mut HashSet<TableId>,
    ) -> Vec<(TableId, TableId)> {
        let mut moves = Vec::new();

        if non_empty_tables.len() < 2 {
            return moves;
        }
        let mut i = 0;

        loop {
            non_empty_tables.sort_by_key(|(_, count)| *count);
            // Get smallest and largest valid tables (excluding empty tables)
            let (small_id, small_count) = *non_empty_tables.first().unwrap();
            let (large_id, large_count) = *non_empty_tables.last().unwrap();

            // Only balance if difference is significant
            if large_count - small_count >= 2 {
                let diff = large_count - small_count;
                // Fix: Handle division by zero case when diff is 0
                let target_moves = if diff == 0 { 0 } else { diff / 2 };

                // Decide how many moves to make based on situation
                let to_move = if diff >= 4 && small_count <= self.min_players_per_table as usize {
                    // For large imbalances with understaffed tables, move 2-3
                    std::cmp::min(target_moves, 3)
                } else if diff >= 3 {
                    // For medium imbalances, move 2
                    std::cmp::min(target_moves, 2)
                } else {
                    // For smaller imbalances, move 1
                    1
                };

                // Add the moves
                for _ in 0..to_move {
                    moves.push((large_id, small_id));
                    affected_tables.insert(large_id);
                    affected_tables.insert(small_id);
                    if let Some((_, count)) =
                        non_empty_tables.iter_mut().find(|(id, _)| *id == large_id)
                    {
                        *count -= 1;
                    }
                    if let Some((_, count)) =
                        non_empty_tables.iter_mut().find(|(id, _)| *id == small_id)
                    {
                        *count += 1;
                    }
                }
            } else {
                // If the difference is less than 2, we can stop balancing
                break;
            }
            i += 1;
            if i == 100 || moves.len() >= 5 {
                break;
            }
        }

        moves
    }

    // Add a new method to check if consolidation is possible
    fn should_consolidate_tables(&self, non_empty_tables: &[(TableId, usize)]) -> bool {
        if non_empty_tables.len() <= 1 {
            return false;
        }

        // Calculate total players
        let total_players: usize = non_empty_tables.iter().map(|(_, count)| *count).sum();

        // Calculate how many tables we need with current max_players
        let min_tables_needed = total_players.div_ceil(self.max_players_per_table as usize);

        // If we can fit all players in fewer tables than we currently have
        non_empty_tables.len() > min_tables_needed
    }
}

impl TournamentData {
    pub fn should_break_table(&self, table_id: TableId) -> bool {
        if let Some(table) = self.tables.get(&table_id) {
            table.players.len() <= 3
        } else {
            false
        }
    }

    pub fn get_table_counts(&self) -> Vec<(TableId, usize)> {
        self.tables
            .iter()
            .map(|(id, table)| (*id, table.players.len()))
            .collect()
    }

    pub fn needs_balancing(&self) -> bool {
        let counts: Vec<_> = self.get_table_counts();
        if counts.len() <= 1 {
            return false;
        }

        let min_count = counts.iter().map(|(_, c)| c).min().unwrap_or(&0);
        let max_count = counts.iter().map(|(_, c)| c).max().unwrap_or(&0);

        max_count - min_count > 2
    }

    pub fn can_move_from_table(&self, table_id: TableId) -> bool {
        if let Some(table) = self.tables.get(&table_id) {
            if let Some(last_balance) = table.last_balance_time {
                // Don't move from tables that were balanced in last 3 hands
                get_time() > last_balance + (3 * 60_000_000_000) // 3 minutes minimum between moves
            } else {
                true
            }
        } else {
            false
        }
    }

    pub fn record_table_move(&mut self, table_id: TableId) -> Result<(), TournamentError> {
        if let Some(table) = self.tables.get_mut(&table_id) {
            table.last_balance_time = Some(get_time());
            Ok(())
        } else {
            Err(TournamentError::TableNotFound)
        }
    }
}

pub fn calculate_players_per_table(tournament: &TournamentData) -> Vec<usize> {
    let total_players = tournament.current_players.len();
    let max_seats_per_table = tournament.table_config.seats as usize;

    // Calculate how many tables we need
    // This ensures we don't create more tables than necessary
    let mut table_count = total_players / max_seats_per_table;
    if total_players % max_seats_per_table > 0 {
        table_count += 1;
    }

    // Calculate base players per table and how many tables get an extra player
    let base_players_per_table = total_players / table_count;
    let tables_with_extra_player = total_players % table_count;

    // Create a vector to track how many players should be at each table
    let mut players_per_table = vec![base_players_per_table; table_count];
    for i in players_per_table.iter_mut().take(tables_with_extra_player) {
        *i += 1;
    }
    players_per_table
}

pub fn get_balance_interval(speed_type: &SpeedType) -> u64 {
    match speed_type {
        SpeedType::HyperTurbo(_) => 120_000_000_000, // 2 minutes
        SpeedType::Turbo(_) => 180_000_000_000,      // 3 minutes
        SpeedType::Regular(_) => 300_000_000_000,    // 5 minutes (current default)
        SpeedType::Custom(params) => {
            // For custom tournaments, scale based on level duration
            // but keep within reasonable bounds
            let level_duration = params.level_duration_ns;
            (level_duration / 5).clamp(120_000_000_000, 300_000_000_000) // 2-5 minutes
        }
    }
}
