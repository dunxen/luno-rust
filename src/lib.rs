//! A Rust wrapper for the Luno API. 🦀🚀
//!
//! In order to use some features of the [Luno API](https://luno.com/api) you need
//! to have an existing Luno account or you can sign up at [https://luno.com/signup](https://luno.com/signup).
//! Go to [https://www.luno.com/wallet/security/api_keys](https://www.luno.com/wallet/security/api_keys) to
//! create a new API key with appropriate permissions.

pub mod accounts;
pub mod beneficiaries;
pub mod client;
pub mod market;
pub mod orders;
pub mod quotes;
pub mod trades;
pub mod transactions;

mod credentials;
mod urls;

pub use accounts::{Account, Balance, ListBalancesBuilder, UpdateAccountNameResponse};
pub use beneficiaries::{Beneficiary, ListBeneficiariesResponse};
pub use client::LunoClient;
pub use credentials::Credentials;
pub use market::{
    Ask, Bid, Currency, ListTickersResponse, ListTradesResponse, Orderbook, Ticker, Trade,
    TradingPair,
};
pub use orders::{
    LimitOrderType, ListOrdersBuilder, ListOrdersResponse, MarketOrderType, Order,
    PostLimitOrderBuilder, PostMarketOrderBuilder, PostOrderResponse, StopDirection,
    StopOrderResponse,
};
pub use quotes::{CreateQuoteBuilder, Quote};
pub use trades::{FeeInfo, ListOwnTradesBuilder, OwnTrade};
pub use transactions::{ListPendingTransactionsResponse, ListTransactionsResponse, Transaction};

use urls::UrlMaker;
