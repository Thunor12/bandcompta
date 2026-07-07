use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    Income,
    Expense,
    Ndf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub id: i64,
    pub name: String,
    pub company: String,
    pub transaction_type: TransactionType,
    pub executed: bool,
    pub date: String,
    pub price_full_tax: f32,
    pub tag: String,
    pub tax_amount: f32,
    pub invoice_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewTransaction {
    pub name: String,
    pub company: String,
    pub transaction_type: TransactionType,
    pub executed: bool,
    pub date: String,
    pub price_full_tax: f32,
    pub tag: String,
    pub tax_amount: f32,
    pub invoice_path: String,
}

impl From<NewTransaction> for Transaction {
    fn from(value: NewTransaction) -> Self {
        Self {
            id: 0,
            name: value.name,
            company: value.company,
            transaction_type: value.transaction_type,
            executed: value.executed,
            date: value.date,
            price_full_tax: value.price_full_tax,
            tag: value.tag,
            tax_amount: value.tax_amount,
            invoice_path: value.invoice_path,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TreasurySummary {
    pub income_total: f32,
    pub expense_total: f32,
    pub ndf_total: f32,
    pub balance: f32,
    pub transaction_count: usize,
}
