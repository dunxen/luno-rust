use luno::{LunoClient, TradingPair};

#[tokio::main]
async fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client
        .list_own_trades(TradingPair::ETHZAR)
        .since(1_561_939_200)
        .get()
        .await
    {
        Err(e) => eprintln!("{:?}", e),
        Ok(result) => {
            if let Some(trades) = result.trades {
                for trade in trades {
                    println!("{:?}", trade);
                }
            }
        }
    }
}
