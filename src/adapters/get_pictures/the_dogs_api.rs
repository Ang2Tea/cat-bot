use reqwest::Url;

use crate::contracts::{models::PictureDto, AsyncGetPictures, GetPictures};

use super::map_request_err;

const GET_DOGS_URL: &str = "https://api.thedogapi.com/v1/images/search";

pub struct TheDogsApi {
    api_key: String,
}

impl TheDogsApi {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl AsyncGetPictures for TheDogsApi {
    async fn get_pictures(
        &self,
        _picture_type: Option<crate::contracts::models::PictureType>,
        limit: Option<u32>,
    ) -> crate::shared::Result<Vec<crate::contracts::models::PictureDto>> {
        let params = [("limit", limit.unwrap_or(1).to_string())];

        let url = Url::parse_with_params(GET_DOGS_URL, &params).map_err(map_request_err)?;

        let client = reqwest::Client::new();

        let dogs: Vec<PictureDto> = client
            .get(url)
            .header("x-api-key", &self.api_key)
            .send()
            .await
            .map_err(map_request_err)?
            .json()
            .await
            .map_err(map_request_err)?;

        Ok(dogs)
    }
}

impl GetPictures for TheDogsApi {
    fn get_pictures(
        &self,
        _picture_type: Option<crate::contracts::models::PictureType>,
        limit: Option<u32>,
    ) -> crate::shared::Result<Vec<crate::contracts::models::PictureDto>> {
        let params = [("limit", limit.unwrap_or(1).to_string())];

        let url = Url::parse_with_params(GET_DOGS_URL, &params).map_err(map_request_err)?;

        let client = reqwest::blocking::Client::new();

        let dogs= client
            .get(url)
            .header("x-api-key", &self.api_key)
            .send()
            .map_err(map_request_err)?
            .json()
            .map_err(map_request_err)?;

        Ok(dogs)
    }
}
