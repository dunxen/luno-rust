use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(EnumString, Display, Debug, Serialize, Deserialize)]
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
	pub ask: Decimal,
	pub bid: Decimal,
	pub last_trade: Decimal,
	pub pair: TradingPair,
	pub rolling_24_hour_volume: Decimal,
	pub timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct ListTickersResponse {
	pub tickers: Vec<Ticker>,
}

#[derive(Debug, Deserialize)]
pub struct Bid {
	pub price: Decimal,
	pub volume: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct Ask {
	pub price: Decimal,
	pub volume: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct Orderbook {
	pub asks: Vec<Ask>,
	pub bids: Vec<Bid>,
	pub timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct Trade {
	pub is_buy: bool,
	pub price: Decimal,
	pub timestamp: u64,
	pub volume: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct ListTradesResponse {
	pub trades: Vec<Trade>,
}
