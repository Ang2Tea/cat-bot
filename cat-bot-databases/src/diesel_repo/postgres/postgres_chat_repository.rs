use cat_bot_entities::{
    chat::Chat,
    repositories::ChatRepository,
    shared::{CreateChatError, GetChatError, UpdateChatError},
};

#[derive(Clone)]
pub struct PostgresChatRepository {}

impl PostgresChatRepository {
    pub fn try_new(database_url: String) -> Result<Self, String> {
        Ok(Self {})
    }
}

impl ChatRepository for PostgresChatRepository {
    async fn create(&self, input: Chat) -> Result<(), CreateChatError> {
        todo!()
    }

    async fn get_list(&self) -> Result<Vec<Chat>, GetChatError> {
        todo!()
    }

    async fn get_by_id(&self, id: i64) -> Result<Chat, GetChatError> {
        todo!()
    }

    async fn update(&self, input: Chat) -> Result<(), UpdateChatError> {
        todo!()
    }

    async fn get_list_for_push(&self) -> Result<Vec<Chat>, GetChatError> {
        todo!()
    }
}
