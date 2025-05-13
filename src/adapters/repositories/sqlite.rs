pub mod chat_repository;

pub use chat_repository::*;

use sqlx::{Pool, Sqlite};

use super::sqlx_helper::inner_init_db;

pub async fn init_db(db_urn: &str) -> std::result::Result<Pool<Sqlite>, String> {
    inner_init_db::<Sqlite>(db_urn, Some("./migrations/sqlite")).await
}
