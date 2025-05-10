use crate::shared::{self, ErrorKind};
use sqlx::{
    Database, Pool,
    migrate::{Migrate, MigrateDatabase},
};
pub mod sqlite_chat_repository;
pub mod postgres_chat_repository;

pub async fn init_db<DB>(db_urn: &str) -> Result<Pool<DB>, String>
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

    let migration_results = sqlx::migrate!().run(&db).await;

    match migration_results {
        Ok(_) => Ok(db),
        Err(error) => Err(error.to_string()),
    }
}

fn map_result<T>(result: sqlx::Result<T>) -> shared::Result<T> {
    match result {
        Ok(r) => Ok(r),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Result::Err(ErrorKind::NotFound),
            _ => Result::Err(ErrorKind::Other(err.to_string())),
        },
    }
}
