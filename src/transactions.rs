use serde::Deserialize;

/// Represents a transaction on an account.
#[derive(Debug, Deserialize)]
pub struct Transaction {
  pub row_index: Option<u64>,
  pub timestamp: u64,
  pub balance: f64,
  pub available: f64,
  pub balance_delta: f64,
  pub available_delta: f64,
  pub currency: String,
  pub description: String,
}

/// Contains a list of transactions.
#[derive(Debug, Deserialize)]
pub struct TransactionList {
  pub id: String,
  pub transactions: Option<Vec<Transaction>>,
}

/// Contains a list of pending transactions.
#[derive(Debug, Deserialize)]
pub struct PendingTransactionList {
  pub id: String,
  pub pending: Option<Vec<Transaction>>,
}
