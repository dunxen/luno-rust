use luno::{LunoClient, TradingPair};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LunoClient::new("LUNO_API_KEY", "LUNO_API_SECRET");

    Ok(println!(
        "{:?}",
        client.list_trades(TradingPair::XBTAUD).await?
    ))
}
