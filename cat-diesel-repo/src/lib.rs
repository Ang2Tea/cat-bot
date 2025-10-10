use cat_core::shared::RepositoryError;
use diesel::result::{DatabaseErrorKind, Error};

#[cfg(feature = "postgres")]
pub mod postgres;

struct DbRepositoryError(RepositoryError);

impl From<DbRepositoryError> for RepositoryError {
    fn from(value: DbRepositoryError) -> Self {
        value.0
    }
}

impl From<Error> for DbRepositoryError {
    fn from(value: Error) -> Self {
        let err = match value {
            Error::NotFound => RepositoryError::Connection,
            Error::DatabaseError(e, _) => match e {
                DatabaseErrorKind::UniqueViolation => RepositoryError::Duplicate,
                DatabaseErrorKind::ClosedConnection => RepositoryError::Connection,
                _ => RepositoryError::Other(value.to_string()),
            },
            _ => RepositoryError::Other(value.to_string()),
        };

        DbRepositoryError(err)
    }
}
