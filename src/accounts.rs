use std::future::Future;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::market::Currency;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: Option<String>,
    pub currency: Currency,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Balance {
    pub account_id: String,
    pub asset: Currency,
    pub balance: Decimal,
    pub reserved: Decimal,
    pub unconfirmed: Decimal,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct BalanceList {
    pub balance: Vec<Balance>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAccountNameResponse {
    pub success: bool,
}
pub struct ListBalancesBuilder<'a> {
    pub(crate) luno_client: &'a client::LunoClient,
    pub(crate) url: reqwest::Url,
    pub(crate) assets: Option<&'a [Currency]>,
}

impl<'a> ListBalancesBuilder<'a> {
    pub fn with_assets(&mut self, assets: &'a [Currency]) -> &mut ListBalancesBuilder<'a> {
        self.assets = Some(assets);
        self
    }

    pub fn list(&self) -> impl Future<Output = Result<BalanceList, reqwest::Error>> + '_ {
        let mut url = self.url.clone();
        if let Some(assets) = self.assets {
            url.query_pairs_mut().append_pair(
                "assets",
                &assets
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        self.luno_client.get(url)
    }
}
