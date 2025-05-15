#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
mod db;

use core::fmt;
use std::sync::Mutex;

use actix_web::{
    body::MessageBody,
    error::{self, InternalError},
    http::StatusCode,
};

use db::{get_connection_pool, models::Transaction, schema::transactions::id, DbPool};

use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use diesel::expression::is_aggregate::No;

struct AppData {
    last_id: i32,
}

impl AppData {
    fn new() -> Self {
        AppData { last_id: 0 }
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/transactions")]
async fn get_transactions(
    app_data: web::Data<Mutex<AppData>>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    // TODO handle error better

    let transactions = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        Transaction::list(&mut conn)
    })
    .await?;

    let last_id = match transactions.iter().max_by(|a,b| a.id.cmp(&b.id)) {
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

    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(e.to_string())),
    };

    tra.id = match app_data.lock() {
        Ok(c) => c.last_id + 1,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(e.to_string())),
    };

    let next_id = tra.id;
    let disp_tra = tra.clone();
    Transaction::insert(tra, &mut conn);

    match app_data.lock() {
        Ok(mut c) => c.last_id = next_id,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(e.to_string())),
    };

    Ok(HttpResponse::Ok().body(format!(
        "Tra {:?} {:?} {:?}!",
        disp_tra.name, disp_tra.company, disp_tra.price_full_tax,
    )))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = get_connection_pool("transactions.db");
    let mut conn = pool.clone().get().expect("couldn't get db connection from pool");

    let transactions = Transaction::list(&mut conn);

    let last_id = match transactions.iter().max_by(|a,b| a.id.cmp(&b.id)) {
        Some(m) => m.id,
        None => 0,
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(Mutex::new(AppData{last_id: last_id + 1})))
            .service(get_transactions)
            .service(post_transactions)
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
