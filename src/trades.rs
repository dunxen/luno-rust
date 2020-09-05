use serde::Deserialize;
use std::future::Future;

use rust_decimal::Decimal;

use crate::{client, LimitOrderType, TradingPair};

/// Represents a trade made on the exchange.
#[derive(Debug, Deserialize)]
pub struct Trade {
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

/// Contains a list of trades.
#[derive(Debug, Deserialize)]
pub struct TradeList {
    pub trades: Option<Vec<Trade>>,
}

/// A builder for the `list_trades()` method.
pub struct ListTradesBuilder<'a> {
    pub(crate) limit: Option<u64>,
    pub(crate) since: Option<u64>,
    pub(crate) before: Option<u64>,
    pub(crate) after_seq: Option<u64>,
    pub(crate) before_seq: Option<u64>,
    pub(crate) sort_desc: Option<bool>,
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

    pub fn before(&mut self, timestamp: u64) -> &mut ListTradesBuilder<'a> {
        self.before = Some(timestamp);
        self
    }

    pub fn after_seq(&mut self, seq: u64) -> &mut ListTradesBuilder<'a> {
        self.after_seq = Some(seq);
        self
    }

    pub fn before_seq(&mut self, seq: u64) -> &mut ListTradesBuilder<'a> {
        self.before_seq = Some(seq);
        self
    }

    pub fn sort_desc(&mut self, sorted: bool) -> &mut ListTradesBuilder<'a> {
        self.sort_desc = Some(sorted);
        self
    }

    pub fn get(&self) -> impl Future<Output = Result<TradeList, reqwest::Error>> + '_ {
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
        self.luno_client.get(url)
    }
}

/// Represents the fee info associated with recent trades.
#[derive(Debug, Deserialize)]
pub struct FeeInfo {
    pub maker_fee: Decimal,
    pub taker_fee: Decimal,
    pub thirty_day_volume: Decimal,
}
