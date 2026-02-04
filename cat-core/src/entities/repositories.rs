use crate::shared::RepositoryError;

use super::chat::Chat;

pub trait ChatRepository: Clone + Send + Sync + 'static {
    fn create(&self, input: Chat) -> impl Future<Output = Result<(), RepositoryError>> + Send;
    fn get_list(&self) -> impl Future<Output = Result<Vec<Chat>, RepositoryError>> + Send;
    fn get_by_id(&self, id: i64) -> impl Future<Output = Result<Chat, RepositoryError>> + Send;
    fn update(&self, input: Chat) -> impl Future<Output = Result<(), RepositoryError>> + Send;

    fn get_list_for_push(&self) -> impl Future<Output = Result<Vec<Chat>, RepositoryError>> + Send;
}
