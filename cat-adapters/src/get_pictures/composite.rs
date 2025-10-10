use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use cat_core::contracts::{GetPictureError, GetPictures, PictureDto, PictureType};

use super::get_picture_enum::GetPictureEnum;

#[derive(Clone)]
pub struct CompositeApi {
    apis: Arc<HashMap<PictureType, GetPictureEnum>>,
}

impl CompositeApi {
    pub fn new(apis: Arc<HashMap<PictureType, GetPictureEnum>>) -> Self {
        Self { apis }
    }
}

impl CompositeApi {
    fn get_random_picture_type(time_in_sec: u64, api_count: usize) -> PictureType {
        if api_count == 0 {
            return PictureType::Cat;
        }

        // Перевод в часы
        let hours = (time_in_sec % (24 * 3600)) / 3600;

        // Предполагаем, что у нас только два типа картинок
        match hours % 2 {
            0 => PictureType::Cat,
            1 => PictureType::Dog,
            _ => unreachable!(),
        }
    }

    fn get_random_picture_type_by_time(api_count: usize) -> PictureType {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(1))
            .as_secs();

        Self::get_random_picture_type(now, api_count)
    }
}

impl GetPictures for CompositeApi {
    async fn get_pictures(
        &self,
        picture_type: Option<PictureType>,
        limit: Option<u32>,
    ) -> Result<Vec<PictureDto>, GetPictureError> {
        let mut limit = limit.unwrap_or(1);

        if limit <= 1 {
            let picture_type = picture_type.unwrap_or(Self::get_random_picture_type_by_time(
                self.apis.iter().count(),
            ));

            return self
                .apis
                .get(&picture_type)
                .ok_or(GetPictureError::UnknownApi)?
                .get_pictures(Some(picture_type), Some(limit))
                .await;
        };

        while limit % self.apis.len() as u32 != 0 {
            limit += 1;
        }

        let mut result = Vec::new();
        let apis_len = self.apis.len() as u32;

        for (_, api) in self.apis.iter() {
            let temp_result = api
                .get_pictures(picture_type, Some(limit / apis_len))
                .await?;

            result.extend(temp_result);
        }

        let result = result.into_iter().take(limit as usize).collect();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use cat_core::contracts::PictureType;

    use super::CompositeApi;

    const API_COUNT: usize = 2;

    #[test]
    fn get_random_picture_type_test() {
        let t = CompositeApi::get_random_picture_type(3600 * 0, API_COUNT);
        assert_eq!(t, PictureType::Cat);

        let t = CompositeApi::get_random_picture_type(3600 * 1, API_COUNT);
        assert_eq!(t, PictureType::Dog);

        let t = CompositeApi::get_random_picture_type(3600 * 2, API_COUNT);
        assert_eq!(t, PictureType::Cat);
    }
}
