pub mod channel;
pub mod kraken;
mod raw_response;
pub mod request_params;
mod websocket;

fn is_correct_symbol(symbol: &str, _: &()) -> garde::Result {
    if !symbol.contains("/") {
        return Err(garde::Error::new(
            "On Kraken, symbol names need to contain a forward slash. e.g. BTC/USD",
        ));
    }
    Ok(())
}
