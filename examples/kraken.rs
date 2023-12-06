use cxc::exchanges::{
    exchange::{KlineProvider, OrderbookProvider, TradeProvider},
    kraken::{
        channel::Channel,
        kraken::{KlineParams, Kraken, OrderbookParams, TradeParams},
        request_params::{Depth, Interval},
    },
};
use futures_util::future::join_all;

#[tokio::main]
async fn main() {
    let mut kraken = Kraken::new();
    let mut tasks = vec![];

    tasks.push(
        kraken
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
            .expect("Failed to watch orderbook"),
    );

    tasks.push(
        kraken
            .watch_trade(
                TradeParams {
                    channel: Channel::MainNetPublic,
                    symbol: "BTC/USD".to_string(),
                },
                |trade| match trade {
                    Ok(trade) => {
                        println!("{:?}", trade);
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                },
            )
            .await
            .expect("Failed to watch trade"),
    );

    tasks.push(
        kraken
            .watch_kline(
                KlineParams {
                    channel: Channel::MainNetPublic,
                    symbol: "BTC/USD".to_string(),
                    interval: Interval::OneMinute,
                },
                |kline| match kline {
                    Ok(kline) => {
                        println!("{:?}", kline);
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                },
            )
            .await
            .expect("Failed to watch kline"),
    );

    let _ = join_all(tasks).await;
}
