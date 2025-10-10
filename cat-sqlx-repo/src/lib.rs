#[cfg(feature = "postgres")]
pub mod postgres;

use sqlx::Error;

use cat_core::shared::RepositoryError;

struct DbRepositoryError(RepositoryError);

impl From<DbRepositoryError> for RepositoryError {
    fn from(value: DbRepositoryError) -> Self {
        value.0
    }
}

impl From<Error> for DbRepositoryError {
    fn from(e: Error) -> Self {
        let err = match e {
            Error::Database(err) if err.constraint().is_some() => RepositoryError::Duplicate,
            Error::RowNotFound => RepositoryError::NotFound,
            _ => RepositoryError::Other(e.to_string()),
        };

        DbRepositoryError(err)
    }
}
