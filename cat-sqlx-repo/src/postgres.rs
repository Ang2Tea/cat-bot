mod chat_repository;

use sqlx::{Pool, Postgres};


#[derive(Debug, Clone)]
pub struct PostgresRepository {
    db: Pool<Postgres>,
}

impl PostgresRepository {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }
}
