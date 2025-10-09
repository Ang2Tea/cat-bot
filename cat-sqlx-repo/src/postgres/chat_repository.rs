use std::vec;

use cat_core::{
    entities::{chat::Chat, repositories::ChatRepository},
    shared::{CreateChatError, GetChatError, UpdateChatError},
};

use crate::postgres::PostgresRepository;

impl ChatRepository for PostgresRepository {
    async fn create(&self, input: Chat) -> Result<(), CreateChatError> {
        let _ = sqlx::query_file!("queries/postgres/chat_insert.sql", input.chat_id, input.name, input.title)
            .execute(&self.pool)
            .await
            .map_err(crate::create_errors)?;

        Ok(())
    }

    async fn get_list(&self) -> Result<Vec<Chat>, GetChatError> {
        let rows = sqlx::query_file!("queries/postgres/chat_select_all.sql")
            .fetch_all(&self.pool)
            .await
            .map_err(crate::get_errors)?;

        // Ok(rows.into_iter().filter_map(map_pg_row).collect())
        Ok(vec![])
    }

    async fn get_by_id(&self, id: i64) -> Result<Chat, GetChatError> {
        let row = sqlx::query_file!("queries/postgres/chat_select_by_id.sql", id)
            .fetch_one(&self.pool)
            .await
            .map_err(crate::get_errors)?;

        Ok(Chat{chat_id: 0, name: None, title: None, enable_push: false})

        // map_pg_row(row).ok_or(GetChatError::NotFound)
    }

    async fn update(&self, input: Chat) -> Result<(), UpdateChatError> {
        let _ = sqlx::query_file!("queries/postgres/chat_update.sql", input.name, input.enable_push, input.title, input.chat_id)
        .execute(&self.pool)
        .await
        .map_err(crate::update_errors)?;

        Ok(())
    }

    async fn get_list_for_push(&self) -> Result<Vec<Chat>, GetChatError> {
        let rows = sqlx::query_file!("queries/postgres/chat_select_enable_push.sql")
            .fetch_all(&self.pool)
            .await
            .map_err(crate::get_errors)?;

            Ok(vec![])
        // Ok(rows.into_iter().filter_map(map_pg_row).collect())
    }
}
