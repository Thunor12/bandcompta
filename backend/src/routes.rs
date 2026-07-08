use std::sync::{Arc, Mutex};

use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch, post},
    Json, Router,
};
use bandcompta_shared::{
    paths, AdjustStock, ContactFilter, DateFilter, LowStockFilter, MerchSale, MerchSaleResult,
    NewContact, NewProduct, NewProductVariant, NewTag, NewTransaction, ProductDetail,
    ProductSummary, ProductVariant, StockMovementDetail, StockMovementFilter, Transaction,
    TreasurySummary, UploadResponse,
};
use rusqlite::Connection;
use tower_http::cors::{Any, CorsLayer};

use crate::contacts::{get_contact, insert_contact, list_contacts};
use crate::db::{get_transaction, insert_transaction, list_transactions, treasury_summary};
use crate::inventory::{
    adjust_stock, get_product, get_product_kind, get_variant, insert_product, insert_variant,
    list_low_stock_alerts, list_products, list_stock_movements, record_merch_sale,
};
use crate::invoices::save_invoice;
use crate::tags::{get_tag, insert_tag, list_tags};

type DbState = Arc<Mutex<Connection>>;

pub fn app(connection: Connection) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route(paths::TRANSACTIONS, get(list_handler).post(create_handler))
        .route(paths::TRANSACTIONS_BY_ID, get(get_handler))
        .route(paths::SUMMARY, get(summary_handler))
        .route(paths::INVOICES_UPLOAD, post(upload_handler))
        .route(paths::CONTACTS, get(list_contacts_handler).post(create_contact_handler))
        .route(paths::CONTACTS_BY_ID, get(get_contact_handler))
        .route(paths::TAGS, get(list_tags_handler).post(create_tag_handler))
        .route(paths::PRODUCTS, get(list_products_handler).post(create_product_handler))
        .route(paths::PRODUCTS_BY_ID, get(get_product_handler))
        .route(paths::PRODUCT_VARIANTS, post(create_variant_handler))
        .route(paths::VARIANT_STOCK, patch(adjust_stock_handler))
        .route(paths::STOCK_MOVEMENTS, get(list_movements_handler))
        .route(paths::STOCK_ALERTS, get(list_alerts_handler))
        .route(paths::MERCH_SALES, post(merch_sale_handler))
        .layer(cors)
        .with_state(Arc::new(Mutex::new(connection)))
}

async fn list_tags_handler(State(db): State<DbState>) -> Result<Json<Vec<bandcompta_shared::Tag>>, AppError> {
    let connection = db.lock().map_err(|_| AppError::Internal)?;
    let tags = list_tags(&connection).map_err(AppError::Database)?;
    Ok(Json(tags))
}

async fn create_tag_handler(
    State(db): State<DbState>,
    Json(body): Json<NewTag>,
) -> Result<(StatusCode, Json<bandcompta_shared::Tag>), AppError> {
    body.validate().map_err(AppError::BadRequest)?;
    let connection = db.lock().map_err(|_| AppError::Internal)?;
    let id = insert_tag(&connection, &body).map_err(AppError::Database)?;
    let tag = get_tag(&connection, id)
        .map_err(AppError::Database)?
        .ok_or(AppError::Internal)?;
    Ok((StatusCode::CREATED, Json(tag)))
}

async fn list_contacts_handler(
    State(db): State<DbState>,
    Query(filter): Query<ContactFilter>,
) -> Result<Json<Vec<bandcompta_shared::Contact>>, AppError> {
    let connection = db.lock().map_err(|_| AppError::Internal)?;
    let contacts = list_contacts(&connection, &filter).map_err(AppError::Database)?;
    Ok(Json(contacts))
}

async fn get_contact_handler(
    State(db): State<DbState>,
    Path(id): Path<i64>,
) -> Result<Json<bandcompta_shared::Contact>, AppError> {
    let connection = db.lock().map_err(|_| AppError::Internal)?;
    let contact = get_contact(&connection, id)
        .map_err(AppError::Database)?
        .ok_or(AppError::NotFound)?;
    Ok(Json(contact))
}

async fn create_contact_handler(
    State(db): State<DbState>,
    Json(body): Json<NewContact>,
) -> Result<(StatusCode, Json<bandcompta_shared::Contact>), AppError> {
    body.validate().map_err(AppError::BadRequest)?;
    let connection = db.lock().map_err(|_| AppError::Internal)?;
    let id = insert_contact(&connection, &body).map_err(AppError::Database)?;
    let contact = get_contact(&connection, id)
        .map_err(AppError::Database)?
        .ok_or(AppError::Internal)?;
    Ok((StatusCode::CREATED, Json(contact)))
}

async fn list_handler(
    State(db): State<DbState>,
    Query(filter): Query<DateFilter>,
) -> Result<Json<Vec<Transaction>>, AppError> {
    filter.validate().map_err(AppError::BadRequest)?;
    let connection = db.lock().map_err(|_| AppError::Internal)?;
    let transactions = list_transactions(&connection, &filter).map_err(AppError::Database)?;
    Ok(Json(transactions))
}

async fn create_handler(
    State(db): State<DbState>,
    Json(body): Json<NewTransaction>,
) -> Result<(StatusCode, Json<Transaction>), AppError> {
    body.validate().map_err(AppError::BadRequest)?;
    let connection = db.lock().map_err(|_| AppError::Internal)?;
    let id = insert_transaction(&connection, &body).map_err(AppError::Database)?;
    let transaction = get_transaction(&connection, id)
        .map_err(AppError::Database)?
        .ok_or(AppError::Internal)?;
    Ok((StatusCode::CREATED, Json(transaction)))
}

async fn get_handler(
    State(db): State<DbState>,
    Path(id): Path<i64>,
) -> Result<Json<Transaction>, AppError> {
    let connection = db.lock().map_err(|_| AppError::Internal)?;
    let transaction = get_transaction(&connection, id)
        .map_err(AppError::Database)?
        .ok_or(AppError::NotFound)?;
    Ok(Json(transaction))
}

async fn summary_handler(
    State(db): State<DbState>,
    Query(filter): Query<DateFilter>,
) -> Result<Json<TreasurySummary>, AppError> {
    filter.validate().map_err(AppError::BadRequest)?;
    let connection = db.lock().map_err(|_| AppError::Internal)?;
    let transactions = list_transactions(&connection, &filter).map_err(AppError::Database)?;
    Ok(Json(treasury_summary(&transactions)))
}

async fn upload_handler(mut multipart: Multipart) -> Result<Json<UploadResponse>, AppError> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|err| AppError::BadRequest(err.to_string()))?
    {
        if field.name() != Some("file") {
            continue;
        }

        let filename = field
            .file_name()
            .map(str::to_string)
            .unwrap_or_else(|| "upload.bin".to_string());
        let data = field
            .bytes()
            .await
            .map_err(|err| AppError::BadRequest(err.to_string()))?;

        let response = save_invoice(&filename, &data)
            .await
            .map_err(AppError::BadRequest)?;
        return Ok(Json(response));
    }

    Err(AppError::BadRequest("fichier manquant".into()))
}

async fn list_products_handler(
    State(db): State<DbState>,
) -> Result<Json<Vec<ProductSummary>>, AppError> {
    let connection = db.lock().map_err(|_| AppError::Internal)?;
    let products = list_products(&connection).map_err(AppError::Database)?;
    Ok(Json(products))
}

async fn get_product_handler(
    State(db): State<DbState>,
    Path(id): Path<i64>,
) -> Result<Json<ProductDetail>, AppError> {
    let connection = db.lock().map_err(|_| AppError::Internal)?;
    let product = get_product(&connection, id)
        .map_err(AppError::Database)?
        .ok_or(AppError::NotFound)?;
    Ok(Json(product))
}

async fn create_product_handler(
    State(db): State<DbState>,
    Json(body): Json<NewProduct>,
) -> Result<(StatusCode, Json<ProductDetail>), AppError> {
    body.validate().map_err(AppError::BadRequest)?;
    let connection = db.lock().map_err(|_| AppError::Internal)?;
    let id = insert_product(&connection, &body).map_err(AppError::Database)?;
    let product = get_product(&connection, id)
        .map_err(AppError::Database)?
        .ok_or(AppError::Internal)?;
    Ok((StatusCode::CREATED, Json(product)))
}

async fn create_variant_handler(
    State(db): State<DbState>,
    Path(product_id): Path<i64>,
    Json(body): Json<NewProductVariant>,
) -> Result<(StatusCode, Json<ProductVariant>), AppError> {
    let connection = db.lock().map_err(|_| AppError::Internal)?;
    let kind = get_product_kind(&connection, product_id)
        .map_err(AppError::Database)?
        .ok_or(AppError::NotFound)?;
    let variant_id = insert_variant(&connection, product_id, kind, &body).map_err(AppError::Database)?;
    let variant = get_variant(&connection, variant_id)
        .map_err(AppError::Database)?
        .ok_or(AppError::Internal)?;
    Ok((StatusCode::CREATED, Json(variant)))
}

async fn adjust_stock_handler(
    State(db): State<DbState>,
    Path(variant_id): Path<i64>,
    Json(body): Json<AdjustStock>,
) -> Result<Json<ProductVariant>, AppError> {
    let connection = db.lock().map_err(|_| AppError::Internal)?;
    let variant = adjust_stock(&connection, variant_id, &body)
        .map_err(AppError::Database)?
        .ok_or(AppError::NotFound)?;
    Ok(Json(variant))
}

async fn list_movements_handler(
    State(db): State<DbState>,
    Query(filter): Query<StockMovementFilter>,
) -> Result<Json<Vec<StockMovementDetail>>, AppError> {
    let connection = db.lock().map_err(|_| AppError::Internal)?;
    let movements = list_stock_movements(&connection, &filter).map_err(AppError::Database)?;
    Ok(Json(movements))
}

async fn list_alerts_handler(
    State(db): State<DbState>,
    Query(filter): Query<LowStockFilter>,
) -> Result<Json<Vec<bandcompta_shared::LowStockAlert>>, AppError> {
    let connection = db.lock().map_err(|_| AppError::Internal)?;
    let alerts = list_low_stock_alerts(&connection, &filter).map_err(AppError::Database)?;
    Ok(Json(alerts))
}

async fn merch_sale_handler(
    State(db): State<DbState>,
    Json(body): Json<MerchSale>,
) -> Result<(StatusCode, Json<MerchSaleResult>), AppError> {
    body.validate().map_err(AppError::BadRequest)?;
    let connection = db.lock().map_err(|_| AppError::Internal)?;
    let result = record_merch_sale(&connection, &body).map_err(AppError::Database)?;
    Ok((StatusCode::CREATED, Json(result)))
}

enum AppError {
    Database(rusqlite::Error),
    NotFound,
    BadRequest(String),
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Database(err) => {
                if let rusqlite::Error::ToSqlConversionFailure(source) = &err {
                    if source
                        .downcast_ref::<std::io::Error>()
                        .is_some_and(|io_err| io_err.kind() == std::io::ErrorKind::InvalidInput)
                    {
                        return (StatusCode::BAD_REQUEST, source.to_string()).into_response();
                    }
                }
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
            }
            Self::NotFound => (StatusCode::NOT_FOUND, "not found").into_response(),
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message).into_response(),
            Self::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response(),
        }
    }
}
