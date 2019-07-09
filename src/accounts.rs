use serde::{Deserialize, Serialize};

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
