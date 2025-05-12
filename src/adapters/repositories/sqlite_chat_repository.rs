use crate::{entities::{chat::Chat, repositories::ChatRepository as IChatRepository}, shared::{CreateChatError, GetChatError, UpdateChatError}};
use sqlx::{Sqlite, SqlitePool};

pub async fn init_db(db_urn: &str) -> std::result::Result<SqlitePool, String> {
    super::inner_init_db::<Sqlite>(db_urn, Some("./migrations/sqlite")).await
}

#[derive(Debug, Clone)]
pub struct ChatRepository {
    pub db: SqlitePool,
}

impl ChatRepository {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

impl IChatRepository for ChatRepository {
    async fn create(&self, input: Chat) -> Result<(), CreateChatError> {
        let _ = sqlx::query("INSERT INTO chats (chat_id, name, title) VALUES (?, ?, ?);")
            .bind(input.chat_id)
            .bind(input.name)
            .bind(input.title)
            .execute(&self.db)
            .await
            .map_err(super::create_errors);

            Ok(())
    }

    async fn get_list(&self) -> Result<Vec<Chat>, GetChatError> {
        let result = sqlx::query_as::<_, Chat>("SELECT * FROM chats;")
            .fetch_all(&self.db)
            .await
            .map_err(super::get_errors);

        result
    }

    async fn get_by_id(&self, id: i64) -> Result<Chat, GetChatError> {
        let result = sqlx::query_as::<_, Chat>("SELECT * FROM chats WHERE chat_id = ?;")
            .bind(id)
            .fetch_one(&self.db)
            .await
            .map_err(super::get_errors);

        result
    }

    async fn update(&self, input: Chat) -> Result<(), UpdateChatError> {
        let _ =
            sqlx::query("UPDATE chats SET name = ?, enable_push = ?, title = ? WHERE chat_id = ?;")
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
        let result = sqlx::query_as::<_, Chat>("SELECT * FROM chats WHERE enable_push;")
            .fetch_all(&self.db)
            .await
            .map_err(super::get_errors);

        result
    }
}