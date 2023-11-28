use core::fmt;

#[derive(Debug)]
pub enum Interval {
    OneSecond,
    OneMinute,
    ThreeMinutes,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    OneHour,
    TwoHours,
    FourHours,
    SixHours,
    EightHours,
    TwelveHours,
    OneDay,
    ThreeDays,
    OneWeek,
    OneMonth,
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Interval::OneSecond => write!(f, "1s"),
            Interval::OneMinute => write!(f, "1m"),
            Interval::ThreeMinutes => write!(f, "3m"),
            Interval::FiveMinutes => write!(f, "5m"),
            Interval::FifteenMinutes => write!(f, "15m"),
            Interval::ThirtyMinutes => write!(f, "30m"),
            Interval::OneHour => write!(f, "1h"),
            Interval::TwoHours => write!(f, "2h"),
            Interval::FourHours => write!(f, "4h"),
            Interval::SixHours => write!(f, "6h"),
            Interval::EightHours => write!(f, "8h"),
            Interval::TwelveHours => write!(f, "12h"),
            Interval::OneDay => write!(f, "1d"),
            Interval::ThreeDays => write!(f, "3d"),
            Interval::OneWeek => write!(f, "1w"),
            Interval::OneMonth => write!(f, "1M"),
        }
    }
}
