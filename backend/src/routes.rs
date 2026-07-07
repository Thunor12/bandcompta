use std::sync::{Arc, Mutex};

use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use bandcompta_shared::{paths, ContactFilter, DateFilter, NewContact, NewTag, NewTransaction, Transaction, TreasurySummary, UploadResponse};
use rusqlite::Connection;
use tower_http::cors::{Any, CorsLayer};

use crate::contacts::{get_contact, insert_contact, list_contacts};
use crate::db::{get_transaction, insert_transaction, list_transactions, treasury_summary};
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
