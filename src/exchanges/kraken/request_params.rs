use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Serialize, PartialEq, Eq)]
pub struct Subscribe {
    pub event: String,
    pub pair: Vec<String>,
    pub subscription: Subscription,
}

#[derive(Serialize, PartialEq, Eq)]
pub struct Subscription {
    pub name: Name,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<Depth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<Interval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratecounter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consolidate_taker: Option<bool>,
}

impl Default for Subscription {
    fn default() -> Self {
        Subscription {
            depth: None,
            interval: None,
            name: Name::All,
            ratecounter: None,
            snapshot: None,
            token: None,
            consolidate_taker: None,
        }
    }
}

#[derive(Serialize_repr, PartialEq, Eq)]
#[repr(u32)]
pub enum Depth {
    Ten = 10,
    TwentyFive = 25,
    OneHundred = 100,
    FiveHundred = 500,
    OneThousand = 1000,
}

#[derive(Serialize, PartialEq, Eq)]
pub enum Interval {
    OneMinute = 1,
    FiveMinutes = 5,
    FifteenMinutes = 15,
    ThirtyMinutes = 30,
    OneHour = 60,
    FourHours = 240,
    OneDay = 1440,
    SevenDays = 10080,
    FifteenDays = 21600,
}

#[derive(PartialEq, Eq)]
pub enum Name {
    Book,
    Ohlc,
    OpenOrders,
    OwnTrades,
    Spread,
    Ticker,
    Trade,
    All,
}

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            Name::Book => serializer.serialize_str("book"),
            Name::Ohlc => serializer.serialize_str("ohlc"),
            Name::OpenOrders => serializer.serialize_str("openOrders"),
            Name::OwnTrades => serializer.serialize_str("ownTrades"),
            Name::Spread => serializer.serialize_str("spread"),
            Name::Ticker => serializer.serialize_str("ticker"),
            Name::Trade => serializer.serialize_str("trade"),
            Name::All => serializer.serialize_str("*"),
        }
    }
}

#[derive(Serialize, PartialEq, Eq)]
pub struct Event {
    event: String,
}

impl Event {
    pub fn ping() -> Event {
        Event {
            event: "ping".to_string(),
        }
    }

    pub fn pong() -> Event {
        Event {
            event: "pong".to_string(),
        }
    }

    pub fn heartbeat() -> Event {
        Event {
            event: "heartbeat".to_string(),
        }
    }
}
