use std::path::Path;

use sqlx::{
    Database, Pool,
    migrate::{Migrate, MigrateDatabase, Migrator},
};

#[cfg(feature = "postgres")]
#[path = "postgres_chat_repository.rs"]
mod chat_repository;

#[cfg(feature = "sqlite")]
#[path = "sqlite_chat_repository.rs"]
mod chat_repository;

pub use chat_repository::*;

async fn inner_init_db<DB>(db_urn: &str) -> Result<Pool<DB>, String>
where
    DB: Database + MigrateDatabase,
    <DB as sqlx::Database>::Connection: Migrate,
{
    if !DB::database_exists(db_urn).await.unwrap_or(false) {
        if let Err(error) = DB::create_database(db_urn).await {
            return Err(error.to_string());
        }
    }

    let db = Pool::<DB>::connect(db_urn).await.unwrap();

    let migration_results = Migrator::new(Path::new("./migrations")).await
        .map_err(|e| e.to_string())?
        .run(&db).await;

    match migration_results {
        Ok(_) => Ok(db),
        Err(error) => Err(error.to_string()),
    }
}
