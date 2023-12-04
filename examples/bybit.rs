use cxc::exchanges::bybit::bybit::{Bybit, OrderBookParams};
use cxc::exchanges::bybit::channel::Channel;
use cxc::exchanges::exchange::OrderbookProvider;
use futures_util::future::join_all;

#[tokio::main]
async fn main() {
    let mut bybit = Bybit::new();
    let mut tasks = vec![];

    tasks.push(
        bybit
            .watch_orderbook(
                OrderBookParams {
                    symbol: "BTCUSDT".to_string(),
                    depth: 2,
                    channel: Channel::MainnetSpot,
                },
                |orderbook| {
                    println!("{:?}", orderbook);
                },
            )
            .await
            .expect("Failed to watch orderbook"),
    );

    join_all(tasks).await;
}
