use serde::Deserialize;
use strum_macros::{Display, EnumString};

#[derive(EnumString, Display)]
pub enum Currency {
    AUD,
    BCH,
    BTC,
    ETH,
    EUR,
    GBP,
    IDR,
    LTC,
    MYR,
    NGN,
    UGX,
    XBT,
    XRP,
    ZAR,
    ZMW,
}

#[derive(EnumString, Display, Debug, Deserialize)]
pub enum TradingPair {
    BCHXBT,
    ETHAUD,
    ETHEUR,
    ETHGBP,
    ETHIDR,
    ETHMYR,
    ETHNGN,
    ETHXBT,
    ETHZAR,
    LTCMYR,
    LTCXBT,
    LTCNGN,
    LTCZAR,
    XBTAUD,
    XBTEUR,
    XBTGBP,
    XBTIDR,
    XBTMYR,
    XBTNGN,
    XBTSGD,
    XBTUGX,
    XBTZAR,
    XBTZMW,
    XRPMYR,
    XRPNGN,
    XRPXBT,
    XRPZAR,
}

#[derive(Debug, Deserialize)]
pub struct Ticker {
    pub ask: String,
    pub bid: String,
    pub last_trade: String,
    pub pair: Option<String>,
    pub rolling_24_hour_volume: String,
    pub timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct TickerList {
    pub tickers: Option<Vec<Ticker>>,
}

#[derive(Debug, Deserialize)]
pub struct Bid {
    pub price: String,
    pub volume: String,
}

#[derive(Debug, Deserialize)]
pub struct Ask {
    pub price: String,
    pub volume: String,
}

#[derive(Debug, Deserialize)]
pub struct Orderbook {
    pub asks: Option<Vec<Ask>>,
    pub bids: Option<Vec<Bid>>,
    pub timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct Trade {
    pub is_buy: bool,
    pub price: String,
    pub timestamp: u64,
    pub volume: String,
}

#[derive(Debug, Deserialize)]
pub struct TradeList {
    pub trades: Option<Vec<Trade>>,
}
