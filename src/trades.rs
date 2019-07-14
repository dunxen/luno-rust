use serde::Deserialize;

use crate::{client, TradingPair};

#[derive(Debug, Deserialize)]
pub struct Trade {
    pub base: String,
    pub counter: String,
    pub fee_base: String,
    pub is_buy: bool,
    pub order_id: String,
    pub pair: TradingPair,
    pub price: String,
    pub timestamp: u64,
    pub r#type: String,
    pub volume: String,
}

#[derive(Debug, Deserialize)]
pub struct TradeList {
    pub trades: Option<Vec<Trade>>,
}

pub struct ListTradesBuilder<'a> {
    pub(crate) limit: Option<u64>,
    pub(crate) since: Option<u64>,
    pub(crate) luno_client: &'a client::LunoClient,
    pub(crate) url: reqwest::Url,
}

impl<'a> ListTradesBuilder<'a> {
    pub fn since(&mut self, timestamp: u64) -> &mut ListTradesBuilder<'a> {
        self.since = Some(timestamp);
        self
    }

    pub fn limit(&mut self, count: u64) -> &mut ListTradesBuilder<'a> {
        self.limit = Some(count);
        self
    }

    pub fn get(&self) -> Result<TradeList, reqwest::Error> {
        let mut url = self.url.clone();
        if self.since.is_some() {
            url.query_pairs_mut()
                .append_pair("since", &self.since.unwrap().to_string());
        }
        if self.limit.is_some() {
            url.query_pairs_mut()
                .append_pair("limit", &self.limit.unwrap().to_string());
        }
        self.luno_client.get(url)
    }
}
