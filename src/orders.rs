use std::collections::HashMap;
use std::future::Future;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::client;
use crate::TradingPair;

/// Represents the type of the limit order.
#[derive(EnumString, Display)]
pub enum LimitOrderType {
    ASK,
    BID,
}

/// Represents the type of the market order.
#[derive(EnumString, Display)]
pub enum MarketOrderType {
    BUY,
    SELL,
}

/// Represents an order.
#[derive(Debug, Deserialize)]
pub struct Order {
    /// The base account ID against which this order is made.
    pub base: String,
    /// The counter account ID against which this order is made.
    pub counter: String,
    /// The UNIX timestamp of creation of the order.
    pub creation_timestamp: u64,
    /// The UNIX timestamp of expiration of this order.
    pub expiration_timestamp: u64,
    /// The UNIX timestamp of completion of this order.
    pub completed_timestamp: u64,
    /// The base fee debited after the trade principal amount.
    pub fee_base: String,
    /// The counter fee debited after the trade principal amount.
    pub fee_counter: String,
    /// The limit price of this order.
    pub limit_price: String,
    /// The limit volume of this order.
    pub limit_volume: String,
    /// The ID of the order.
    pub order_id: String,
    /// The market trading pair.
    pub pair: TradingPair,
    /// The state of the order.
    pub state: OrderState,
    /// The type of the order.
    pub r#type: String,
}

/// A builder for the `list_orders()` method.
pub struct ListOrdersBuilder<'a> {
    pub(crate) state: Option<OrderState>,
    pub(crate) pair: Option<TradingPair>,
    pub(crate) luno_client: &'a client::LunoClient,
    pub(crate) url: reqwest::Url,
}

impl<'a> ListOrdersBuilder<'a> {
    pub fn filter_state(&mut self, state: OrderState) -> &mut ListOrdersBuilder<'a> {
        self.state = Some(state);
        self
    }

    pub fn filter_pair(&mut self, pair: TradingPair) -> &mut ListOrdersBuilder<'a> {
        self.pair = Some(pair);
        self
    }

    pub fn get(&self) -> impl Future<Output = Result<OrderList, reqwest::Error>> + '_ {
        let mut url = self.url.clone();
        if let Some(state) = &self.state {
            url.query_pairs_mut()
                .append_pair("state", &state.to_string());
        }
        if let Some(pair) = &self.pair {
            url.query_pairs_mut().append_pair("pair", &pair.to_string());
        }
        self.luno_client.get(url)
    }
}

/// Contains a list of orders.
#[derive(Debug, Deserialize)]
pub struct OrderList {
    pub orders: Option<Vec<Order>>,
}

/// Represents a limit order made on the exchange.
#[derive(Debug, Serialize)]
pub struct LimitOrder {
    pub pair: String,
    pub r#type: String,
    pub volume: String,
    pub price: String,
    pub base_account_id: String,
    pub counter_account_id: String,
    pub post_only: bool,
}

/// Contains information regarding the posted order.
#[derive(Debug, Deserialize)]
pub struct PostOrderResponse {
    pub order_id: Option<String>,
    pub error: Option<String>,
}

/// Contains information regarding the stopped order.
#[derive(Debug, Deserialize)]
pub struct StopOrderResponse {
    pub success: bool,
}

/// Represents the state of an order.
#[derive(EnumString, Display, Debug, Deserialize)]
pub enum OrderState {
    COMPLETE,
    PENDING,
}

/// A builder for the `limit_order()` method.
pub struct PostLimitOrderBuilder<'a> {
    pub(crate) luno_client: &'a client::LunoClient,
    pub(crate) url: reqwest::Url,
    pub(crate) params: HashMap<&'a str, String>,
}

impl<'a> PostLimitOrderBuilder<'a> {
    pub fn with_base_account(&mut self, id: &'a str) -> &mut PostLimitOrderBuilder<'a> {
        self.params.insert("base_account_id", id.to_owned());
        self
    }

    pub fn with_counter_account(&mut self, id: &'a str) -> &mut PostLimitOrderBuilder<'a> {
        self.params.insert("counter_account_id", id.to_owned());
        self
    }

    pub fn post_only(&mut self) -> &mut PostLimitOrderBuilder<'a> {
        self.params.insert("post_only", "true".to_owned());
        self
    }

    pub async fn post(&mut self) -> Result<PostOrderResponse, reqwest::Error> {
        let url = self.url.clone();

        self.luno_client
            .http
            .post(url)
            .basic_auth(
                self.luno_client.credentials.key.to_owned(),
                Some(self.luno_client.credentials.secret.to_owned()),
            )
            .form(&self.params)
            .send()
            .await?
            .json()
            .await
    }
}

/// A builder for the `market_order()` method.
pub struct PostMarketOrderBuilder<'a> {
    pub(crate) luno_client: &'a client::LunoClient,
    pub(crate) url: reqwest::Url,
    pub(crate) params: HashMap<&'a str, String>,
}

impl<'a> PostMarketOrderBuilder<'a> {
    pub fn with_base_account(&mut self, id: &'a str) -> &mut PostMarketOrderBuilder<'a> {
        self.params.insert("base_account_id", id.to_owned());
        self
    }

    pub fn with_counter_account(&mut self, id: &'a str) -> &mut PostMarketOrderBuilder<'a> {
        self.params.insert("counter_account_id", id.to_owned());
        self
    }

    pub async fn post(&mut self) -> Result<PostOrderResponse, reqwest::Error> {
        let url = self.url.clone();
        self.luno_client
            .http
            .post(url)
            .basic_auth(
                self.luno_client.credentials.key.to_owned(),
                Some(self.luno_client.credentials.secret.to_owned()),
            )
            .form(&self.params)
            .send()
            .await?
            .json()
            .await
    }
}
