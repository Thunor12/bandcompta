use crate::DateFilter;

pub const DEFAULT_API_BASE: &str = "http://127.0.0.1:3000";
pub const DEFAULT_API_ADDR: &str = "127.0.0.1:3000";

pub mod paths {
    pub const TRANSACTIONS: &str = "/api/transactions";
    pub const TRANSACTIONS_BY_ID: &str = "/api/transactions/{id}";
    pub const SUMMARY: &str = "/api/summary";
    pub const INVOICES_UPLOAD: &str = "/api/invoices/upload";
    pub const CONTACTS: &str = "/api/contacts";
    pub const CONTACTS_BY_ID: &str = "/api/contacts/{id}";
    pub const TAGS: &str = "/api/tags";

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

#[allow(dead_code)]
pub fn contacts_url(base: &str, filter: &crate::ContactFilter) -> String {
    format!("{}{}{}", base, paths::CONTACTS, filter.to_query_string())
}

#[allow(dead_code)]
pub fn tags_url(base: &str) -> String {
    format!("{}{}", base, paths::TAGS)
}

#[cfg(target_arch = "wasm32")]
use crate::{Contact, ContactFilter, NewContact, NewTag, NewTransaction, Tag, Transaction, TreasurySummary, UploadResponse};

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

    pub async fn create_transaction(&self, transaction: &NewTransaction) -> Result<Transaction, String> {
        gloo_net::http::Request::post(&format!("{}{}", self.base_url, paths::TRANSACTIONS))
            .header("Content-Type", "application/json")
            .json(transaction)
            .map_err(|err| err.to_string())?
            .send()
            .await
            .map_err(|err| err.to_string())?
            .json()
            .await
            .map_err(|err| err.to_string())
    }

    pub async fn upload_invoice(&self, file: web_sys::File) -> Result<UploadResponse, String> {
        let form = web_sys::FormData::new().map_err(|err| format!("FormData: {err:?}"))?;
        form.append_with_blob_and_filename("file", &file, &file.name())
            .map_err(|err| format!("append file: {err:?}"))?;

        gloo_net::http::Request::post(&format!("{}{}", self.base_url, paths::INVOICES_UPLOAD))
            .body(form)
            .map_err(|err| err.to_string())?
            .send()
            .await
            .map_err(|err| err.to_string())?
            .json()
            .await
            .map_err(|err| err.to_string())
    }

    pub async fn list_contacts(&self, filter: &ContactFilter) -> Result<Vec<Contact>, String> {
        gloo_net::http::Request::get(&contacts_url(&self.base_url, filter))
            .send()
            .await
            .map_err(|err| err.to_string())?
            .json()
            .await
            .map_err(|err| err.to_string())
    }

    pub async fn create_contact(&self, contact: &NewContact) -> Result<Contact, String> {
        gloo_net::http::Request::post(&format!("{}{}", self.base_url, paths::CONTACTS))
            .header("Content-Type", "application/json")
            .json(contact)
            .map_err(|err| err.to_string())?
            .send()
            .await
            .map_err(|err| err.to_string())?
            .json()
            .await
            .map_err(|err| err.to_string())
    }

    pub async fn list_tags(&self) -> Result<Vec<Tag>, String> {
        gloo_net::http::Request::get(&tags_url(&self.base_url))
            .send()
            .await
            .map_err(|err| err.to_string())?
            .json()
            .await
            .map_err(|err| err.to_string())
    }

    pub async fn create_tag(&self, tag: &NewTag) -> Result<Tag, String> {
        gloo_net::http::Request::post(&format!("{}{}", self.base_url, paths::TAGS))
            .header("Content-Type", "application/json")
            .json(tag)
            .map_err(|err| err.to_string())?
            .send()
            .await
            .map_err(|err| err.to_string())?
            .json()
            .await
            .map_err(|err| err.to_string())
    }
}
