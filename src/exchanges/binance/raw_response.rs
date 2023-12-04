use serde::Deserialize;
use serde::Serialize;

use crate::exchanges::current_timestamp;
use crate::response;
use crate::response::PriceAndQuantity;
use crate::response::Side;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Orderbook {
    pub last_update_id: i64,
    pub bids: Vec<Vec<String>>,
    pub asks: Vec<Vec<String>>,
}

impl Orderbook {
    pub fn standardize(self, symbol: String, depth: u32, raw: String) -> response::Orderbook {
        response::Orderbook {
            symbol,
            bids: self
                .bids
                .iter()
                .take(depth as usize)
                .map(|bid| PriceAndQuantity {
                    price: bid[0].parse::<f64>().unwrap(),
                    quantity: bid[1].parse::<f64>().unwrap(),
                })
                .collect(),
            asks: self
                .asks
                .iter()
                .take(depth as usize)
                .map(|ask| PriceAndQuantity {
                    price: ask[0].parse::<f64>().unwrap(),
                    quantity: ask[1].parse::<f64>().unwrap(),
                })
                .collect(),
            timestamp: current_timestamp(),
            raw: Some(raw),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub e: String,
    #[serde(rename = "E")]
    pub e2: i64,
    pub s: String,
    pub a: i64,
    pub p: String,
    pub q: String,
    pub f: i64,
    pub l: i64,
    #[serde(rename = "T")]
    pub t: u128,
    pub m: bool,
    #[serde(rename = "M")]
    pub m2: Option<bool>,
}

impl Trade {
    pub fn standardize(self, raw: String) -> response::Trade {
        let trade_data = response::TradeData {
            id: self.a.to_string(),
            price: self.p.parse::<f64>().unwrap(),
            quantity: self.q.parse::<f64>().unwrap(),
            side: if self.m { Side::Sell } else { Side::Buy },
            symbol: self.s,
            timestamp: self.t,
        };
        response::Trade {
            data: vec![trade_data],
            raw: Some(raw),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    pub e: String,
    #[serde(rename = "E")]
    pub e2: i64,
    pub s: String,
    pub k: K,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct K {
    pub t: i64,
    #[serde(rename = "T")]
    pub t2: i64,
    pub s: String,
    pub i: String,
    pub f: i64,
    #[serde(rename = "L")]
    pub l: i64,
    pub o: String,
    pub c: String,
    pub h: String,
    #[serde(rename = "l")]
    pub l2: String,
    pub v: String,
    pub n: i64,
    pub x: bool,
    pub q: String,
    #[serde(rename = "V")]
    pub v2: String,
    #[serde(rename = "Q")]
    pub q2: String,
    #[serde(rename = "B")]
    pub b: String,
}

impl Kline {
    pub fn standardize(self, raw: String) -> response::Kline {
        response::Kline {
            symbol: self.s,
            volume: self.k.v.parse::<f64>().unwrap(),
            open: self.k.o.parse::<f64>().unwrap(),
            high: self.k.h.parse::<f64>().unwrap(),
            low: self.k.l2.parse::<f64>().unwrap(),
            close: self.k.c.parse::<f64>().unwrap(),
            start: self.k.t as u128,
            end: self.k.t2 as u128,
            timestamp: self.e2 as u128,
            raw: Some(raw),
        }
    }
}
