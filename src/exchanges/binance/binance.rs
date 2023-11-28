use crate::{
    error::AppError,
    exchanges::exchange::{
        Exchange, KlineProvider, LiquidationProvider, OrderbookProvider, TradeProvider,
    },
    response::Orderbook,
};

use garde::Validate;

use super::{channel::Channel, interval::Interval};

struct Binance;
impl Exchange for Binance {}

impl OrderbookProvider for Binance {
    type Params = OrderBookParams;
    async fn watch_orderbook(
        &self,
        params: Self::Params,
        callback: impl FnMut(Result<crate::error::AppError, Orderbook>) + Send + 'static,
    ) -> Result<(), AppError> {
        params.validate(&())?;
        // TODO params.depth の補正
        todo!()
    }
}

impl TradeProvider for Binance {
    type Params = TradeParams;
    async fn watch_trade(
        &self,
        params: Self::Params,
        callback: impl FnMut(Result<crate::error::AppError, crate::response::Trade>) + Send + 'static,
    ) {
        todo!()
    }
}

impl KlineProvider for Binance {
    type Params = KlineParams;
    async fn watch_kline(
        &self,
        params: Self::Params,
        callback: impl FnMut(Result<crate::error::AppError, crate::response::Kline>) + Send + 'static,
    ) {
        todo!()
    }
}

impl LiquidationProvider for Binance {
    type Params = LiquidationParams;
    async fn watch_liquidation(
        &self,
        params: Self::Params,
        callback: impl FnMut(Result<crate::error::AppError, crate::response::Liquidation>)
            + Send
            + 'static,
    ) {
        todo!()
    }
}

#[derive(Debug, Validate)]
struct OrderBookParams {
    #[garde(skip)]
    symbol: String,
    #[garde(range(min = 1, max = 20))]
    depth: u32,
    #[garde(skip)]
    channel: Channel,
}

#[derive(Debug)]
struct TradeParams {
    symbol: String,
}

#[derive(Debug)]
struct KlineParams {
    channel: Channel,
    symbol: String,
    interval: Interval,
}

#[derive(Debug)]
struct LiquidationParams {
    channel: Channel,
    symbol: String,
}
