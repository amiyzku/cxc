use cxc::exchanges::{
    exchange::{OrderbookProvider, TradeProvider},
    kraken::{
        channel::Channel,
        kraken::{Kraken, OrderbookParams, TradeParams},
        request_params::Depth,
    },
};

#[tokio::main]
async fn main() {
    let mut kraken = Kraken::new();
    // let handle = kraken
    //     .watch_orderbook(
    //         OrderbookParams {
    //             channel: Channel::MainNetPublic,
    //             depth: Depth::OneHundred,
    //             symbol: "BTC/USD".to_string(),
    //         },
    //         |orderbook| match orderbook {
    //             Ok(orderbook) => {
    //                 println!("{:?}", orderbook);
    //             }
    //             Err(e) => {
    //                 println!("{}", e);
    //             }
    //         },
    //     )
    //     .await
    //     .unwrap();

    let handle = kraken
        .watch_trade(
            TradeParams {
                channel: Channel::MainNetPublic,
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

    let _ = tokio::join!(handle);
}
