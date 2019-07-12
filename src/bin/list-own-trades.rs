use luno::{LunoClient, TradingPair};

fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client.list_own_trades(TradingPair::XBTZAR).get() {
        Err(e) => eprintln!("{:?}", e),
        Ok(result) => {
            if let Some(trade) = result.trades {
                println!("{:?}", trade);
            }
        }
    }
}
