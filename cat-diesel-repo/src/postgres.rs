mod models;
mod schema;

pub mod chat_repository;

use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

use cat_core::shared::RepositoryError;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/postgres");

pub fn run_migrations(db_url: &str) -> Result<(), RepositoryError> {
    use diesel::Connection;

    let mut connection =
        diesel::pg::PgConnection::establish(db_url).map_err(|_| RepositoryError::Connection)?;

    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|e| RepositoryError::Other(e.to_string()))?;

    Ok(())
}

#[derive(Clone)]
pub struct PostgresRepository {
    pool: Pool<AsyncPgConnection>,
}

impl PostgresRepository {
    pub fn try_new(db_url: String) -> Result<Self, RepositoryError> {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
        let pool = Pool::builder(config)
            .build()
            .map_err(|e| RepositoryError::Other(e.to_string()))?;

        Ok(Self { pool })
    }
}
