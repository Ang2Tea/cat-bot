use diesel::prelude::*;

use diesel_async::RunQueryDsl;

use cat_core::{
    entities::{chat::Chat, repositories::ChatRepository},
    shared::RepositoryError,
};

use crate::postgres::models::NewChat;

use super::PostgresRepository;

use super::{
    models,
    schema::chats::{self, *},
};

impl ChatRepository for PostgresRepository {
    async fn create(&self, input: Chat) -> Result<(), RepositoryError> {
        let mut conn = self.pool.get().await.unwrap();

        let new_chat = NewChat {
            chat_id: input.chat_id,
            name: input.name.as_deref(),
            title: input.title.as_deref(),
        };

        diesel::insert_into(chats::table)
            .values(&new_chat)
            .execute(&mut conn)
            .await
            .unwrap();

        Ok(())
    }

    async fn get_list(&self) -> Result<Vec<Chat>, RepositoryError> {
        let mut conn = self.pool.get().await.unwrap();

        let results = chats::table
            .select(models::Chat::as_select())
            .load(&mut conn)
            .await
            .unwrap();

        Ok(results.into_iter().map(Chat::from).collect())
    }

    async fn get_by_id(&self, id: i64) -> Result<Chat, RepositoryError> {
        let mut conn = self.pool.get().await.unwrap();

        let result = chats::table
            .filter(chat_id.eq(id))
            .select(models::Chat::as_select())
            .first(&mut conn)
            .await
            .unwrap();

        Ok(Chat::from(result))
    }

    async fn update(&self, input: Chat) -> Result<(), RepositoryError> {
        let mut conn = self.pool.get().await.unwrap();

        diesel::update(chats::dsl::chats.find(input.chat_id))
            .set((
                name.eq(input.name),
                title.eq(input.title),
                enable_push.eq(input.enable_push),
            ))
            .execute(&mut conn)
            .await
            .unwrap();

        Ok(())
    }

    async fn get_list_for_push(&self) -> Result<Vec<Chat>, RepositoryError> {
        let mut conn = self.pool.get().await.unwrap();

        let result = chats::table
            .filter(enable_push.eq(true))
            .select(models::Chat::as_select())
            .load(&mut conn)
            .await
            .unwrap();

        Ok(result.into_iter().map(Chat::from).collect())
    }
}
