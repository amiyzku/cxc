use tokio::task::JoinHandle;

use crate::{
    error::AppError,
    response::{Kline, Liquidation, Orderbook, Trade},
};

pub trait Exchange:
    OrderbookProvider + TradeProvider + KlineProvider + LiquidationProvider
{
}

pub trait OrderbookProvider {
    type Params;
    async fn watch_orderbook(
        &mut self,
        params: Self::Params,
        callback: impl FnMut(Result<Orderbook, AppError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, AppError>;
}

pub trait TradeProvider {
    type Params;
    async fn watch_trade(
        &self,
        params: Self::Params,
        callback: impl FnMut(Result<Trade, AppError>) + Send + 'static,
    );
}

pub trait KlineProvider {
    type Params;
    async fn watch_kline(
        &self,
        params: Self::Params,
        callback: impl FnMut(Result<Kline, AppError>) + Send + 'static,
    );
}

pub trait LiquidationProvider {
    type Params;
    async fn watch_liquidation(
        &self,
        params: Self::Params,
        callback: impl FnMut(Result<Liquidation, AppError>) + Send + 'static,
    );
}
