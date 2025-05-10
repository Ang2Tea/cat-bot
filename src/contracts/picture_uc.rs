use crate::shared::Result;
use super::models::{ChatDto, PictureType};

pub trait PictureGetUC: Clone + Send + Sync + 'static {
    fn get_picture(&self, picture_type: Option<PictureType>) -> impl std::future::Future<Output = Result<String>> + Send;
    fn get_picture_for_notification(&self) -> impl std::future::Future<Output = Result<Vec<(String, ChatDto)>>> + Send;
}