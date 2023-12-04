use tokio::task::JoinHandle;

use crate::{
    error::AppError,
    response::{Kline, Liquidation, Orderbook, Trade},
};

pub trait Exchange:
    OrderbookProvider + TradeProvider + KlineProvider + LiquidationProvider
{
}

#[allow(async_fn_in_trait)]
pub trait OrderbookProvider {
    type Params;
    async fn watch_orderbook(
        &mut self,
        params: Self::Params,
        callback: impl FnMut(Result<Orderbook, AppError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, AppError>;
}

#[allow(async_fn_in_trait)]
pub trait TradeProvider {
    type Params;
    async fn watch_trade(
        &mut self,
        params: Self::Params,
        callback: impl FnMut(Result<Trade, AppError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, AppError>;
}

#[allow(async_fn_in_trait)]
pub trait KlineProvider {
    type Params;
    async fn watch_kline(
        &mut self,
        params: Self::Params,
        callback: impl FnMut(Result<Kline, AppError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, AppError>;
}

#[allow(async_fn_in_trait)]
pub trait LiquidationProvider {
    type Params;
    async fn watch_liquidation(
        &mut self,
        params: Self::Params,
        callback: impl FnMut(Result<Liquidation, AppError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, AppError>;
}
