use crate::{
    entities::{
        chat::Chat,
        repositories::{ChatRepository, Repository},
    },
    shared::{ErrorKind, Result},
};
use sqlx::SqlitePool;

fn map_result<T>(result: sqlx::Result<T>) -> Result<T> {
    match result {
        Ok(r) => Ok(r),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Result::Err(ErrorKind::NotFound),
            _ => Result::Err(ErrorKind::Other(err.to_string())),
        },
    }
}

pub struct SqlLiteChatRepository {
    pub db: SqlitePool,
}

impl SqlLiteChatRepository {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

impl Repository for SqlLiteChatRepository {
    type Model = Chat;
    type Id = i64;

    async fn create(&self, input: Self::Model) -> crate::shared::Result<()> {
        let result = sqlx::query("INSERT INTO chats (chat_id, name, title) VALUES (?, ?, ?);")
            .bind(input.chat_id)
            .bind(input.name)
            .bind(input.title)
            .execute(&self.db)
            .await;

            map_result(result.map(|_| ()))
    }

    async fn get_list(&self) -> crate::shared::Result<Vec<Self::Model>> {
        let result = sqlx::query_as::<_, Chat>("SELECT * FROM chats;")
            .fetch_all(&self.db)
            .await;

        map_result(result)
    }

    async fn get_by_id(&self, id: Self::Id) -> crate::shared::Result<Self::Model> {
        let result = sqlx::query_as::<_, Chat>("SELECT * FROM chats WHERE chat_id = ?;")
            .bind(id)
            .fetch_one(&self.db)
            .await;

        map_result(result)
    }

    async fn update(&self, input: Self::Model) -> crate::shared::Result<()> {
        let result =
            sqlx::query("UPDATE chats SET name = ?, enable_push = ?, title = ? WHERE chat_id = ?;")
                .bind(input.name)
                .bind(input.enable_push)
                .bind(input.title)
                .bind(input.chat_id)
                .execute(&self.db)
                .await;

        map_result(result.map(|_| ()))
    }
}

impl ChatRepository for SqlLiteChatRepository {
    async fn get_list_for_push(&self) -> Result<Vec<Chat>> {
        let result = sqlx::query_as::<_, Chat>("SELECT * FROM chats WHERE enable_push;")
            .fetch_all(&self.db)
            .await;

        map_result(result)
    }
}
