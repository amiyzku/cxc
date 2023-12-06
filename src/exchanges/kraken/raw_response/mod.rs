pub mod orderbook;
pub mod trade;

type ChannelID = i64;
type ChannelName = String;
type Pair = String;
type Price = String;
type Volume = String;
type TimestampSec = String;
type Checksum = String;
type RepublishFlag = String;

fn sec_to_ms(sec: String) -> u128 {
    let f = sec.parse::<f64>().unwrap();
    (f * 1000.0) as u128
}
