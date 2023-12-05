use serde::Deserialize;
use serde::Serialize;

use crate::response;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    pub topic: String,
    pub data: Vec<Data>,
    pub ts: u128,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub start: i64,
    pub end: i64,
    pub interval: String,
    pub open: String,
    pub close: String,
    pub high: String,
    pub low: String,
    pub volume: String,
    pub turnover: String,
    pub confirm: bool,
    pub timestamp: u128,
}

impl Kline {
    pub fn standardize(self, raw: String) -> response::Kline {
        let data = self.data[0].to_owned();
        response::Kline {
            symbol: self.topic,
            open: data.open.parse::<f64>().unwrap(),
            high: data.high.parse::<f64>().unwrap(),
            low: data.low.parse::<f64>().unwrap(),
            close: data.close.parse::<f64>().unwrap(),
            volume: data.volume.parse::<f64>().unwrap(),
            start: data.start as u128,
            end: data.end as u128,
            timestamp: self.ts,
            raw,
        }
    }
}
