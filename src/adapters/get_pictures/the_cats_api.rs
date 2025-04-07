use reqwest::Url;

use crate::contracts::{AsyncGetPictures, GetPictures, models::PictureDto};

use super::map_request_err;

const GET_CATS_URL: &str = "https://api.thecatapi.com/v1/images/search";

pub struct TheCatsApi {
    api_key: String,
}

impl TheCatsApi {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl AsyncGetPictures for TheCatsApi {
    async fn get_pictures(
        &self,
        _picture_type: Option<crate::contracts::models::PictureType>,
        limit: Option<u32>,
    ) -> crate::shared::Result<Vec<crate::contracts::models::PictureDto>> {
        let params = [("limit", limit.unwrap_or(1).to_string())];

        let url = Url::parse_with_params(GET_CATS_URL, &params).map_err(map_request_err)?;

        let client = reqwest::Client::new();

        let cats: Vec<PictureDto> = client
            .get(url)
            .header("x-api-key", &self.api_key)
            .send()
            .await
            .map_err(map_request_err)?
            .json()
            .await
            .map_err(map_request_err)?;

        Ok(cats)
    }
}

impl GetPictures for TheCatsApi {
    fn get_pictures(
        &self,
        _picture_type: Option<crate::contracts::models::PictureType>,
        limit: Option<u32>,
    ) -> crate::shared::Result<Vec<crate::contracts::models::PictureDto>> {
        let params = [("limit", limit.unwrap_or(1).to_string())];

        let url = Url::parse_with_params(GET_CATS_URL, &params).map_err(map_request_err)?;

        let client = reqwest::blocking::Client::new();

        let cats: Vec<PictureDto> = client
            .get(url)
            .header("x-api-key", &self.api_key)
            .send()
            .map_err(map_request_err)?
            .json()
            .map_err(map_request_err)?;

        Ok(cats)
    }
}
