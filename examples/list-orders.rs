use luno::{orders::OrderState, LunoClient};

#[tokio::main]
async fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client
        .list_orders()
        .filter_state(OrderState::COMPLETE)
        .filter_created_before(1390168800000)
        .get()
        .await
    {
        Err(e) => eprintln!("{:?}", e),
        Ok(result) => {
            if let Some(orders) = result.orders {
                println!("{:?}", orders);
            }
        }
    }
}
