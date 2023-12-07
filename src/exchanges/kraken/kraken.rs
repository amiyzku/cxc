use std::time::Duration;

use garde::Validate;
use tokio::{task::JoinHandle, time::timeout};
use tokio_tungstenite::tungstenite::Message;

use crate::{
    error::CxcError,
    exchanges::{
        exchange::{
            Exchange, KlineProvider, LiquidationProvider, OrderbookProvider, TradeProvider,
        },
        kraken::{raw_response, websocket::Websocket},
        scheduled_ping_signal,
    },
};

use super::{
    channel::Channel,
    is_correct_symbol,
    request_params::{Interval, Name, Subscribe, Subscription},
};

pub struct Kraken {}
impl Kraken {
    pub fn new() -> Self {
        Self {}
    }

    fn run_forever(
        &mut self,
        mut ws: Websocket,
        mut callback: impl FnMut(Result<String, CxcError>) + Send + 'static,
    ) -> JoinHandle<()> {
        let mut ping_signal = scheduled_ping_signal(20);
        tokio::spawn(async move {
            loop {
                if let Ok(true) = ping_signal.try_recv() {
                    if let Err(e) = ws.ping().await {
                        callback(Err(e));
                    }
                }

                match timeout(Duration::from_secs(8), ws.base.read()).await {
                    Ok(Ok(msg)) => match msg {
                        Message::Text(msg) => {
                            if msg.contains(r#""event":"pong"#)
                                || msg.contains(r#""event":"heartbeat""#)
                                || msg.contains(r#""event":"systemStatus""#)
                                || msg.contains(r#""event":"subscriptionStatus""#)
                            {
                                continue;
                            }
                            callback(Ok(msg))
                        }
                        _ => {}
                    },
                    Ok(Err(e)) => {
                        callback(Err(e));
                    }
                    Err(_) => {
                        // To maintain connection even with infrequently updated streams
                        continue;
                    }
                }
            }
        })
    }
}

impl Exchange for Kraken {}

#[derive(Validate)]
#[garde(context(()))]
#[garde(allow_unvalidated)]
pub struct OrderbookParams {
    pub channel: Channel,
    #[garde(range(min = 1, max = 1000))]
    pub depth: u32,
    #[garde(custom(is_correct_symbol))]
    pub symbol: String,
}
impl OrderbookProvider for Kraken {
    type Params = OrderbookParams;
    async fn watch_orderbook(
        &mut self,
        params: Self::Params,
        mut callback: impl FnMut(Result<crate::response::Orderbook, CxcError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, CxcError> {
        params.validate(&())?;

        let depth = match params.depth {
            1..=10 => 10,
            11..=25 => 25,
            26..=100 => 100,
            101..=500 => 500,
            501..=1000 => 500,
            _ => unreachable!("Invalid depth"),
        };

        let subscribe = Subscribe {
            event: "subscribe".to_string(),
            pair: vec![params.symbol.clone()],
            subscription: Subscription {
                name: Name::Book,
                depth: Some(depth),
                ..Default::default()
            },
        };

        let mut ws = Websocket::connect(params.channel.to_string()).await?;
        ws.subscribe(subscribe).await?;

        let handle = self.run_forever(ws, move |msg| match msg {
            Ok(msg) => {
                let json = serde_json::from_str::<raw_response::orderbook::Orderbook>(&msg);
                match json {
                    Ok(json) => {
                        let orderbook = json.standardize(params.symbol.clone(), depth, msg);
                        callback(Ok(orderbook));
                    }
                    Err(e) => {
                        callback(Err(CxcError::JsonDeserializeError(e)));
                    }
                }
            }
            Err(e) => {
                callback(Err(e));
            }
        });
        Ok(handle)
    }
}

#[derive(Validate)]
#[garde(context(()))]
#[garde(allow_unvalidated)]
pub struct TradeParams {
    pub channel: Channel,
    #[garde(custom(is_correct_symbol))]
    pub symbol: String,
}
impl TradeProvider for Kraken {
    type Params = TradeParams;
    async fn watch_trade(
        &mut self,
        params: Self::Params,
        mut callback: impl FnMut(Result<crate::response::Trade, CxcError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, CxcError> {
        params.validate(&())?;

        let subscribe = Subscribe {
            event: "subscribe".to_string(),
            pair: vec![params.symbol.clone()],
            subscription: Subscription {
                name: Name::Trade,
                ..Default::default()
            },
        };

        let mut ws = Websocket::connect(params.channel.to_string()).await?;
        ws.subscribe(subscribe).await?;

        let handle = self.run_forever(ws, move |msg| match msg {
            Ok(msg) => {
                let json = serde_json::from_str::<raw_response::trade::Trade>(&msg);
                match json {
                    Ok(json) => {
                        let orderbook = json.standardize(params.symbol.clone(), msg);
                        callback(Ok(orderbook));
                    }
                    Err(e) => {
                        callback(Err(CxcError::JsonDeserializeError(e)));
                    }
                }
            }
            Err(e) => {
                callback(Err(e));
            }
        });
        Ok(handle)
    }
}

#[derive(Validate)]
#[garde(context(()))]
#[garde(allow_unvalidated)]
pub struct KlineParams {
    pub channel: Channel,
    #[garde(custom(is_correct_symbol))]
    pub symbol: String,
    pub interval: Interval,
}
impl KlineProvider for Kraken {
    type Params = KlineParams;
    async fn watch_kline(
        &mut self,
        params: Self::Params,
        mut callback: impl FnMut(Result<crate::response::Kline, CxcError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, CxcError> {
        params.validate(&())?;

        let subscribe = Subscribe {
            event: "subscribe".to_string(),
            pair: vec![params.symbol.clone()],
            subscription: Subscription {
                name: Name::Ohlc,
                interval: Some(params.interval),
                ..Default::default()
            },
        };

        let mut ws = Websocket::connect(params.channel.to_string()).await?;
        ws.subscribe(subscribe).await?;

        let handle = self.run_forever(ws, move |msg| match msg {
            Ok(msg) => {
                let json = serde_json::from_str::<raw_response::kline::Kline>(&msg);
                match json {
                    Ok(json) => {
                        let orderbook =
                            json.standardize(params.symbol.clone(), params.interval, msg);
                        callback(Ok(orderbook));
                    }
                    Err(e) => {
                        callback(Err(CxcError::JsonDeserializeError(e)));
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
                callback(Err(e));
            }
        });
        Ok(handle)
    }
}

impl LiquidationProvider for Kraken {
    type Params = ();
    async fn watch_liquidation(
        &mut self,
        _: Self::Params,
        _: impl FnMut(Result<crate::response::Liquidation, CxcError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, CxcError> {
        unimplemented!("Kraken does not support liquidation")
    }
}
