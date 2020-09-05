use luno::LunoClient;

#[tokio::main]
async fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client.stop_order("BXBY6JS2BB2MFV2").await {
        Err(e) => eprintln!("{:?}", e),
        Ok(result) => {
            println!("{:?}", result);
        }
    }
}
