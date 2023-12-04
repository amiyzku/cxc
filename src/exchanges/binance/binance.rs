use std::time::Duration;

use crate::{
    error::CxcError,
    exchanges::{
        binance::{raw_response, websocket::Websocket},
        exchange::{
            Exchange, KlineProvider, LiquidationProvider, OrderbookProvider, TradeProvider,
        },
    },
    response::{Orderbook, Trade},
};

use garde::Validate;
use tokio::{task::JoinHandle, time::timeout};
use tokio_tungstenite::tungstenite::Message;

use super::{channel::Channel, interval::Interval};

pub struct Binance {}
impl Exchange for Binance {}

impl Binance {
    pub fn new() -> Self {
        Self {}
    }

    fn run_forever(
        &mut self,
        mut ws: Websocket,
        mut callback: impl FnMut(Result<String, CxcError>) + Send + 'static,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                match timeout(Duration::from_secs(60 * 4), ws.base.read()).await {
                    Ok(Ok(msg)) => match msg {
                        Message::Text(msg) => callback(Ok(msg)),
                        Message::Ping(_) => {
                            if let Err(e) = ws.pong().await {
                                callback(Err(e));
                            }
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

impl OrderbookProvider for Binance {
    type Params = OrderBookParams;
    async fn watch_orderbook(
        &mut self,
        params: Self::Params,
        mut callback: impl FnMut(Result<Orderbook, CxcError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, CxcError> {
        params.validate(&())?;
        let depth = match params.depth {
            1..=5 => 5,
            6..=10 => 10,
            11..=20 => 20,
            _ => unreachable!("garde should have caught this"),
        };

        let endpoint = format!(
            "{}/ws/{}@depth{}@100ms",
            params.channel.to_string(),
            params.symbol.to_ascii_lowercase(),
            depth
        );

        let ws = Websocket::connect(endpoint).await?;

        let handle = self.run_forever(ws, move |msg| match msg {
            Ok(msg) => {
                let json = serde_json::from_str::<raw_response::Orderbook>(&msg);
                match json {
                    Ok(json) => {
                        let orderbook = json.standardize(params.symbol.clone(), params.depth, msg);
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

impl TradeProvider for Binance {
    type Params = TradeParams;
    async fn watch_trade(
        &mut self,
        params: Self::Params,
        mut callback: impl FnMut(Result<Trade, CxcError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, CxcError> {
        let endpoint = format!(
            "{}/ws/{}@aggTrade",
            params.channel.to_string(),
            params.symbol.to_lowercase()
        );

        let ws = Websocket::connect(endpoint).await?;

        let handle = self.run_forever(ws, move |msg| match msg {
            Ok(msg) => {
                let json = serde_json::from_str::<raw_response::Trade>(&msg);
                match json {
                    Ok(json) => {
                        let trade = json.standardize(msg);
                        callback(Ok(trade));
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

impl KlineProvider for Binance {
    type Params = KlineParams;
    async fn watch_kline(
        &mut self,
        params: Self::Params,
        mut callback: impl FnMut(Result<crate::response::Kline, CxcError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, CxcError> {
        let endpoint = format!(
            "{}/ws/{}@kline_{}",
            params.channel.to_string(),
            params.symbol.to_lowercase(),
            params.interval.to_string()
        );

        let ws = Websocket::connect(endpoint).await?;

        let handle = self.run_forever(ws, move |msg| match msg {
            Ok(msg) => {
                let json = serde_json::from_str::<raw_response::Kline>(&msg);
                match json {
                    Ok(json) => {
                        let kline = json.standardize(msg);
                        callback(Ok(kline));
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

impl LiquidationProvider for Binance {
    type Params = LiquidationParams;
    async fn watch_liquidation(
        &mut self,
        params: Self::Params,
        mut callback: impl FnMut(Result<crate::response::Liquidation, CxcError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, CxcError> {
        let endpoint = format!(
            "{}/ws/{}@forceOrder",
            params.channel.to_string(),
            params.symbol.to_lowercase()
        );

        let ws = Websocket::connect(endpoint).await?;

        let handle = self.run_forever(ws, move |msg| match msg {
            Ok(msg) => {
                let json = serde_json::from_str::<raw_response::Liquidation>(&msg);
                match json {
                    Ok(json) => {
                        let liquidation = json.standardize(msg);
                        callback(Ok(liquidation));
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

#[derive(Debug, Validate)]
pub struct OrderBookParams {
    #[garde(skip)]
    pub channel: Channel,
    #[garde(skip)]
    pub symbol: String,
    #[garde(range(min = 1, max = 20))]
    pub depth: u32,
}

#[derive(Debug)]
pub struct TradeParams {
    pub channel: Channel,
    pub symbol: String,
}

#[derive(Debug)]
pub struct KlineParams {
    pub channel: Channel,
    pub symbol: String,
    pub interval: Interval,
}

#[derive(Debug)]
pub struct LiquidationParams {
    pub channel: Channel,
    pub symbol: String,
}
