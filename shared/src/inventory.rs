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
