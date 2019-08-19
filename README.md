# (Unofficial) Luno Rust API Wrapper 🦀🚀

[![Build Status](https://travis-ci.org/duncandean/luno-rust.svg?branch=master)](https://travis-ci.org/duncandean/luno-rust)
[![codecov](https://codecov.io/gh/duncandean/luno-rust/branch/master/graph/badge.svg)](https://codecov.io/gh/duncandean/luno-rust)

A work-in-progress.

Please read the license. This software is by no means production ready and use of it is at your own risk!

## Installation

A crate does not yet exist on `crates.io` but you can include this repository directly in your `cargo.toml`:

```toml
[dependencies]
luno = "0.1.0"
```

## Documentation

Documentation can be found [here](https://duncandean.github.io/luno-rust).

## Examples

Examples of calls can be found in the `examples/` directory. Clone this repository and run a specific example, such as `get-trades.rs` with:

```bash
cargo run --example get-trades
```

Example with `get_trades()`:

```rust
use luno::{LunoClient, TradingPair};

fn main() {
    let key = String::from("LUNO_API_KEY");
    let secret = String::from("LUNO_API_SECRET");

    let client = LunoClient::new(key, secret);

    match client.get_trades(TradingPair::XBTZAR) {
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
Trade { volume: "0.005686", timestamp: 1561918948454, price: "173001.00", is_buy: false }
Trade { volume: "0.007", timestamp: 1561918942586, price: "173002.00", is_buy: true }
Trade { volume: "0.006936", timestamp: 1561918937500, price: "173002.00", is_buy: true }
Trade { volume: "0.006345", timestamp: 1561918911780, price: "173378.00", is_buy: true }
Trade { volume: "0.0005", timestamp: 1561918878415, price: "173585.00", is_buy: false }
Trade { volume: "0.00577", timestamp: 1561918867525, price: "173590.00", is_buy: false }
...
```

At the moment, all implemented calls are synchronous and return a `Result<T, reqwest::Error>`. We aim to implement async functionality and streams when [_we are async_](https://areweasyncyet.rs/).
