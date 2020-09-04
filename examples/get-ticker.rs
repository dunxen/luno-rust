use luno::{LunoClient, TradingPair};

#[tokio::main]
async fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client.get_ticker(TradingPair::XBTAUD).await {
        Err(e) => eprintln!("{:?}", e),
        Ok(result) => {
            println!("Bid: {}, Ask: {}", result.bid, result.ask);
        }
    }
}
