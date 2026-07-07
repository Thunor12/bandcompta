use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use bandcompta_shared::{paths, DateFilter, Transaction, TreasurySummary};
use rusqlite::Connection;
use tower_http::cors::{Any, CorsLayer};

use crate::db::{get_transaction, list_transactions, treasury_summary};

type DbState = Arc<Mutex<Connection>>;

pub fn app(connection: Connection) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route(paths::TRANSACTIONS, get(list_handler))
        .route(paths::TRANSACTIONS_BY_ID, get(get_handler))
        .route(paths::SUMMARY, get(summary_handler))
        .layer(cors)
        .with_state(Arc::new(Mutex::new(connection)))
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

enum AppError {
    Database(rusqlite::Error),
    NotFound,
    BadRequest(String),
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Database(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
            Self::NotFound => (StatusCode::NOT_FOUND, "transaction not found").into_response(),
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message).into_response(),
            Self::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response(),
        }
    }
}
