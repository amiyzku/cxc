use serde::Deserialize;
use serde::Serialize;

use crate::response;
use crate::response::PriceAndQuantity;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Orderbook {
    pub topic: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub ts: u128,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub s: String,
    pub b: Vec<Vec<String>>,
    pub a: Vec<Vec<String>>,
    pub u: u128,
    pub seq: u128,
}

impl Orderbook {
    pub fn standardize(self, raw: String, depth: u32) -> response::Orderbook {
        response::Orderbook {
            symbol: self.data.s,
            bids: self
                .data
                .b
                .into_iter()
                .take(depth as usize)
                .map(|bid| PriceAndQuantity {
                    price: bid[0].parse::<f64>().unwrap(),
                    quantity: bid[1].parse::<f64>().unwrap(),
                })
                .collect(),
            asks: self
                .data
                .a
                .into_iter()
                .take(depth as usize)
                .map(|ask| PriceAndQuantity {
                    price: ask[0].parse::<f64>().unwrap(),
                    quantity: ask[1].parse::<f64>().unwrap(),
                })
                .collect(),
            timestamp: self.data.u,
            raw,
        }
    }
}
