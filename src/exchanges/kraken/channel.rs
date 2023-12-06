use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Channel {
    MainNetPublic,
    TestNetPublic,
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MainNetPublic => "wss://ws.kraken.com",
            Self::TestNetPublic => "wss://beta-ws.kraken.com",
        };
        write!(f, "{}", s)?;
        Ok(())
    }
}
