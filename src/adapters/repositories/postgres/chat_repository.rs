use crate::{
    adapters::repositories::sqlx_helper,
    entities::{chat::Chat, repositories::ChatRepository as IChatRepository},
    shared::{CreateChatError, GetChatError, UpdateChatError},
};
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct ChatRepository {
    pub db: Pool<Postgres>,
}

impl ChatRepository {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }
}

impl IChatRepository for ChatRepository {
    async fn create(&self, input: Chat) -> Result<(), CreateChatError> {
        sqlx::query("INSERT INTO chats (chat_id, name, title) VALUES ($1, $2, $3);")
            .bind(input.chat_id)
            .bind(input.name)
            .bind(input.title)
            .execute(&self.db)
            .await
            .map_err(sqlx_helper::create_errors)?;

        Ok(())
    }

    async fn get_list(&self) -> Result<Vec<Chat>, GetChatError> {
        sqlx::query_as::<_, Chat>("SELECT * FROM chats;")
            .fetch_all(&self.db)
            .await
            .map_err(sqlx_helper::get_errors)
    }

    async fn get_by_id(&self, id: i64) -> Result<Chat, GetChatError> {
        sqlx::query_as::<_, Chat>("SELECT * FROM chats WHERE chat_id = $1;")
            .bind(id)
            .fetch_one(&self.db)
            .await
            .map_err(sqlx_helper::get_errors)
    }

    async fn update(&self, input: Chat) -> Result<(), UpdateChatError> {
        let _ = sqlx::query(
            "UPDATE chats SET name = $1, enable_push = $2, title = $3 WHERE chat_id = $4;",
        )
        .bind(input.name)
        .bind(input.enable_push)
        .bind(input.title)
        .bind(input.chat_id)
        .execute(&self.db)
        .await
        .map_err(sqlx_helper::update_errors)?;

        Ok(())
    }

    async fn get_list_for_push(&self) -> Result<Vec<Chat>, GetChatError> {
        sqlx::query_as::<_, Chat>("SELECT * FROM chats WHERE enable_push;")
            .fetch_all(&self.db)
            .await
            .map_err(sqlx_helper::get_errors)
    }
}
