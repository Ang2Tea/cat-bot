use crate::{interface::GetCats, models::CatDto};

const GET_RANDOM_CATS_URL: &str = "https://api.thecatapi.com/v1/images/search";

pub struct TheCatsApi {
    api_key: String,
}

impl TheCatsApi {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl GetCats for TheCatsApi {
    async fn get_random_cats(&self) -> Option<Vec<CatDto>> {
        let client = reqwest::Client::new();

        let cats: Vec<CatDto> = client
            .get(GET_RANDOM_CATS_URL)
            .header("x-api-key", &self.api_key)
            .send()
            .await
            .ok()?
            .json()
            .await
            .ok()?;

        Some(cats)
    }
}
