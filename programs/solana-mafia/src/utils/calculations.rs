use crate::constants::*;

/// Calculate daily earnings for a business
pub fn calculate_daily_earnings(invested_amount: u64, daily_rate: u16) -> u64 {
    (invested_amount as u128 * daily_rate as u128 / 10_000) as u64
}

/// Calculate earnings for a specific time period
pub fn calculate_earnings_for_period(
    invested_amount: u64,
    daily_rate: u16,
    seconds: i64,
) -> u64 {
    let daily_earnings = calculate_daily_earnings(invested_amount, daily_rate);
    let seconds_earnings = daily_earnings as u128 / 86_400;
    (seconds_earnings * seconds as u128) as u64
}

/// Calculate early sell fee based on days held
pub fn calculate_early_sell_fee(invested_amount: u64, days_held: u64) -> u64 {
    let fee_percent = match days_held {
        0..=7 => 25,
        8..=14 => 20,
        15..=21 => 15,
        22..=28 => 10,
        29..=30 => 5,
        _ => 0,
    };
    
    (invested_amount * fee_percent) / 100
}

