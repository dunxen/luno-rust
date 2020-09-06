use std::collections::HashMap;
use std::string::ToString;

use reqwest::Client;
use rust_decimal::Decimal;
use serde::de::DeserializeOwned;

use crate::{
    Account, Beneficiary, CreateQuoteBuilder, Credentials, Currency, FeeInfo, LimitOrderType,
    ListBalancesBuilder, ListBeneficiariesResponse, ListOrdersBuilder, ListOwnTradesBuilder,
    ListPendingTransactionsResponse, ListTickersResponse, ListTradesResponse,
    ListTransactionsResponse, MarketOrderType, Order, Orderbook, PostLimitOrderBuilder,
    PostMarketOrderBuilder, Quote, StopOrderResponse, Ticker, Trade, TradingPair,
    UpdateAccountNameResponse, UrlMaker,
};

const API_BASE: &str = "https://api.luno.com/api/1/";

/// The top level client for interacting with the Luno API.
pub struct LunoClient {
    pub(crate) credentials: Credentials,
    pub(crate) http: Client,
    pub(crate) url_maker: UrlMaker,
}

impl LunoClient {
    pub fn new(key: &str, secret: &str) -> LunoClient {
        let credentials = Credentials::new(key, secret);
        let http = Client::new();
        let url_maker = UrlMaker::new(API_BASE);

        LunoClient {
            credentials,
            url_maker,
            http,
        }
    }

    pub(crate) async fn get<T>(&self, url: reqwest::Url) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned,
    {
        self.http
            .get(url)
            .basic_auth(
                self.credentials.key.to_owned(),
                Some(self.credentials.secret.to_owned()),
            )
            .send()
            .await?
            .json::<T>()
            .await
    }

    pub(crate) async fn put<T>(&self, url: reqwest::Url) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned,
    {
        self.http
            .put(url)
            .basic_auth(
                self.credentials.key.to_owned(),
                Some(self.credentials.secret.to_owned()),
            )
            .send()
            .await?
            .json::<T>()
            .await
    }

    pub(crate) async fn delete<T>(&self, url: reqwest::Url) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned,
    {
        self.http
            .delete(url)
            .basic_auth(
                self.credentials.key.to_owned(),
                Some(self.credentials.secret.to_owned()),
            )
            .send()
            .await?
            .json::<T>()
            .await
    }

    /// Returns the latest ticker indicators.
    pub async fn get_ticker(&self, pair: TradingPair) -> Result<Ticker, reqwest::Error> {
        let url = self.url_maker.ticker(pair);
        self.get(url).await
    }

    /// Returns the latest ticker indicators from all active Luno exchanges.
    pub async fn list_tickers(&self) -> Result<Vec<Ticker>, reqwest::Error> {
        let url = self.url_maker.tickers();
        Ok(self.get::<ListTickersResponse>(url).await?.tickers)
    }

    /// Returns a list of the top 100 bids and asks in the order book.
    /// Ask orders are sorted by price ascending.
    /// Bid orders are sorted by price descending. Orders of the same price are aggregated.
    pub async fn get_orderbook_top(&self, pair: TradingPair) -> Result<Orderbook, reqwest::Error> {
        let url = self.url_maker.orderbook_top(pair);
        self.get(url).await
    }

    /// Returns a list of all bids and asks in the order book.
    /// Ask orders are sorted by price ascending. Bid orders are sorted by price descending.
    /// Multiple orders at the same price are not aggregated.
    ///
    /// Warning: This may return a large amount of data. Generally you should rather use `get_orderbook_top` or the Streaming API.
    pub async fn get_orderbook(&self, pair: TradingPair) -> Result<Orderbook, reqwest::Error> {
        let url = self.url_maker.orderbook(pair);
        self.get(url).await
    }

    /// Returns a list of the most recent trades that happened in the last 24h.
    /// At most 100 results are returned per call.
    pub async fn list_trades(&self, pair: TradingPair) -> Result<Vec<Trade>, reqwest::Error> {
        let url = self.url_maker.trades(pair);
        Ok(self.get::<ListTradesResponse>(url).await?.trades)
    }

    /// This request creates an account for the specified currency.
    /// Please note that the balances for the Account will be displayed based on the `asset` value,
    /// which is the currency the account is based on.
    ///
    /// Permissions required: `Perm_W_Addresses`.
    pub async fn create_account(
        &self,
        currency: Currency,
        name: &str,
    ) -> Result<Account, reqwest::Error> {
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
            .send()
            .await?
            .json::<Account>()
            .await
    }

    /// Update the name of an account with a given ID,
    ///
    /// `Perm_W_Addresses`
    pub async fn update_account_name(
        &self,
        account_id: &str,
        name: &str,
    ) -> Result<UpdateAccountNameResponse, reqwest::Error> {
        let url = self.url_maker.account_name(account_id, name);
        self.put(url).await
    }

    /// Get a list of all accounts and their respective balances for the requesting user.
    ///
    /// Note that `balances()` returns a `ListBalancesBuilder`
    /// that allows you to specify which assets you are interested in
    /// using `with_assets()` and then executing the query with the `list()`
    /// method on `ListBalancesBuilder`.
    ///
    /// Permissions required: `Perm_R_Balance`.
    pub fn balances(&self) -> ListBalancesBuilder {
        ListBalancesBuilder {
            assets: None,
            url: self.url_maker.balance(),
            luno_client: self,
        }
    }

    /// Return a list of transaction entries from an account.
    ///
    /// Transaction entry rows are numbered sequentially starting from 1, where 1 is the oldest entry.
    /// The range of rows to return are specified with the `min_row` (inclusive) and `max_row` (exclusive) parameters.
    /// At most 1000 rows can be requested per call.
    ///
    /// If `min_row` or `max_row` is non-positive, the range wraps around the most recent row.
    /// For example, to fetch the 100 most recent rows, use `min_row=-100` and `max_row=0`.
    ///
    /// Permissions required: `Perm_R_Transactions`.
    pub async fn list_transactions(
        &self,
        account_id: &str,
        min_row: i64,
        max_row: i64,
    ) -> Result<ListTransactionsResponse, reqwest::Error> {
        let url = self.url_maker.transactions(account_id, min_row, max_row);
        self.get(url).await
    }

    /// Return a list of all transactions that have not completed for the account.
    ///
    /// Pending transactions are not numbered, and may be reordered, deleted or updated at any time.
    ///
    /// Permissions required: `Perm_R_Transactions`.
    pub async fn list_pending_transactions(
        &self,
        account_id: &str,
    ) -> Result<ListPendingTransactionsResponse, reqwest::Error> {
        let url = self.url_maker.pending_transactions(account_id);
        self.get(url).await
    }

    /// Returns a list of bank beneficiaries
    ///
    /// Permissions required: Perm_R_Beneficiaries
    pub async fn list_beneficiaries(&self) -> Result<Vec<Beneficiary>, reqwest::Error> {
        let url = self.url_maker.beneficiaries();
        Ok(self
            .get::<ListBeneficiariesResponse>(url)
            .await?
            .beneficiaries)
    }

    /// Get a list of the most recently placed orders.
    /// Note that `list_orders()` returns a `ListOrdersBuilder`
    /// that allows you chain pair and state filters onto your
    /// request.
    pub fn orders(&self) -> ListOrdersBuilder {
        ListOrdersBuilder {
            luno_client: self,
            url: self.url_maker.list_orders(),
            limit: None,
            created_before: None,
            pair: None,
            state: None,
        }
    }

    /// Create a new trade order.
    ///
    /// Warning! Orders cannot be reversed once they have executed.
    /// Please ensure your program has been thoroughly tested before submitting orders.
    ///
    /// If no `base_account_id` or `counter_account_id` are specified, your default base currency or counter currency account will be used.
    /// You can find your account IDs by calling `list_balances()`.
    pub fn limit_order(
        &self,
        pair: TradingPair,
        order_type: LimitOrderType,
        volume: Decimal,
        price: Decimal,
    ) -> PostLimitOrderBuilder {
        let mut params = HashMap::new();
        params.insert("pair", pair.to_string());
        params.insert("type", order_type.to_string());
        params.insert("volume", volume.to_string());
        params.insert("price", price.to_string());
        PostLimitOrderBuilder {
            luno_client: self,
            url: self.url_maker.post_order(),
            params,
        }
    }

    /// Create a new market order.
    ///
    /// A market order executes immediately, and either buys as much cryptocurrency that can be bought for
    /// a set amount of fiat currency, or sells a set amount of cryptocurrency for as much fiat as possible.
    ///
    /// Warning! Orders cannot be reversed once they have executed.
    /// Please ensure your program has been thoroughly tested before submitting orders.
    ///
    /// If no base_account_id or counter_account_id are specified, your default base currency or counter currency account will be used.
    /// You can find your account IDs by calling the `list_balances()`.
    pub fn market_order(
        &self,
        pair: TradingPair,
        order_type: MarketOrderType,
        volume: Decimal,
    ) -> PostMarketOrderBuilder {
        let mut params = HashMap::new();
        params.insert("pair", pair.to_string());
        params.insert("type", order_type.to_string());
        match order_type {
            MarketOrderType::BUY => params.insert("counter_volume", volume.to_string()),
            MarketOrderType::SELL => params.insert("base_volume", volume.to_string()),
        };
        PostMarketOrderBuilder {
            luno_client: self,
            url: self.url_maker.market_order(),
            params,
        }
    }

    /// Request to cancel an order.
    pub async fn cancel_order(&self, order_id: &str) -> Result<StopOrderResponse, reqwest::Error> {
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
            .send()
            .await?
            .json::<StopOrderResponse>()
            .await
    }

    /// Get an order by its ID.
    pub async fn get_order(&self, order_id: &str) -> Result<Order, reqwest::Error> {
        let url = self.url_maker.orders(order_id);
        self.get(url).await
    }

    /// Returns a list of your recent trades for a given pair, sorted by oldest first. If `before` is specified, then the trades are returned sorted by most recent first.
    ///
    /// `type` in the response indicates the type of order that you placed in order to participate in the trade. Possible types: `BID`, `ASK`.
    ///
    /// If `is_buy` in the response is true, then the order which completed the trade (market taker) was a bid order.
    ///
    /// Results of this query may lag behind the latest data.
    pub fn list_own_trades(&self, pair: TradingPair) -> ListOwnTradesBuilder {
        ListOwnTradesBuilder {
            luno_client: self,
            url: self.url_maker.list_trades(pair),
            since: None,
            before: None,
            after_seq: None,
            before_seq: None,
            sort_desc: None,
            limit: None,
        }
    }

    /// Returns the fees and 30 day trading volume (as of midnight) for a given currency pair.
    /// For complete details, please see [Fees & Features](https://www.luno.com/en/countries).
    pub async fn get_fee_info(&self, pair: TradingPair) -> Result<FeeInfo, reqwest::Error> {
        let url = self.url_maker.fee_info(pair);
        self.get(url).await
    }

    /// Creates a new quote to buy or sell a particular amount of a base currency for a counter currency.
    ///
    /// Users can specify either the exact amount to pay or the exact amount to receive.
    ///
    /// For example, to buy exactly 0.1 Bitcoin using ZAR, you would create a quote to BUY 0.1 XBTZAR.
    /// The returned quote includes the appropriate ZAR amount. To buy Bitcoin using exactly ZAR 100, create a quote to SELL 100 ZARXBT.
    /// The returned quote specifies the Bitcoin as the counter amount returned.
    ///
    /// An error is returned if the Account is not verified for the currency pair, or if the Account would have insufficient balance to ever exercise the quote.
    ///
    /// Permissions required: `Perm_W_Orders`
    pub fn quote(
        &self,
        order_type: MarketOrderType,
        base_amount: Decimal,
        pair: TradingPair,
    ) -> CreateQuoteBuilder {
        let mut params = HashMap::new();
        params.insert("type", order_type.to_string());
        params.insert("base_amount", base_amount.to_string());
        params.insert("pair", pair.to_string());
        CreateQuoteBuilder {
            luno_client: self,
            url: self.url_maker.quotes(),
            params,
        }
    }

    /// Get the latest status of a quote by its id.
    ///
    /// Permissions required: `Perm_R_Orders`
    pub async fn get_quote(&self, id: &str) -> Result<Quote, reqwest::Error> {
        let url = self.url_maker.quote_action(id);
        self.get(url).await
    }

    /// Exercise a quote to perform the Trade.
    /// If there is sufficient balance available in the Account, it will be debited and the counter amount credited.
    ///
    /// An error is returned if the quote has expired or if the Account has insufficient available balance.
    ///
    /// Permissions required: `Perm_W_Orders`
    pub async fn exercise_quote(&self, id: &str) -> Result<Quote, reqwest::Error> {
        let url = self.url_maker.quote_action(id);
        self.put(url).await
    }

    /// Discard a Quote.
    /// Once a Quote has been discarded, it cannot be exercised even if it has not expired.
    ///
    /// Permissions required: `Perm_W_Orders`
    pub async fn discard_quote(&self, id: &str) -> Result<Quote, reqwest::Error> {
        let url = self.url_maker.quote_action(id);
        self.delete(url).await
    }
}
