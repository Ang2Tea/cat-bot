mod composite;
mod get_picture_enum;
mod the_cats_api;
mod the_dogs_api;

pub use composite::*;
pub use get_picture_enum::*;
pub use the_cats_api::*;
pub use the_dogs_api::*;

use reqwest::{Error, StatusCode};

use crate::shared::GetPictureError;

fn get_errors(e: Error) -> GetPictureError {
    if e.is_decode() {
        return GetPictureError::DecodeError(e.to_string());
    }

    if e.is_status() {
        return GetPictureError::InvalidStatus {
            status: e.status().unwrap_or(StatusCode::BAD_REQUEST).as_u16(),
            message: e.to_string(),
        };
    }
    
    if e.is_request() {
        return  GetPictureError::RequestError(e.to_string());
    }

    GetPictureError::Other(e.to_string())
}
