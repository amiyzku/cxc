use serde::Deserialize;
use serde::Serialize;

use crate::response;

type ChannelID = i64;
type ChannelName = String;
type Pair = String;
type Price = String;
type Volume = String;
type TimestampSec = String;
type Checksum = String;
type RepublishFlag = String;

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
    fn sec_to_ms(sec: String) -> u128 {
        let f = sec.parse::<f64>().unwrap();
        (f * 1000.0) as u128
    }

    pub fn standardize(self, symbol: String, raw: String) -> response::Orderbook {
        let mut bids = Vec::new();
        let mut asks = Vec::new();

        let mut timestamp_ms = 0;

        match self.book {
            Books::AskOnly { a, .. } => {
                for book in a {
                    asks.push(response::PriceAndQuantity {
                        price: book.price.parse().unwrap(),
                        quantity: book.volume.parse().unwrap(),
                    });
                    timestamp_ms = Self::sec_to_ms(book.timestamp_sec);
                }
            }
            Books::BidOnly { b, .. } => {
                for book in b {
                    bids.push(response::PriceAndQuantity {
                        price: book.price.parse().unwrap(),
                        quantity: book.volume.parse().unwrap(),
                    });
                    timestamp_ms = Self::sec_to_ms(book.timestamp_sec);
                }
            }
            Books::AskAndBid { a, b, .. } => {
                for book in a {
                    asks.push(response::PriceAndQuantity {
                        price: book.price.parse().unwrap(),
                        quantity: book.volume.parse().unwrap(),
                    });
                    timestamp_ms = Self::sec_to_ms(book.timestamp_sec);
                }
                for book in b {
                    bids.push(response::PriceAndQuantity {
                        price: book.price.parse().unwrap(),
                        quantity: book.volume.parse().unwrap(),
                    });
                    timestamp_ms = Self::sec_to_ms(book.timestamp_sec);
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
