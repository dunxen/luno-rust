use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const API_BASE: &str = "https://api.mybitx.com/api/1/";

struct UrlMaker {
    api_base: reqwest::Url,
}

impl UrlMaker {
    /// Convenience constructor for UrlMaker.
    pub fn new(api_base: String) -> UrlMaker {
        let url = reqwest::Url::parse(&api_base).unwrap();
        UrlMaker { api_base: url }
    }

    /// Append a path to the API root
    fn build_url(&self, path: &str) -> reqwest::Url {
        self.api_base.join(path).unwrap()
    }

    /// Build https://api.mybitx.com/api/1/ticker?pair=...
    pub fn ticker(&self, pair: &str) -> reqwest::Url {
        let mut url = self.build_url("ticker");
        url.query_pairs_mut().append_pair("pair", pair);
        url
    }

    /// Build https://api.mybitx.com/api/1/tickers
    pub fn tickers(&self) -> reqwest::Url {
        self.build_url("tickers")
    }

    /// Build https://api.mybitx.com/api/1/orderbook_top?pair=...
    pub fn orderbook_top(&self, pair: &str) -> reqwest::Url {
        let mut url = self.build_url("orderbook_top");
        url.query_pairs_mut().append_pair("pair", pair);
        url
    }

    /// Build https://api.mybitx.com/api/1/orderbook?pair=...
    pub fn orderbook(&self, pair: &str) -> reqwest::Url {
        let mut url = self.build_url("orderbook");
        url.query_pairs_mut().append_pair("pair", pair);
        url
    }

    /// Build https://api.mybitx.com/api/1/trades?pair=...
    pub fn trades(&self, pair: &str) -> reqwest::Url {
        let mut url = self.build_url("trades");
        url.query_pairs_mut().append_pair("pair", pair);
        url
    }

    // Build https://api.mybitx.com/api/1/accounts
    pub fn accounts(&self) -> reqwest::Url {
        self.build_url("accounts")
    }

    // Build https://api.mybitx.com/api/1/balance
    pub fn balance(&self) -> reqwest::Url {
        self.build_url("balance")
    }

    // Build https://api.mybitx.com/api/1/account/:id/transactions
    pub fn transactions(&self, account_id: &str, min_row: u64, max_row: u64) -> reqwest::Url {
        let mut url = self.accounts();
        url.path_segments_mut()
            .unwrap()
            .extend(&[account_id, "transactions"]);
        url.query_pairs_mut()
            .append_pair("min_row", &min_row.to_string())
            .append_pair("max_row", &max_row.to_string());
        url
    }

    // Build https://api.mybitx.com/api/1/account/:id/pending
    pub fn pending_transactions(&self, account_id: &str) -> reqwest::Url {
        let mut url = self.accounts();
        url.path_segments_mut()
            .unwrap()
            .extend(&[account_id, "pending"]);
        url
    }

    // Build https://api.mybitx.com/api/1/listorders
    pub fn list_orders(&self) -> reqwest::Url {
        self.build_url("listorders")
    }
}

struct Credentials {
    key: String,
    secret: String,
}

impl Credentials {
    fn new(key: String, secret: String) -> Credentials {
        Credentials { key, secret }
    }
}

/// The top level interface for interacting with the remote service.
pub struct LunoClient {
    credentials: Credentials,
    http: Client,
    url_maker: UrlMaker,
}

impl LunoClient {
    pub fn new(key: String, secret: String) -> LunoClient {
        let credentials = Credentials::new(key, secret);
        let http = Client::new();
        let url_maker = UrlMaker::new(API_BASE.to_owned());

        LunoClient {
            credentials,
            url_maker,
            http,
        }
    }

    fn get<T>(&self, url: reqwest::Url) -> Result<T, reqwest::Error>
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
    fn post_son<T>(&self, url: reqwest::Url, entity: T) -> Result<T, reqwest::Error>
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
    pub fn get_ticker(&self, pair: &str) -> Result<Ticker, reqwest::Error> {
        let url = self.url_maker.ticker(pair);
        self.get(url)
    }

    /// Get tickers for all available trading pairs.
    pub fn get_tickers(&self) -> Result<TickerList, reqwest::Error> {
        let url = self.url_maker.tickers();
        self.get(url)
    }

    /// Get a list of the top 100 bids and asks in the order book for a trading pair.
    pub fn get_orderbook_top(&self, pair: &str) -> Result<Orderbook, reqwest::Error> {
        let url = self.url_maker.orderbook_top(pair);
        self.get(url)
    }

    /// Get the full list of bids and asks in the order book for a trading pair.
    pub fn get_orderbook(&self, pair: &str) -> Result<Orderbook, reqwest::Error> {
        let url = self.url_maker.orderbook(pair);
        self.get(url)
    }

    /// Get the latest trades for a trading pair (limited to 100).
    pub fn get_trades(&self, pair: &str) -> Result<TradeList, reqwest::Error> {
        let url = self.url_maker.trades(pair);
        self.get(url)
    }

    /// Create an additional account for the specified currency..
    pub fn create_account(&self, currency: &str, name: &str) -> Result<Account, reqwest::Error> {
        let url = self.url_maker.accounts();
        let mut params = HashMap::new();
        params.insert("currency", currency);
        params.insert("name", name);

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
    pub fn get_balances(&self) -> Result<BalanceList, reqwest::Error> {
        let url = self.url_maker.balance();
        self.get(url)
    }

    pub fn get_transactions(
        &self,
        account_id: &str,
        min_row: u64,
        max_row: u64,
    ) -> Result<TransactionList, reqwest::Error> {
        let url = self.url_maker.transactions(account_id, min_row, max_row);
        self.get(url)
    }

    pub fn get_pending_transactions(
        &self,
        account_id: &str,
    ) -> Result<PendingTransactionList, reqwest::Error> {
        let url = self.url_maker.pending_transactions(account_id);
        self.get(url)
    }

    pub fn list_orders(&self) -> ListOrdersBuilder {
        ListOrdersBuilder {
            luno_client: self,
            url: self.url_maker.list_orders(),
            pair: None,
            state: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Ticker {
    pub ask: String,
    pub timestamp: u64,
    pub bid: String,
    pub rolling_24_hour_volume: String,
    pub last_trade: String,
    pub pair: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TickerList {
    pub tickers: Option<Vec<Ticker>>,
}

#[derive(Debug, Deserialize)]
pub struct Bid {
    pub volume: String,
    pub price: String,
}

#[derive(Debug, Deserialize)]
pub struct Ask {
    pub volume: String,
    pub price: String,
}

#[derive(Debug, Deserialize)]
pub struct Orderbook {
    pub timestamp: u64,
    pub bids: Option<Vec<Bid>>,
    pub asks: Option<Vec<Ask>>,
}

#[derive(Debug, Deserialize)]
pub struct Trade {
    pub volume: String,
    pub timestamp: u64,
    pub price: String,
    pub is_buy: bool,
}

#[derive(Debug, Deserialize)]
pub struct TradeList {
    pub trades: Option<Vec<Trade>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: Option<String>,
    pub currency: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Balance {
    pub account_id: String,
    pub asset: String,
    pub balance: String,
    pub reserved: String,
    pub unconfirmed: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BalanceList {
    pub balance: Option<Vec<Balance>>,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub row_index: Option<u64>,
    pub timestamp: u64,
    pub balance: f64,
    pub available: f64,
    pub balance_delta: f64,
    pub available_delta: f64,
    pub currency: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct TransactionList {
    pub id: String,
    pub transactions: Option<Vec<Transaction>>,
}

#[derive(Debug, Deserialize)]
pub struct PendingTransactionList {
    pub id: String,
    pub pending: Option<Vec<Transaction>>,
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
    pub pair: String,
    pub state: String,
    pub r#type: String,
}

pub struct ListOrdersBuilder<'a> {
    state: Option<&'a str>,
    pair: Option<&'a str>,
    luno_client: &'a LunoClient,
    url: reqwest::Url,
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
pub struct PostLimitOrderResponse {
    pub order_id: String,
}

pub enum OrderState {
    Complete,
    Pending,
}
