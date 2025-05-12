use crate::{contracts::{GetPictures, PictureDto}, shared::GetPictureError};

use super::{TheCatsApi, TheDogsApi};

#[derive(Debug, Clone)]
pub enum GetPictureEnum {
    Cat(TheCatsApi),
    Dog(TheDogsApi),
}

impl GetPictures for GetPictureEnum {
    async fn get_pictures(
        &self,
        picture_type: Option<crate::contracts::PictureType>,
        limit: Option<u32>,
    ) -> Result<Vec<PictureDto>, GetPictureError>{
        match self {
            GetPictureEnum::Cat(api) => api.get_pictures(picture_type, limit).await,
            GetPictureEnum::Dog(api) => api.get_pictures(picture_type, limit).await,
        }
    }
}