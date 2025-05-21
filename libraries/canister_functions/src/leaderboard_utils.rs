pub const PERCENTAGE_PAYOUT: [u8; 5] = [45, 25, 15, 10, 5];
pub const TOTAL_ICP_PRIZE_POOL_WEEKDAY: u8 = 25;
pub const TOTAL_ICP_PRIZE_POOL_WEEKEND: u8 = 15;
pub const TOTAL_BTC_PRIZE_POOL_WEEKDAY: u64 = 150000;
pub const TOTAL_BTC_PRIZE_POOL_WEEKEND: u64 = 50000;

pub fn is_weekend() -> bool {
    let now = ic_cdk::api::time();

    // Get days since epoch
    let days_since_epoch = now / (24 * 60 * 60 * 1_000_000_000);

    // Calculate day of week (0 = Sunday, 6 = Saturday)
    let day_of_week = (days_since_epoch + 4) % 7;

    // Check if it's Saturday (6) or Sunday (0)
    // OR if it's Monday (1) at exactly midnight (time_of_day == 0)
    day_of_week == 0 || // Sunday
    day_of_week == 6 || // Saturday
    day_of_week == 1 // Monday at exactly midnight
}

pub fn calculate_amount_to_transfer(percentage: u8) -> u64 {
    let total_icp = if is_weekend() {
        TOTAL_ICP_PRIZE_POOL_WEEKEND as f64
    } else {
        TOTAL_ICP_PRIZE_POOL_WEEKDAY as f64
    } * 1e8;
    ((total_icp * (percentage as f64)) / 100.0) as u64
}

pub fn calculate_amount_to_transfer_pure_poker(percentage: u8) -> u64 {
    let total_btc = if is_weekend() {
        TOTAL_BTC_PRIZE_POOL_WEEKEND
    } else {
        TOTAL_BTC_PRIZE_POOL_WEEKDAY
    };
    ((total_btc * (percentage as u64)) / 100) as u64
}
