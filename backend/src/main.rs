mod contacts;
mod db;
mod inventory;
mod invoices;
mod routes;
mod tags;

use std::net::SocketAddr;

use bandcompta_shared::DEFAULT_API_ADDR;
use tokio::net::TcpListener;

const DB_PATH: &str = "test.db";

#[tokio::main]
async fn main() {
    invoices::ensure_invoices_dir().expect("failed to create invoices directory");

    let connection = db::init_db(DB_PATH).expect("failed to initialize database");
    let app = routes::app(connection);

    let addr: SocketAddr = DEFAULT_API_ADDR.parse().expect("invalid listen address");
    let listener = TcpListener::bind(addr).await.expect("failed to bind API server");

    println!("bandcompta API listening on http://{addr}");

    axum::serve(listener, app).await.expect("server error");
}
