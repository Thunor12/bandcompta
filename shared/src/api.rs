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
    pub const PRODUCTS: &str = "/api/products";
    pub const PRODUCTS_BY_ID: &str = "/api/products/{id}";
    pub const PRODUCT_VARIANTS: &str = "/api/products/{id}/variants";
    pub const VARIANT_STOCK: &str = "/api/variants/{id}/stock";

    pub fn transaction(id: i64) -> String {
        format!("/api/transactions/{id}")
    }

    pub fn product(id: i64) -> String {
        format!("/api/products/{id}")
    }

    pub fn product_variants(id: i64) -> String {
        format!("/api/products/{id}/variants")
    }

    pub fn variant_stock(id: i64) -> String {
        format!("/api/variants/{id}/stock")
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

#[allow(dead_code)]
pub fn products_url(base: &str) -> String {
    format!("{}{}", base, paths::PRODUCTS)
}

#[allow(dead_code)]
pub fn product_url(base: &str, id: i64) -> String {
    format!("{}{}", base, paths::product(id))
}

#[allow(dead_code)]
pub fn product_variants_url(base: &str, product_id: i64) -> String {
    format!("{}{}", base, paths::product_variants(product_id))
}

#[allow(dead_code)]
pub fn variant_stock_url(base: &str, variant_id: i64) -> String {
    format!("{}{}", base, paths::variant_stock(variant_id))
}

#[cfg(target_arch = "wasm32")]
use crate::{
    AdjustStock, Contact, ContactFilter, NewContact, NewProduct, NewProductVariant, NewTag,
    NewTransaction, ProductDetail, ProductSummary, ProductVariant, Tag, Transaction,
    TreasurySummary, UploadResponse,
};

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

    pub async fn list_products(&self) -> Result<Vec<ProductSummary>, String> {
        gloo_net::http::Request::get(&products_url(&self.base_url))
            .send()
            .await
            .map_err(|err| err.to_string())?
            .json()
            .await
            .map_err(|err| err.to_string())
    }

    pub async fn get_product(&self, id: i64) -> Result<ProductDetail, String> {
        gloo_net::http::Request::get(&product_url(&self.base_url, id))
            .send()
            .await
            .map_err(|err| err.to_string())?
            .json()
            .await
            .map_err(|err| err.to_string())
    }

    pub async fn create_product(&self, product: &NewProduct) -> Result<ProductDetail, String> {
        gloo_net::http::Request::post(&products_url(&self.base_url))
            .header("Content-Type", "application/json")
            .json(product)
            .map_err(|err| err.to_string())?
            .send()
            .await
            .map_err(|err| err.to_string())?
            .json()
            .await
            .map_err(|err| err.to_string())
    }

    pub async fn create_variant(
        &self,
        product_id: i64,
        variant: &NewProductVariant,
    ) -> Result<ProductVariant, String> {
        gloo_net::http::Request::post(&product_variants_url(&self.base_url, product_id))
            .header("Content-Type", "application/json")
            .json(variant)
            .map_err(|err| err.to_string())?
            .send()
            .await
            .map_err(|err| err.to_string())?
            .json()
            .await
            .map_err(|err| err.to_string())
    }

    pub async fn adjust_stock(
        &self,
        variant_id: i64,
        adjustment: &AdjustStock,
    ) -> Result<ProductVariant, String> {
        gloo_net::http::Request::patch(&variant_stock_url(&self.base_url, variant_id))
            .header("Content-Type", "application/json")
            .json(adjustment)
            .map_err(|err| err.to_string())?
            .send()
            .await
            .map_err(|err| err.to_string())?
            .json()
            .await
            .map_err(|err| err.to_string())
    }
}
