use crate::{
    entities::{chat::Chat, repositories::ChatRepository as IChatRepository},
    shared::{CreateChatError, GetChatError, UpdateChatError},
};
use sqlx::{PgPool, Postgres};

pub async fn init_db(db_urn: &str) -> std::result::Result<PgPool, String> {
    super::inner_init_db::<Postgres>(db_urn, Some("./migrations/postgres")).await
}

#[derive(Debug, Clone)]
pub struct ChatRepository {
    pub db: PgPool,
}

impl ChatRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

impl IChatRepository for ChatRepository {
    async fn create(&self, input: Chat) -> Result<(), CreateChatError> {
        let t = sqlx::query("INSERT INTO chats (chat_id, name, title) VALUES ($1, $2, $3);")
            .bind(input.chat_id)
            .bind(input.name)
            .bind(input.title)
            .execute(&self.db)
            .await;

        let _ = t.map_err(super::create_errors)?;

        Ok(())
    }

    async fn get_list(&self) -> Result<Vec<Chat>, GetChatError> {
        sqlx::query_as::<_, Chat>("SELECT * FROM chats;")
            .fetch_all(&self.db)
            .await
            .map_err(super::get_errors)
    }

    async fn get_by_id(&self, id: i64) -> Result<Chat, GetChatError> {
        sqlx::query_as::<_, Chat>("SELECT * FROM chats WHERE chat_id = $1;")
            .bind(id)
            .fetch_one(&self.db)
            .await
            .map_err(super::get_errors)
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
        .map_err(super::update_errors)?;

        Ok(())
    }

    async fn get_list_for_push(&self) -> Result<Vec<Chat>, GetChatError> {
        sqlx::query_as::<_, Chat>("SELECT * FROM chats WHERE enable_push;")
            .fetch_all(&self.db)
            .await
            .map_err(super::get_errors)
    }
}
