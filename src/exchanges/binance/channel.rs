use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Channel {
    Spot,
    Margin,
    Savings,
    Mining,
    UsdMFutures,
    CoinMFutures,
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Spot | Self::Margin | Self::Savings | Self::Mining => {
                "wss://stream.binance.com:9443"
            }
            Self::UsdMFutures => "wss://fstream.binance.com",
            Self::CoinMFutures => "wss://dstream.binance.com",
        };
        write!(f, "{}", s)?;
        Ok(())
    }
}
