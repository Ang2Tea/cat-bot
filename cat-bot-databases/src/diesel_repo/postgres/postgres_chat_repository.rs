use diesel::prelude::*;

use diesel_async::{
    AsyncPgConnection, RunQueryDsl,
    pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool},
};

use cat_bot_entities::{
    chat::Chat,
    repositories::ChatRepository,
    shared::{CreateChatError, GetChatError, UpdateChatError},
};

use super::{
    models,
    schema::chats::{self, *},
};

#[derive(Clone)]
pub struct PostgresChatRepository {
    pool: Pool<AsyncPgConnection>,
}

impl PostgresChatRepository {
    pub fn try_new(connection_url: String) -> Result<Self, String> {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(connection_url);
        let pool = Pool::builder(config).build().map_err(|e| e.to_string())?;

        Ok(Self { pool })
    }
}

impl ChatRepository for PostgresChatRepository {
    async fn create(&self, input: Chat) -> Result<(), CreateChatError> {
        todo!()
    }

    async fn get_list(&self) -> Result<Vec<Chat>, GetChatError> {
        let mut conn = self.pool.get().await.unwrap();

        let results = chats::table
            .select(models::Chat::as_select())
            .load(&mut conn)
            .await
            .expect("Error loading posts");

        Ok(results.into_iter().map(Chat::from).collect())
    }

    async fn get_by_id(&self, id: i64) -> Result<Chat, GetChatError> {
        let mut conn = self.pool.get().await.unwrap();

        let result = chats::table
            .filter(chat_id.eq(id))
            .select(models::Chat::as_select())
            .first(&mut conn)
            .await
            .expect("Error loading posts");

        Ok(Chat::from(result))
    }

    async fn update(&self, input: Chat) -> Result<(), UpdateChatError> {
        todo!()
    }

    async fn get_list_for_push(&self) -> Result<Vec<Chat>, GetChatError> {
        todo!()
    }
}
