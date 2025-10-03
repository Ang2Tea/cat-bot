mod chat_repository;

pub use chat_repository::*;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use cat_bot_entities::chat::Chat;

pub async fn init_db() -> std::result::Result<Arc<RwLock<HashMap<i64, Chat>>>, String> {
    Ok(Arc::new(RwLock::new(HashMap::new())))
}
