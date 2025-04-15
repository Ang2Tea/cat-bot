use crate::contracts::AsyncGetPictures;

use super::{TheCatsApi, TheDogsApi};

pub enum GetPictureEnum {
    Cat(TheCatsApi),
    Dog(TheDogsApi),
}

impl AsyncGetPictures for GetPictureEnum {
    async fn get_pictures(
        &self,
        picture_type: Option<crate::contracts::PictureType>,
        limit: Option<u32>,
    ) -> crate::shared::Result<Vec<crate::contracts::PictureDto>>{
        match self {
            GetPictureEnum::Cat(api) => api.get_pictures(picture_type, limit).await,
            GetPictureEnum::Dog(api) => api.get_pictures(picture_type, limit).await,
        }
    }
}