use luno::LunoClient;

#[tokio::main]
async fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client.list_pending_transactions("ACCOUNT_ID").await {
        Err(e) => eprintln!("{:?}", e),
        Ok(result) => {
            if let Some(txn) = result.pending {
                println!("{:?}", txn);
            }
        }
    }
}
