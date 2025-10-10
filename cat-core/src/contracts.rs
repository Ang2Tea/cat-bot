mod chat_uc;
mod errors;
mod get_picture;
mod models;
mod picture_uc;

pub use chat_uc::*;
pub use errors::*;
pub use get_picture::*;
pub use models::*;
pub use picture_uc::*;

use crate::entities::chat::Chat;

impl From<Chat> for ChatDto {
    fn from(value: Chat) -> Self {
        ChatDto {
            chat_id: value.chat_id,
            enable_push: value.enable_push,
        }
    }
}
