use crate::{
    entities::{
        chat::Chat,
        repositories::{ChatRepository, Repository},
    },
    shared::Result,
};
use sqlx::PgPool;

pub struct PostgresChatRepository {
    pub db: PgPool,
}

impl PostgresChatRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

impl Repository for PostgresChatRepository {
    type Model = Chat;
    type Id = i64;

    async fn create(&self, input: Self::Model) -> crate::shared::Result<()> {
        let result = sqlx::query("INSERT INTO chats (chat_id, name, title) VALUES (?, ?, ?);")
            .bind(input.chat_id)
            .bind(input.name)
            .bind(input.title)
            .execute(&self.db)
            .await;

        super::map_result(result.map(|_| ()))
    }

    async fn get_list(&self) -> crate::shared::Result<Vec<Self::Model>> {
        let result = sqlx::query_as::<_, Chat>("SELECT * FROM chats;")
            .fetch_all(&self.db)
            .await;

        super::map_result(result)
    }

    async fn get_by_id(&self, id: Self::Id) -> crate::shared::Result<Self::Model> {
        let result = sqlx::query_as::<_, Chat>("SELECT * FROM chats WHERE chat_id = ?;")
            .bind(id)
            .fetch_one(&self.db)
            .await;

        super::map_result(result)
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

        super::map_result(result.map(|_| ()))
    }
}

impl ChatRepository for PostgresChatRepository {
    async fn get_list_for_push(&self) -> Result<Vec<Chat>> {
        let result = sqlx::query_as::<_, Chat>("SELECT * FROM chats WHERE enable_push;")
            .fetch_all(&self.db)
            .await;

        super::map_result(result)
    }
}
