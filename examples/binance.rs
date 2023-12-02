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
    let result = binance
        .watch_orderbook(
            OrderBookParams {
                symbol: "BTCUSDT".to_string(),
                depth: 5,
                channel: Channel::Spot,
            },
            |orderbook| {
                println!("{:?}", orderbook);
            },
        )
        .await;
    binance.start_watch().await;
}
