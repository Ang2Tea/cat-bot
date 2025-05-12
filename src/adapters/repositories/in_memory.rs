mod chat_repository;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub use chat_repository::*;

use crate::entities::chat::Chat;

#[allow(unused_variables)]
pub async fn init_db(db_urn: &str) -> std::result::Result<Arc<RwLock<HashMap<i64, Chat>>>, String> {
    Ok(Arc::new(RwLock::new(HashMap::new())))
}
