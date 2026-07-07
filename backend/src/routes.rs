use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use rusqlite::Connection;
use tower_http::cors::{Any, CorsLayer};

use crate::db::{get_transaction, list_transactions, treasury_summary};
use crate::models::{Transaction, TreasurySummary};

type DbState = Arc<Mutex<Connection>>;

pub fn app(connection: Connection) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/transactions", get(list_handler))
        .route("/api/transactions/{id}", get(get_handler))
        .route("/api/summary", get(summary_handler))
        .layer(cors)
        .with_state(Arc::new(Mutex::new(connection)))
}

async fn list_handler(State(db): State<DbState>) -> Result<Json<Vec<Transaction>>, AppError> {
    let connection = db.lock().map_err(|_| AppError::Internal)?;
    let transactions = list_transactions(&connection).map_err(AppError::Database)?;
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

async fn summary_handler(State(db): State<DbState>) -> Result<Json<TreasurySummary>, AppError> {
    let connection = db.lock().map_err(|_| AppError::Internal)?;
    let transactions = list_transactions(&connection).map_err(AppError::Database)?;
    Ok(Json(treasury_summary(&transactions)))
}

enum AppError {
    Database(rusqlite::Error),
    NotFound,
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Database(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
            Self::NotFound => (StatusCode::NOT_FOUND, "transaction not found").into_response(),
            Self::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response(),
        }
    }
}
