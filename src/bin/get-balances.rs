use luno::LunoClient;

fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client.get_balances() {
        Err(e) => eprintln!("{:?}", e),
        Ok(result) => {
            for balance in result.balance.into_iter() {
                println!("{:?}", balance);
            }
        }
    }
}
