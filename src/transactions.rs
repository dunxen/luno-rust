use rust_decimal::Decimal;
use serde::Deserialize;

/// Represents a transaction on an account.
#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub row_index: Option<u64>,
    pub timestamp: u64,
    pub balance: Decimal,
    pub available: Decimal,
    pub balance_delta: Decimal,
    pub available_delta: Decimal,
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
