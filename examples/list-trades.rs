use luno::{LunoClient, TradingPair};

#[tokio::main]
async fn main() {
    let client = LunoClient::new("LUNO_API_KEY", "LUNO_API_SECRET");

    match client.list_trades(TradingPair::XBTZAR).await {
        Err(e) => eprintln!("{:?}", e),
        Ok(trades) => {
            for trade in trades {
                println!("{:?}", trade);
            }
        }
    }
}
