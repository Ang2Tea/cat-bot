use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

pub async fn init_db(db_urn: &str) {
    if !Sqlite::database_exists(db_urn).await.unwrap_or(false) {
        match Sqlite::create_database(db_urn).await {
            Ok(_) => log::info!("Create db success {}", db_urn),
            Err(error) => log::error!("error: {}", error),
        }
    } else {
        log::debug!("Database already exists");
    }

    let db = SqlitePool::connect(db_urn).await.unwrap();

    let migration_results = sqlx::migrate!()
        .run(&db)
        .await;

    match migration_results {
        Ok(_) => log::debug!("Migration success"),
        Err(error) => {
            log::error!("error: {}", error);
        }
    }
}