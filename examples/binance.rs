use cxc::exchanges::{
    binance::{
        binance::{Binance, OrderBookParams},
        channel::Channel,
    },
    exchange::OrderbookProvider,
};

#[tokio::main]
async fn main() {
    let mut binance = Binance::new();
    let handle = binance
        .watch_orderbook(
            OrderBookParams {
                symbol: "BTCUSDT".to_string(),
                depth: 1,
                channel: Channel::Spot,
            },
            |orderbook| {
                println!("{:?}", orderbook);
            },
        )
        .await
        .expect("Failed to watch orderbook");

    let _ = tokio::join!(handle);
}
