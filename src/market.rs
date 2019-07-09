use serde::Deserialize;

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
