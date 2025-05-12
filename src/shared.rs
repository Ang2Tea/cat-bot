use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateChatError {
    #[error("сan't create chat/s")]
    Duplicate,
    #[error("{0}")]
    Other(String),
}

#[derive(Debug, Error)]
pub enum GetChatError {
    #[error("сan't get chat/s")]
    NotFound,
    #[error("{0}")]
    Other(String),
}

#[derive(Debug, Error)]
pub enum UpdateChatError {
    #[error(transparent)]
    GetChatError(#[from] GetChatError),
    #[error("{0}")]
    Other(String),
}

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
    GetChatError(#[from] GetChatError),
    #[error("сan't get picture/s")]
    NotFound,
    #[error("{0}")]
    Other(String),
}
