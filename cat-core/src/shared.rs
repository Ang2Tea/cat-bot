use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("сan't get chat/s")]
    NotFound,
    #[error("сan't create chat/s")]
    Duplicate,
    #[error("сan't connect to database")]
    Connection,
    #[error("{0}")]
    Other(String),
}
