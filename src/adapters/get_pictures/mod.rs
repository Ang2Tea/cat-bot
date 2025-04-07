mod composite;
mod the_cats_api;
mod the_dogs_api;

pub use composite::*;
pub use the_cats_api::*;
pub use the_dogs_api::*;

fn map_request_err<T: std::error::Error>(error: T) -> crate::shared::ErrorKind {
    crate::shared::ErrorKind::Other(error.to_string())
}
