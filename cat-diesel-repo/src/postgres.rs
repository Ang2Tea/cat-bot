mod models;
mod schema;

pub mod chat_repository;

use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/postgres");

pub fn run_migrations(db_url: &str) {
    use diesel::Connection;

    let mut connection = diesel::pg::PgConnection::establish(db_url).unwrap();
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Error running migrations");
}

#[derive(Clone)]
pub struct PostgresRepository {
    pool: Pool<AsyncPgConnection>,
}

impl PostgresRepository {
    pub fn new(db_url: String) -> Self {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
        let pool = Pool::builder(config).build().unwrap();

        Self { pool }
    }
}
