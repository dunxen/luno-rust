use luno::LunoClient;

fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client.create_account("XBT", "My Account") {
        Err(e) => eprintln!("{:?}", e),
        Ok(result) => {
            println!("{:?}", result);
        }
    }
}
