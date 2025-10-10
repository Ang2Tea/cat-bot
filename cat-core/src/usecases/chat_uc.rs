use crate::entities::{chat::Chat, repositories::ChatRepository};

use crate::contracts::{
    ChangeChatDto, ChatCreateUC, ChatDto, ChatGetUC, ChatUCError, ChatUpdateUC,
};

#[derive(Debug, Clone)]
pub struct ChatUC<R>
where
    R: ChatRepository,
{
    repository: R,
}

impl<R> ChatUC<R>
where
    R: ChatRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> ChatCreateUC for ChatUC<R>
where
    R: ChatRepository,
{
    async fn create(&self, dto: ChangeChatDto) -> Result<(), ChatUCError> {
        let new_chat = Chat::new(dto.chat_id, dto.name, dto.title);

        self.repository.create(new_chat).await?;

        Ok(())
    }
}

impl<R> ChatGetUC for ChatUC<R>
where
    R: ChatRepository,
{
    async fn get_by_id(&self, id: i64) -> Result<ChatDto, ChatUCError> {
        let result = self.repository.get_by_id(id).await.map(ChatDto::from)?;

        Ok(result)
    }

    async fn get_list(&self) -> Result<Vec<ChatDto>, ChatUCError> {
        let chats = self.repository.get_list().await?;

        Ok(chats.into_iter().map(ChatDto::from).collect())
    }
}

impl<R> ChatUpdateUC for ChatUC<R>
where
    R: ChatRepository,
{
    async fn change_push(&self, id: i64) -> Result<bool, ChatUCError> {
        let mut chat = self.repository.get_by_id(id).await?;
        let current_push = !chat.enable_push;

        chat.enable_push = current_push;

        self.repository.update(chat).await?;

        Ok(current_push)
    }
}
