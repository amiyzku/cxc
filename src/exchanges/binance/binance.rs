use std::time::Duration;

use crate::{
    error::AppError,
    exchanges::{
        binance::{raw_response, websocket::Websocket},
        exchange::{
            Exchange, KlineProvider, LiquidationProvider, OrderbookProvider, TradeProvider,
        },
    },
    response::Orderbook,
};

use garde::Validate;
use tokio::{
    task::{AbortHandle, JoinHandle, JoinSet},
    time::timeout,
};
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
        mut callback: impl FnMut(Result<String, AppError>) + Send + 'static,
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
        mut callback: impl FnMut(Result<Orderbook, AppError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, AppError> {
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
                        callback(Err(AppError::JsonDeserializeError(e)));
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
        &self,
        params: Self::Params,
        callback: impl FnMut(Result<crate::response::Trade, AppError>) + Send + 'static,
    ) {
        todo!()
    }
}

impl KlineProvider for Binance {
    type Params = KlineParams;
    async fn watch_kline(
        &self,
        params: Self::Params,
        callback: impl FnMut(Result<crate::response::Kline, AppError>) + Send + 'static,
    ) {
        todo!()
    }
}

impl LiquidationProvider for Binance {
    type Params = LiquidationParams;
    async fn watch_liquidation(
        &self,
        params: Self::Params,
        callback: impl FnMut(Result<crate::response::Liquidation, AppError>) + Send + 'static,
    ) {
        todo!()
    }
}

#[derive(Debug, Validate)]
pub struct OrderBookParams {
    #[garde(skip)]
    pub symbol: String,
    #[garde(range(min = 1, max = 20))]
    pub depth: u32,
    #[garde(skip)]
    pub channel: Channel,
}

#[derive(Debug)]
pub struct TradeParams {
    symbol: String,
}

#[derive(Debug)]
pub struct KlineParams {
    channel: Channel,
    symbol: String,
    interval: Interval,
}

#[derive(Debug)]
pub struct LiquidationParams {
    channel: Channel,
    symbol: String,
}
