use serde::Deserialize;
use serde::Serialize;

use crate::response;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub topic: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub ts: u128,
    pub data: Vec<Data>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "T")]
    pub t: u128,
    pub s: String,
    #[serde(rename = "S")]
    pub s2: String,
    pub v: String,
    pub p: String,
    pub i: String,
    #[serde(rename = "BT")]
    pub bt: bool,
}

impl Trade {
    pub fn standardize(self, raw: String) -> response::Trade {
        response::Trade {
            data: self
                .data
                .into_iter()
                .map(|trade| response::TradeData {
                    symbol: trade.s,
                    id: trade.i,
                    side: if trade.s2 == "Buy" {
                        response::Side::Buy
                    } else {
                        response::Side::Sell
                    },
                    price: trade.p.parse::<f64>().unwrap(),
                    quantity: trade.v.parse::<f64>().unwrap(),
                    timestamp: trade.t,
                })
                .collect(),
            raw,
        }
    }
}
