use reqwest::Url;

use crate::{
    contracts::{GetPictures, PictureDto},
    shared::GetPictureError,
};

use super::get_errors;

const GET_DOGS_URL: &str = "https://api.thedogapi.com/v1/images/search";

#[derive(Debug, Clone)]
pub struct TheDogsApi {
    api_key: String,
}

impl TheDogsApi {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl GetPictures for TheDogsApi {
    async fn get_pictures(
        &self,
        _picture_type: Option<crate::contracts::PictureType>,
        limit: Option<u32>,
    ) -> Result<Vec<crate::contracts::PictureDto>, GetPictureError> {
        let params = [("limit", limit.unwrap_or(1).to_string())];

        let url = Url::parse_with_params(GET_DOGS_URL, &params)
            .map_err(|_| GetPictureError::IncorrectUrl)?;

        let client = reqwest::Client::new();

        let dogs: Vec<PictureDto> = client
            .get(url)
            .header("x-api-key", &self.api_key)
            .send()
            .await
            .map_err(get_errors)?
            .json()
            .await
            .map_err(get_errors)?;

        Ok(dogs)
    }
}
