use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[repr(i32)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductKind {
    Tshirt = 0,
    Patch = 1,
    Pin = 2,
    Print = 3,
    Album = 4,
}

impl ProductKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Tshirt => "T-shirt",
            Self::Patch => "Patch",
            Self::Pin => "Pin",
            Self::Print => "Print",
            Self::Album => "Album",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[repr(i32)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VariantOption {
    Color = 0,
    Size = 1,
    Format = 2,
    VinylColor = 3,
}

impl VariantOption {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Color => "Couleur",
            Self::Size => "Taille",
            Self::Format => "Format",
            Self::VinylColor => "Couleur vinyle",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariantAttribute {
    pub option: VariantOption,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub kind: ProductKind,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewProduct {
    pub name: String,
    pub kind: ProductKind,
    pub description: String,
}

impl NewProduct {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("le nom du produit est obligatoire".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductVariant {
    pub id: i64,
    pub product_id: i64,
    pub sku: String,
    pub stock_quantity: i32,
    pub unit_price: f32,
    pub attributes: Vec<VariantAttribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewProductVariant {
    pub sku: String,
    pub stock_quantity: i32,
    pub unit_price: f32,
    pub attributes: Vec<VariantAttribute>,
}

impl NewProductVariant {
    pub fn validate(&self, kind: ProductKind) -> Result<(), String> {
        if self.stock_quantity < 0 {
            return Err("le stock ne peut pas être négatif".into());
        }
        validate_attributes_for_kind(kind, &self.attributes)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductSummary {
    pub id: i64,
    pub name: String,
    pub kind: ProductKind,
    pub description: String,
    pub variant_count: usize,
    pub total_stock: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductDetail {
    pub product: Product,
    pub variants: Vec<ProductVariant>,
    pub total_stock: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdjustStock {
    pub quantity_delta: i32,
    pub note: String,
}

impl AdjustStock {
    pub fn validate(&self) -> Result<(), String> {
        if self.quantity_delta == 0 {
            return Err("la variation de stock doit être non nulle".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MovementType {
    Sale,
    Adjustment,
    Restock,
}

impl MovementType {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Sale => "Vente",
            Self::Adjustment => "Ajustement",
            Self::Restock => "Réappro",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StockMovement {
    pub id: i64,
    pub variant_id: i64,
    pub quantity_delta: i32,
    pub note: String,
    pub created_at: String,
    pub movement_type: MovementType,
    pub transaction_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StockMovementDetail {
    pub id: i64,
    pub variant_id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub variant_label: String,
    pub sku: String,
    pub quantity_delta: i32,
    pub note: String,
    pub created_at: String,
    pub movement_type: MovementType,
    pub transaction_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct StockMovementFilter {
    pub variant_id: Option<i64>,
    pub product_id: Option<i64>,
    pub limit: Option<u32>,
}

impl StockMovementFilter {
    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();
        if let Some(variant_id) = self.variant_id {
            params.push(format!("variant_id={variant_id}"));
        }
        if let Some(product_id) = self.product_id {
            params.push(format!("product_id={product_id}"));
        }
        if let Some(limit) = self.limit {
            params.push(format!("limit={limit}"));
        }
        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MerchSale {
    pub variant_id: i64,
    pub quantity: i32,
    pub date: String,
    pub company: String,
    pub unit_price: Option<f32>,
    pub tax_amount: f32,
    pub executed: bool,
    pub invoice_path: String,
    pub note: String,
}

impl MerchSale {
    pub fn validate(&self) -> Result<(), String> {
        if self.quantity <= 0 {
            return Err("la quantité vendue doit être positive".into());
        }
        if self.date.len() != 10 {
            return Err("la date doit être au format AAAA-MM-JJ".into());
        }
        if self.company.trim().is_empty() {
            return Err("la société / point de vente est obligatoire".into());
        }
        if let Some(price) = self.unit_price {
            if price <= 0.0 {
                return Err("le prix unitaire doit être positif".into());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MerchSaleResult {
    pub transaction: crate::Transaction,
    pub variant: ProductVariant,
    pub movement: StockMovement,
}

pub const DEFAULT_LOW_STOCK_THRESHOLD: i32 = 5;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LowStockAlert {
    pub variant_id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub variant_label: String,
    pub sku: String,
    pub stock_quantity: i32,
    pub threshold: i32,
    pub unit_price: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct LowStockFilter {
    pub threshold: Option<i32>,
}

impl LowStockFilter {
    pub fn to_query_string(&self) -> String {
        match self.threshold {
            Some(threshold) => format!("?threshold={threshold}"),
            None => String::new(),
        }
    }
}

pub const SHIRT_SIZES: &[&str] = &["XS", "S", "M", "L", "XL", "2XL", "3XL"];
pub const ALBUM_FORMATS: &[&str] = &["CD", "Cassette", "Vinyle"];

pub fn validate_attributes_for_kind(
    kind: ProductKind,
    attributes: &[VariantAttribute],
) -> Result<(), String> {
    let has = |opt: VariantOption| {
        attributes
            .iter()
            .any(|attr| attr.option == opt && !attr.value.trim().is_empty())
    };

    match kind {
        ProductKind::Tshirt => {
            if !has(VariantOption::Color) {
                return Err("couleur obligatoire pour un t-shirt".into());
            }
            if !has(VariantOption::Size) {
                return Err("taille obligatoire pour un t-shirt".into());
            }
        }
        ProductKind::Album => {
            if !has(VariantOption::Format) {
                return Err("format obligatoire pour un album".into());
            }
            let format = attributes
                .iter()
                .find(|attr| attr.option == VariantOption::Format)
                .map(|attr| attr.value.as_str());
            if format == Some("Vinyle") && !has(VariantOption::VinylColor) {
                return Err("couleur vinyle obligatoire pour un vinyle".into());
            }
        }
        ProductKind::Print => {
            if !has(VariantOption::Format) {
                return Err("format obligatoire pour un print".into());
            }
        }
        ProductKind::Patch | ProductKind::Pin => {}
    }
    Ok(())
}

pub fn variant_label(attributes: &[VariantAttribute]) -> String {
    if attributes.is_empty() {
        return "Standard".into();
    }
    attributes
        .iter()
        .map(|attr| attr.value.clone())
        .collect::<Vec<_>>()
        .join(" / ")
}
