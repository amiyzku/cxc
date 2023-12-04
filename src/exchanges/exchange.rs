use tokio::task::JoinHandle;

use crate::{
    error::CxcError,
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
        callback: impl FnMut(Result<Orderbook, CxcError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, CxcError>;
}

#[allow(async_fn_in_trait)]
pub trait TradeProvider {
    type Params;
    async fn watch_trade(
        &mut self,
        params: Self::Params,
        callback: impl FnMut(Result<Trade, CxcError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, CxcError>;
}

#[allow(async_fn_in_trait)]
pub trait KlineProvider {
    type Params;
    async fn watch_kline(
        &mut self,
        params: Self::Params,
        callback: impl FnMut(Result<Kline, CxcError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, CxcError>;
}

#[allow(async_fn_in_trait)]
pub trait LiquidationProvider {
    type Params;
    async fn watch_liquidation(
        &mut self,
        params: Self::Params,
        callback: impl FnMut(Result<Liquidation, CxcError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, CxcError>;
}
