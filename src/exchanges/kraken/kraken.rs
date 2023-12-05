use tokio::task::JoinHandle;

use crate::{
    error::CxcError,
    exchanges::exchange::{
        Exchange, KlineProvider, LiquidationProvider, OrderbookProvider, TradeProvider,
    },
};

pub struct Kraken {}
impl Exchange for Kraken {}

pub struct OrderbookParams {}
impl OrderbookProvider for Kraken {
    type Params = OrderbookParams;
    async fn watch_orderbook(
        &mut self,
        params: Self::Params,
        callback: impl FnMut(Result<crate::response::Orderbook, CxcError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, CxcError> {
        todo!()
    }
}

pub struct TradeParams {}
impl TradeProvider for Kraken {
    type Params = TradeParams;
    async fn watch_trade(
        &mut self,
        params: Self::Params,
        callback: impl FnMut(Result<crate::response::Trade, CxcError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, CxcError> {
        todo!()
    }
}

pub struct KlineParams {}
impl KlineProvider for Kraken {
    type Params = KlineParams;
    async fn watch_kline(
        &mut self,
        params: Self::Params,
        callback: impl FnMut(Result<crate::response::Kline, CxcError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, CxcError> {
        todo!()
    }
}

pub struct LiquidationParams {}
impl LiquidationProvider for Kraken {
    type Params = LiquidationParams;
    async fn watch_liquidation(
        &mut self,
        params: Self::Params,
        callback: impl FnMut(Result<crate::response::Liquidation, CxcError>) + Send + 'static,
    ) -> Result<JoinHandle<()>, CxcError> {
        todo!()
    }
}
