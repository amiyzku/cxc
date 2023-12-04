use garde::Validate;

use crate::exchanges::exchange::{
    Exchange, KlineProvider, LiquidationProvider, OrderbookProvider, TradeProvider,
};

use super::{channel::Channel, interval::Interval};

pub struct Bybit {}
impl Exchange for Bybit {}

#[derive(Debug, Validate)]
pub struct OrderBookParams {
    #[garde(skip)]
    pub channel: Channel,
    #[garde(skip)]
    pub symbol: String,
    #[garde(range(min = 1, max = 500))]
    pub depth: u32,
}

impl OrderbookProvider for Bybit {
    type Params = OrderBookParams;
    async fn watch_orderbook(
        &mut self,
        params: Self::Params,
        callback: impl FnMut(Result<crate::response::Orderbook, crate::error::AppError>)
            + Send
            + 'static,
    ) -> Result<tokio::task::JoinHandle<()>, crate::error::AppError> {
        todo!()
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
        callback: impl FnMut(Result<crate::response::Trade, crate::error::AppError>) + Send + 'static,
    ) -> Result<tokio::task::JoinHandle<()>, crate::error::AppError> {
        todo!()
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
        callback: impl FnMut(Result<crate::response::Kline, crate::error::AppError>) + Send + 'static,
    ) -> Result<tokio::task::JoinHandle<()>, crate::error::AppError> {
        todo!()
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
        callback: impl FnMut(Result<crate::response::Liquidation, crate::error::AppError>)
            + Send
            + 'static,
    ) -> Result<tokio::task::JoinHandle<()>, crate::error::AppError> {
        todo!()
    }
}
