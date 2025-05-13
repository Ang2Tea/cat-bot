use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    entities::{chat::Chat, repositories::ChatRepository as IChatRepository},
    shared::{CreateChatError, GetChatError, UpdateChatError},
};

pub struct ChatRepository {
    data: Arc<RwLock<HashMap<i64, Chat>>>,
}

impl ChatRepository {
    pub fn new(db: Arc<RwLock<HashMap<i64, Chat>>>) -> Self {
        Self {
            data: db,
        }
    }
}

impl IChatRepository for ChatRepository {
    async fn create(&self, input: Chat) -> Result<(), CreateChatError> {
        let mut data = self.data.write().await;

        if data.contains_key(&input.chat_id) {
            return Err(CreateChatError::Duplicate);
        }

        data.insert(input.chat_id, input);
        Ok(())
    }

    async fn get_list(&self) -> Result<Vec<Chat>, GetChatError> {
        let data = self.data.read().await;
        Ok(data.values().cloned().collect())
    }

    async fn get_by_id(&self, id: i64) -> Result<Chat, GetChatError> {
        let data = self.data.read().await;
        data.get(&id).cloned().ok_or(GetChatError::NotFound)
    }

    async fn update(&self, input: Chat) -> Result<(), UpdateChatError> {
        let mut data = self.data.write().await;

        if !data.contains_key(&input.chat_id) {
            return Err(UpdateChatError::GetChatError(GetChatError::NotFound));
        }

        data.insert(input.chat_id, input);
        Ok(())
    }

    async fn get_list_for_push(&self) -> Result<Vec<Chat>, GetChatError> {
        let data = self.data.read().await;
        Ok(data.values().filter(|x| x.enable_push).cloned().collect())
    }
}

impl Clone for ChatRepository {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
        }
    }
}
