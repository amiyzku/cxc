use serde::Deserialize;
use serde::Serialize;

use crate::response;

use super::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Book {
    pub price: Price,
    pub volume: Volume,
    pub timestamp_sec: TimestampSec,
    pub republish_flag: RepublishFlag,
}

impl Default for Book {
    fn default() -> Self {
        Self {
            price: "".to_string(),
            volume: "".to_string(),
            timestamp_sec: "".to_string(),
            republish_flag: "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Books {
    AskOnly {
        a: Vec<Book>,
        c: Checksum,
    },
    BidOnly {
        b: Vec<Book>,
        c: Checksum,
    },
    AskAndBid {
        a: Vec<Book>,
        b: Vec<Book>,
        c: Checksum,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Orderbook {
    pub channel_id: ChannelID,
    pub book: Books,
    pub channel_name: ChannelName,
    pub pair: Pair,
}

impl Orderbook {
    pub fn standardize(self, symbol: String, depth: u32, raw: String) -> response::Orderbook {
        let mut bids = Vec::new();
        let mut asks = Vec::new();

        let mut timestamp_ms = 0;

        match self.book {
            Books::AskOnly { a, .. } => {
                for book in a.iter().take(depth as usize) {
                    asks.push(response::PriceAndQuantity {
                        price: book.price.parse().unwrap(),
                        quantity: book.volume.parse().unwrap(),
                    });
                    timestamp_ms = sec_to_ms(&book.timestamp_sec);
                }
            }
            Books::BidOnly { b, .. } => {
                for book in b.iter().take(depth as usize) {
                    bids.push(response::PriceAndQuantity {
                        price: book.price.parse().unwrap(),
                        quantity: book.volume.parse().unwrap(),
                    });
                    timestamp_ms = sec_to_ms(&book.timestamp_sec);
                }
            }
            Books::AskAndBid { a, b, .. } => {
                for book in a.iter().take(depth as usize) {
                    asks.push(response::PriceAndQuantity {
                        price: book.price.parse().unwrap(),
                        quantity: book.volume.parse().unwrap(),
                    });
                    timestamp_ms = sec_to_ms(&book.timestamp_sec);
                }
                for book in b.iter().take(depth as usize) {
                    bids.push(response::PriceAndQuantity {
                        price: book.price.parse().unwrap(),
                        quantity: book.volume.parse().unwrap(),
                    });
                    timestamp_ms = sec_to_ms(&book.timestamp_sec);
                }
            }
        }

        response::Orderbook {
            bids,
            asks,
            symbol,
            timestamp: timestamp_ms,
            raw,
        }
    }
}
