use serde::Deserialize;
use serde::Serialize;

use crate::exchanges::current_timestamp;
use crate::response;
use crate::response::PriceAndQuantity;

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
