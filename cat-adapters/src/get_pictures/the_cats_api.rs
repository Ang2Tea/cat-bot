use cat_core::contracts::{GetPictureError, GetPictures, PictureDto, PictureType};
use reqwest::Url;

use super::AdapterGetPictureError;

const GET_CATS_URL: &str = "https://api.thecatapi.com/v1/images/search";

#[derive(Debug, Clone)]
pub struct TheCatsApi {
    client: reqwest::Client,
}

impl TheCatsApi {
    pub fn try_new(api_key: &str) -> Result<Self, String> {
        let client = super::get_client(api_key)?;

        Ok(Self { client })
    }
}

impl GetPictures for TheCatsApi {
    async fn get_pictures(
        &self,
        _picture_type: Option<PictureType>,
        limit: Option<u32>,
    ) -> Result<Vec<PictureDto>, GetPictureError> {
        let params = [("limit", limit.unwrap_or(1).to_string())];

        let url = Url::parse_with_params(GET_CATS_URL, &params)
            .map_err(|_| GetPictureError::IncorrectUrl)?;

        let cats: Vec<PictureDto> = self
            .client
            .get(url)
            .send()
            .await
            .map_err(AdapterGetPictureError::from)?
            .json()
            .await
            .map_err(AdapterGetPictureError::from)?;

        Ok(cats)
    }
}
