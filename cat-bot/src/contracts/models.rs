use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum PictureType {
    Cat = 1,
    Dog = 2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PictureDto {
    pub url: String,
}

pub struct ChangeChatDto {
    pub chat_id: i64,
    pub name: Option<String>,
    pub title: Option<String>,
    pub enable_push: bool,
}

pub struct ChatDto {
    pub chat_id: i64,
    pub enable_push: bool,
}
