use std::{collections::HashMap, sync::Arc};

use rand::Rng;

use crate::{
    contracts::{AsyncGetPictures, GetPictures, PictureType},
    shared::ErrorKind,
};

pub struct CompositeApi {
    apis: HashMap<PictureType, Arc<dyn GetPictures + Send + Sync + 'static>>,
}

impl CompositeApi {
    pub fn new(apis: HashMap<PictureType, Arc<dyn GetPictures + Send + Sync + 'static>>) -> Self {
        Self { apis }
    }

    fn get_random_picture_type(&self) -> PictureType {
        let mut rng = rand::rng();

        match rng.random_range(0..2) {
            0 => PictureType::Cat,
            1 => PictureType::Dog,
            _ => PictureType::Dog,
        }
    }
}

impl AsyncGetPictures for CompositeApi {
    async fn get_pictures(
        &self,
        picture_type: Option<PictureType>,
        limit: Option<u32>,
    ) -> crate::shared::Result<Vec<crate::contracts::PictureDto>> {
        let mut limit = limit.unwrap_or(1);

        if limit <= 1 {
            let picture_type = picture_type.unwrap_or(self.get_random_picture_type());

            return self
                .apis
                .get(&picture_type)
                .ok_or(ErrorKind::NotFound)?
                .get_pictures(Some(picture_type), Some(limit));
        };

        while limit % self.apis.len() as u32 != 0 {
            limit += 1;
        }

        let result = self
            .apis
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
