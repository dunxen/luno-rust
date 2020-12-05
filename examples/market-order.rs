use rust_decimal_macros::*;

use luno::{LunoClient, MarketOrderType, TradingPair};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LunoClient::new("LUNO_API_KEY", "LUNO_API_SECRET");

    Ok(println!(
        "{:?}",
        client
            .market_order(TradingPair::XBTZAR, MarketOrderType::BUY, dec!(0.0))
            .post()
            .await?
    ))
}
