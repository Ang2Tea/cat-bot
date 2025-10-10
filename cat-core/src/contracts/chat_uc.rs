use crate::shared::RepositoryError;

use super::models::{ChangeChatDto, ChatDto};

pub trait ChatCreateUC: Clone + Send + Sync + 'static {
    fn create(
        &self,
        dto: ChangeChatDto,
    ) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send;
}

pub trait ChatGetUC: Clone + Send + Sync + 'static {
    fn get_by_id(
        &self,
        id: i64,
    ) -> impl std::future::Future<Output = Result<ChatDto, RepositoryError>> + Send;
    fn get_list(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<ChatDto>, RepositoryError>> + Send;
}

pub trait ChatUpdateUC: Clone + Send + Sync + 'static {
    fn change_push(
        &self,
        id: i64,
    ) -> impl std::future::Future<Output = Result<bool, RepositoryError>> + Send;
}
