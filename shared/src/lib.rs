mod api;
mod inventory;
mod models;

pub use api::{
    paths, contacts_url, merch_sales_url, product_url, product_variants_url, products_url,
    stock_alerts_url, stock_movements_url, summary_url, tags_url, transaction_url,
    transactions_url, variant_stock_url, DEFAULT_API_ADDR, DEFAULT_API_BASE,
};
#[cfg(target_arch = "wasm32")]
pub use api::ApiClient;
pub use inventory::{
    AdjustStock, LowStockAlert, LowStockFilter, MerchSale, MerchSaleResult, MovementType,
    NewProduct, NewProductVariant, Product, ProductDetail, ProductKind, ProductSummary,
    ProductVariant, StockMovement, StockMovementDetail, StockMovementFilter, VariantAttribute,
    VariantOption, ALBUM_FORMATS, DEFAULT_LOW_STOCK_THRESHOLD, SHIRT_SIZES, variant_label,
};
pub use models::{
    compute_treasury_summary, Contact, ContactFilter, ContactKind, DateFilter, NewContact,
    NewTag, NewTransaction, Tag, Transaction, TransactionType, TreasurySummary, UploadResponse,
    PREDEFINED_TAGS,
};
