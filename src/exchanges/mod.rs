pub mod binance;
pub mod bybit;
pub mod exchange;

use std::time::{SystemTime, UNIX_EPOCH};

fn current_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
