use luno::{Currency, LunoClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LunoClient::new("LUNO_API_KEY", "LUNO_API_SECRET");

    println!(
        "{:?}",
        client.create_account(Currency::XBT, "My Account").await?
    );
    Ok(())
}
