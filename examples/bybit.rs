use cxc::exchanges::bybit::bybit::{
    Bybit, KlineParams, LiquidationParams, OrderBookParams, TradeParams,
};
use cxc::exchanges::bybit::channel::Channel;
use cxc::exchanges::bybit::interval::Interval;
use cxc::exchanges::exchange::{
    KlineProvider, LiquidationProvider, OrderbookProvider, TradeProvider,
};
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

    tasks.push(
        bybit
            .watch_trade(
                TradeParams {
                    symbol: "BTCUSDT".to_string(),
                    channel: Channel::MainnetSpot,
                },
                |trade| {
                    println!("{:?}", trade);
                },
            )
            .await
            .expect("Failed to watch trade"),
    );

    tasks.push(
        bybit
            .watch_kline(
                KlineParams {
                    symbol: "BTCUSDT".to_string(),
                    channel: Channel::MainnetSpot,
                    interval: Interval::OneMinute,
                },
                |kline| {
                    println!("{:?}", kline);
                },
            )
            .await
            .expect("Failed to watch kline"),
    );

    tasks.push(
        bybit
            .watch_liquidation(
                LiquidationParams {
                    symbol: "BTCUSDT".to_string(),
                    channel: Channel::MainnetLinear,
                },
                |liquidation| {
                    println!("{:?}", liquidation);
                },
            )
            .await
            .expect("Failed to watch liquidation"),
    );

    join_all(tasks).await;
}
