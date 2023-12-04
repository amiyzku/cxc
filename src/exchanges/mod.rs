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

pub fn scheduled_ping_signal(secs: u64) -> tokio::sync::mpsc::Receiver<bool> {
    let (tx, rx) = tokio::sync::mpsc::channel::<bool>(1);

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(secs)).await;
            tx.send(true).await.unwrap();
        }
    });
    rx
}
