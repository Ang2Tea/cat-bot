use thiserror::Error;
use crate::shared::RepositoryError;

#[derive(Debug, Error)]
pub enum GetPictureError {
    #[error("decode error: {0}")]
    DecodeError(String),
    #[error("request error: {0}")]
    RequestError(String),
    #[error("Status {status}; Message: {message}")]
    InvalidStatus { status: u16, message: String },

    #[error("unknown api type")]
    UnknownApi,
    #[error("incorrect url")]
    IncorrectUrl,
    #[error(transparent)]
    RepositoryError(#[from] RepositoryError),
    #[error("сan't get picture/s")]
    NotFound,
    #[error("{0}")]
    Other(String),
}
