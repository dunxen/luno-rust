use std::env;

use futures::StreamExt;
use telegram_bot::*;

use luno::{orders::OrderState, LunoClient, TradingPair};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let (key, secret) = (
        env::var("LUNO_API_KEY").expect("LUNO_API_KEY not set"),
        env::var("LUNO_API_SECRET").expect("LUNO_API_SECRET not set"),
    );
    let api = Api::new(token);
    let client = LunoClient::new(key, secret);

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        match update.kind {
            UpdateKind::CallbackQuery(callback_query) => {
                if let Some(message) = callback_query.message {
                    api.send(message.chat.text("You clicked a button")).await?;
                }
            }
            UpdateKind::Message(message) => {
                if let Some(username) = &message.from.username {
                    if username != "gitstash" {
                        api.send(message.text_reply(format!(
                            "Hi, {}! You are not authorised to access this bot.",
                            &message.from.first_name
                        )))
                        .await?;
                    } else {
                        if let MessageKind::Location { data } = &message.kind {
                            api.send(message.text_reply(format!(
                                "Hi, {}! You sent me a location with latitude of '{}', and longitude of '{}'",
                                &message.from.first_name, data.latitude, data.longitude
                            )))
                            .await?;
                        }
                        if let MessageKind::Text { data, .. } = &message.kind {
                            match data.as_str() {
                                "/trading_summary" => {
                                    let summary = client.get_fee_info(TradingPair::XBTZAR).await?;
                                    api.send(message.text_reply(format!(
                                        "Hi, {}! Your trading summary for the past 30 days is as follows:\n\n'{:#?}'",
                                        &message.from.first_name, summary
                                    )))
                                    .await?;
                                }
                                "/list_open_orders" => {
                                    let inline_keyboard = reply_markup!(inline_keyboard,
                                        ["button 1" callback "0,0", "button 2" callback "0,1"],
                                        ["button3" callback "1,0", "button 4" callback "1,1"]
                                    );
                                    let open_orders = client
                                        .orders()
                                        .filter_state(OrderState::PENDING)
                                        .filter_pair(TradingPair::XBTZAR)
                                        .list()
                                        .await?;
                                    if let Some(orders) = open_orders {
                                        api.send(
                                            message
                                                .chat
                                                .text(format!(
                                            "Hi, {}! You have the following open orders:\n\n'{:#?}'",
                                            &message.from.first_name, orders
                                        ))
                                                .reply_markup(inline_keyboard),
                                        )
                                        .await?;
                                    } else {
                                        api.send(message.text_reply(format!(
                                            "Hi, {}! You don't have any open orders currently.",
                                            &message.from.first_name
                                        )))
                                        .await?;
                                    }
                                }
                                _ => {
                                    api.send(message.text_reply(format!(
                                        "Hi, {}! I didn't understand your query 🤔",
                                        &message.from.first_name
                                    )))
                                    .await?;
                                }
                            }
                        }
                    }
                    println!("<{}>: {}", &message.from.first_name, message.date);
                }
            }
            _ => {}
        }
    }
    Ok(())
}
