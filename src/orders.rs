use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::{Display, EnumString};

use crate::client;
use crate::TradingPair;

#[derive(EnumString, Display)]
pub enum LimitOrderType {
    ASK,
    BID,
}

#[derive(EnumString, Display)]
pub enum MarketOrderType {
    BUY,
    SELL,
}

#[derive(Debug, Deserialize)]
pub struct Order {
    pub base: String,
    pub counter: String,
    pub creation_timestamp: u64,
    pub expiration_timestamp: u64,
    pub completed_timestamp: u64,
    pub fee_base: String,
    pub fee_counter: String,
    pub limit_price: String,
    pub limit_volume: String,
    pub order_id: String,
    pub pair: TradingPair,
    pub state: String,
    pub r#type: String,
}

pub struct ListOrdersBuilder<'a> {
    pub(crate) state: Option<&'a str>,
    pub(crate) pair: Option<&'a str>,
    pub(crate) luno_client: &'a client::LunoClient,
    pub(crate) url: reqwest::Url,
}

impl<'a> ListOrdersBuilder<'a> {
    pub fn filter_state(&mut self, state: OrderState) -> &mut ListOrdersBuilder<'a> {
        match state {
            OrderState::Complete => self.state = Some("COMPLETE"),
            OrderState::Pending => self.state = Some("PENDING"),
        }
        self
    }

    pub fn filter_pair(&mut self, pair: &'a str) -> &mut ListOrdersBuilder<'a> {
        self.pair = Some(pair);
        self
    }

    pub fn get(&self) -> Result<OrderList, reqwest::Error> {
        let mut url = self.url.clone();
        if self.state.is_some() {
            url.query_pairs_mut()
                .append_pair("state", &self.state.unwrap());
        }
        if self.pair.is_some() {
            url.query_pairs_mut()
                .append_pair("pair", &self.pair.unwrap());
        }
        self.luno_client.get(url)
    }
}

#[derive(Debug, Deserialize)]
pub struct OrderList {
    pub orders: Option<Vec<Order>>,
}

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

#[derive(Debug, Deserialize)]
pub struct PostOrderResponse {
    pub order_id: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StopOrderResponse {
    pub success: bool,
}

pub enum OrderState {
    Complete,
    Pending,
}

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

    pub fn post(&mut self) -> Result<PostOrderResponse, reqwest::Error> {
        let url = self.url.clone();
        self.luno_client
            .http
            .post(url)
            .basic_auth(
                self.luno_client.credentials.key.to_owned(),
                Some(self.luno_client.credentials.secret.to_owned()),
            )
            .form(&self.params)
            .send()?
            .json()
    }
}

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

    pub fn post(&mut self) -> Result<PostOrderResponse, reqwest::Error> {
        let url = self.url.clone();
        self.luno_client
            .http
            .post(url)
            .basic_auth(
                self.luno_client.credentials.key.to_owned(),
                Some(self.luno_client.credentials.secret.to_owned()),
            )
            .form(&self.params)
            .send()?
            .json()
    }
}
