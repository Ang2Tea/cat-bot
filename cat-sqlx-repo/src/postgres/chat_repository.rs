use cat_core::{
    entities::{chat::Chat, repositories::ChatRepository},
    shared::{CreateChatError, GetChatError, UpdateChatError},
};
use sqlx::{Row as _, postgres::PgRow};

use crate::postgres::PostgresRepository;

fn map_pg_row(r: PgRow) -> Option<Chat> {
    let chat_id = r.try_get("chat_id").ok()?;
    let name = r.try_get("name").ok()?;
    let title = r.try_get("title").ok()?;
    let enable_push = r.try_get("enable_push").ok()?;

    Some(Chat {
        chat_id,
        name,
        title,
        enable_push,
    })
}

impl ChatRepository for PostgresRepository {
    async fn create(&self, input: Chat) -> Result<(), CreateChatError> {
        sqlx::query("INSERT INTO chats (chat_id, name, title) VALUES ($1, $2, $3);")
            .bind(input.chat_id)
            .bind(input.name)
            .bind(input.title)
            .execute(&self.pool)
            .await
            .map_err(crate::create_errors)?;

        Ok(())
    }

    async fn get_list(&self) -> Result<Vec<Chat>, GetChatError> {
        let rows = sqlx::query_file!("sql-scripts/postgres/chat_select_all.sql")
            .fetch_all(&self.pool)
            .await
            .map_err(crate::get_errors)?;

        let rows = sqlx::query("SELECT * FROM chats;")
            .fetch_all(&self.pool)
            .await
            .map_err(crate::get_errors)?;

        Ok(rows.into_iter().filter_map(map_pg_row).collect())
    }

    async fn get_by_id(&self, id: i64) -> Result<Chat, GetChatError> {
        let row = sqlx::query("SELECT * FROM chats WHERE chat_id = $1;")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(crate::get_errors)?;

        map_pg_row(row).ok_or(GetChatError::NotFound)
    }

    async fn update(&self, input: Chat) -> Result<(), UpdateChatError> {
        let _ = sqlx::query(
            "UPDATE chats SET name = $1, enable_push = $2, title = $3 WHERE chat_id = $4;",
        )
        .bind(input.name)
        .bind(input.enable_push)
        .bind(input.title)
        .bind(input.chat_id)
        .execute(&self.pool)
        .await
        .map_err(crate::update_errors)?;

        Ok(())
    }

    async fn get_list_for_push(&self) -> Result<Vec<Chat>, GetChatError> {
        let rows = sqlx::query("SELECT * FROM chats WHERE enable_push;")
            .fetch_all(&self.pool)
            .await
            .map_err(crate::get_errors)?;

        Ok(rows.into_iter().filter_map(map_pg_row).collect())
    }
}
