use luno::{orders::OrderState, LunoClient};

fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client
        .list_orders()
        .filter_state(OrderState::COMPLETE)
        .get()
    {
        Err(e) => eprintln!("{:?}", e),
        Ok(result) => {
            if let Some(order) = result.orders {
                println!("{:?}", order);
            }
        }
    }
}
