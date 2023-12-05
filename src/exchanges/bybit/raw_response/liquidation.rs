use serde::Deserialize;
use serde::Serialize;

use crate::response;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Liquidation {
    pub data: Data,
    pub topic: String,
    pub ts: u128,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub price: String,
    pub side: String,
    pub size: String,
    pub symbol: String,
    pub updated_time: u128,
}

impl Liquidation {
    pub fn standardize(self, raw: String) -> response::Liquidation {
        response::Liquidation {
            symbol: self.data.symbol,
            price: self.data.price.parse::<f64>().unwrap(),
            quantity: self.data.size.parse::<f64>().unwrap(),
            side: if self.data.side == "Buy" {
                response::Side::Buy
            } else {
                response::Side::Sell
            },
            timestamp: self.ts,
            raw,
        }
    }
}
