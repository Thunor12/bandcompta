mod db;
mod models;
mod routes;

use std::net::SocketAddr;

use tokio::net::TcpListener;

const DB_PATH: &str = "test.db";
const API_ADDR: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() {
    let connection = db::init_db(DB_PATH).expect("failed to initialize database");
    let app = routes::app(connection);

    let addr: SocketAddr = API_ADDR.parse().expect("invalid listen address");
    let listener = TcpListener::bind(addr).await.expect("failed to bind API server");

    println!("bandcompta API listening on http://{addr}");

    axum::serve(listener, app).await.expect("server error");
}
