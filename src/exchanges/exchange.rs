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
        &self,
        params: Self::Params,
        callback: impl FnMut(Result<AppError, Orderbook>) + Send + 'static,
    ) -> Result<(), AppError>;
}

pub trait TradeProvider {
    type Params;
    async fn watch_trade(
        &self,
        params: Self::Params,
        callback: impl FnMut(Result<AppError, Trade>) + Send + 'static,
    );
}

pub trait KlineProvider {
    type Params;
    async fn watch_kline(
        &self,
        params: Self::Params,
        callback: impl FnMut(Result<AppError, Kline>) + Send + 'static,
    );
}

pub trait LiquidationProvider {
    type Params;
    async fn watch_liquidation(
        &self,
        params: Self::Params,
        callback: impl FnMut(Result<AppError, Liquidation>) + Send + 'static,
    );
}
