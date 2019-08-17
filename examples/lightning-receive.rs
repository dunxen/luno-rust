use luno::{Currency, LunoClient};

fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client
        .lightning_receive(0.001, 1572595154)
        .with_currency(Currency::XBT)
        .with_description("hello")
        .create()
    {
        Err(e) => eprintln!("{:?}", e),
        Ok(invoice) => {
                println!("{:?}", invoice);
        }
    }
}
