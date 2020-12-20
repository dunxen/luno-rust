# (Unofficial) Luno Rust API Wrapper 🦀🚀

![Rust](https://github.com/duncandean/luno-rust/workflows/Rust/badge.svg)
[![codecov](https://codecov.io/gh/duncandean/luno-rust/branch/trunk/graph/badge.svg)](https://codecov.io/gh/duncandean/luno-rust)

A work-in-progress.

Please read the license. This software is by no means production ready and use of it is at your own risk!

## Installation

In your `Cargo.toml` include:

```toml
[dependencies]
luno = "0.3.0"
```

## Documentation

Documentation can be found [here](https://docs.rs/luno).

## Examples

Examples of calls can be found in the `examples/` directory. Clone this repository and run a specific example, such as `get-trades.rs` with:

```bash
cargo run --example get-trades
```

Example with `list_trades()`:

```rust
use luno::{LunoClient, TradingPair};

#[tokio::main]
async fn main() {
    let client = LunoClient::new("LUNO_API_KEY", "LUNO_API_SECRET");

    match client.list_trades(TradingPair::XBTZAR).await {
        Err(e) => eprintln!("{:?}", e),
        Ok(result) => {
            if let Some(trade) = result.trades {
                println!("{:?}", trade);
            }
        }
    }
}
```

Results:

```
Trade { is_buy: false, price: 357379.00, timestamp: 1608452000171, volume: 0.037895 }
Trade { is_buy: false, price: 357380.00, timestamp: 1608452000161, volume: 0.012887 }
Trade { is_buy: true, price: 357672.00, timestamp: 1608451984425, volume: 0.001678 }
Trade { is_buy: false, price: 357369.00, timestamp: 1608451981078, volume: 0.000986 }
Trade { is_buy: true, price: 357682.00, timestamp: 1608451969439, volume: 0.001677 }
Trade { is_buy: false, price: 357357.00, timestamp: 1608451961962, volume: 0.002143 }
...
```

By default, all methods are asynchronous and return a `Future` wrapping `Result<T, reqwest::Error>`.
