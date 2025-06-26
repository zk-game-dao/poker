pub mod clan;
pub mod table;
pub mod tournament;
pub mod transfer;
pub mod user;

pub fn get_current_time_ns() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}
