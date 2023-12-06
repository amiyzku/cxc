use serde::{Deserialize, Serialize};

use crate::response;

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    pub channel_id: ChannelID,
    pub data: Vec<Data>,
    pub channel_name: ChannelName,
    pub pair: Pair,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub price: Price,
    pub volume: Volume,
    pub timestamp_sec: TimestampSec,
    pub side: String,
    pub order_type: String,
    pub misc: String,
}

impl Trade {
    pub fn standardize(self, symbol: String, raw: String) -> response::Trade {
        response::Trade {
            data: self
                .data
                .into_iter()
                .map(|trade| response::TradeData {
                    symbol: symbol.clone(),
                    id: "".to_string(),
                    side: if trade.side == "b" {
                        response::Side::Buy
                    } else {
                        response::Side::Sell
                    },
                    price: trade.price.parse::<f64>().unwrap(),
                    quantity: trade.volume.parse::<f64>().unwrap(),
                    timestamp: sec_to_ms(trade.timestamp_sec),
                })
                .collect(),
            raw,
        }
    }
}
