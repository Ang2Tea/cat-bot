mod composite;
mod get_picture_enum;
mod the_cats_api;
mod the_dogs_api;

pub use composite::*;
pub use get_picture_enum::*;
pub use the_cats_api::*;
pub use the_dogs_api::*;

use cat_core::contracts::GetPictureError;
use reqwest::{Error, StatusCode, header};

fn get_client(token: &str) -> Result<reqwest::Client, String> {
    let header_value = header::HeaderValue::from_str(token).map_err(|e| e.to_string())?;

    let mut headers = header::HeaderMap::new();
    headers.insert("x-api-key", header_value);

    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| e.to_string())
}

struct AdapterGetPictureError(GetPictureError);

impl From<AdapterGetPictureError> for GetPictureError {
    fn from(value: AdapterGetPictureError) -> Self {
        value.0
    }
}

impl From<Error> for AdapterGetPictureError {
    fn from(value: Error) -> Self {
        let err = {
            if value.is_decode() {
                GetPictureError::DecodeError(value.to_string());
            }

            if value.is_status() {
                GetPictureError::InvalidStatus {
                    status: value.status().unwrap_or(StatusCode::BAD_REQUEST).as_u16(),
                    message: value.to_string(),
                };
            }

            if value.is_request() {
                GetPictureError::RequestError(value.to_string());
            }

            GetPictureError::Other(value.to_string())
        };

        AdapterGetPictureError(err)
    }
}
