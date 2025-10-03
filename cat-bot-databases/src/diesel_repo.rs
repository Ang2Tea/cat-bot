#[cfg(feature = "postgres")]
pub mod postgres;

#[cfg(feature = "postgres")]
#[path = "diesel_repo/postgres/schema.rs"]
pub mod __postgres_schema;