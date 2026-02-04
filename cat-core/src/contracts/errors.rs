use crate::shared::RepositoryError;
use thiserror::Error;

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
    #[error("сan't get picture/s")]
    NotFound,
    #[error("{0}")]
    Other(String),
}

#[derive(Debug, Error)]
pub enum ChatUCError {
    #[error(transparent)]
    RepositoryError(#[from] RepositoryError),
}

#[derive(Debug, Error)]
pub enum PictureUCError {
    #[error(transparent)]
    RepositoryError(#[from] RepositoryError),
    #[error(transparent)]
    GetPictureError(#[from] GetPictureError),
}
