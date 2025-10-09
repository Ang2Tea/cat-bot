#[cfg(feature = "postgres")]
pub mod postgres;

use sqlx::Error;

use cat_core::shared::{CreateChatError, GetChatError, UpdateChatError};

fn create_errors(e: Error) -> CreateChatError {
    match e {
        Error::Database(err) if err.constraint().is_some() => CreateChatError::Duplicate,
        _ => CreateChatError::Other(e.to_string()),
    }
}

fn get_errors(e: Error) -> GetChatError {
    match e {
        Error::RowNotFound => GetChatError::NotFound,
        _ => GetChatError::Other(e.to_string()),
    }
}

fn update_errors(e: Error) -> UpdateChatError {
    match e {
        Error::RowNotFound => UpdateChatError::GetChatError(GetChatError::NotFound),
        _ => UpdateChatError::Other(e.to_string()),
    }
}
