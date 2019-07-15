use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::string::ToString;

use crate::accounts;
use crate::credentials;
use crate::lightning;
use crate::market;
use crate::orders;
use crate::trades;
use crate::transactions;
use crate::urls;

const API_BASE: &str = "https://api.mybitx.com/api/1/";

/// The top level interface for interacting with the remote service.
pub struct LunoClient {
    pub(crate) credentials: credentials::Credentials,
    pub(crate) http: Client,
    pub(crate) url_maker: urls::UrlMaker,
}

impl LunoClient {
    pub fn new(key: String, secret: String) -> LunoClient {
        let credentials = credentials::Credentials::new(key, secret);
        let http = Client::new();
        let url_maker = urls::UrlMaker::new(API_BASE.to_owned());

        LunoClient {
            credentials,
            url_maker,
            http,
        }
    }

    pub(crate) fn get<T>(&self, url: reqwest::Url) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned,
    {
        self.http
            .get(url)
            .basic_auth(
                self.credentials.key.to_owned(),
                Some(self.credentials.secret.to_owned()),
            )
            .send()?
            .json()
    }

    #[allow(dead_code)]
    pub(crate) fn post_json<T>(&self, url: reqwest::Url, entity: T) -> Result<T, reqwest::Error>
    where
        T: Serialize + DeserializeOwned,
    {
        self.http
            .post(url)
            .basic_auth(
                self.credentials.key.to_owned(),
                Some(self.credentials.secret.to_owned()),
            )
            .json(&entity)
            .send()?
            .json()
    }

    /// Get the current ticker for a given trading pair.
    pub fn get_ticker(&self, pair: market::TradingPair) -> Result<market::Ticker, reqwest::Error> {
        let url = self.url_maker.ticker(&pair.to_string());
        self.get(url)
    }

    /// Get tickers for all available trading pairs.
    pub fn get_tickers(&self) -> Result<market::TickerList, reqwest::Error> {
        let url = self.url_maker.tickers();
        self.get(url)
    }

    /// Get a list of the top 100 bids and asks in the order book for a trading pair.
    pub fn get_orderbook_top(
        &self,
        pair: market::TradingPair,
    ) -> Result<market::Orderbook, reqwest::Error> {
        let url = self.url_maker.orderbook_top(&pair.to_string());
        self.get(url)
    }

    /// Get the full list of bids and asks in the order book for a trading pair.
    pub fn get_orderbook(
        &self,
        pair: market::TradingPair,
    ) -> Result<market::Orderbook, reqwest::Error> {
        let url = self.url_maker.orderbook(&pair.to_string());
        self.get(url)
    }

    /// Get the latest trades for a trading pair (limited to 100).
    pub fn get_trades(
        &self,
        pair: market::TradingPair,
    ) -> Result<market::TradeList, reqwest::Error> {
        let url = self.url_maker.trades(&pair.to_string());
        self.get(url)
    }

    /// Create an additional account for the specified currency..
    pub fn create_account(
        &self,
        currency: market::Currency,
        name: &str,
    ) -> Result<accounts::Account, reqwest::Error> {
        let url = self.url_maker.accounts();
        let mut params = HashMap::new();
        params.insert("currency", currency.to_string());
        params.insert("name", name.to_string());

        self.http
            .post(url)
            .basic_auth(
                self.credentials.key.to_owned(),
                Some(self.credentials.secret.to_owned()),
            )
            .form(&params)
            .send()?
            .json()
    }

    /// Get a list of all accounts and their respective balances.
    pub fn get_balances(&self) -> Result<accounts::BalanceList, reqwest::Error> {
        let url = self.url_maker.balance();
        self.get(url)
    }

    pub fn get_transactions(
        &self,
        account_id: &str,
        min_row: u64,
        max_row: u64,
    ) -> Result<transactions::TransactionList, reqwest::Error> {
        let url = self.url_maker.transactions(account_id, min_row, max_row);
        self.get(url)
    }

    pub fn get_pending_transactions(
        &self,
        account_id: &str,
    ) -> Result<transactions::PendingTransactionList, reqwest::Error> {
        let url = self.url_maker.pending_transactions(account_id);
        self.get(url)
    }

    /// Get a list of the most recently placed orders.
    /// Note that `list_orders()` returns a `ListOrdersBuilder`
    /// that allows you chain pair and state filters onto your
    /// request.
    ///
    /// ```
    pub fn list_orders(&self) -> orders::ListOrdersBuilder {
        orders::ListOrdersBuilder {
            luno_client: self,
            url: self.url_maker.list_orders(),
            pair: None,
            state: None,
        }
    }

    /// Create a new trade order.
    pub fn limit_order(
        &self,
        pair: market::TradingPair,
        r#type: orders::LimitOrderType,
        volume: f64,
        price: f64,
    ) -> orders::PostLimitOrderBuilder {
        let mut params = HashMap::new();
        params.insert("pair", pair.to_string());
        params.insert("type", r#type.to_string());
        params.insert("volume", volume.to_string());
        params.insert("price", price.to_string());
        orders::PostLimitOrderBuilder {
            luno_client: self,
            url: self.url_maker.post_order(),
            params,
        }
    }

    /// Create a new market order.
    /// A market order executes immediately, and either buys as much bitcoin that can be bought for
    /// as set amount of fiat currency, or sells a set amount of bitcoin for as much fiat as possible.
    ///
    /// Optionally specify a specific counter and base account with `.with_counter_account(id: &str)` and
    /// `.with_base_account(id: &str)`
    ///
    /// NOTE: Please see the fees associated with trades at https://www.luno.com/en/countries
    pub fn market_order(
        &self,
        pair: market::TradingPair,
        r#type: orders::MarketOrderType,
        volume: f64,
    ) -> orders::PostMarketOrderBuilder {
        let mut params = HashMap::new();
        params.insert("pair", pair.to_string());
        params.insert("type", r#type.to_string());
        match r#type {
            orders::MarketOrderType::BUY => params.insert("counter_volume", volume.to_string()),
            orders::MarketOrderType::SELL => params.insert("base_volume", volume.to_string()),
        };
        orders::PostMarketOrderBuilder {
            luno_client: self,
            url: self.url_maker.market_order(),
            params,
        }
    }

    /// Request to stop an order.
    pub fn stop_order(&self, order_id: &str) -> Result<orders::StopOrderResponse, reqwest::Error> {
        let url = self.url_maker.stop_order();
        let mut params = HashMap::new();
        params.insert("order_id", order_id.to_string());

        self.http
            .post(url)
            .basic_auth(
                self.credentials.key.to_owned(),
                Some(self.credentials.secret.to_owned()),
            )
            .form(&params)
            .send()?
            .json()
    }

    /// Get an order by its ID.
    pub fn get_order(&self, order_id: &str) -> Result<orders::Order, reqwest::Error> {
        let url = self.url_maker.orders(order_id);
        self.get(url)
    }

    /// Returns a list of your recent trades for a given pair, sorted by oldest first. If before is specified, then the trades are returned sorted by most-recent first.
    ///
    /// type in the response indicates the type of order that you placed in order to participate in the trade. Possible types: BID, ASK.
    ///
    /// If is_buy in the response is true, then the order which completed the trade (market taker) was a bid order.
    ///
    /// Results of this query may lag behind the latest data.
    pub fn list_own_trades(&self, pair: market::TradingPair) -> trades::ListTradesBuilder {
        trades::ListTradesBuilder {
            luno_client: self,
            url: self.url_maker.list_trades(&pair.to_string()),
            since: None,
            limit: None,
        }
    }

    /// Returns your fees and 30 day trading volume (as of midnight) for a given pair.
    pub fn get_fee_info(
        &self,
        pair: market::TradingPair,
    ) -> Result<trades::FeeInfo, reqwest::Error> {
        let url = self.url_maker.fee_info(&pair.to_string());
        self.get(url)
    }

    pub fn lightning_send(&self, payment_request: &str) -> lightning::LightningSendBuilder {
        let mut params = HashMap::new();
        params.insert("payment_request", payment_request.to_string());
        lightning::LightningSendBuilder {
            luno_client: self,
            url: self.url_maker.lightning_send(),
            params,
        }
    }

    pub fn lightning_receive(
        &self,
        amount: &f64,
        expires_at: u64,
    ) -> lightning::LightningReceiveBuilder {
        let mut params = HashMap::new();
        params.insert("amount", amount.to_string());
        params.insert("expires_at", expires_at.to_string());
        lightning::LightningReceiveBuilder {
            luno_client: self,
            url: self.url_maker.lightning_receive(),
            params,
        }
    }
}
