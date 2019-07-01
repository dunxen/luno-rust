# (Unofficial) Luno Rust API Wrapper

A work-in-progress.

Currently implemented calls:

- `get_orderbook_top(pair: &str)`
- `get_orderbook(pair: &str)`
- `get_ticker(pair: &str)`
- `get_tickers()`
- `get_trades(pair: &str)`
- `create_account(currency: &str, name: &str)`
- `get_transactions(account_id: &str, min_row: u64, max_row: max_64)`
- `get_pending_transactions(account_id: &str)`

Example with `get_trades()`:

```rust
use luno::LunoClient;

fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client.get_trades("XBTZAR") {
        Err(e) => eprintln!("{:?}", e),
        Ok(result) => {
            for trade in result.trades.into_iter() {
                println!("{:?}", trade);
            }
        }
    }
}
```

Results:

```
Trade { volume: "0.005686", timestamp: 1561918948454, price: "173001.00", is_buy: false }
Trade { volume: "0.007", timestamp: 1561918942586, price: "173002.00", is_buy: true }
Trade { volume: "0.006936", timestamp: 1561918937500, price: "173002.00", is_buy: true }
Trade { volume: "0.006345", timestamp: 1561918911780, price: "173378.00", is_buy: true }
Trade { volume: "0.0005", timestamp: 1561918878415, price: "173585.00", is_buy: false }
Trade { volume: "0.00577", timestamp: 1561918867525, price: "173590.00", is_buy: false }
...
```

At the moment, all implemented calls are synchronous and return a `Result<T, reqwest::Error>`.
