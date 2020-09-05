use rust_decimal_macros::*;

use luno::{LimitOrderType, LunoClient, StopDirection, TradingPair};

#[tokio::main]
async fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client
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
        .await
    {
        Err(e) => eprintln!("{:?}", e),
        Ok(result) => {
            println!("{:?}", result);
        }
    }
}
