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
