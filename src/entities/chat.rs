#[derive(Debug, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Chat {
    pub chat_id: i64,
    pub name: Option<String>,
    pub title: Option<String>,
    pub enable_push: bool,
}

impl Chat {
    pub fn new(chat_id: i64, name: Option<String>, title: Option<String>) -> Self {
        Self {
            chat_id,
            name,
            title,
            enable_push: false,
        }
    }
}