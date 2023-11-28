use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

type UnixTimeMs = u128;
type Price = f64;
type Quantity = f64;
type Symbol = String;
type Raw = Option<String>;
type Id = String;
type Volume = f64;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

impl Default for Side {
    fn default() -> Self {
        Side::Buy
    }
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::Buy => write!(f, "Buy"),
            Side::Sell => write!(f, "Sell"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Orderbook {
    pub bids: Vec<PriceAndQuantity>,
    pub asks: Vec<PriceAndQuantity>,
    pub symbol: Symbol,
    pub timestamp: UnixTimeMs,
    pub raw: Raw,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub data: Vec<TradeData>,
    pub raw: Raw,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    pub symbol: Symbol,
    pub open: Price,
    pub high: Price,
    pub low: Price,
    pub close: Price,
    pub volume: Volume,
    pub start: UnixTimeMs,
    pub end: UnixTimeMs,
    pub timestamp: UnixTimeMs,
    pub raw: Raw,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Liquidation {
    pub symbol: Symbol,
    pub price: Price,
    pub quantity: Quantity,
    pub side: Side,
    pub timestamp: UnixTimeMs,
    pub raw: Raw,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceAndQuantity {
    pub price: Price,
    pub quantity: Quantity,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeData {
    pub id: Id,
    pub symbol: Symbol,
    pub price: Price,
    pub quantity: Quantity,
    pub side: Side,
    pub timestamp: UnixTimeMs,
}
