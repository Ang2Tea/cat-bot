use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow)]
pub struct ChatDto {
    pub chat_id: i64,
    pub name: Option<String>,
    pub enable_push: bool,
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatDto {
    pub url: String,
}
