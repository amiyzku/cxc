use cxc::exchanges::{
    exchange::OrderbookProvider,
    kraken::{
        channel::Channel,
        kraken::{Kraken, OrderbookParams},
        request_params::Depth,
    },
};

#[tokio::main]
async fn main() {
    let mut kraken = Kraken::new();
    let handle = kraken
        .watch_orderbook(
            OrderbookParams {
                channel: Channel::MainNetPublic,
                depth: Depth::OneHundred,
                symbol: "BTC/USD".to_string(),
            },
            |orderbook| match orderbook {
                Ok(orderbook) => {
                    println!("{:?}", orderbook);
                }
                Err(e) => {
                    println!("{}", e);
                }
            },
        )
        .await
        .unwrap();

    tokio::join!(handle);
}
