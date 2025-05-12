use crate::shared::GetPictureError;

use super::models::{PictureDto, PictureType};

pub trait GetPictures: Clone + Send + Sync + 'static {
    fn get_pictures(
        &self,
        picture_type: Option<PictureType>,
        limit: Option<u32>,
    ) -> impl std::future::Future<Output = Result<Vec<PictureDto>, GetPictureError>> + Send;
}