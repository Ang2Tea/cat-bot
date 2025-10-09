#[cfg(feature = "postgres")]
pub mod postgres;

use sqlx::{
    Database, Error, Pool,
    migrate::{Migrate, MigrateDatabase, Migrator},
};
use std::path::Path;

use cat_core::shared::{CreateChatError, GetChatError, UpdateChatError};

pub async fn init_db<DB>(db_urn: &str) -> std::result::Result<Pool<DB>, String>
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

    Ok(db)
}

pub async fn migrate<DB>(db: &Pool<DB>, migration_path: Option<&str>) -> Result<(), String>
where
    DB: Database + MigrateDatabase,
    <DB as sqlx::Database>::Connection: Migrate,
{
    let migration_path = migration_path.unwrap_or("./migrations");

    let migration_results = Migrator::new(Path::new(migration_path))
        .await
        .map_err(|e| e.to_string())?
        .run(db)
        .await;

    match migration_results {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

fn create_errors(e: Error) -> CreateChatError {
    match e {
        Error::Database(err) if err.constraint().is_some() => CreateChatError::Duplicate,
        _ => CreateChatError::Other(e.to_string()),
    }
}

fn get_errors(e: Error) -> GetChatError {
    match e {
        Error::RowNotFound => GetChatError::NotFound,
        _ => GetChatError::Other(e.to_string()),
    }
}

fn update_errors(e: Error) -> UpdateChatError {
    match e {
        Error::RowNotFound => UpdateChatError::GetChatError(GetChatError::NotFound),
        _ => UpdateChatError::Other(e.to_string()),
    }
}
