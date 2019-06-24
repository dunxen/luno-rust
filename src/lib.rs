use std::cell::RefCell;
use std::io;

use futures::{Future, Stream};
use hyper::client::HttpConnector;
use hyper::header::{Authorization, Basic};
use hyper::{Client, Headers, Uri};
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use serde_json::Value as JsValue;
use tokio_core::reactor::Core;
use url::Url;

struct UriMaker {
    api_base: String,
}

impl UriMaker {
    /// Convenience constructor for UriMaker.
    pub fn new(api_base: String) -> UriMaker {
        UriMaker { api_base }
    }

    /// Convert from a `url::Url` to a `hyper::Uri`.
    fn url_to_uri(url: &url::Url) -> Uri {
        url.as_str().parse().unwrap()
    }

    /// Append a path to the API root
    fn build_url(&self, path: &str) -> Result<Url, url::ParseError> {
        let url = Url::parse(&self.api_base)?.join(path)?;

        Ok(url)
    }
    
    /// Build https://api.mybitx.com/api/1/ticker?pair=...
    pub fn ticker(&self, pair: &str) -> Uri {
        let mut url = self.build_url("ticker").unwrap();
        url.query_pairs_mut().append_pair("pair", pair);
        Self::url_to_uri(&url)
    }

    /// Build https://api.mybitx.com/api/1/tickers
    pub fn tickers(&self) -> Uri {
        let url = self.build_url("tickers").unwrap();
        Self::url_to_uri(&url)
    }

    /// Build https://api.mybitx.com/api/1/orderbook_top?pair=...
    pub fn orderbook_top(&self, pair: &str) -> Uri {
        let mut url = self.build_url("orderbook_top").unwrap();
        url.query_pairs_mut().append_pair("pair", pair);
        Self::url_to_uri(&url)
    }

    /// Build https://api.mybitx.com/api/1/orderbook?pair=...
    pub fn orderbook(&self, pair: &str) -> Uri {
        let mut url = self.build_url("orderbook").unwrap();
        url.query_pairs_mut().append_pair("pair", pair);
        Self::url_to_uri(&url)
    }

    /// Build https://api.mybitx.com/api/1/trades?pair=...
    pub fn trades(&self, pair: &str) -> Uri {
        let mut url = self.build_url("trades").unwrap();
        url.query_pairs_mut().append_pair("pair", pair);
        Self::url_to_uri(&url)
    }
}

/// type alias for a custom hyper client, configured for HTTPS
/// instead of HTTP.
type HttpsClient = Client<HttpsConnector<HttpConnector>, hyper::Body>;

/// The top level interface for interacting with the remote service.
pub struct LunoClient {
    uri_maker: UriMaker,
    core: RefCell<Core>,
    http: HttpsClient,
}

fn to_io_error<E>(err: E) -> io::Error
where
    E: Into<Box<std::error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, err)
}

impl LunoClient {
    pub fn new(key: String, secret: String) -> LunoClient {
        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
            username: key,
            password: Some(secret),
        }));

        let uri_maker = UriMaker::new("https://api.mybitx.com/api/1/".to_owned());

        let core = Core::new().unwrap();

        let http = {
            let handle = core.handle();
            let connector = HttpsConnector::new(4, &handle).unwrap();

            Client::configure().connector(connector).build(&handle)
        };

        LunoClient {
            uri_maker,
            core: RefCell::new(core),
            http,
        }
    }

    fn get_json(&self, uri: hyper::Uri) -> Box<Future<Item = JsValue, Error = io::Error>> {
        let f = self
            .http
            .get(uri)
            .and_then(|res| {
                res.body().concat2().and_then(move |body| {
                    let value: JsValue = serde_json::from_slice(&body).map_err(to_io_error)?;
                    Ok(value)
                })
            })
            .map_err(to_io_error);
        Box::new(f)
    }

    /// Get the current ticker for a given trading pair.
    pub fn get_ticker(&self, pair: &str) -> Result<Ticker, io::Error> {
        let uri = self.uri_maker.ticker(pair);
        let work = self.get_json(uri).and_then(|value| {
            let ticker: Ticker = serde_json::from_value(value).map_err(to_io_error)?;
            Ok(ticker)
        });
        self.core.borrow_mut().run(work)
    }

    /// Get tickers for all available trading pairs.
    pub fn get_tickers(&self) -> Result<TickerList, io::Error> {
        let uri = self.uri_maker.tickers();
        let work = self.get_json(uri).and_then(|value| {
            let tickers: TickerList = serde_json::from_value(value).map_err(to_io_error)?;
            Ok(tickers)
        });
        self.core.borrow_mut().run(work)
    }

    /// Get a list of the top 100 bids and asks in the order book for a trading pair.
    pub fn get_orderbook_top(&self, pair: &str) -> Result<Orderbook, io::Error> {
        let uri = self.uri_maker.orderbook_top(pair);
        let work = self.get_json(uri).and_then(|value| {
            let orderbook_top: Orderbook = serde_json::from_value(value).map_err(to_io_error)?;
            Ok(orderbook_top)
        });
        self.core.borrow_mut().run(work)
    }

    /// Get the full list of bids and asks in the order book for a trading pair.
    pub fn get_orderbook(&self, pair: &str) -> Result<Orderbook, io::Error> {
        let uri = self.uri_maker.orderbook(pair);
        let work = self.get_json(uri).and_then(|value| {
            let orderbook: Orderbook = serde_json::from_value(value).map_err(to_io_error)?;
            Ok(orderbook)
        });
        self.core.borrow_mut().run(work)
    }

    /// Get the latest trades for a trading pair (limited to 100).
    pub fn get_trades(&self, pair: &str) -> Result<TradeList, io::Error> {
        let uri = self.uri_maker.trades(pair);
        let work = self.get_json(uri).and_then(|value| {
            let trades: TradeList = serde_json::from_value(value).map_err(to_io_error)?;
            Ok(trades)
        });
        self.core.borrow_mut().run(work)
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
    pub tickers: Vec<Ticker>,
}

#[derive(Debug, Deserialize)]
pub struct Bid {
    volume: String,
    price: String,
}

#[derive(Debug, Deserialize)]
pub struct Ask {
    volume: String,
    price: String,
}

#[derive(Debug, Deserialize)]
pub struct Orderbook {
    timestamp: u64,
    bids: Vec<Bid>,
    asks: Vec<Ask>,
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
    pub trades: Vec<Trade>,
}
