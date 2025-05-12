pub mod chat_repository;

pub use chat_repository::*;

use sqlx::{Pool, Sqlite};

pub async fn init_db(db_urn: &str) -> std::result::Result<Pool<Sqlite>, String> {
    super::inner_init_db::<Sqlite>(db_urn, Some("./migrations/sqlite")).await
}
