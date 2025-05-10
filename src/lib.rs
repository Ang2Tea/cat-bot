pub mod shared;
pub mod entities;
pub mod contracts;
pub mod usecases;
pub mod adapters;
pub mod configs;

#[cfg(all(feature = "postgres", feature = "sqlite"))]
compile_error!("Нельзя включить одновременно postgres и sqlite");

#[cfg(not(any(feature = "postgres", feature = "sqlite")))]
compile_error!("Необходимо включить либо postgres, либо sqlite");