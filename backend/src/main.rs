#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
mod db;

use thiserror::Error;

use diesel::{Connection, SqliteConnection};
use dotenv::dotenv;
use env::VarError;

use actix_cors::Cors;
use r2d2::PooledConnection;
use serde::de::Error;

use core::fmt;
use std::sync::Mutex;

use actix_web::{
    body::MessageBody,
    error::{self, InternalError},
    http::{StatusCode, header::LastModified},
};

use db::{
    DbPool, MIGRATIONS, get_connection_pool,
    models::Transaction,
    schema::transactions::{self, id},
};

#[derive(Error, Debug)]
enum BandComptaError {
    #[error("Failed to load env")]
    EnvError(#[from] VarError),

    #[error("Failed communicate with db")]
    DataBaseError(#[from] r2d2::Error),

    #[error("unknown error")]
    Other(String),
}

use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::{MigrationHarness, embed_migrations};

use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, post, web};
struct AppData {
    last_id: i32,
}

impl AppData {
    fn new() -> Self {
        AppData { last_id: 0 }
    }
}

#[delete("/transactions")]
async fn delete_transaction(
    pool: web::Data<DbPool>,
    json: web::Json<Transaction>,
) -> actix_web::Result<impl Responder> {
    let tra = json.0;

    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(e.to_string())),
    };

    let disp_tra = tra.clone();

    match Transaction::remove_by_id(&tra.id, &mut conn) {
        Some(_) => Ok(HttpResponse::Ok().body(format!(
            "Deleted transaction: {:?} {:?} {:?}!",
            disp_tra.name, disp_tra.company, disp_tra.price_full_tax,
        ))),

        None => Ok(HttpResponse::Ok().body(format!(
            "Transaction not in db: {:?} {:?} {:?}!",
            disp_tra.name, disp_tra.company, disp_tra.price_full_tax,
        ))),
    }
}

#[get("/transactions")]
async fn get_transactions(
    app_data: web::Data<Mutex<AppData>>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let transactions: Result<Vec<Transaction>, BandComptaError> = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.

        let mut conn = pool.get()?;

        Ok(Transaction::list(&mut conn))
    })
    .await?;

    let transactions = match transactions {
        Ok(t) => t,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(e.to_string())),
    };

    let last_id = match transactions.iter().max_by(|a, b| a.id.cmp(&b.id)) {
        Some(m) => m.id,
        None => 0,
    };

    match app_data.lock() {
        Ok(mut c) => c.last_id = last_id + 1,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(e.to_string())),
    };

    Ok(HttpResponse::Ok().json(transactions))
}

#[post("/transactions")]
async fn post_transactions(
    pool: web::Data<DbPool>,
    app_data: web::Data<Mutex<AppData>>,
    json: web::Json<Transaction>,
) -> actix_web::Result<impl Responder> {
    let mut tra = json.0;
    tra.id = match app_data.lock() {
        Ok(c) => c.last_id + 1,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(e.to_string())),
    };

    let next_id = tra.id;
    let disp_tra = tra.clone();

    let r: Result<(), BandComptaError> = web::block(move || {
        let mut conn = pool.get()?;
        Transaction::insert(tra, &mut conn);
        Ok(())
    })
    .await?;

    if let Some(e) = r.err() {
        return Ok(HttpResponse::InternalServerError().body(e.to_string()));
    }

    match app_data.lock() {
        Ok(mut c) => c.last_id = next_id,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(e.to_string())),
    };

    Ok(HttpResponse::Ok().body(format!(
        "Adding Transanction: {:?} {:?} {:?}",
        disp_tra.name, disp_tra.company, disp_tra.price_full_tax,
    )))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = get_connection_pool("transactions.db");
    let mut conn = pool
        .clone()
        .get()
        .expect("couldn't get db connection from pool");

    for m in conn.pending_migrations(MIGRATIONS).unwrap() {
        let _ = m.run(&mut conn);
    }

    let transactions = Transaction::list(&mut conn);

    let last_id = match transactions.iter().max_by(|a, b| a.id.cmp(&b.id)) {
        Some(m) => m.id,
        None => 0,
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(Mutex::new(AppData { last_id: last_id })))
            .service(get_transactions)
            .service(post_transactions)
            .service(delete_transaction)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
