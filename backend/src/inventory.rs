use rusqlite::{Connection, OptionalExtension, Result as SqlResult};
use serde_rusqlite::{from_rows, to_params_named};

use bandcompta_shared::{
    AdjustStock, LowStockAlert, LowStockFilter, MerchSale, MerchSaleResult, MovementType,
    NewProduct, NewProductVariant, NewTransaction, Product, ProductDetail, ProductKind,
    ProductSummary, ProductVariant, StockMovement, StockMovementDetail, StockMovementFilter,
    TransactionType, VariantAttribute, VariantOption, variant_label,
};

use crate::db::{get_transaction, insert_transaction, map_serde_err};

const PRODUCTS_SCHEMA: &str = "CREATE TABLE IF NOT EXISTS products (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    kind INTEGER NOT NULL,
    description TEXT NOT NULL DEFAULT ''
)";

const VARIANTS_SCHEMA: &str = "CREATE TABLE IF NOT EXISTS product_variants (
    id INTEGER PRIMARY KEY,
    product_id INTEGER NOT NULL,
    sku TEXT NOT NULL DEFAULT '',
    stock_quantity INTEGER NOT NULL DEFAULT 0,
    unit_price REAL NOT NULL DEFAULT 0,
    low_stock_threshold INTEGER NOT NULL DEFAULT 5,
    FOREIGN KEY (product_id) REFERENCES products(id)
)";

const ATTRIBUTES_SCHEMA: &str = "CREATE TABLE IF NOT EXISTS variant_attributes (
    id INTEGER PRIMARY KEY,
    variant_id INTEGER NOT NULL,
    option_type INTEGER NOT NULL,
    value TEXT NOT NULL,
    FOREIGN KEY (variant_id) REFERENCES product_variants(id)
)";

const MOVEMENTS_SCHEMA: &str = "CREATE TABLE IF NOT EXISTS stock_movements (
    id INTEGER PRIMARY KEY,
    variant_id INTEGER NOT NULL,
    quantity_delta INTEGER NOT NULL,
    note TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL,
    movement_type TEXT NOT NULL DEFAULT 'ADJUSTMENT',
    transaction_id INTEGER,
    FOREIGN KEY (variant_id) REFERENCES product_variants(id),
    FOREIGN KEY (transaction_id) REFERENCES transactions(id)
)";

pub fn init_inventory(connection: &Connection) -> SqlResult<()> {
    connection.execute(PRODUCTS_SCHEMA, [])?;
    connection.execute(VARIANTS_SCHEMA, [])?;
    connection.execute(ATTRIBUTES_SCHEMA, [])?;
    connection.execute(MOVEMENTS_SCHEMA, [])?;
    migrate_inventory(connection)?;
    seed_inventory_if_empty(connection)?;
    Ok(())
}

fn migrate_inventory(connection: &Connection) -> SqlResult<()> {
    let _ = connection.execute(
        "ALTER TABLE stock_movements ADD COLUMN movement_type TEXT NOT NULL DEFAULT 'ADJUSTMENT'",
        [],
    );
    let _ = connection.execute(
        "ALTER TABLE stock_movements ADD COLUMN transaction_id INTEGER REFERENCES transactions(id)",
        [],
    );
    let _ = connection.execute(
        "ALTER TABLE product_variants ADD COLUMN low_stock_threshold INTEGER NOT NULL DEFAULT 5",
        [],
    );
    Ok(())
}

fn seed_inventory_if_empty(connection: &Connection) -> SqlResult<()> {
    let count: i64 = connection.query_row("SELECT COUNT(*) FROM products", [], |row| row.get(0))?;
    if count > 0 {
        return Ok(());
    }

    let tshirt_id = insert_product(
        connection,
        &NewProduct {
            name: "T-shirt Tour 2025".into(),
            kind: ProductKind::Tshirt,
            description: "Logo recto tour".into(),
        },
    )?;
    for (color, size, stock) in [
        ("Noir", "S", 5),
        ("Noir", "M", 12),
        ("Noir", "L", 8),
        ("Blanc", "M", 4),
        ("Blanc", "L", 6),
    ] {
        insert_variant(
            connection,
            tshirt_id,
            ProductKind::Tshirt,
            &NewProductVariant {
                sku: format!("TSH-{color}-{size}"),
                stock_quantity: stock,
                unit_price: 25.0,
                attributes: vec![
                    VariantAttribute {
                        option: VariantOption::Color,
                        value: color.into(),
                    },
                    VariantAttribute {
                        option: VariantOption::Size,
                        value: size.into(),
                    },
                ],
            },
        )?;
    }

    let pin_id = insert_product(
        connection,
        &NewProduct {
            name: "Pin logo".into(),
            kind: ProductKind::Pin,
            description: "Pin émail".into(),
        },
    )?;
    insert_variant(
        connection,
        pin_id,
        ProductKind::Pin,
        &NewProductVariant {
            sku: "PIN-LOGO".into(),
            stock_quantity: 150,
            unit_price: 8.0,
            attributes: vec![],
        },
    )?;

    let album_id = insert_product(
        connection,
        &NewProduct {
            name: "Premier Album".into(),
            kind: ProductKind::Album,
            description: "Album studio".into(),
        },
    )?;
    for (format, vinyl_color, stock, sku) in [
        ("CD", "", 100, "ALB-CD"),
        ("Cassette", "", 30, "ALB-CAS"),
        ("Vinyle", "Noir", 50, "ALB-VIN-NOIR"),
        ("Vinyle", "Rouge", 25, "ALB-VIN-ROUGE"),
    ] {
        let mut attributes = vec![VariantAttribute {
            option: VariantOption::Format,
            value: format.into(),
        }];
        if format == "Vinyle" {
            attributes.push(VariantAttribute {
                option: VariantOption::VinylColor,
                value: vinyl_color.into(),
            });
        }
        insert_variant(
            connection,
            album_id,
            ProductKind::Album,
            &NewProductVariant {
                sku: sku.into(),
                stock_quantity: stock,
                unit_price: 20.0,
                attributes,
            },
        )?;
    }

    Ok(())
}

pub fn list_products(connection: &Connection) -> SqlResult<Vec<ProductSummary>> {
    let mut statement = connection.prepare(
        "SELECT p.id, p.name, p.kind, p.description,
                COUNT(v.id) AS variant_count,
                COALESCE(SUM(v.stock_quantity), 0) AS total_stock
         FROM products p
         LEFT JOIN product_variants v ON v.product_id = p.id
         GROUP BY p.id
         ORDER BY p.name ASC",
    )?;
    let mut rows = statement.query([])?;
    let mut products = Vec::new();
    while let Some(row) = rows.next()? {
        products.push(ProductSummary {
            id: row.get(0)?,
            name: row.get(1)?,
            kind: kind_from_row(&row, 2)?,
            description: row.get(3)?,
            variant_count: row.get::<_, i64>(4)? as usize,
            total_stock: row.get::<_, i64>(5)? as i32,
        });
    }
    Ok(products)
}

pub fn get_product(connection: &Connection, id: i64) -> SqlResult<Option<ProductDetail>> {
    let mut statement =
        connection.prepare("SELECT id, name, kind, description FROM products WHERE id = ?1")?;
    let mut rows = from_rows::<Product>(statement.query([id])?);
    let Some(product) = rows.next().transpose().map_err(map_serde_err)? else {
        return Ok(None);
    };

    let variants = list_variants_for_product(connection, id)?;
    let total_stock = variants.iter().map(|variant| variant.stock_quantity).sum();

    Ok(Some(ProductDetail {
        product,
        variants,
        total_stock,
    }))
}

pub fn insert_product(connection: &Connection, product: &NewProduct) -> SqlResult<i64> {
    let params = to_params_named(product).map_err(map_serde_err)?;
    connection.execute(
        "INSERT INTO products (name, kind, description) VALUES (:name, :kind, :description)",
        params.to_slice().as_slice(),
    )?;
    Ok(connection.last_insert_rowid())
}

pub fn insert_variant(
    connection: &Connection,
    product_id: i64,
    kind: ProductKind,
    variant: &NewProductVariant,
) -> SqlResult<i64> {
    variant.validate(kind).map_err(invalid_input)?;

    connection.execute(
        "INSERT INTO product_variants (product_id, sku, stock_quantity, unit_price)
         VALUES (?1, ?2, ?3, ?4)",
        (
            product_id,
            variant.sku.as_str(),
            variant.stock_quantity,
            variant.unit_price,
        ),
    )?;
    let variant_id = connection.last_insert_rowid();
    for attribute in &variant.attributes {
        insert_attribute(connection, variant_id, attribute)?;
    }
    Ok(variant_id)
}

fn insert_attribute(connection: &Connection, variant_id: i64, attribute: &VariantAttribute) -> SqlResult<()> {
    connection.execute(
        "INSERT INTO variant_attributes (variant_id, option_type, value) VALUES (?1, ?2, ?3)",
        (variant_id, attribute.option as i32, attribute.value.as_str()),
    )?;
    Ok(())
}

fn list_variants_for_product(connection: &Connection, product_id: i64) -> SqlResult<Vec<ProductVariant>> {
    let mut statement = connection.prepare(
        "SELECT id, product_id, sku, stock_quantity, unit_price
         FROM product_variants
         WHERE product_id = ?1
         ORDER BY id ASC",
    )?;
    let rows = from_rows::<ProductVariantRow>(statement.query([product_id])?);
    let mut variants = Vec::new();
    for row in rows.filter_map(|row| row.ok()) {
        let attributes = list_attributes_for_variant(connection, row.id)?;
        variants.push(ProductVariant {
            id: row.id,
            product_id: row.product_id,
            sku: row.sku,
            stock_quantity: row.stock_quantity,
            unit_price: row.unit_price,
            attributes,
        });
    }
    Ok(variants)
}

fn list_attributes_for_variant(connection: &Connection, variant_id: i64) -> SqlResult<Vec<VariantAttribute>> {
    let mut statement = connection.prepare(
        "SELECT option_type, value FROM variant_attributes WHERE variant_id = ?1 ORDER BY option_type ASC",
    )?;
    let mut rows = statement.query([variant_id])?;
    let mut attributes = Vec::new();
    while let Some(row) = rows.next()? {
        attributes.push(VariantAttribute {
            option: option_from_i32(row.get(0)?),
            value: row.get(1)?,
        });
    }
    Ok(attributes)
}

pub fn adjust_stock(
    connection: &Connection,
    variant_id: i64,
    adjustment: &AdjustStock,
) -> SqlResult<Option<ProductVariant>> {
    adjustment.validate().map_err(invalid_input)?;

    let movement_type = if adjustment.quantity_delta > 0 {
        MovementType::Restock
    } else {
        MovementType::Adjustment
    };

    apply_stock_change(
        connection,
        variant_id,
        adjustment.quantity_delta,
        &adjustment.note,
        movement_type,
        None,
    )?;

    get_variant(connection, variant_id)
}

pub fn record_merch_sale(connection: &Connection, sale: &MerchSale) -> SqlResult<MerchSaleResult> {
    sale.validate().map_err(invalid_input)?;

    let variant = get_variant(connection, sale.variant_id)?
        .ok_or_else(|| invalid_input("variante introuvable"))?;
    let product = get_product(connection, variant.product_id)?
        .ok_or_else(|| invalid_input("produit introuvable"))?;

    let unit_price = sale.unit_price.unwrap_or(variant.unit_price);
    let total = unit_price * sale.quantity as f32;
    let label = variant_label(&variant.attributes);
    let transaction_name = format!("Vente merch — {} ({label})", product.product.name);

    let new_transaction = NewTransaction {
        name: transaction_name,
        company: sale.company.clone(),
        transaction_type: TransactionType::Income,
        executed: sale.executed,
        date: sale.date.clone(),
        price_full_tax: total,
        tag: "Merch".into(),
        tax_amount: sale.tax_amount,
        invoice_path: sale.invoice_path.clone(),
    };

    let tx = connection.unchecked_transaction()?;
    let transaction_id = insert_transaction(&tx, &new_transaction)?;
    apply_stock_change_tx(
        &tx,
        sale.variant_id,
        -sale.quantity,
        &sale.note,
        MovementType::Sale,
        Some(transaction_id),
    )?;
    tx.commit()?;

    let transaction = get_transaction(connection, transaction_id)?
        .ok_or_else(|| invalid_input("transaction introuvable"))?;
    let variant = get_variant(connection, sale.variant_id)?
        .ok_or_else(|| invalid_input("variante introuvable"))?;
    let movement = get_latest_movement_for_variant(connection, sale.variant_id)?
        .ok_or_else(|| invalid_input("mouvement introuvable"))?;

    Ok(MerchSaleResult {
        transaction,
        variant,
        movement,
    })
}

pub fn list_stock_movements(
    connection: &Connection,
    filter: &StockMovementFilter,
) -> SqlResult<Vec<StockMovementDetail>> {
    let limit = filter.limit.unwrap_or(50).min(200) as i64;
    let mut sql = String::from(
        "SELECT m.id, m.variant_id, v.product_id, p.name, m.quantity_delta, m.note,
                m.created_at, m.movement_type, m.transaction_id, v.sku
         FROM stock_movements m
         JOIN product_variants v ON v.id = m.variant_id
         JOIN products p ON p.id = v.product_id
         WHERE 1=1",
    );

    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(variant_id) = filter.variant_id {
        sql.push_str(" AND m.variant_id = ?");
        params.push(Box::new(variant_id));
    }
    if let Some(product_id) = filter.product_id {
        sql.push_str(" AND v.product_id = ?");
        params.push(Box::new(product_id));
    }
    sql.push_str(" ORDER BY m.created_at DESC, m.id DESC LIMIT ?");
    params.push(Box::new(limit));

    let mut statement = connection.prepare(&sql)?;
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let mut rows = statement.query(param_refs.as_slice())?;
    let mut movements = Vec::new();
    while let Some(row) = rows.next()? {
        let variant_id: i64 = row.get(1)?;
        let attributes = list_attributes_for_variant(connection, variant_id)?;
        movements.push(StockMovementDetail {
            id: row.get(0)?,
            variant_id,
            product_id: row.get(2)?,
            product_name: row.get(3)?,
            variant_label: variant_label(&attributes),
            sku: row.get(9)?,
            quantity_delta: row.get(4)?,
            note: row.get(5)?,
            created_at: row.get(6)?,
            movement_type: movement_type_from_str(&row.get::<_, String>(7)?),
            transaction_id: row.get(8)?,
        });
    }
    Ok(movements)
}

pub fn list_low_stock_alerts(
    connection: &Connection,
    filter: &LowStockFilter,
) -> SqlResult<Vec<LowStockAlert>> {
    let sql = if filter.threshold.is_some() {
        "SELECT v.id, v.product_id, p.name, v.sku, v.stock_quantity, v.low_stock_threshold, v.unit_price
         FROM product_variants v
         JOIN products p ON p.id = v.product_id
         WHERE v.stock_quantity <= ?1
         ORDER BY v.stock_quantity ASC, p.name ASC"
    } else {
        "SELECT v.id, v.product_id, p.name, v.sku, v.stock_quantity, v.low_stock_threshold, v.unit_price
         FROM product_variants v
         JOIN products p ON p.id = v.product_id
         WHERE v.stock_quantity <= v.low_stock_threshold
         ORDER BY v.stock_quantity ASC, p.name ASC"
    };

    let mut statement = connection.prepare(sql)?;
    let mut rows = if let Some(threshold) = filter.threshold {
        statement.query([threshold])?
    } else {
        statement.query([])?
    };
    let mut alerts = Vec::new();
    while let Some(row) = rows.next()? {
        let variant_id: i64 = row.get(0)?;
        let attributes = list_attributes_for_variant(connection, variant_id)?;
        alerts.push(LowStockAlert {
            variant_id,
            product_id: row.get(1)?,
            product_name: row.get(2)?,
            variant_label: variant_label(&attributes),
            sku: row.get(3)?,
            stock_quantity: row.get(4)?,
            threshold: row.get(5)?,
            unit_price: row.get(6)?,
        });
    }
    Ok(alerts)
}

fn apply_stock_change(
    connection: &Connection,
    variant_id: i64,
    quantity_delta: i32,
    note: &str,
    movement_type: MovementType,
    transaction_id: Option<i64>,
) -> SqlResult<()> {
    let tx = connection.unchecked_transaction()?;
    apply_stock_change_tx(&tx, variant_id, quantity_delta, note, movement_type, transaction_id)?;
    tx.commit()
}

fn apply_stock_change_tx(
    tx: &rusqlite::Transaction<'_>,
    variant_id: i64,
    quantity_delta: i32,
    note: &str,
    movement_type: MovementType,
    transaction_id: Option<i64>,
) -> SqlResult<()> {
    let current: i32 = tx.query_row(
        "SELECT stock_quantity FROM product_variants WHERE id = ?1",
        [variant_id],
        |row| row.get(0),
    )?;
    let new_stock = current + quantity_delta;
    if new_stock < 0 {
        return Err(invalid_input("stock insuffisant"));
    }

    tx.execute(
        "UPDATE product_variants SET stock_quantity = ?1 WHERE id = ?2",
        (new_stock, variant_id),
    )?;

    tx.execute(
        "INSERT INTO stock_movements (variant_id, quantity_delta, note, created_at, movement_type, transaction_id)
         VALUES (?1, ?2, ?3, datetime('now'), ?4, ?5)",
        (
            variant_id,
            quantity_delta,
            note,
            movement_type_to_str(movement_type),
            transaction_id,
        ),
    )?;
    Ok(())
}

fn get_latest_movement_for_variant(
    connection: &Connection,
    variant_id: i64,
) -> SqlResult<Option<StockMovement>> {
    let mut statement = connection.prepare(
        "SELECT id, variant_id, quantity_delta, note, created_at, movement_type, transaction_id
         FROM stock_movements
         WHERE variant_id = ?1
         ORDER BY id DESC
         LIMIT 1",
    )?;
    let mut rows = statement.query([variant_id])?;
    if let Some(row) = rows.next()? {
        Ok(Some(row_to_movement(row)?))
    } else {
        Ok(None)
    }
}

fn row_to_movement(row: &rusqlite::Row<'_>) -> SqlResult<StockMovement> {
    Ok(StockMovement {
        id: row.get(0)?,
        variant_id: row.get(1)?,
        quantity_delta: row.get(2)?,
        note: row.get(3)?,
        created_at: row.get(4)?,
        movement_type: movement_type_from_str(&row.get::<_, String>(5)?),
        transaction_id: row.get(6)?,
    })
}

fn movement_type_to_str(kind: MovementType) -> &'static str {
    match kind {
        MovementType::Sale => "SALE",
        MovementType::Adjustment => "ADJUSTMENT",
        MovementType::Restock => "RESTOCK",
    }
}

fn movement_type_from_str(value: &str) -> MovementType {
    match value {
        "SALE" => MovementType::Sale,
        "RESTOCK" => MovementType::Restock,
        _ => MovementType::Adjustment,
    }
}

pub fn get_variant(connection: &Connection, variant_id: i64) -> SqlResult<Option<ProductVariant>> {
    let mut statement = connection.prepare(
        "SELECT id, product_id, sku, stock_quantity, unit_price
         FROM product_variants WHERE id = ?1",
    )?;
    let mut rows = from_rows::<ProductVariantRow>(statement.query([variant_id])?);
    let Some(row) = rows.next().transpose().map_err(map_serde_err)? else {
        return Ok(None);
    };
    let attributes = list_attributes_for_variant(connection, variant_id)?;
    Ok(Some(ProductVariant {
        id: row.id,
        product_id: row.product_id,
        sku: row.sku,
        stock_quantity: row.stock_quantity,
        unit_price: row.unit_price,
        attributes,
    }))
}

pub fn get_product_kind(connection: &Connection, product_id: i64) -> SqlResult<Option<ProductKind>> {
    connection
        .query_row("SELECT kind FROM products WHERE id = ?1", [product_id], |row| {
            kind_from_row(row, 0)
        })
        .optional()
}

fn kind_from_row(row: &rusqlite::Row<'_>, idx: usize) -> SqlResult<ProductKind> {
    let value: String = row.get(idx)?;
    Ok(parse_kind_str(&value))
}

fn parse_kind_str(value: &str) -> ProductKind {
    match value {
        "TSHIRT" => ProductKind::Tshirt,
        "PATCH" => ProductKind::Patch,
        "PIN" => ProductKind::Pin,
        "PRINT" => ProductKind::Print,
        "ALBUM" => ProductKind::Album,
        _ => {
            if let Ok(raw) = value.parse::<i32>() {
                kind_from_i32(raw)
            } else {
                ProductKind::Pin
            }
        }
    }
}

fn kind_from_i32(raw: i32) -> ProductKind {
    match raw {
        0 => ProductKind::Tshirt,
        1 => ProductKind::Patch,
        2 => ProductKind::Pin,
        3 => ProductKind::Print,
        4 => ProductKind::Album,
        _ => ProductKind::Pin,
    }
}

fn option_from_i32(raw: i32) -> VariantOption {
    match raw {
        0 => VariantOption::Color,
        1 => VariantOption::Size,
        2 => VariantOption::Format,
        _ => VariantOption::VinylColor,
    }
}

fn invalid_input(message: impl Into<String>) -> rusqlite::Error {
    rusqlite::Error::ToSqlConversionFailure(Box::new(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        message.into(),
    )))
}

#[derive(Debug, serde::Deserialize)]
struct ProductVariantRow {
    id: i64,
    product_id: i64,
    sku: String,
    stock_quantity: i32,
    unit_price: f32,
}
