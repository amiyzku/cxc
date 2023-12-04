use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct MaxAliveTime {
    value: i32,
}

impl MaxAliveTime {
    fn new(value: i32) -> Self {
        if (1..=600).contains(&value) {
            return Self { value };
        } else {
            panic!("Second must be between 1 and 600")
        }
    }
}

impl fmt::Display for MaxAliveTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Channel {
    MainnetSpot,
    MainnetLinear,
    MainnetInverse,
    MainnetOption,
    MainnetPrivate(Option<MaxAliveTime>),
    TestnetSpot,
    TestnetLinear,
    TestnetInverse,
    TestnetOption,
    TestnetPrivate(Option<MaxAliveTime>),
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Channel::MainnetSpot => "wss://stream.bybit.com/v5/public/spot",
            Channel::MainnetLinear => "wss://stream.bybit.com/v5/public/linear",
            Channel::MainnetInverse => "wss://stream.bybit.com/v5/public/inverse",
            Channel::MainnetOption => "wss://stream.bybit.com/v5/public/option",
            Channel::MainnetPrivate(max_alive_time) => {
                if let Some(value) = max_alive_time {
                    return write!(
                        f,
                        "wss://stream.bybit.com/v5/private?max_alive_time={}s",
                        value
                    );
                }
                "wss://stream.bybit.com/v5/private"
            }

            Channel::TestnetSpot => "wss://stream-testnet.bybit.com/v5/public/spot",
            Channel::TestnetLinear => "wss://stream-testnet.bybit.com/v5/public/linear",
            Channel::TestnetInverse => "wss://stream-testnet.bybit.com/v5/public/inverse",
            Channel::TestnetOption => "wss://stream-testnet.bybit.com/v5/public/option",
            Channel::TestnetPrivate(max_alive_time) => {
                if let Some(value) = max_alive_time {
                    return write!(
                        f,
                        "wss://stream-testnet.bybit.com/v5/private?max_alive_time={}s",
                        value
                    );
                }
                "wss://stream-testnet.bybit.com/v5/private"
            }
        };
        write!(f, "{}", s)?;
        Ok(())
    }
}
