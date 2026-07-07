use crate::DateFilter;

pub const DEFAULT_API_BASE: &str = "http://127.0.0.1:3000";
pub const DEFAULT_API_ADDR: &str = "127.0.0.1:3000";

pub mod paths {
    pub const TRANSACTIONS: &str = "/api/transactions";
    pub const TRANSACTIONS_BY_ID: &str = "/api/transactions/{id}";
    pub const SUMMARY: &str = "/api/summary";

    pub fn transaction(id: i64) -> String {
        format!("/api/transactions/{id}")
    }
}

#[allow(dead_code)]
pub fn transactions_url(base: &str, filter: &DateFilter) -> String {
    format!("{}{}{}", base, paths::TRANSACTIONS, filter.to_query_string())
}

#[allow(dead_code)]
pub fn summary_url(base: &str, filter: &DateFilter) -> String {
    format!("{}{}{}", base, paths::SUMMARY, filter.to_query_string())
}

#[allow(dead_code)]
pub fn transaction_url(base: &str, id: i64) -> String {
    format!("{}{}", base, paths::transaction(id))
}

#[cfg(target_arch = "wasm32")]
use crate::{Transaction, TreasurySummary};

#[cfg(target_arch = "wasm32")]
pub struct ApiClient {
    base_url: String,
}

#[cfg(target_arch = "wasm32")]
impl ApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    pub fn default_client() -> Self {
        Self::new(DEFAULT_API_BASE)
    }

    pub async fn list_transactions(&self, filter: &DateFilter) -> Result<Vec<Transaction>, String> {
        gloo_net::http::Request::get(&transactions_url(&self.base_url, filter))
            .send()
            .await
            .map_err(|err| err.to_string())?
            .json()
            .await
            .map_err(|err| err.to_string())
    }

    pub async fn get_transaction(&self, id: i64) -> Result<Transaction, String> {
        gloo_net::http::Request::get(&transaction_url(&self.base_url, id))
            .send()
            .await
            .map_err(|err| err.to_string())?
            .json()
            .await
            .map_err(|err| err.to_string())
    }

    pub async fn summary(&self, filter: &DateFilter) -> Result<TreasurySummary, String> {
        gloo_net::http::Request::get(&summary_url(&self.base_url, filter))
            .send()
            .await
            .map_err(|err| err.to_string())?
            .json()
            .await
            .map_err(|err| err.to_string())
    }
}
