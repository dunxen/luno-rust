use luno::LunoClient;

#[tokio::main]
async fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client.get_transactions("ACCOUNT_ID", 1, 100).await {
        Err(e) => eprintln!("{:?}", e),
        Ok(result) => {
            if let Some(txn) = result.transactions {
                println!("{:?}", txn);
            }
        }
    }
}
