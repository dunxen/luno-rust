use luno::{LunoClient, TradingPair};

fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client.get_trades(TradingPair::XBTZAR) {
        Err(e) => eprintln!("{:?}", e),
        Ok(result) => {
            if let Some(trade) = result.trades {
                println!("{:?}", trade);
            }
        }
    }
}
