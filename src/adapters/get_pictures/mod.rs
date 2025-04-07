use std::{collections::HashMap, sync::Arc};

use rand::Rng;

use crate::{
    contracts::{AsyncGetPictures, GetPictures, models::PictureType},
    shared::ErrorKind,
};

pub mod the_cats_api;
pub mod the_dogs_api;

fn map_request_err<T: std::error::Error>(error: T) -> ErrorKind {
    ErrorKind::Other(error.to_string())
}

pub struct GetPicturesBox {
    apis: HashMap<PictureType, Arc<dyn GetPictures + Send + Sync + 'static>>,
}

impl GetPicturesBox {
    pub fn new(apis: HashMap<PictureType, Arc<dyn GetPictures + Send + Sync + 'static>>) -> Self {
        Self { apis }
    }

    fn get_picture_type(&self) -> PictureType {
        let mut rng = rand::rng();

        match rng.random_range(0..2) {
            0 => PictureType::Cat,
            1 => PictureType::Dog,
            _ => PictureType::Dog
        }
        
    }
}

impl AsyncGetPictures for GetPicturesBox {
    async fn get_pictures(
        &self,
        picture_type: Option<PictureType>,
        limit: Option<u32>,
    ) -> crate::shared::Result<Vec<crate::contracts::models::PictureDto>> {
        let mut limit = limit.unwrap_or(1);

        if limit <= 1 {
            let picture_type = picture_type.unwrap_or(self.get_picture_type());

            return self
                .apis
                .get(&picture_type)
                .ok_or(ErrorKind::NotFound)?
                .get_pictures(Some(picture_type), Some(limit));
        };

        while limit % self.apis.len() as u32 != 0 {
            limit += 1;
        }

        let result = self.apis
        .iter()
        .flat_map(|(_, api)| {
            api.get_pictures(picture_type, Some(limit / self.apis.len() as u32))
                .unwrap_or_else(|_| Vec::new())
        })
        .take(limit as usize)
        .collect();

        Ok(result)
    }
}
