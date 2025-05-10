use crate::contracts::GetPictures;

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
    ) -> crate::shared::Result<Vec<crate::contracts::PictureDto>>{
        match self {
            GetPictureEnum::Cat(api) => api.get_pictures(picture_type, limit).await,
            GetPictureEnum::Dog(api) => api.get_pictures(picture_type, limit).await,
        }
    }
}