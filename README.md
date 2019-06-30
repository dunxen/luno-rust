# Luno Rust API Wrapper

A work-in-progress.

Currently implemented calls:

- `get_orderbook_top(pair: &str)`
- `get_orderbook(pair: &str)`
- `get_ticker(pair: &str)`
- `get_tickers()`
- `get_trades(pair: &str)`

At the moment, all implemented calls are synchronous and return a `Result<T, reqwest::Error>`.
