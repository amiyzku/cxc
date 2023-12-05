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
        scheduled_ping_signal,
    },
    response::{Kline, Liquidation, Orderbook, Trade},
};

use super::{channel::Channel, interval::Interval, raw_response, websocket::Websocket};

pub struct Bybit {}
impl Exchange for Bybit {}

impl Bybit {
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
                            if msg.contains(r#""success":true"#) {
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

#[derive(Debug, Validate)]
pub struct OrderbookParams {
    #[garde(skip)]
    pub channel: Channel,
    #[garde(skip)]
    pub symbol: String,
    #[garde(range(min = 1, max = 500))]
    pub depth: u32,
}

impl OrderbookProvider for Bybit {
    type Params = OrderbookParams;
    async fn watch_orderbook(
        &mut self,
        params: Self::Params,
        mut callback: impl FnMut(Result<Orderbook, CxcError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, CxcError> {
        params.validate(&())?;
        let depth = match params.channel {
            Channel::MainnetInverse
            | Channel::MainnetLinear
            | Channel::TestnetLinear
            | Channel::TestnetInverse => match params.depth {
                1 => 1,
                2..=50 => 50,
                51..=200 => 200,
                201..=500 => 500,
                _ => unreachable!("depth must be in range 1..=500"),
            },
            Channel::MainnetSpot | Channel::TestnetSpot => match params.depth {
                1 => 1,
                2..=50 => 50,
                51..=200 => 100,
                _ => Err(CxcError::InvalidParamsError(
                    "Spot channel depth must be in range 1..=200".to_string(),
                ))?,
            },
            Channel::MainnetOption | Channel::TestnetOption => match params.depth {
                1..=25 => 25,
                26..=100 => 100,
                _ => Err(CxcError::InvalidParamsError(
                    "Option channel depth must be in range 1..=200".to_string(),
                ))?,
            },
        };

        let mut ws = Websocket::connect(params.channel.to_string()).await?;

        ws.subscribe(&vec![format!(
            "orderbook.{}.{}",
            depth,
            params.symbol.to_uppercase()
        )])
        .await?;

        let handle = self.run_forever(ws, move |msg| match msg {
            Ok(msg) => {
                let json = serde_json::from_str::<raw_response::orderbook::Orderbook>(&msg);
                match json {
                    Ok(json) => {
                        let orderbook = json.standardize(msg, params.depth);
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

#[derive(Debug)]
pub struct TradeParams {
    pub channel: Channel,
    pub symbol: String,
}

impl TradeProvider for Bybit {
    type Params = TradeParams;
    async fn watch_trade(
        &mut self,
        params: Self::Params,
        mut callback: impl FnMut(Result<Trade, CxcError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, CxcError> {
        let mut ws = Websocket::connect(params.channel.to_string()).await?;

        ws.subscribe(&vec![format!(
            "publicTrade.{}",
            params.symbol.to_uppercase()
        )])
        .await?;

        let handle = self.run_forever(ws, move |msg| match msg {
            Ok(msg) => {
                let json = serde_json::from_str::<raw_response::trade::Trade>(&msg);
                match json {
                    Ok(json) => {
                        let orderbook = json.standardize(msg);
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

#[derive(Debug)]
pub struct KlineParams {
    pub channel: Channel,
    pub symbol: String,
    pub interval: Interval,
}

impl KlineProvider for Bybit {
    type Params = KlineParams;
    async fn watch_kline(
        &mut self,
        params: Self::Params,
        mut callback: impl FnMut(Result<Kline, CxcError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, CxcError> {
        let mut ws = Websocket::connect(params.channel.to_string()).await?;

        ws.subscribe(&vec![format!(
            "kline.{}.{}",
            params.interval.to_string(),
            params.symbol.to_uppercase()
        )])
        .await?;

        let handle = self.run_forever(ws, move |msg| match msg {
            Ok(msg) => {
                let json = serde_json::from_str::<raw_response::kline::Kline>(&msg);
                match json {
                    Ok(json) => {
                        let orderbook = json.standardize(msg);
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

#[derive(Debug)]
pub struct LiquidationParams {
    pub channel: Channel,
    pub symbol: String,
}

impl LiquidationProvider for Bybit {
    type Params = LiquidationParams;
    async fn watch_liquidation(
        &mut self,
        params: Self::Params,
        mut callback: impl FnMut(Result<Liquidation, CxcError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, CxcError> {
        let mut ws = Websocket::connect(params.channel.to_string()).await?;

        ws.subscribe(&vec![format!(
            "liquidation.{}",
            params.symbol.to_uppercase()
        )])
        .await?;

        let handle = self.run_forever(ws, move |msg| match msg {
            Ok(msg) => {
                println!("{:?}", msg);
                let json = serde_json::from_str::<raw_response::liquidation::Liquidation>(&msg);
                match json {
                    Ok(json) => {
                        let orderbook = json.standardize(msg);
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
