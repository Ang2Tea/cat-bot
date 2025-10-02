use crate::shared::GetPictureError;

use super::models::{ChatDto, PictureType};

pub trait PictureGetUC: Clone + Send + Sync + 'static {
    fn get_picture(&self, picture_type: Option<PictureType>) -> impl std::future::Future<Output = Result<String, GetPictureError>> + Send;
    fn get_picture_for_notification(&self) -> impl std::future::Future<Output = Result<Vec<(String, ChatDto)>, GetPictureError>> + Send;
}