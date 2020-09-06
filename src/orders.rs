use std::collections::HashMap;

use reqwest::{Error, Url};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::LunoClient;
use crate::TradingPair;

/// Represents the type of the limit order.
#[derive(EnumString, Display, Deserialize, Serialize, Debug)]
pub enum LimitOrderType {
    ASK,
    BID,
}

/// Represents the type of the market order.
#[derive(EnumString, Display, Deserialize, Debug)]
pub enum MarketOrderType {
    BUY,
    SELL,
}

/// Represents an order.
#[derive(Debug, Deserialize)]
pub struct Order {
    /// The base account ID against which this order is made.
    pub base: Decimal,
    /// The counter account ID against which this order is made.
    pub counter: Decimal,
    /// The UNIX timestamp of creation of the order.
    pub creation_timestamp: u64,
    /// The UNIX timestamp of expiration of this order.
    pub expiration_timestamp: u64,
    /// The UNIX timestamp of completion of this order.
    pub completed_timestamp: u64,
    /// The base fee debited after the trade principal amount.
    pub fee_base: Decimal,
    /// The counter fee debited after the trade principal amount.
    pub fee_counter: Decimal,
    /// The limit price of this order.
    pub limit_price: Decimal,
    /// The limit volume of this order.
    pub limit_volume: Decimal,
    /// The ID of the order.
    pub order_id: String,
    /// The market trading pair.
    pub pair: TradingPair,
    /// The state of the order.
    pub state: OrderState,
    /// The type of the order.
    #[serde(alias = "type")]
    pub order_type: LimitOrderType,
}

/// A builder for the `list_orders()` method.
pub struct ListOrdersBuilder<'a> {
    pub(crate) state: Option<OrderState>,
    pub(crate) pair: Option<TradingPair>,
    pub(crate) created_before: Option<u64>,
    pub(crate) limit: Option<u64>,
    pub(crate) luno_client: &'a LunoClient,
    pub(crate) url: Url,
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

    pub fn filter_created_before(&mut self, timestamp: u64) -> &mut ListOrdersBuilder<'a> {
        self.created_before = Some(timestamp);
        self
    }

    pub fn filter_limit(&mut self, limit: u64) -> &mut ListOrdersBuilder<'a> {
        self.limit = Some(limit);
        self
    }

    /// Executes the list query with the specified parameters.
    pub async fn list(&self) -> Result<Vec<Order>, Error> {
        let mut url = self.url.clone();
        if let Some(state) = &self.state {
            url.query_pairs_mut()
                .append_pair("state", &state.to_string());
        }
        if let Some(pair) = &self.pair {
            url.query_pairs_mut().append_pair("pair", &pair.to_string());
        }
        if let Some(timestamp) = &self.created_before {
            url.query_pairs_mut()
                .append_pair("created_before", &timestamp.to_string());
        }
        if let Some(limit) = &self.limit {
            url.query_pairs_mut()
                .append_pair("limit", &limit.to_string());
        }
        Ok(self
            .luno_client
            .get::<ListOrdersResponse>(url)
            .await?
            .orders)
    }
}

/// Contains a list of orders.
#[derive(Debug, Deserialize)]
pub struct ListOrdersResponse {
    pub orders: Vec<Order>,
}

/// Represents a limit order made on the exchange.
#[derive(Debug, Serialize)]
pub struct LimitOrder {
    pub pair: String,
    #[serde(alias = "type")]
    pub order_type: LimitOrderType,
    pub volume: String,
    pub price: String,
    pub stop_price: String,
    pub stop_direction: String,
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

/// Side of the trigger (stop) price to activate the order.
#[derive(EnumString, Display, Debug, Serialize)]
pub enum StopDirection {
    BELOW,
    ABOVE,
    /// Automatically infers the direction based on the
    /// last trade price and the stop price. If last trade
    /// price is less than stop price then stop direction is
    /// ABOVE, otherwise BELOW.
    #[allow(non_camel_case_types)]
    RELATIVE_LAST_TRADE,
}

/// A builder for the `limit_order()` method.
pub struct PostLimitOrderBuilder<'a> {
    pub(crate) luno_client: &'a LunoClient,
    pub(crate) url: Url,
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

    pub fn with_stop_price(&mut self, price: Decimal) -> &mut PostLimitOrderBuilder<'a> {
        self.params.insert("stop_price", price.to_string());
        self
    }

    pub fn with_stop_direction(
        &mut self,
        direction: StopDirection,
    ) -> &mut PostLimitOrderBuilder<'a> {
        self.params.insert("stop_direction", direction.to_string());
        self
    }

    pub async fn post(&mut self) -> Result<PostOrderResponse, Error> {
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
    pub(crate) luno_client: &'a LunoClient,
    pub(crate) url: Url,
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

    pub async fn post(&mut self) -> Result<PostOrderResponse, Error> {
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
