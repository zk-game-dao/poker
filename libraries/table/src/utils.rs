use crate::poker::game::{
    table_functions::{table::Table, types::DealStage},
    types::PublicTable,
};
// TODO: Use generics.
pub fn is_game_ongoing(table: &PublicTable) -> bool {
    table.deal_stage != DealStage::Opening
        && table.deal_stage != DealStage::Fresh
        && table.sorted_users.is_none()
}

pub fn is_table_game_ongoing(table: &Table) -> bool {
    table.deal_stage != DealStage::Opening
        && table.deal_stage != DealStage::Fresh
        && table.sorted_users.is_none()
}
