use cat_core::{
    entities::{chat::Chat, repositories::ChatRepository},
    shared::RepositoryError,
};

use crate::{DbRepositoryError, postgres::PostgresRepository};

impl ChatRepository for PostgresRepository {
    async fn create(&self, input: Chat) -> Result<(), RepositoryError> {
        let _ = sqlx::query_file!(
            "queries/postgres/chat_insert.sql",
            input.chat_id,
            input.name,
            input.title
        )
        .execute(&self.pool)
        .await
        .map_err(DbRepositoryError::from)?;

        Ok(())
    }

    async fn get_list(&self) -> Result<Vec<Chat>, RepositoryError> {
        let rows = sqlx::query_file!("queries/postgres/chat_select_all.sql")
            .fetch_all(&self.pool)
            .await
            .map_err(DbRepositoryError::from)?;

        let result = rows
            .into_iter()
            .map(|r| Chat {
                chat_id: r.chat_id,
                name: r.name,
                title: r.title,
                enable_push: r.enable_push,
            })
            .collect();

        Ok(result)
    }

    async fn get_by_id(&self, id: i64) -> Result<Chat, RepositoryError> {
        let row = sqlx::query_file!("queries/postgres/chat_select_by_id.sql", id)
            .fetch_one(&self.pool)
            .await
            .map_err(DbRepositoryError::from)?;

        Ok(Chat {
            chat_id: row.chat_id,
            name: row.name,
            title: row.title,
            enable_push: row.enable_push,
        })
    }

    async fn update(&self, input: Chat) -> Result<(), RepositoryError> {
        let _ = sqlx::query_file!(
            "queries/postgres/chat_update.sql",
            input.name,
            input.enable_push,
            input.title,
            input.chat_id
        )
        .execute(&self.pool)
        .await
        .map_err(DbRepositoryError::from)?;

        Ok(())
    }

    async fn get_list_for_push(&self) -> Result<Vec<Chat>, RepositoryError> {
        let rows = sqlx::query_file!("queries/postgres/chat_select_enable_push.sql")
            .fetch_all(&self.pool)
            .await
            .map_err(DbRepositoryError::from)?;

        let result = rows
            .into_iter()
            .map(|r| Chat {
                chat_id: r.chat_id,
                name: r.name,
                title: r.title,
                enable_push: r.enable_push,
            })
            .collect();

        Ok(result)
    }
}
