use luno::{LunoClient, TradingPair};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LunoClient::new("LUNO_API_KEY", "LUNO_API_SECRET");

    let result = client.get_ticker(TradingPair::XBTAUD).await?;
    Ok(println!("Bid: {}, Ask: {}", result.bid, result.ask))
}
