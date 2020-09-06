use luno::{orders::OrderState, LunoClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LunoClient::new("LUNO_API_KEY", "LUNO_API_SECRET");

    println!(
        "{:?}",
        client
            .orders()
            .filter_state(OrderState::COMPLETE)
            .filter_created_before(1390168800000)
            .list()
            .await?
    );
    Ok(())
}
