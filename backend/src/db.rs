pub mod models;
pub mod schema;

use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

// https://diesel.rs/guides/migration_guide.html
// https://levelup.gitconnected.com/using-sqlite-with-rust-and-actix-web-with-tests-11a935ac3d95

use dotenv::dotenv;
use env;

use diesel::prelude::*;
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::{MigrationHarness, embed_migrations};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection() -> SqliteConnection {
    if cfg!(test) {
        let mut conn = SqliteConnection::establish(":memory:")
            .unwrap_or_else(|_| panic!("Error creating test database"));

        for m in conn.pending_migrations(MIGRATIONS).unwrap() {
            let _ = m.run(&mut conn);
        }

        conn
    } else {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}

pub fn get_connection_pool(url: &str) -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
