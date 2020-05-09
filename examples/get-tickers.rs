use luno::LunoClient;

#[tokio::main]
async fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client.list_tickers().await {
        Err(e) => eprintln!("{:?}", e),
        Ok(result) => {
            if let Some(ticker) = result.tickers {
                println!("{:?}", ticker);
            }
        }
    }
}
