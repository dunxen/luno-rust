use luno::{LimitOrderType, LunoClient, TradingPair};

fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client
        .limit_order(TradingPair::XBTZAR, LimitOrderType::ASK, 0.0, 0.0)
        .post_only()
        .post()
    {
        Err(e) => eprintln!("{:?}", e),
        Ok(result) => {
            println!("{:?}", result);
        }
    }
}
