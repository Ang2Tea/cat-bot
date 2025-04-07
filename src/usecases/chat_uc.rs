use std::sync::Arc;

use crate::{
    contracts::{
        ChatCreateUC, ChatGetUC, ChatUpdateUC,
        models::{ChangeChatDto, ChatDto},
    },
    entities::{chat::Chat, repositories::ChatRepository},
};

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
    R: ChatRepository + Send + Sync + 'static,
{
    async fn create(&self, dto: ChangeChatDto) -> crate::shared::Result<()> {
        let new_chat = Chat::new(dto.chat_id, dto.name, dto.title);

        self.repository.create(new_chat).await
    }
}

impl<R> ChatGetUC for ChatUC<R>
where
    R: ChatRepository + Send + Sync + 'static,
{
    async fn get_by_id(&self, id: i64) -> crate::shared::Result<ChatDto> {
        let chat = self.repository.get_by_id(id).await.map(|chat| ChatDto {
            chat_id: chat.chat_id,
            enable_push: chat.enable_push,
        });

        chat
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
    R: ChatRepository + Send + Sync + 'static,
{
    async fn update(&self, dto: ChangeChatDto) -> crate::shared::Result<()> {
        let mut chat = self.repository.get_by_id(dto.chat_id).await?;

        chat.enable_push = dto.enable_push;
        chat.name = dto.name;
        chat.title = dto.title;

        self.repository.update(chat).await
    }
}
