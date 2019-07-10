use luno::{MarketOrderType, LunoClient, TradingPair};

fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client
        .market_order(TradingPair::XBTZAR, MarketOrderType::BUY, 0.0)
        .post()
    {
        Err(e) => eprintln!("{:?}", e),
        Ok(result) => {
            println!("{:?}", result);
        }
    }
}
