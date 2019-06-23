use std::cell::RefCell;
use std::io;

use futures::{Future, Stream};
use hyper::client::HttpConnector;
use hyper::header::{Authorization, Basic};
use hyper::{Client, Uri, Headers};
use hyper_tls::HttpsConnector;
use serde::{Deserialize};
use serde_json::Value as JsValue;
use tokio_core::reactor::Core;
use url::Url;

struct UriMaker {
    /// The prefix of every url we'll be producing.
    api_base: String,
}

impl UriMaker {
    /// Convenience constructor for UriMaker.
    pub fn new(api_base: String) -> UriMaker {
        UriMaker {
            api_base,
        }
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

    /// Get ticker for given trading pair. e.g. XBTZAR
    pub fn ticker(&self, pair: &str) -> Uri {
        let mut url = self.build_url("ticker").unwrap();
        url.query_pairs_mut().append_pair("pair", pair);
        Self::url_to_uri(&url)
    }
}

/// type alias for a custom hyper client, configured for HTTPS
/// instead of HTTP.
type HttpsClient = Client<HttpsConnector<HttpConnector>, hyper::Body>;

/// The top level interface for interacting with the remote service.
pub struct LunoClient {
    /// The `UriMaker` we built in Part 1 of the series.
    uri_maker: UriMaker,
    /// tokio "core" to run our requests in.
    core: RefCell<Core>,
    /// hyper http client to build requests with.
    http: HttpsClient,
}

fn to_io_error<E>(err: E) -> io::Error
where
    E: Into<Box<std::error::Error + Send + Sync>>,
{
    // We can create a new IO Error with an ErrorKind of "other", then
    // pass in the actual error as data inside the wrapper type.
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

    pub fn get_ticker(&self, pair: &str) -> Result<Ticker, io::Error> {
        let uri = self.uri_maker.ticker(pair);
        let work = self.get_json(uri).and_then(|value| {
            println!("{}", value);
            let ticker: Ticker = serde_json::from_value(value).map_err(to_io_error)?;

            Ok(ticker)
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
}
