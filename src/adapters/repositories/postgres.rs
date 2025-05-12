mod chat_repository;

pub use chat_repository::*;

use sqlx::{Pool, Postgres};

use super::sqlx_helper::inner_init_db;

pub async fn init_db(db_urn: &str) -> std::result::Result<Pool<Postgres>, String> {
    inner_init_db::<Postgres>(db_urn, Some("./migrations/postgres")).await
}
