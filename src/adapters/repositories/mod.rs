use sqlx::{Pool, Sqlite, SqlitePool, migrate::MigrateDatabase};

pub mod sqlite_chat_repository;

pub async fn init_db(db_urn: &str) -> Result<Pool<Sqlite>, String> {
    if !Sqlite::database_exists(db_urn).await.unwrap_or(false) {
        if let Err(error) = Sqlite::create_database(db_urn).await {
            return Err(error.to_string());
        }
    }

    let db = SqlitePool::connect(db_urn).await.unwrap();

    let migration_results = sqlx::migrate!().run(&db).await;

    match migration_results {
        Ok(_) => Ok(db),
        Err(error) => Err(error.to_string()),
    }
}
