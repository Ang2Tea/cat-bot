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