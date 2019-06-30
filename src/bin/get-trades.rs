use luno::LunoClient;

fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client.get_trades("XBTZAR") {
        Err(e) => eprintln!("{:?}", e),
        Ok(result) => {
            for trade in result.trades.into_iter() {
                println!("{:?}", trade);
            }
        }
    }
}
