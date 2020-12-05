use rust_decimal_macros::*;

use luno::{LimitOrderType, LunoClient, StopDirection, TradingPair};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LunoClient::new("LUNO_API_KEY", "LUNO_API_SECRET");

    Ok(println!(
        "{:?}",
        client
            .limit_order(
                TradingPair::XBTZAR,
                LimitOrderType::ASK,
                dec!(0.0005),
                dec!(211_000),
            )
            .with_stop_price(dec!(210_000))
            .with_stop_direction(StopDirection::RELATIVE_LAST_TRADE)
            .post_only()
            .post()
            .await?
    ))
}
