use cat_core::contracts::{GetPictureError, GetPictures, PictureDto, PictureType};

use super::{TheCatsApi, TheDogsApi};

#[derive(Debug, Clone)]
pub enum GetPictureEnum {
    Cat(TheCatsApi),
    Dog(TheDogsApi),
}

impl GetPictures for GetPictureEnum {
    async fn get_pictures(
        &self,
        picture_type: Option<PictureType>,
        limit: Option<u32>,
    ) -> Result<Vec<PictureDto>, GetPictureError>{
        match self {
            GetPictureEnum::Cat(api) => api.get_pictures(picture_type, limit).await,
            GetPictureEnum::Dog(api) => api.get_pictures(picture_type, limit).await,
        }
    }
}