pub mod models;
use crate::shared::Result;

use models::{ChangeChatDto, ChatDto, PictureDto, PictureType};

pub trait AsyncGetPictures {
    fn get_pictures(
        &self,
        picture_type: Option<PictureType>,
        limit: Option<u32>,
    ) -> impl std::future::Future<Output = Result<Vec<PictureDto>>> + Send;
}

pub trait GetPictures {
    fn get_pictures(
        &self,
        picture_type: Option<PictureType>,
        limit: Option<u32>,
    ) -> Result<Vec<PictureDto>>;
}


pub trait PictureGetUC {
    fn get_picture(&self, picture_type: Option<PictureType>) -> impl std::future::Future<Output = Result<String>> + Send;
    fn get_picture_for_notification(&self) -> impl std::future::Future<Output = Result<Vec<(String, ChatDto)>>> + Send;
}

pub trait ChatCreateUC {
    fn create(&self, dto: ChangeChatDto) -> impl std::future::Future<Output = Result<()>> + Send;
}

pub trait ChatGetUC {
    fn get_by_id(&self, id: i64) -> impl std::future::Future<Output = Result<ChatDto>> + Send;
    fn get_list(&self) -> impl std::future::Future<Output = Result<Vec<ChatDto>>> + Send;
}

pub trait ChatUpdateUC {
    fn update(&self, dto: ChangeChatDto) ->impl std::future::Future<Output = Result<()>> + Send;
}
