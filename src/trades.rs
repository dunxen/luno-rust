use serde::Deserialize;

use reqwest::Url;
use rust_decimal::Decimal;

use crate::{LimitOrderType, LunoClient, TradingPair};

/// Represents a trade made on the exchange.
#[derive(Debug, Deserialize)]
pub struct OwnTrade {
    pub base: Decimal,
    pub counter: Decimal,
    pub fee_base: Decimal,
    pub is_buy: bool,
    pub order_id: String,
    pub pair: TradingPair,
    pub price: Decimal,
    pub timestamp: u64,
    #[serde(alias = "type")]
    pub order_type: LimitOrderType,
    pub volume: Decimal,
}

/// A builder for the `list_trades()` method.
pub struct ListOwnTradesBuilder<'a> {
    pub(crate) limit: Option<u64>,
    pub(crate) since: Option<u64>,
    pub(crate) before: Option<u64>,
    pub(crate) after_seq: Option<u64>,
    pub(crate) before_seq: Option<u64>,
    pub(crate) sort_desc: Option<bool>,
    pub(crate) luno_client: &'a LunoClient,
    pub(crate) url: Url,
}

impl<'a> ListOwnTradesBuilder<'a> {
    pub fn since(&mut self, timestamp: u64) -> &mut ListOwnTradesBuilder<'a> {
        self.since = Some(timestamp);
        self
    }

    pub fn limit(&mut self, count: u64) -> &mut ListOwnTradesBuilder<'a> {
        self.limit = Some(count);
        self
    }

    pub fn before(&mut self, timestamp: u64) -> &mut ListOwnTradesBuilder<'a> {
        self.before = Some(timestamp);
        self
    }

    pub fn after_seq(&mut self, seq: u64) -> &mut ListOwnTradesBuilder<'a> {
        self.after_seq = Some(seq);
        self
    }

    pub fn before_seq(&mut self, seq: u64) -> &mut ListOwnTradesBuilder<'a> {
        self.before_seq = Some(seq);
        self
    }

    pub fn sort_desc(&mut self, sorted: bool) -> &mut ListOwnTradesBuilder<'a> {
        self.sort_desc = Some(sorted);
        self
    }

    pub async fn list(&self) -> Result<Vec<OwnTrade>, reqwest::Error> {
        let mut url = self.url.clone();
        if let Some(since) = self.since {
            url.query_pairs_mut()
                .append_pair("since", &since.to_string());
        }
        if let Some(limit) = self.limit {
            url.query_pairs_mut()
                .append_pair("limit", &limit.to_string());
        }
        if let Some(timestamp) = self.before {
            url.query_pairs_mut()
                .append_pair("before", &timestamp.to_string());
        }
        if let Some(seq) = self.after_seq {
            url.query_pairs_mut()
                .append_pair("after_seq", &seq.to_string());
        }
        if let Some(seq) = self.before_seq {
            url.query_pairs_mut()
                .append_pair("before_seq", &seq.to_string());
        }
        if let Some(sorted) = self.sort_desc {
            url.query_pairs_mut()
                .append_pair("sort_desc", &sorted.to_string());
        }
        Ok(self
            .luno_client
            .get::<ListOwnTradesResponse>(url)
            .await?
            .trades)
    }
}

#[derive(Debug, Deserialize)]
pub struct ListOwnTradesResponse {
    pub trades: Vec<OwnTrade>,
}

/// Represents the fee info associated with recent trades.
#[derive(Debug, Deserialize)]
pub struct FeeInfo {
    pub maker_fee: Decimal,
    pub taker_fee: Decimal,
    pub thirty_day_volume: Decimal,
}
