use crate::entities::repositories::ChatRepository;

use crate::contracts::{
    ChatDto, GetPictureError, GetPictures, PictureGetUC, PictureType, PictureUCError,
};

#[derive(Debug, Clone)]
pub struct PictureUC<A, R>
where
    A: GetPictures,
    R: ChatRepository,
{
    chat_repository: R,
    get_pictures: A,
}

impl<A, R> PictureUC<A, R>
where
    A: GetPictures,
    R: ChatRepository,
{
    pub fn new(get_pictures: A, chat_repository: R) -> Self {
        Self {
            get_pictures,
            chat_repository,
        }
    }
}

impl<A, R> PictureGetUC for PictureUC<A, R>
where
    A: GetPictures + Send + Sync + 'static,
    R: ChatRepository + Send + Sync + 'static,
{
    async fn get_picture(
        &self,
        picture_type: Option<PictureType>,
    ) -> Result<String, PictureUCError> {
        let pictures = self
            .get_pictures
            .get_pictures(picture_type, Some(1))
            .await?;

        let first = pictures.first().ok_or(GetPictureError::NotFound)?;

        Ok(first.url.clone())
    }

    async fn get_picture_for_notification(&self) -> Result<Vec<(String, ChatDto)>, PictureUCError> {
        let chats = self.chat_repository.get_list_for_push().await?;

        let pictures = self
            .get_pictures
            .get_pictures(None, Some(chats.len() as u32))
            .await?;

        let result = chats
            .iter()
            .zip(pictures.iter())
            .map(|(chat, picture)| {
                let chat_dto = ChatDto {
                    chat_id: chat.chat_id,
                    enable_push: chat.enable_push,
                };
                (picture.url.clone(), chat_dto)
            })
            .collect();

        Ok(result)
    }
}
