use std::collections::HashMap;

use futures::Future;
use serde::{Deserialize, Serialize};

use crate::client;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: Option<String>,
    pub currency: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Balance {
    pub account_id: String,
    pub asset: String,
    pub balance: String,
    pub reserved: String,
    pub unconfirmed: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BalanceList {
    pub balance: Option<Vec<Balance>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAccountNameResponse {
    pub success: bool,
}
pub struct ListBalancesBuilder<'a> {
    pub(crate) luno_client: &'a client::LunoClient,
    pub(crate) url: reqwest::Url,
    pub(crate) assets: Option<&'a [&'a str]>,
}

impl<'a> ListBalancesBuilder<'a> {
    pub fn with_assets(&mut self, assets: &'a [&'a str]) -> &mut ListBalancesBuilder<'a> {
        self.assets = Some(assets);
        self
    }

    pub fn list(&self) -> impl Future<Output = Result<BalanceList, reqwest::Error>> + '_ {
        let mut url = self.url.clone();
        if let Some(assets) = self.assets {
            url.query_pairs_mut()
                .append_pair("assets", &assets.join(","));
        }
        self.luno_client.get(url)
    }
}
