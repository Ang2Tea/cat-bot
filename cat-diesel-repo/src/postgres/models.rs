use cat_core::entities::chat;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::chats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Chat {
    pub chat_id: i64,
    pub name: Option<String>,
    pub title: Option<String>,
    pub enable_push: bool,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::chats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewChat<'a> {
    pub chat_id: i64,
    pub name: Option<&'a str>,
    pub title: Option<&'a str>,
}

impl From<Chat> for chat::Chat {
    fn from(value: Chat) -> Self {
        Self {
            chat_id: value.chat_id,
            name: value.name,
            title: value.title,
            enable_push: value.enable_push,
        }
    }
}
