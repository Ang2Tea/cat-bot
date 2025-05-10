use super::chat::Chat;
use crate::shared::Result;

pub trait Repository: Clone + Send + Sync + 'static {
    type Model;
    type Id;

    fn create(&self, input: Self::Model) -> impl Future<Output = Result<()>> + Send;
    fn get_list(&self) -> impl Future<Output = Result<Vec<Self::Model>>> + Send;
    fn get_by_id(&self, id: Self::Id) -> impl Future<Output = Result<Self::Model>> + Send;
    fn update(&self, input: Self::Model) -> impl Future<Output =  Result<()>> + Send;
}

pub trait ChatRepository: Repository<Model = Chat, Id = i64> {
    fn get_list_for_push(&self) -> impl Future<Output = Result<Vec<Chat>>> + Send;
}
