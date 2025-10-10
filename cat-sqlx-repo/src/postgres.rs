mod chat_repository;

use sqlx::PgPool;

pub async fn migrate(db_url: &str) -> Result<(), String> {
    let conn = PgPool::connect(db_url).await.map_err(|e| e.to_string())?;

    sqlx::migrate!("migrations/postgres")
        .run(&conn)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[derive(Debug, Clone)]
pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub async fn try_new(db_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(db_url).await?;

        Ok(Self { pool })
    }
}
