use luno::{LunoClient, TradingPair};

fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client.get_ticker(TradingPair::XBTZAR) {
        Err(e) => eprintln!("{:?}", e),
        Ok(result) => {
            println!("Bid: {}, Ask: {}", result.bid, result.ask);
        }
    }
}
