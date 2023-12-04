use cxc::exchanges::{
    binance::{
        binance::{Binance, KlineParams, OrderBookParams, TradeParams},
        channel::Channel,
        interval::Interval,
    },
    exchange::{KlineProvider, OrderbookProvider, TradeProvider},
};
use futures_util::future::join_all;

#[tokio::main]
async fn main() {
    let mut binance = Binance::new();
    let mut tasks = vec![];

    // tasks.push(
    //     binance
    //         .watch_orderbook(
    //             OrderBookParams {
    //                 symbol: "BTCUSDT".to_string(),
    //                 depth: 1,
    //                 channel: Channel::Spot,
    //             },
    //             |orderbook| {
    //                 println!("{:?}", orderbook);
    //             },
    //         )
    //         .await
    //         .expect("Failed to watch orderbook"),
    // );

    // tasks.push(
    //     binance
    //         .watch_trade(
    //             TradeParams {
    //                 symbol: "BTCUSDT".to_string(),
    //                 channel: Channel::Spot,
    //             },
    //             |trade| {
    //                 println!("{:?}", trade);
    //             },
    //         )
    //         .await
    //         .expect("Failed to watch trade"),
    // );

    tasks.push(
        binance
            .watch_kline(
                KlineParams {
                    symbol: "BTCUSDT".to_string(),
                    interval: Interval::OneMinute,
                    channel: Channel::Spot,
                },
                |kline| {
                    println!("{:?}", kline);
                },
            )
            .await
            .expect("Failed to watch kline"),
    );

    join_all(tasks).await;
}
