use crate::shared::Result;
use super::models::{PictureDto, PictureType};

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