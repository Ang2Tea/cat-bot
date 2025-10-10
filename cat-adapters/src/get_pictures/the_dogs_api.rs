use reqwest::Url;

use cat_core::contracts::{GetPictureError, GetPictures, PictureDto, PictureType};

use crate::get_pictures::AdapterGetPictureError;

const GET_DOGS_URL: &str = "https://api.thedogapi.com/v1/images/search";

#[derive(Debug, Clone)]
pub struct TheDogsApi {
    client: reqwest::Client,
}

impl TheDogsApi {
    pub fn try_new(api_key: &str) -> Result<Self, String> {
        let client = super::get_client(api_key)?;

        Ok(Self { client })
    }
}

impl GetPictures for TheDogsApi {
    async fn get_pictures(
        &self,
        _picture_type: Option<PictureType>,
        limit: Option<u32>,
    ) -> Result<Vec<PictureDto>, GetPictureError> {
        let params = [("limit", limit.unwrap_or(1).to_string())];

        let url = Url::parse_with_params(GET_DOGS_URL, &params)
            .map_err(|_| GetPictureError::IncorrectUrl)?;

        let dogs: Vec<PictureDto> = self
            .client
            .get(url)
            .send()
            .await
            .map_err(AdapterGetPictureError::from)?
            .json()
            .await
            .map_err(AdapterGetPictureError::from)?;

        Ok(dogs)
    }
}
