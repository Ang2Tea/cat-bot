use reqwest::Url;

use crate::{
    contracts::{GetPictures, PictureDto},
    shared::GetPictureError,
};

use super::get_errors;

const GET_CATS_URL: &str = "https://api.thecatapi.com/v1/images/search";

#[derive(Debug, Clone)]
pub struct TheCatsApi {
    api_key: String,
}

impl TheCatsApi {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl GetPictures for TheCatsApi {
    async fn get_pictures(
        &self,
        _picture_type: Option<crate::contracts::PictureType>,
        limit: Option<u32>,
    ) -> Result<Vec<PictureDto>, GetPictureError> {
        let params = [("limit", limit.unwrap_or(1).to_string())];

        let url = Url::parse_with_params(GET_CATS_URL, &params)
            .map_err(|_| GetPictureError::IncorrectUrl)?;

        let client = reqwest::Client::new();

        let cats: Vec<PictureDto> = client
            .get(url)
            .header("x-api-key", &self.api_key)
            .send()
            .await
            .map_err(get_errors)?
            .json()
            .await
            .map_err(get_errors)?;

        Ok(cats)
    }
}
