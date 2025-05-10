use std::sync::Arc;

use crate::{
    contracts::{ChangeChatDto, ChatCreateUC, ChatDto, ChatGetUC, ChatUpdateUC},
    entities::{chat::Chat, repositories::ChatRepository},
};

#[derive(Debug, Clone)]
pub struct ChatUC<R>
where
    R: ChatRepository,
{
    repository: Arc<R>,
}

impl<R> ChatUC<R>
where
    R: ChatRepository,
{
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

impl<R> ChatCreateUC for ChatUC<R>
where
    R: ChatRepository,
{
    async fn create(&self, dto: ChangeChatDto) -> crate::shared::Result<()> {
        let new_chat = Chat::new(dto.chat_id, dto.name, dto.title);

        self.repository.create(new_chat).await
    }
}

impl<R> ChatGetUC for ChatUC<R>
where
    R: ChatRepository,
{
    async fn get_by_id(&self, id: i64) -> crate::shared::Result<ChatDto> {
        self.repository.get_by_id(id).await.map(|chat| ChatDto {
            chat_id: chat.chat_id,
            enable_push: chat.enable_push,
        })
    }

    async fn get_list(&self) -> crate::shared::Result<Vec<ChatDto>> {
        let chats = self.repository.get_list().await.map(|chats| {
            chats
                .iter()
                .map(|chat| ChatDto {
                    chat_id: chat.chat_id,
                    enable_push: chat.enable_push,
                })
                .collect()
        });

        chats
    }
}

impl<R> ChatUpdateUC for ChatUC<R>
where
    R: ChatRepository,
{
    async fn change_push(&self, id: i64) -> crate::shared::Result<bool> {
        let mut chat = self.repository.get_by_id(id).await?;
        let current_push = !chat.enable_push;

        chat.enable_push = current_push;

        self.repository.update(chat).await?;

        Ok(current_push)
    }
}
