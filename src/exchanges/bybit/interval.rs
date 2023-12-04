use core::fmt;

#[derive(Debug)]
pub enum Interval {
    OneMinute,
    ThreeMinute,
    FiveMinute,
    FifteenMinute,
    ThirtyMinute,
    OneHour,
    TwoHour,
    FourHour,
    SixHour,
    TwelveHour,
    OneDay,
    OneWeek,
    OneMonth,
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Interval::OneMinute => write!(f, "1"),
            Interval::ThreeMinute => write!(f, "3"),
            Interval::FiveMinute => write!(f, "5"),
            Interval::FifteenMinute => write!(f, "15"),
            Interval::ThirtyMinute => write!(f, "30"),
            Interval::OneHour => write!(f, "60"),
            Interval::TwoHour => write!(f, "120"),
            Interval::FourHour => write!(f, "240"),
            Interval::SixHour => write!(f, "360"),
            Interval::TwelveHour => write!(f, "720"),
            Interval::OneDay => write!(f, "D"),
            Interval::OneWeek => write!(f, "W"),
            Interval::OneMonth => write!(f, "M"),
        }
    }
}
