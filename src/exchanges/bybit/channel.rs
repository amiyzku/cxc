use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Channel {
    MainnetSpot,
    MainnetLinear,
    MainnetInverse,
    MainnetOption,
    TestnetSpot,
    TestnetLinear,
    TestnetInverse,
    TestnetOption,
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Channel::MainnetSpot => "wss://stream.bybit.com/v5/public/spot",
            Channel::MainnetLinear => "wss://stream.bybit.com/v5/public/linear",
            Channel::MainnetInverse => "wss://stream.bybit.com/v5/public/inverse",
            Channel::MainnetOption => "wss://stream.bybit.com/v5/public/option",

            Channel::TestnetSpot => "wss://stream-testnet.bybit.com/v5/public/spot",
            Channel::TestnetLinear => "wss://stream-testnet.bybit.com/v5/public/linear",
            Channel::TestnetInverse => "wss://stream-testnet.bybit.com/v5/public/inverse",
            Channel::TestnetOption => "wss://stream-testnet.bybit.com/v5/public/option",
        };
        write!(f, "{}", s)?;
        Ok(())
    }
}
