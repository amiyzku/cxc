use serde::{Deserialize, Serialize};

use crate::{exchanges::kraken::request_params::Interval, response};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Kline {
    pub channel_id: ChannelID,
    pub data: Data,
    pub channel_name: ChannelName,
    pub pair: Pair,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub timestamp_sec: TimestampSec,
    pub etime: String,
    pub open: Price,
    pub high: Price,
    pub low: Price,
    pub close: Price,
    pub vwap: Volume,
    pub volume: Volume,
    pub count: i64,
}

impl Kline {
    pub fn standardize(self, symbol: String, interval: Interval, raw: String) -> response::Kline {
        let data = &self.data;
        let kline_end = data.etime.parse::<f64>().unwrap();
        let kline_start = kline_end - (interval.to_minute() * 60) as f64;
        return response::Kline {
            symbol,
            open: data.open.parse::<f64>().unwrap(),
            high: data.high.parse::<f64>().unwrap(),
            low: data.low.parse::<f64>().unwrap(),
            close: data.close.parse::<f64>().unwrap(),
            volume: data.volume.parse::<f64>().unwrap(),
            start: sec_to_ms(kline_start.to_string()),
            end: sec_to_ms(data.etime.to_string()),
            timestamp: sec_to_ms(data.timestamp_sec.to_string()),
            raw,
        };
    }
}
